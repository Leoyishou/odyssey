//! Native sidebar material beneath the GPUI surface.
use core_graphics::geometry::{CGRect, CGSize};
use gpui::*;
use objc::{msg_send, sel, sel_impl, runtime::{Class, Object, YES, NO}};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::cell::RefCell;

/// How far the glass runs under the note, past the sidebar's right edge.
const UNDERLAP: f64 = 12.;

pub fn supported() -> bool { Class::get("NSGlassEffectView").is_some() }

/// The sidebar material sits below the transparent GPUI surface, so text and
/// scrolling keep their existing renderer and event handling.
#[derive(Default)]
pub struct NativeSidebar { view: RefCell<Option<SidebarView>> }
struct SidebarView { view: *mut Object, frame: CGRect }
impl NativeSidebar {
    pub fn sync(&self, width: f32, window: &Window) {
        if !supported() { return; }
        let Ok(handle) = HasWindowHandle::window_handle(window) else { return; };
        let RawWindowHandle::AppKit(handle) = handle.as_raw() else { return; };
        unsafe {
            let gpui = handle.ns_view.as_ptr() as *mut Object;
            let parent: *mut Object = msg_send![gpui, superview];
            if parent.is_null() { return; }
            let base: CGRect = msg_send![gpui, frame];
            // The glass draws a light rim inside its own edge; running it under
            // the opaque note hides the rim, so sidebar and note meet in one step.
            let frame = CGRect::new(&base.origin, &CGSize::new(if width > 0. { width as f64 + UNDERLAP } else { 0. }, base.size.height));
            let mut slot = self.view.borrow_mut();
            if slot.is_none() && width > 0. {
                let view: *mut Object = msg_send![Class::get("NSGlassEffectView").unwrap(), alloc];
                let view: *mut Object = msg_send![view, initWithFrame:frame];
                let _: () = msg_send![view, setCornerRadius:0f64];
                let _: () = msg_send![parent, addSubview:view positioned:-1isize relativeTo:gpui];
                *slot = Some(SidebarView { view, frame });
            }
            if let Some(sidebar) = slot.as_mut() {
                if sidebar.frame.origin.x != frame.origin.x || sidebar.frame.origin.y != frame.origin.y || sidebar.frame.size.width != frame.size.width || sidebar.frame.size.height != frame.size.height {
                    let _: () = msg_send![sidebar.view, setFrame:frame];
                    sidebar.frame = frame;
                }
                let _: () = msg_send![sidebar.view, setHidden:if width > 0. { NO } else { YES }];
            }
        }
    }
}
impl Drop for SidebarView {
    fn drop(&mut self) { unsafe {
        let _: () = msg_send![self.view, removeFromSuperview];
        let _: () = msg_send![self.view, release];
    } }
}
