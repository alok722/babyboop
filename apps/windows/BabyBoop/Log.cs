using System.IO;

namespace BabyBoop;

public static class Log
{
    public static readonly string Dir = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "BabyBoop");
    private static readonly string File = Path.Combine(Dir, "BabyBoop.log");

    public static void Reset()
    {
        Directory.CreateDirectory(Dir);
        System.IO.File.WriteAllText(File, "");
    }

    public static void Write(string s)
    {
        try { System.IO.File.AppendAllText(File, $"{DateTime.Now:O} {s}\n"); } catch (IOException) { }
    }
}
