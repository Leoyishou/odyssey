//! 下载为 → PDF and 打印 on A4. macOS: the exported HTML page loads in an
//! off-screen WKWebView, then goes through an `NSPrintOperation` — saved
//! straight to a file for PDF, or behind the system print panel for 打印.
//! Windows: the same page in a hidden WebView2, then `PrintToPdf`, or the
//! system print dialog followed by WebView2's `Print` to the chosen printer.

use gpui::*;
use std::path::PathBuf;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use std::time::Duration;

pub const SUPPORTED: bool = cfg!(any(target_os = "macos", target_os = "windows"));

pub enum Job {
    #[cfg_attr(not(any(target_os = "macos", target_os = "windows")), allow(dead_code))] // No backend elsewhere.
    Pdf(PathBuf),
    Print,
}

/// Lays out `html` and runs the job; `Ok(false)` when the user cancelled
/// the print panel.
pub async fn run(html: String, title: String, job: Job, parent: usize, cx: &mut AsyncApp) -> Result<bool, String> {
    #[cfg(target_os = "macos")]
    {
        native::run(html, title, job, parent, cx).await
    }
    #[cfg(target_os = "windows")]
    {
        webview2::run(html, title, job, parent, cx).await
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        let _ = (html, title, job, parent, cx);
        Err(crate::settings::text("当前系统不支持").into())
    }
}

/// The GPUI window's native handle: its NSView on macOS (the print panel
/// opens as its sheet), its HWND on Windows (owner of the print dialog).
pub fn parent_of(window: &Window) -> usize {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    match HasWindowHandle::window_handle(window).ok().map(|h| h.as_raw()) {
        Some(RawWindowHandle::AppKit(h)) => h.ns_view.as_ptr() as usize,
        Some(RawWindowHandle::Win32(h)) => h.hwnd.get() as usize,
        _ => 0,
    }
}

#[cfg(target_os = "macos")]
mod native {
    use super::*;
    use core_graphics::geometry::{CGPoint, CGRect, CGSize};
    use objc::declare::ClassDecl;
    use objc::runtime::{BOOL, Class, NO, Object, Sel, YES};
    use objc::{class, msg_send, sel, sel_impl};
    use std::sync::atomic::{AtomicU8, Ordering};

    #[link(name = "WebKit", kind = "framework")]
    unsafe extern "C" {}

    #[link(name = "AppKit", kind = "framework")]
    unsafe extern "C" {
        static NSPrintJobSavingURL: *mut Object;
        static NSPrintSaveJob: *mut Object;
    }

    /// A4 in points, and the page margins.
    const PAPER: (f64, f64) = (595.28, 841.89);
    const MARGIN_X: f64 = 48.;
    const MARGIN_Y: f64 = 40.;
    /// CSS px per point when printing (96 dpi on 72 dpi paper).
    const PX_PER_PT: f64 = 96. / 72.;
    const LOAD_TIMEOUT: Duration = Duration::from_secs(20);

    /// 0 running, 1 done, 2 cancelled or failed.
    static STATE: AtomicU8 = AtomicU8::new(0);

    extern "C" fn did_run(_: &Object, _: Sel, _op: *mut Object, success: BOOL, _ctx: *mut std::ffi::c_void) {
        STATE.store(if success == YES { 1 } else { 2 }, Ordering::SeqCst);
    }

    fn delegate_class() -> &'static Class {
        if let Some(c) = Class::get("FDLPrintDelegate") {
            return c;
        }
        let mut decl = ClassDecl::new("FDLPrintDelegate", class!(NSObject)).expect("FDLPrintDelegate");
        unsafe {
            decl.add_method(
                sel!(printOperationDidRun:success:contextInfo:),
                did_run as extern "C" fn(&Object, Sel, *mut Object, BOOL, *mut std::ffi::c_void),
            );
        }
        decl.register()
    }

    unsafe fn ns_string(s: &str) -> *mut Object {
        unsafe {
            let o: *mut Object = msg_send![class!(NSString), alloc];
            let o: *mut Object = msg_send![o, initWithBytes: s.as_ptr() length: s.len() encoding: 4usize];
            msg_send![o, autorelease]
        }
    }

    fn rect(x: f64, y: f64, w: f64, h: f64) -> CGRect {
        CGRect::new(&CGPoint::new(x, y), &CGSize::new(w, h))
    }

    pub async fn run(html: String, title: String, job: Job, ns_view: usize, cx: &mut AsyncApp) -> Result<bool, String> {
        let width_px = (PAPER.0 - 2. * MARGIN_X) * PX_PER_PT;
        let (holder, web) = unsafe {
            let holder: *mut Object = msg_send![class!(NSWindow), alloc];
            let holder: *mut Object = msg_send![holder,
                initWithContentRect: rect(-20000., -20000., width_px, 1000.)
                styleMask: 0usize
                backing: 2usize
                defer: NO];
            let _: () = msg_send![holder, setReleasedWhenClosed: NO];
            let conf: *mut Object = msg_send![class!(WKWebViewConfiguration), new];
            let web: *mut Object = msg_send![class!(WKWebView), alloc];
            let web: *mut Object = msg_send![web, initWithFrame: rect(0., 0., width_px, 1000.) configuration: conf];
            let _: () = msg_send![conf, release];
            let _: () = msg_send![holder, setContentView: web];
            let nil: *mut Object = std::ptr::null_mut();
            let _: *mut Object = msg_send![web, loadHTMLString: ns_string(&html) baseURL: nil];
            (holder, web)
        };
        let cleanup = move || unsafe {
            let _: () = msg_send![holder, setContentView: std::ptr::null_mut::<Object>()];
            let _: () = msg_send![web, release];
            let _: () = msg_send![holder, close];
            let _: () = msg_send![holder, release];
        };
        let started = std::time::Instant::now();
        loop {
            cx.background_executor().timer(Duration::from_millis(50)).await;
            let loading: BOOL = unsafe { msg_send![web, isLoading] };
            if loading == NO {
                break;
            }
            if started.elapsed() > LOAD_TIMEOUT {
                cleanup();
                return Err(crate::settings::text("打印超时，请稍后重试").into());
            }
        }
        // Let images decode and the page settle before it is paginated.
        cx.background_executor().timer(Duration::from_millis(300)).await;
        STATE.store(0, Ordering::SeqCst);
        let print = matches!(job, Job::Print);
        unsafe {
            let shared: *mut Object = msg_send![class!(NSPrintInfo), sharedPrintInfo];
            let info: *mut Object = msg_send![shared, copy];
            let _: () = msg_send![info, setPaperSize: CGSize::new(PAPER.0, PAPER.1)];
            let _: () = msg_send![info, setOrientation: 0isize];
            let _: () = msg_send![info, setLeftMargin: MARGIN_X];
            let _: () = msg_send![info, setRightMargin: MARGIN_X];
            let _: () = msg_send![info, setTopMargin: MARGIN_Y];
            let _: () = msg_send![info, setBottomMargin: MARGIN_Y];
            let _: () = msg_send![info, setHorizontalPagination: 1usize]; // NSPrintingPaginationModeFit
            let _: () = msg_send![info, setVerticalPagination: 0usize]; // automatic
            let _: () = msg_send![info, setHorizontallyCentered: NO];
            let _: () = msg_send![info, setVerticallyCentered: NO];
            if let Job::Pdf(path) = &job {
                let _: () = msg_send![info, setJobDisposition: NSPrintSaveJob];
                let url: *mut Object = msg_send![class!(NSURL), fileURLWithPath: ns_string(&path.to_string_lossy())];
                let dict: *mut Object = msg_send![info, dictionary];
                let _: () = msg_send![dict, setObject: url forKey: NSPrintJobSavingURL];
            }
            let op: *mut Object = msg_send![web, printOperationWithPrintInfo: info];
            let _: () = msg_send![info, release];
            let _: () = msg_send![op, setShowsPrintPanel: if print { YES } else { NO }];
            let _: () = msg_send![op, setShowsProgressPanel: NO];
            let _: () = msg_send![op, setJobTitle: ns_string(&title)];
            // WebKit paginates its print view at this frame's width.
            let view: *mut Object = msg_send![op, view];
            let _: () = msg_send![view, setFrame: rect(0., 0., width_px, 1000.)];
            let delegate: *mut Object = msg_send![delegate_class(), new];
            let parent: *mut Object = if ns_view != 0 { msg_send![ns_view as *mut Object, window] } else { holder };
            let _: () = msg_send![op,
                runOperationModalForWindow: parent
                delegate: delegate
                didRunSelector: sel!(printOperationDidRun:success:contextInfo:)
                contextInfo: std::ptr::null_mut::<std::ffi::c_void>()];
            // The operation keeps no strong reference to its delegate.
            let keep = delegate as usize;
            loop {
                cx.background_executor().timer(Duration::from_millis(100)).await;
                if STATE.load(Ordering::SeqCst) != 0 {
                    break;
                }
            }
            let _: () = msg_send![keep as *mut Object, release];
        }
        cleanup();
        Ok(STATE.load(Ordering::SeqCst) == 1)
    }
}

#[cfg(target_os = "windows")]
mod webview2 {
    use super::*;
    use std::cell::RefCell;
    use std::fmt::Write as _;
    use std::rc::Rc;
    use std::time::Instant;
    use webview2_com::Microsoft::Web::WebView2::Win32::*;
    use webview2_com::{
        CreateCoreWebView2ControllerCompletedHandler, CreateCoreWebView2EnvironmentCompletedHandler, NavigationCompletedEventHandler,
        PrintCompletedHandler, PrintToPdfCompletedHandler,
    };
    use windows::Win32::Foundation::{GlobalFree, HWND, RECT};
    use windows::Win32::Graphics::Gdi::*;
    use windows::Win32::System::Memory::{GlobalLock, GlobalUnlock};
    use windows::Win32::UI::Controls::Dialogs::*;
    use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, DestroyWindow, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_POPUP};
    use windows::core::{HSTRING, Interface, PCWSTR, w};

    /// A4 and the page margins in inches: the same 595.28 × 841.89 pt page and
    /// 48 / 40 pt margins as macOS.
    const PAPER: (f64, f64) = (595.28 / 72., 841.89 / 72.);
    const MARGIN_X: f64 = 48. / 72.;
    const MARGIN_Y: f64 = 40. / 72.;
    const LOAD_TIMEOUT: Duration = Duration::from_secs(20);

    type Slot<T> = Rc<RefCell<Option<T>>>;

    /// Waits on the UI thread (whose message loop delivers WebView2's
    /// callbacks) until a callback fills `slot`.
    async fn wait<T>(slot: &Slot<T>, limit: Option<Duration>, cx: &mut AsyncApp) -> Option<T> {
        let started = Instant::now();
        loop {
            if let Some(v) = slot.borrow_mut().take() {
                return Some(v);
            }
            if limit.is_some_and(|l| started.elapsed() > l) {
                return None;
            }
            cx.background_executor().timer(Duration::from_millis(30)).await;
        }
    }

    /// What the print dialog chose.
    struct Choice {
        printer: String,
        copies: i32,
        collate: bool,
        gray: bool,
        duplex: COREWEBVIEW2_PRINT_DUPLEX,
        /// "1-3,5", empty for all pages.
        pages: String,
    }

    /// The system print dialog; `None` when cancelled.
    fn ask_printer(owner: HWND) -> Result<Option<Choice>, String> {
        let mut ranges = [PRINTPAGERANGE::default(); 16];
        let mut pd = PRINTDLGEXW {
            lStructSize: std::mem::size_of::<PRINTDLGEXW>() as u32,
            hwndOwner: owner,
            Flags: PD_NOSELECTION | PD_NOCURRENTPAGE | PD_USEDEVMODECOPIESANDCOLLATE,
            nMaxPageRanges: ranges.len() as u32,
            lpPageRanges: ranges.as_mut_ptr(),
            nMinPage: 1,
            nMaxPage: 9999,
            nCopies: 1,
            nStartPage: START_PAGE_GENERAL,
            ..Default::default()
        };
        let shown = unsafe { PrintDlgExW(&mut pd) };
        let read = |pd: &PRINTDLGEXW| unsafe {
            let mut c = Choice { printer: String::new(), copies: 1, collate: true, gray: false, duplex: COREWEBVIEW2_PRINT_DUPLEX_DEFAULT, pages: String::new() };
            let names = GlobalLock(pd.hDevNames) as *const DEVNAMES;
            if !names.is_null() {
                let s = (names as *const u16).add((*names).wDeviceOffset as usize);
                c.printer = PCWSTR(s).to_string().unwrap_or_default();
                let _ = GlobalUnlock(pd.hDevNames);
            }
            let mode = GlobalLock(pd.hDevMode) as *const DEVMODEW;
            if !mode.is_null() {
                let m = &*mode;
                if m.dmFields.contains(DM_COPIES) {
                    c.copies = (m.Anonymous1.Anonymous1.dmCopies as i32).max(1);
                }
                if m.dmFields.contains(DM_COLLATE) {
                    c.collate = m.dmCollate == DMCOLLATE_TRUE;
                }
                if m.dmFields.contains(DM_COLOR) {
                    c.gray = m.dmColor == DMCOLOR_MONOCHROME;
                }
                if m.dmFields.contains(DM_DUPLEX) {
                    c.duplex = match m.dmDuplex {
                        DMDUP_SIMPLEX => COREWEBVIEW2_PRINT_DUPLEX_ONE_SIDED,
                        DMDUP_VERTICAL => COREWEBVIEW2_PRINT_DUPLEX_TWO_SIDED_LONG_EDGE,
                        DMDUP_HORIZONTAL => COREWEBVIEW2_PRINT_DUPLEX_TWO_SIDED_SHORT_EDGE,
                        _ => COREWEBVIEW2_PRINT_DUPLEX_DEFAULT,
                    };
                }
                let _ = GlobalUnlock(pd.hDevMode);
            }
            if pd.Flags.contains(PD_PAGENUMS) {
                for r in &ranges[..pd.nPageRanges as usize] {
                    let sep = if c.pages.is_empty() { "" } else { "," };
                    let _ = if r.nFromPage == r.nToPage { write!(c.pages, "{sep}{}", r.nFromPage) } else { write!(c.pages, "{sep}{}-{}", r.nFromPage, r.nToPage) };
                }
            }
            c
        };
        let choice = (shown.is_ok() && pd.dwResultAction == PD_RESULT_PRINT).then(|| read(&pd));
        unsafe {
            for h in [pd.hDevMode, pd.hDevNames] {
                if !h.is_invalid() {
                    let _ = GlobalFree(Some(h));
                }
            }
        }
        shown.map_err(|_| crate::settings::text("打印失败，请重试").to_string())?;
        Ok(choice)
    }

    /// `path` as a file:// URL; WebView2's NavigateToString stops at 2 MB,
    /// less than a page with a few embedded images.
    fn file_url(path: &std::path::Path) -> String {
        let mut url = String::from("file:///");
        for b in path.to_string_lossy().replace('\\', "/").bytes() {
            if b.is_ascii_alphanumeric() || b"/:-_.~".contains(&b) {
                url.push(b as char);
            } else {
                let _ = write!(url, "%{b:02X}");
            }
        }
        url
    }

    pub async fn run(html: String, title: String, job: Job, parent: usize, cx: &mut AsyncApp) -> Result<bool, String> {
        let _ = title; // WebView2 names the print job after the page's <title>.
        let owner = HWND(parent as *mut std::ffi::c_void);
        let print = matches!(job, Job::Print);
        let fail = || crate::settings::text(if print { "打印失败，请重试" } else { "导出失败，请重试" }).to_string();
        // Ask first, so cancelling never starts a browser process.
        let choice = match job {
            Job::Print => match ask_printer(owner)? {
                Some(c) => Some(c),
                None => return Ok(false),
            },
            Job::Pdf(_) => None,
        };
        let page = std::env::temp_dir().join(format!("quiet-md-print-{}.html", std::process::id()));
        std::fs::write(&page, html).map_err(|_| fail())?;
        let host = unsafe {
            CreateWindowExW(WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE, w!("STATIC"), PCWSTR::null(), WS_POPUP, -20000, -20000, 800, 1000, (parent != 0).then_some(owner), None, None, None)
        };
        let Ok(host) = host else {
            let _ = std::fs::remove_file(&page);
            return Err(fail());
        };
        let res = print_in(host, &page, job, choice, cx).await;
        unsafe {
            let _ = DestroyWindow(host);
        }
        let _ = std::fs::remove_file(&page);
        res.map_err(|e| e.unwrap_or_else(fail))
    }

    /// `Err(None)` is a plain failure, `Err(Some(_))` one with its own message.
    async fn print_in(host: HWND, page: &std::path::Path, job: Job, choice: Option<Choice>, cx: &mut AsyncApp) -> Result<bool, Option<String>> {
        // The profile can't live next to the exe (Program Files is read-only).
        let data = dirs::data_local_dir().unwrap_or_else(std::env::temp_dir).join("quiet.md").join("WebView2");
        let env_slot: Slot<windows::core::Result<ICoreWebView2Environment>> = Rc::default();
        let s = env_slot.clone();
        let made = unsafe {
            CreateCoreWebView2EnvironmentWithOptions(
                PCWSTR::null(),
                &HSTRING::from(data.as_os_str()),
                None::<&ICoreWebView2EnvironmentOptions>,
                &CreateCoreWebView2EnvironmentCompletedHandler::create(Box::new(move |hr, env| {
                    *s.borrow_mut() = Some(hr.and_then(|_| env.ok_or_else(|| windows::core::Error::from(windows::Win32::Foundation::E_POINTER))));
                    Ok(())
                })),
            )
        };
        let missing = || Some(crate::settings::text("需要 Microsoft Edge WebView2 运行时").to_string());
        made.map_err(|_| missing())?;
        let env = wait(&env_slot, Some(LOAD_TIMEOUT), cx).await.ok_or(None)?.map_err(|_| missing())?;

        let ctl_slot: Slot<windows::core::Result<ICoreWebView2Controller>> = Rc::default();
        let s = ctl_slot.clone();
        unsafe {
            env.CreateCoreWebView2Controller(
                host,
                &CreateCoreWebView2ControllerCompletedHandler::create(Box::new(move |hr, c| {
                    *s.borrow_mut() = Some(hr.and_then(|_| c.ok_or_else(|| windows::core::Error::from(windows::Win32::Foundation::E_POINTER))));
                    Ok(())
                })),
            )
            .map_err(|_| None)?;
        }
        let controller = wait(&ctl_slot, Some(LOAD_TIMEOUT), cx).await.ok_or(None)?.map_err(|_| None)?;
        let res = print_with(&env, &controller, page, job, choice, cx).await;
        unsafe {
            let _ = controller.Close();
        }
        res
    }

    async fn print_with(
        env: &ICoreWebView2Environment,
        controller: &ICoreWebView2Controller,
        page: &std::path::Path,
        job: Job,
        choice: Option<Choice>,
        cx: &mut AsyncApp,
    ) -> Result<bool, Option<String>> {
        let web = unsafe {
            let _ = controller.SetBounds(RECT { left: 0, top: 0, right: 800, bottom: 1000 });
            let _ = controller.SetIsVisible(true);
            controller.CoreWebView2().map_err(|_| None)?
        };
        let nav_slot: Slot<bool> = Rc::default();
        let s = nav_slot.clone();
        let mut token = 0i64;
        unsafe {
            web.add_NavigationCompleted(
                &NavigationCompletedEventHandler::create(Box::new(move |_, args| {
                    let mut ok = windows::core::BOOL(0);
                    if let Some(a) = args {
                        let _ = a.IsSuccess(&mut ok);
                    }
                    *s.borrow_mut() = Some(ok.as_bool());
                    Ok(())
                })),
                &mut token,
            )
            .map_err(|_| None)?;
            web.Navigate(&HSTRING::from(file_url(page))).map_err(|_| None)?;
        }
        match wait(&nav_slot, Some(LOAD_TIMEOUT), cx).await {
            Some(true) => {}
            Some(false) => return Err(None),
            None => return Err(Some(crate::settings::text("打印超时，请稍后重试").to_string())),
        }
        // Let images decode and the page settle before it is paginated.
        cx.background_executor().timer(Duration::from_millis(300)).await;

        let settings = unsafe {
            let env6: ICoreWebView2Environment6 = env.cast().map_err(|_| None)?;
            let p = env6.CreatePrintSettings().map_err(|_| None)?;
            let _ = p.SetOrientation(COREWEBVIEW2_PRINT_ORIENTATION_PORTRAIT);
            let _ = p.SetPageWidth(PAPER.0);
            let _ = p.SetPageHeight(PAPER.1);
            let _ = p.SetMarginLeft(MARGIN_X);
            let _ = p.SetMarginRight(MARGIN_X);
            let _ = p.SetMarginTop(MARGIN_Y);
            let _ = p.SetMarginBottom(MARGIN_Y);
            let _ = p.SetScaleFactor(1.);
            let _ = p.SetShouldPrintBackgrounds(true);
            let _ = p.SetShouldPrintHeaderAndFooter(false);
            p
        };
        match job {
            Job::Pdf(path) => {
                let done: Slot<bool> = Rc::default();
                let s = done.clone();
                unsafe {
                    let web7: ICoreWebView2_7 = web.cast().map_err(|_| None)?;
                    web7.PrintToPdf(
                        &HSTRING::from(path.as_os_str()),
                        &settings,
                        &PrintToPdfCompletedHandler::create(Box::new(move |hr, ok| {
                            *s.borrow_mut() = Some(hr.is_ok() && ok);
                            Ok(())
                        })),
                    )
                    .map_err(|_| None)?;
                }
                Ok(wait(&done, None, cx).await.unwrap_or(false))
            }
            Job::Print => {
                let c = choice.ok_or(None)?;
                let done: Slot<bool> = Rc::default();
                let s = done.clone();
                unsafe {
                    let p2: ICoreWebView2PrintSettings2 = settings.cast().map_err(|_| None)?;
                    let _ = p2.SetMediaSize(COREWEBVIEW2_PRINT_MEDIA_SIZE_CUSTOM);
                    let _ = p2.SetPrinterName(&HSTRING::from(c.printer.as_str()));
                    let _ = p2.SetCopies(c.copies);
                    let _ = p2.SetCollation(if c.collate { COREWEBVIEW2_PRINT_COLLATION_COLLATED } else { COREWEBVIEW2_PRINT_COLLATION_UNCOLLATED });
                    let _ = p2.SetColorMode(if c.gray { COREWEBVIEW2_PRINT_COLOR_MODE_GRAYSCALE } else { COREWEBVIEW2_PRINT_COLOR_MODE_COLOR });
                    let _ = p2.SetDuplex(c.duplex);
                    if !c.pages.is_empty() {
                        let _ = p2.SetPageRanges(&HSTRING::from(c.pages.as_str()));
                    }
                    let web16: ICoreWebView2_16 = web.cast().map_err(|_| None)?;
                    web16
                        .Print(
                            &settings,
                            &PrintCompletedHandler::create(Box::new(move |hr, status| {
                                *s.borrow_mut() = Some(hr.is_ok() && status == COREWEBVIEW2_PRINT_STATUS_SUCCEEDED);
                                Ok(())
                            })),
                        )
                        .map_err(|_| None)?;
                }
                if wait(&done, None, cx).await.unwrap_or(false) { Ok(true) } else { Err(None) }
            }
        }
    }
}
