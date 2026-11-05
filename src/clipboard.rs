//! The system pasteboard beyond what GPUI reads.

/// Devtools stand-in for the pasteboard, (HTML, plain text): scripted tests
/// must not touch the shared system clipboard.
pub static FAKE_CLIPBOARD: std::sync::Mutex<Option<(Option<String>, String)>> = std::sync::Mutex::new(None);

/// The `public.html` flavor of the general pasteboard (gpui reads only
/// plain text and images).
#[cfg(target_os = "macos")]
pub fn clipboard_html() -> Option<String> {
    use objc::runtime::Object;
    use objc::{class, msg_send, sel, sel_impl};
    const TYPE: &str = "public.html";
    unsafe {
        let pb: *mut Object = msg_send![class!(NSPasteboard), generalPasteboard];
        let ty: *mut Object = msg_send![class!(NSString), alloc];
        let ty: *mut Object = msg_send![ty,
            initWithBytes: TYPE.as_ptr()
            length: TYPE.len()
            encoding: 4usize]; // NSUTF8StringEncoding
        let s: *mut Object = msg_send![pb, stringForType: ty];
        let _: () = msg_send![ty, release];
        if s.is_null() {
            return None;
        }
        let p: *const std::os::raw::c_char = msg_send![s, UTF8String];
        (!p.is_null()).then(|| std::ffi::CStr::from_ptr(p).to_string_lossy().into_owned())
    }
}

#[cfg(not(target_os = "macos"))]
pub fn clipboard_html() -> Option<String> {
    None
}
