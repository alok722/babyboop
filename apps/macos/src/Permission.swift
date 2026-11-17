import AppKit
import ApplicationServices

private var lastTrusted: Bool?
private var permissionTimer: Timer?

func startPermissionWatch() {
    permissionTimer?.invalidate()
    lastTrusted = nil
    let tick = {
        let trusted = AXIsProcessTrusted()
        guard trusted != lastTrusted else { return }
        lastTrusted = trusted
        log("permission=\(trusted)")
        js("nativePermission(\(trusted))")
    }
    tick()
    permissionTimer = Timer.scheduledTimer(withTimeInterval: 1, repeats: true) { _ in tick() }
}

func openAccessibilitySettings() {
    let opts = [kAXTrustedCheckOptionPrompt.takeUnretainedValue() as String: true] as CFDictionary
    _ = AXIsProcessTrustedWithOptions(opts)
    NSWorkspace.shared.open(URL(string: "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")!)
}
