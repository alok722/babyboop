namespace BabyBoop.Core;

public static class KeyMap
{
    public const int VK_ESCAPE = 0x1B, VK_K = 0x4B;

    public static string? ToPageKey(int vk) =>
        vk is >= 0x41 and <= 0x5A or >= 0x30 and <= 0x39 ? ((char)vk).ToString() : null;
}
