namespace BabyBoop;

public static class Config
{
    private static readonly string[] Args = Environment.GetCommandLineArgs();

    public static readonly bool SelfTest = Args.Contains("--selftest");

    public static readonly double? ExitAfter =
        Array.IndexOf(Args, "--exit-after") is var i and >= 0 && i + 1 < Args.Length && double.TryParse(Args[i + 1], out var s) ? s : null;

    public static bool TestMode => SelfTest || ExitAfter != null;
}
