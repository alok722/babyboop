using System.Diagnostics;
using System.IO;
using System.Windows;
using System.Windows.Threading;
using Microsoft.Web.WebView2.Core;

namespace BabyBoop;

public partial class MainWindow : Window
{
    private const string Origin = "https://app.babyboop/";
    private const string RuntimeUrl = "https://go.microsoft.com/fwlink/p/?LinkId=2124703";
    private readonly InputLock _lock;

    public MainWindow()
    {
        InitializeComponent();
        _lock = new InputLock(this, Js);
        _lock.Stopped += () => { if (Config.TestMode) Application.Current.Shutdown(); };
        Loaded += async (_, _) => await InitWebView();
    }

    private void Js(string script) => _ = Web.CoreWebView2?.ExecuteScriptAsync(script);

    private async Task InitWebView()
    {
        CoreWebView2Environment env;
        try
        {
            var options = new CoreWebView2EnvironmentOptions("--autoplay-policy=no-user-gesture-required");
            env = await CoreWebView2Environment.CreateAsync(null, Path.Combine(Log.Dir, "WebView2"), options);
            await Web.EnsureCoreWebView2Async(env);
        }
        catch (Exception ex)
        {
            Log.Write($"webview2 init failed: {ex}");
            var answer = MessageBox.Show(
                "BabyBoop needs the Microsoft Edge WebView2 Runtime, and it couldn't be started on this PC.\n\n" +
                "Click Yes to open Microsoft's download page. Install it, then open BabyBoop again.",
                "BabyBoop", MessageBoxButton.YesNo, MessageBoxImage.Information);
            if (answer == MessageBoxResult.Yes)
                Process.Start(new ProcessStartInfo(RuntimeUrl) { UseShellExecute = true });
            Application.Current.Shutdown();
            return;
        }

        var core = Web.CoreWebView2;
        core.Settings.AreDefaultContextMenusEnabled = false;
        core.Settings.AreDevToolsEnabled = false;
        core.Settings.AreBrowserAcceleratorKeysEnabled = false;
        core.Settings.IsZoomControlEnabled = false;
        core.Settings.IsStatusBarEnabled = false;
        core.AddWebResourceRequestedFilter(Origin + "*", CoreWebView2WebResourceContext.All);
        core.WebResourceRequested += (_, e) => e.Response = ServePage(env);
        core.WebMessageReceived += (_, e) => OnMessage(e.TryGetWebMessageAsString());
        core.NavigationCompleted += (_, _) => OnLoaded();
        core.Navigate(Origin + "index.html");
    }

    private static CoreWebView2WebResourceResponse ServePage(CoreWebView2Environment env)
    {
        var page = typeof(MainWindow).Assembly.GetManifestResourceStream("index.html")!;
        return env.CreateWebResourceResponse(page, 200, "OK", "Content-Type: text/html; charset=utf-8");
    }

    private void OnMessage(string message)
    {
        Log.Write($"page: {message}");
        switch (message)
        {
            case "start": _lock.Start(); break;
            case "quit": Application.Current.Shutdown(); break;
        }
    }

    private void OnLoaded()
    {
        Log.Write("page loaded");
        if (!Config.TestMode) return;
        _lock.Start();
        if (Config.ExitAfter is { } s)
        {
            var t = new DispatcherTimer { Interval = TimeSpan.FromSeconds(s) };
            t.Tick += (_, _) => { t.Stop(); if (_lock.Locked) _lock.Stop($"exit-after {s}s"); };
            t.Start();
        }
        if (Config.SelfTest) SelfTest.Run();
    }

    protected override void OnClosed(EventArgs e)
    {
        if (_lock.Locked) _lock.Stop("window closed");
        base.OnClosed(e);
    }
}
