using System.Runtime.InteropServices;
using System.Windows.Threading;
using static BabyBoop.Native;

namespace BabyBoop;

public static class SelfTest
{
    private const ushort VK_A = 0x41, VK_LWIN = 0x5B, VK_MENU = 0x12, VK_TAB = 0x09, VK_CONTROL = 0x11,
        VK_ESCAPE = 0x1B, VK_MEDIA_PLAY_PAUSE = 0xB3;

    public static void Run()
    {
        var steps = new (double At, string Name, (ushort Vk, bool Up)[] Keys)[]
        {
            (1.0, "a", [(VK_A, false), (VK_A, true)]),
            (1.7, "LWin", [(VK_LWIN, false), (VK_LWIN, true)]),
            (2.4, "Alt+Tab", [(VK_MENU, false), (VK_TAB, false), (VK_TAB, true), (VK_MENU, true)]),
            (3.1, "Ctrl+Esc", [(VK_CONTROL, false), (VK_ESCAPE, false), (VK_ESCAPE, true), (VK_CONTROL, true)]),
            (3.8, "Media Play/Pause", [(VK_MEDIA_PLAY_PAUSE, false), (VK_MEDIA_PLAY_PAUSE, true)]),
            (4.5, "Esc down (short)", [(VK_ESCAPE, false)]),
            (5.5, "Esc up after 1s", [(VK_ESCAPE, true)]),
            (6.5, "Esc down (long)", [(VK_ESCAPE, false)]),
            (12.5, "Esc up after 6s", [(VK_ESCAPE, true)]),
        };
        foreach (var step in steps)
        {
            var t = new DispatcherTimer { Interval = TimeSpan.FromSeconds(step.At) };
            t.Tick += (_, _) => { t.Stop(); Send(step.Keys); Log.Write($"selftest posted {step.Name}"); };
            t.Start();
        }
    }

    private static void Send((ushort Vk, bool Up)[] keys)
    {
        var inputs = keys.Select(k => new INPUT
        {
            Type = INPUT_KEYBOARD,
            U = new InputUnion
            {
                Ki = new KEYBDINPUT
                {
                    Vk = k.Vk,
                    Flags = (k.Up ? KEYEVENTF_KEYUP : 0) | (k.Vk is VK_LWIN or VK_MEDIA_PLAY_PAUSE ? KEYEVENTF_EXTENDEDKEY : 0),
                },
            },
        }).ToArray();
        SendInput((uint)inputs.Length, inputs, Marshal.SizeOf<INPUT>());
    }
}
