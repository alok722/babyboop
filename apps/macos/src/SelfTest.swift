import CoreGraphics
import Foundation

func postSelfTestKeys() {
    let src = CGEventSource(stateID: .hidSystemState)
    let keys: [(CGKeyCode, CGEventFlags, String)] = [
        (0, [], "a"), (99, [], "F3"), (118, [], "F4"), (160, [], "MissionControl"),
        (48, .maskCommand, "Cmd+Tab"), (126, .maskControl, "Ctrl+Up"), (49, .maskCommand, "Cmd+Space"),
    ]
    for (i, (code, flags, name)) in keys.enumerated() {
        DispatchQueue.main.asyncAfter(deadline: .now() + 1 + Double(i) * 0.7) {
            for down in [true, false] {
                let e = CGEvent(keyboardEventSource: src, virtualKey: code, keyDown: down)
                e?.flags = flags
                e?.post(tap: .cghidEventTap)
            }
            log("selftest posted \(name)")
        }
    }
    let t0 = 1 + Double(keys.count) * 0.7
    let esc: [(Double, Bool, String)] = [(t0, true, "Esc down (short)"), (t0 + 1, false, "Esc up after 1s"),
                                        (t0 + 2, true, "Esc down (long)"), (t0 + 8, false, "Esc up after 6s")]
    for (at, down, name) in esc {
        DispatchQueue.main.asyncAfter(deadline: .now() + at) {
            CGEvent(keyboardEventSource: src, virtualKey: CGKeyCode(keycodeEsc), keyDown: down)?.post(tap: .cghidEventTap)
            log("selftest posted \(name)")
        }
    }
}
