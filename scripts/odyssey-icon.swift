import AppKit
let image = NSImage(size: NSSize(width: 1024, height: 1024))
image.lockFocus()
NSColor(calibratedRed: 0.12, green: 0.19, blue: 0.28, alpha: 1).setFill()
NSBezierPath(roundedRect: NSRect(x: 0, y: 0, width: 1024, height: 1024), xRadius: 210, yRadius: 210).fill()
NSColor.white.setStroke()
let ring = NSBezierPath(ovalIn: NSRect(x: 210, y: 210, width: 604, height: 604))
ring.lineWidth = 36; ring.stroke()
NSColor(calibratedRed: 0.42, green: 0.82, blue: 0.75, alpha: 1).setFill()
let needle = NSBezierPath(); needle.move(to: NSPoint(x: 700, y: 740)); needle.line(to: NSPoint(x: 420, y: 550)); needle.line(to: NSPoint(x: 324, y: 284)); needle.line(to: NSPoint(x: 604, y: 474)); needle.close(); needle.fill()
image.unlockFocus()
let data = NSBitmapImageRep(data: image.tiffRepresentation!)!.representation(using: .png, properties: [:])!
try data.write(to: URL(fileURLWithPath: CommandLine.arguments[1]))
