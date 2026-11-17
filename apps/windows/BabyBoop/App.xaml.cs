using System.Windows;

namespace BabyBoop;

public partial class App : Application
{
    protected override void OnStartup(StartupEventArgs e)
    {
        Log.Reset();
        base.OnStartup(e);
    }
}
