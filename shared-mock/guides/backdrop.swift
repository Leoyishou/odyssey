import AppKit
// backdrop.swift x y w h seconds wallpaper : a borderless, click-through window
// showing the desktop picture (mapped as macOS "fill" maps it to the main
// screen) at that screen rect, one level below floating, without activating.
let a = CommandLine.arguments
let (x, y, w, h) = (Double(a[1])!, Double(a[2])!, Double(a[3])!, Double(a[4])!)
let secs = Double(a[5])!
let app = NSApplication.shared
app.setActivationPolicy(.accessory)
let screen = NSScreen.screens[0].frame
let img = NSImage(contentsOfFile: a[6])!
let cg = img.cgImage(forProposedRect: nil, context: nil, hints: nil)!
let (iw, ih) = (Double(cg.width), Double(cg.height))
let s = max(screen.width / iw, screen.height / ih)
let ox = (iw * s - screen.width) / 2, oy = (ih * s - screen.height) / 2
let crop = cg.cropping(to: CGRect(x: (x + ox) / s, y: (y + oy) / s, width: w / s, height: h / s))!
let win = NSWindow(contentRect: NSRect(x: x, y: screen.height - y - h, width: w, height: h), styleMask: .borderless, backing: .buffered, defer: false)
win.level = NSWindow.Level(rawValue: 2)
win.ignoresMouseEvents = true
win.hasShadow = false
win.collectionBehavior = [.canJoinAllSpaces, .stationary, .ignoresCycle]
let view = NSImageView(frame: NSRect(x: 0, y: 0, width: w, height: h))
view.image = NSImage(cgImage: crop, size: NSSize(width: w, height: h))
view.imageScaling = .scaleAxesIndependently
win.contentView = view
win.orderFrontRegardless()
DispatchQueue.main.asyncAfter(deadline: .now() + secs) { exit(0) }
app.run()
