import AppKit
import ApplicationServices
import WebKit

var webView: WKWebView!
var window: NSWindow!
var tapPort: CFMachPort?
var cursor = CGPoint.zero
var screenSize = CGSize.zero
var scrollAccum = 0.0
var exitHoldStart: Date?
var exitHoldKey: Int64 = -1
var seen: [UInt32: Int] = [:]
var exitAfterTimer: Timer?

func js(_ s: String) { webView.evaluateJavaScript(s, completionHandler: nil) }

func input(_ key: String?, at p: CGPoint? = nil) {
    let k = key.map { "\"\($0)\"" } ?? "null"
    if let p { js("nativeInput(\(k), \(p.x), \(p.y))") } else { js("nativeInput(\(k))") }
}

func keyString(_ e: CGEvent) -> String? {
    guard let c = NSEvent(cgEvent: e)?.charactersIgnoringModifiers, c.count == 1,
          let ch = c.unicodeScalars.first, ch.isASCII, CharacterSet.alphanumerics.contains(ch) else { return nil }
    return c
}

func isExitCombo(_ f: CGEventFlags) -> Bool { f.contains(.maskAlternate) && f.contains(.maskShift) }

func startLock() {
    guard tapPort == nil else { return }
    guard AXIsProcessTrusted() else { log("start refused: untrusted"); return }
    window.level = .screenSaver
    NSApp.activate(ignoringOtherApps: true)
    NSApp.presentationOptions = [.hideDock, .hideMenuBar, .disableProcessSwitching, .disableForceQuit,
                                 .disableSessionTermination, .disableHideApplication, .disableAppleMenu]
    NSCursor.hide()
    cursor = CGPoint(x: screenSize.width / 2, y: screenSize.height / 2)
    seen = [:]
    tapPort = CGEvent.tapCreate(tap: .cghidEventTap, place: .headInsertEventTap, options: .defaultTap,
                                eventsOfInterest: CGEventMask.max, callback: tapCallback, userInfo: nil)
    guard let tap = tapPort else { log("tapCreate FAILED"); endLock("no tap"); return }
    CFRunLoopAddSource(CFRunLoopGetMain(), CFMachPortCreateRunLoopSource(nil, tap, 0), .commonModes)
    CGEvent.tapEnable(tap: tap, enable: true)
    js("nativeStart()")
    log("lock on, exitAfter=\(exitAfter.map { String($0) } ?? "none") selftest=\(selfTest)")
    if let s = exitAfter {
        exitAfterTimer = Timer.scheduledTimer(withTimeInterval: s, repeats: false) { _ in endLock("exit-after \(s)s") }
    }
    if selfTest { postSelfTestKeys() }
}

func endLock(_ reason: String) {
    let summary = seen.sorted { $0.key < $1.key }.map { "\($0.key):\($0.value)" }.joined(separator: " ")
    log("lock off reason=\(reason) eventTypesSeen=[\(summary)]")
    if let t = tapPort { CGEvent.tapEnable(tap: t, enable: false); CFMachPortInvalidate(t) }
    tapPort = nil
    exitHoldStart = nil
    exitAfterTimer?.invalidate()
    NSApp.presentationOptions = []
    NSCursor.unhide()
    window.level = .normal
    js("nativeStop()")
    if testMode { NSApp.terminate(nil) }
}

let tapCallback: CGEventTapCallBack = { _, type, event, _ in
    if type == .tapDisabledByTimeout || type == .tapDisabledByUserInput {
        if let t = tapPort { CGEvent.tapEnable(tap: t, enable: true) }
        log("tap re-enabled after \(type.rawValue)")
        return Unmanaged.passUnretained(event)
    }
    if seen[type.rawValue] == nil { log("first event type \(type.rawValue)") }
    seen[type.rawValue, default: 0] += 1

    switch type {
    case .keyDown:
        let code = event.getIntegerValueField(.keyboardEventKeycode)
        if exitHoldStart == nil && (code == keycodeEsc || (code == keycodeK && isExitCombo(event.flags))) {
            exitHoldStart = Date(); exitHoldKey = code
        }
        if event.getIntegerValueField(.keyboardEventAutorepeat) == 0 { input(keyString(event)) }
    case .keyUp:
        if event.getIntegerValueField(.keyboardEventKeycode) == exitHoldKey { exitHoldStart = nil }
    case .flagsChanged:
        if exitHoldKey == keycodeK && !isExitCombo(event.flags) { exitHoldStart = nil }
    case .mouseMoved, .leftMouseDragged, .rightMouseDragged, .otherMouseDragged:
        cursor.x = min(max(0, cursor.x + CGFloat(event.getDoubleValueField(.mouseEventDeltaX))), screenSize.width)
        cursor.y = min(max(0, cursor.y + CGFloat(event.getDoubleValueField(.mouseEventDeltaY))), screenSize.height)
    case .leftMouseDown, .rightMouseDown, .otherMouseDown:
        input(nil, at: cursor)
    case .scrollWheel:
        scrollAccum += abs(event.getDoubleValueField(.scrollWheelEventPointDeltaAxis1))
            + abs(event.getDoubleValueField(.scrollWheelEventPointDeltaAxis2))
        if scrollAccum > 120 { scrollAccum = 0; input(nil, at: cursor) }
    default:
        if type.rawValue == 14, let ns = NSEvent(cgEvent: event), ns.subtype.rawValue == 8,
           (ns.data1 & 0xFF00) >> 8 == 0x0A {
            input(nil)
        }
    }
    return nil
}
