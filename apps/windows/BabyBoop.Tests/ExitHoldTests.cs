using BabyBoop.Core;
using Xunit;

public class ExitHoldTests
{
    private DateTime _now = new(2026, 1, 1, 0, 0, 0, DateTimeKind.Utc);
    private readonly ExitHold _hold;

    public ExitHoldTests() => _hold = new ExitHold(() => _now);

    private void At(double seconds) => _now = new DateTime(2026, 1, 1, 0, 0, 0, DateTimeKind.Utc).AddSeconds(seconds);

    [Fact]
    public void EscHeld5sTriggers()
    {
        _hold.KeyDown(KeyMap.VK_ESCAPE, false, false);
        At(5);
        Assert.True(_hold.Triggered);
    }

    [Fact]
    public void EscHeld4_9sDoesNot()
    {
        _hold.KeyDown(KeyMap.VK_ESCAPE, false, false);
        At(4.9);
        Assert.False(_hold.Triggered);
        Assert.Equal(0.98, _hold.Progress, 3);
    }

    [Fact]
    public void ReleaseResets()
    {
        _hold.KeyDown(KeyMap.VK_ESCAPE, false, false);
        At(1);
        _hold.KeyUp(KeyMap.VK_ESCAPE);
        At(2);
        _hold.KeyDown(KeyMap.VK_ESCAPE, false, false);
        At(6);
        Assert.False(_hold.Triggered);
        At(7);
        Assert.True(_hold.Triggered);
    }

    [Fact]
    public void AutorepeatDoesNotRestart()
    {
        _hold.KeyDown(KeyMap.VK_ESCAPE, false, false);
        At(3);
        _hold.KeyDown(KeyMap.VK_ESCAPE, false, false);
        At(5);
        Assert.True(_hold.Triggered);
    }

    [Fact]
    public void KNeedsAltShift()
    {
        _hold.KeyDown(KeyMap.VK_K, false, true);
        At(3);
        Assert.Equal(0, _hold.Progress);
    }

    [Fact]
    public void AltShiftK2s()
    {
        _hold.KeyDown(KeyMap.VK_K, true, true);
        At(2);
        Assert.True(_hold.Triggered);
    }

    [Fact]
    public void ResetClearsInProgressHold()
    {
        _hold.KeyDown(KeyMap.VK_ESCAPE, false, false);
        At(5);
        Assert.True(_hold.Triggered);
        _hold.Reset();
        Assert.Equal(0, _hold.Progress);
        _hold.KeyDown(KeyMap.VK_ESCAPE, false, false);
        At(6);
        Assert.False(_hold.Triggered);
    }

    [Fact]
    public void DroppingShiftResetsK()
    {
        _hold.KeyDown(KeyMap.VK_K, true, true);
        At(1);
        _hold.ModifiersChanged(true, false);
        At(3);
        Assert.False(_hold.Triggered);
    }
}
