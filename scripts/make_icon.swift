import AppKit

let size = 1024.0
let rep = NSBitmapImageRep(bitmapDataPlanes: nil, pixelsWide: Int(size), pixelsHigh: Int(size), bitsPerSample: 8,
                           samplesPerPixel: 4, hasAlpha: true, isPlanar: false, colorSpaceName: .deviceRGB,
                           bytesPerRow: 0, bitsPerPixel: 0)!
NSGraphicsContext.current = NSGraphicsContext(bitmapImageRep: rep)

let tile = NSRect(x: 100, y: 100, width: 824, height: 824)
let shape = NSBezierPath(roundedRect: tile, xRadius: 185, yRadius: 185)

NSGraphicsContext.saveGraphicsState()
let shadow = NSShadow()
shadow.shadowColor = NSColor.black.withAlphaComponent(0.35)
shadow.shadowBlurRadius = 30
shadow.shadowOffset = NSSize(width: 0, height: -12)
shadow.set()
NSColor.white.setFill()
shape.fill()
NSGraphicsContext.restoreGraphicsState()

NSGraphicsContext.saveGraphicsState()
shape.addClip()
NSGradient(colors: [NSColor(red: 1.0, green: 0.43, blue: 0.77, alpha: 1),
                    NSColor(red: 0.47, green: 0.45, blue: 0.96, alpha: 1)])!.draw(in: tile, angle: -60)

let confetti: [(CGFloat, CGFloat, CGFloat, NSColor)] = [
    (230, 760, 34, .systemYellow), (790, 790, 28, .systemGreen), (180, 330, 26, .systemTeal),
    (820, 300, 32, .systemOrange), (310, 860, 22, .white), (700, 190, 24, .systemYellow),
    (880, 560, 20, .white), (150, 560, 22, .systemOrange), (560, 880, 26, .systemTeal),
]
for (x, y, r, c) in confetti {
    c.setFill()
    NSBezierPath(ovalIn: NSRect(x: x - r / 2, y: y - r / 2, width: r, height: r)).fill()
}

let emoji = "🥳" as NSString
let attrs: [NSAttributedString.Key: Any] = [.font: NSFont.systemFont(ofSize: 520)]
let s = emoji.size(withAttributes: attrs)
emoji.draw(at: NSPoint(x: (size - s.width) / 2, y: (size - s.height) / 2 - 20), withAttributes: attrs)
NSGraphicsContext.restoreGraphicsState()

let out = CommandLine.arguments.count > 1 ? CommandLine.arguments[1] : "icon_1024.png"
try! rep.representation(using: .png, properties: [:])!.write(to: URL(fileURLWithPath: out))
