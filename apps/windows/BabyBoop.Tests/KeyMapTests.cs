using BabyBoop.Core;
using Xunit;

public class KeyMapTests
{
    [Theory]
    [InlineData(0x41, "A")]
    [InlineData(0x5A, "Z")]
    [InlineData(0x30, "0")]
    [InlineData(0x39, "9")]
    [InlineData(0x1B, null)]
    [InlineData(0x5B, null)]
    public void MapsVirtualKeys(int vk, string? expected) => Assert.Equal(expected, KeyMap.ToPageKey(vk));
}
