namespace BabyBoop.Core;

public sealed class ExitHold
{
    private static readonly Dictionary<int, double> HoldSeconds = new()
    {
        [KeyMap.VK_ESCAPE] = 5,
        [KeyMap.VK_K] = 2,
    };

    private readonly Func<DateTime> _now;
    private DateTime? _start;
    private int _key = -1;

    public ExitHold(Func<DateTime> now) => _now = now;

    public void KeyDown(int vk, bool alt, bool shift, bool isRepeat)
    {
        if (_start != null) return;
        if (vk == KeyMap.VK_ESCAPE || (vk == KeyMap.VK_K && alt && shift))
        {
            _start = _now();
            _key = vk;
        }
    }

    public void KeyUp(int vk)
    {
        if (vk == _key) Reset();
    }

    public void ModifiersChanged(bool alt, bool shift)
    {
        if (_key == KeyMap.VK_K && !(alt && shift)) Reset();
    }

    public double Progress => _start is { } s ? Math.Min(1, (_now() - s).TotalSeconds / HoldSeconds[_key]) : 0;

    public bool Triggered => Progress >= 1;

    public void Reset()
    {
        _start = null;
        _key = -1;
    }
}
