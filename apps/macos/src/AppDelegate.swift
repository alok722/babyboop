import AppKit
import ApplicationServices
import WebKit

final class Bridge: NSObject, WKScriptMessageHandler {
    func userContentController(_ c: WKUserContentController, didReceive m: WKScriptMessage) {
        log("page: \(m.body)")
        switch m.body as? String {
        case "start": startLock()
        case "quit": NSApp.terminate(nil)
        case "openSettings": openAccessibilitySettings()
        default: break
        }
    }
}

final class LoadDelegate: NSObject, WKNavigationDelegate {
    func webView(_ w: WKWebView, didFinish n: WKNavigation!) {
        log("page loaded")
        startPermissionWatch()
        if testMode {
            guard AXIsProcessTrusted() else {
                log("test mode quit: Accessibility not granted")
                NSApp.terminate(nil)
                return
            }
            startLock()
        }
    }
}

final class KeyWindow: NSWindow {
    override var canBecomeKey: Bool { true }
    override var canBecomeMain: Bool { true }
}

final class AppDelegate: NSObject, NSApplicationDelegate {
    let bridge = Bridge()
    let loadDelegate = LoadDelegate()

    func applicationDidFinishLaunching(_ n: Notification) {
        let screen = NSScreen.main!
        screenSize = screen.frame.size
        cursor = CGPoint(x: screenSize.width / 2, y: screenSize.height / 2)

        let config = WKWebViewConfiguration()
        config.mediaTypesRequiringUserActionForPlayback = []
        config.userContentController.add(bridge, name: "native")
        webView = WKWebView(frame: screen.frame, configuration: config)
        webView.navigationDelegate = loadDelegate
        let page = Bundle.main.url(forResource: "index", withExtension: "html")!
        webView.loadFileURL(page, allowingReadAccessTo: page.deletingLastPathComponent())

        window = KeyWindow(contentRect: screen.frame, styleMask: .borderless, backing: .buffered, defer: false, screen: screen)
        window.contentView = webView
        window.makeKeyAndOrderFront(nil)

        let menu = NSMenu(), appItem = NSMenuItem(), appMenu = NSMenu()
        appMenu.addItem(withTitle: "Quit BabyBoop", action: #selector(NSApplication.terminate(_:)), keyEquivalent: "q")
        appItem.submenu = appMenu
        menu.addItem(appItem)
        NSApp.mainMenu = menu
        NSApp.activate(ignoringOtherApps: true)

        var lastProgress = -1.0
        Timer.scheduledTimer(withTimeInterval: 0.1, repeats: true) { _ in
            let p = exitHoldStart.map { min(1, Date().timeIntervalSince($0) / exitHolds[exitHoldKey]!) } ?? 0
            if p != lastProgress { js("nativeExitProgress(\(p))"); lastProgress = p }
            if p >= 1 { endLock("exit combo") }
        }
    }
}
