import AppKit

resetLog()
let delegate = AppDelegate()
NSApplication.shared.delegate = delegate
NSApp.setActivationPolicy(.regular)
NSApp.run()
