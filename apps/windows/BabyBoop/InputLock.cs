using System.Runtime.InteropServices;
using Microsoft.Win32;
using System.Windows;
using System.Windows.Input;
using System.Windows.Threading;
using BabyBoop.Core;
using static BabyBoop.Native;

namespace BabyBoop;

public sealed class InputLock
{
    private const int VK_SHIFT = 0x10, VK_MENU = 0x12, VK_LSHIFT = 0xA0, VK_RSHIFT = 0xA1, VK_LMENU = 0xA4, VK_RMENU = 0xA5;
    private const int WheelStep = 120;

    private readonly Window _window;
    private readonly Action<string> _js;
    private readonly HookProc _keyProc, _mouseProc;
    private readonly ExitHold _exit = new(() => DateTime.UtcNow);
    private readonly HashSet<uint> _down = new();
    private readonly DispatcherTimer _timer = new() { Interval = TimeSpan.FromMilliseconds(100) };
    private readonly Dictionary<int, int> _seen = new();
    private readonly object _gate = new();
    private IntPtr _keyHook, _mouseHook;
    private Thread? _hookThread;
    private uint _hookThreadId;
    private POINT _park;
    private double _scale = 1, _x, _y, _wheel, _lastProgress = -1;

    public bool Locked { get; private set; }
    public event Action? Stopped;

    public InputLock(Window window, Action<string> js)
    {
        _window = window;
        _js = js;
        _keyProc = OnKey;
        _mouseProc = OnMouse;
        _timer.Tick += (_, _) => Tick();
        _window.Activated += (_, _) => ForgetHeldKeys("window activated");
        SystemEvents.SessionSwitch += (_, _) => _window.Dispatcher.BeginInvoke(() => ForgetHeldKeys("session switch"));
    }

    private void ForgetHeldKeys(string why)
    {
        if (!Locked) return;
        lock (_gate) { _down.Clear(); _exit.Reset(); }
        Log.Write($"held keys cleared ({why})");
    }

    private bool InstallHooks()
    {
        var ok = false;
        using var ready = new ManualResetEventSlim();
        _hookThread = new Thread(() =>
        {
            _hookThreadId = GetCurrentThreadId();
            var module = GetModuleHandle(null);
            _keyHook = SetWindowsHookEx(WH_KEYBOARD_LL, _keyProc, module, 0);
            if (_keyHook == IntPtr.Zero) Log.Write($"keyboard hook FAILED error={Marshal.GetLastWin32Error()}");
            _mouseHook = SetWindowsHookEx(WH_MOUSE_LL, _mouseProc, module, 0);
            if (_mouseHook == IntPtr.Zero) Log.Write($"mouse hook FAILED error={Marshal.GetLastWin32Error()}");
            ok = _keyHook != IntPtr.Zero && _mouseHook != IntPtr.Zero;
            ready.Set();
            if (ok) while (GetMessage(out _, IntPtr.Zero, 0, 0) > 0) { }
            if (_keyHook != IntPtr.Zero) UnhookWindowsHookEx(_keyHook);
            if (_mouseHook != IntPtr.Zero) UnhookWindowsHookEx(_mouseHook);
            _keyHook = _mouseHook = IntPtr.Zero;
        }) { IsBackground = true, Name = "BabyBoop input hooks" };
        _hookThread.Start();
        ready.Wait();
        return ok;
    }

    private void RemoveHooks()
    {
        if (_hookThread == null) return;
        PostThreadMessage(_hookThreadId, WM_QUIT, IntPtr.Zero, IntPtr.Zero);
        _hookThread.Join(1000);
        _hookThread = null;
    }

    public void Start()
    {
        if (Locked) return;
        _window.WindowState = WindowState.Normal;
        _window.WindowStyle = WindowStyle.None;
        _window.ResizeMode = ResizeMode.NoResize;
        _window.Topmost = true;
        _window.Left = 0;
        _window.Top = 0;
        _window.Width = SystemParameters.PrimaryScreenWidth;
        _window.Height = SystemParameters.PrimaryScreenHeight;
        _window.Activate();

        int w = GetSystemMetrics(SM_CXSCREEN), h = GetSystemMetrics(SM_CYSCREEN);
        _scale = w / SystemParameters.PrimaryScreenWidth;
        _park = new POINT { X = w / 2, Y = h / 2 };
        SetCursorPos(_park.X, _park.Y);
        var clip = new RECT { Left = 0, Top = 0, Right = w, Bottom = h };
        ClipCursor(ref clip);
        Mouse.OverrideCursor = Cursors.None;
        _x = SystemParameters.PrimaryScreenWidth / 2;
        _y = SystemParameters.PrimaryScreenHeight / 2;
        lock (_gate) { _down.Clear(); _seen.Clear(); _exit.Reset(); }
        _lastProgress = -1;

        Locked = true;
        if (!InstallHooks())
        {
            Stop("no hook");
            return;
        }
        _timer.Start();
        _js("nativeStart()");
        _window.Dispatcher.BeginInvoke(DispatcherPriority.Background, () =>
        {
            SetCursorPos(_park.X + 1, _park.Y);
            SetCursorPos(_park.X, _park.Y);
        });
        Log.Write($"lock on, exitAfter={Config.ExitAfter?.ToString() ?? "none"} selftest={Config.SelfTest}");
    }

    public void Stop(string reason)
    {
        Locked = false;
        _timer.Stop();
        RemoveHooks();
        string summary;
        lock (_gate)
        {
            summary = string.Join(" ", _seen.OrderBy(p => p.Key).Select(p => $"0x{p.Key:X}:{p.Value}"));
            _down.Clear();
            _exit.Reset();
        }
        _lastProgress = -1;
        Log.Write($"lock off reason={reason} messagesSeen=[{summary}]");
        ClipCursorNone(IntPtr.Zero);
        Mouse.OverrideCursor = null;
        _window.Topmost = false;
        _window.WindowStyle = WindowStyle.SingleBorderWindow;
        _window.ResizeMode = ResizeMode.CanResize;
        _window.WindowState = WindowState.Maximized;
        _js("nativeStop()");
        Stopped?.Invoke();
    }

    private void Tick()
    {
        double p;
        lock (_gate) p = _exit.Progress;
        if (p != _lastProgress) { _js($"nativeExitProgress({p.ToString(System.Globalization.CultureInfo.InvariantCulture)})"); _lastProgress = p; }
        if (p >= 1) Stop("exit combo");
    }

    private bool Alt => _down.Contains(VK_MENU) || _down.Contains(VK_LMENU) || _down.Contains(VK_RMENU);
    private bool Shift => _down.Contains(VK_SHIFT) || _down.Contains(VK_LSHIFT) || _down.Contains(VK_RSHIFT);

    private static bool IsModifier(uint vk) => vk is VK_SHIFT or VK_MENU or VK_LSHIFT or VK_RSHIFT or VK_LMENU or VK_RMENU;

    private IntPtr OnKey(int nCode, IntPtr wParam, IntPtr lParam)
    {
        if (nCode < 0 || !Locked) return CallNextHookEx(IntPtr.Zero, nCode, wParam, lParam);
        var k = Marshal.PtrToStructure<KBDLLHOOKSTRUCT>(lParam);
        var msg = (int)wParam;
        lock (_gate) HandleKey(k, msg);
        return 1;
    }

    private void HandleKey(KBDLLHOOKSTRUCT k, int msg)
    {
        Count(msg);
        if (msg is WM_KEYDOWN or WM_SYSKEYDOWN)
        {
            var repeat = !_down.Add(k.VkCode);
            _exit.KeyDown((int)k.VkCode, Alt, Shift);
            if (IsModifier(k.VkCode)) _exit.ModifiersChanged(Alt, Shift);
            else if (!repeat) Input(KeyMap.ToPageKey((int)k.VkCode));
        }
        else if (msg is WM_KEYUP or WM_SYSKEYUP)
        {
            _down.Remove(k.VkCode);
            _exit.KeyUp((int)k.VkCode);
            if (IsModifier(k.VkCode)) _exit.ModifiersChanged(Alt, Shift);
        }
    }

    private IntPtr OnMouse(int nCode, IntPtr wParam, IntPtr lParam)
    {
        if (nCode < 0 || !Locked) return CallNextHookEx(IntPtr.Zero, nCode, wParam, lParam);
        var m = Marshal.PtrToStructure<MSLLHOOKSTRUCT>(lParam);
        var msg = (int)wParam;
        lock (_gate) HandleMouse(m, msg);
        return 1;
    }

    private void HandleMouse(MSLLHOOKSTRUCT m, int msg)
    {
        Count(msg);
        switch (msg)
        {
            case WM_MOUSEMOVE:
                _x = Math.Clamp(_x + (m.Pt.X - _park.X) / _scale, 0, SystemParameters.PrimaryScreenWidth);
                _y = Math.Clamp(_y + (m.Pt.Y - _park.Y) / _scale, 0, SystemParameters.PrimaryScreenHeight);
                break;
            case WM_LBUTTONDOWN or WM_RBUTTONDOWN or WM_MBUTTONDOWN or WM_XBUTTONDOWN:
                Input(null, true);
                break;
            case WM_MOUSEWHEEL or WM_MOUSEHWHEEL:
                _wheel += Math.Abs((short)(m.MouseData >> 16));
                if (_wheel >= WheelStep) { _wheel = 0; Input(null, true); }
                break;
        }
    }

    private void Count(int msg)
    {
        if (!_seen.ContainsKey(msg)) Log.Write($"first message 0x{msg:X}");
        _seen[msg] = _seen.GetValueOrDefault(msg) + 1;
    }

    private void Input(string? key, bool atCursor = false)
    {
        var k = key == null ? "null" : $"\"{key}\"";
        var inv = System.Globalization.CultureInfo.InvariantCulture;
        var call = atCursor ? $"nativeInput({k}, {_x.ToString(inv)}, {_y.ToString(inv)})" : $"nativeInput({k})";
        _window.Dispatcher.BeginInvoke(() => _js(call));
    }
}
