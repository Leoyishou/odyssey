//! A log file installed before anything else starts, so a failed launch still
//! leaves a trace: the Windows build has no console for `eprintln!`. Paths stay
//! `PathBuf` end to end (wide APIs on Windows), never UTF-8 or ANSI strings.
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

static FILE: OnceLock<Mutex<File>> = OnceLock::new();
static OPENED: OnceLock<PathBuf> = OnceLock::new();

/// `~/Library/Logs/quiet.md` on macOS, `%LOCALAPPDATA%\quiet.md\logs` on
/// Windows; independent of where the program itself lives.
pub fn dir() -> PathBuf {
    if let Some(d) = std::env::var_os("QUIET_MD_LOG_DIR") { return PathBuf::from(d) }
    #[cfg(target_os = "macos")]
    let base = dirs::home_dir().map(|h| h.join("Library").join("Logs").join("quiet.md"));
    #[cfg(not(target_os = "macos"))]
    let base = dirs::data_local_dir().map(|d| d.join("quiet.md").join("logs"));
    base.unwrap_or_else(|| std::env::temp_dir().join("quiet.md-logs"))
}

/// The file actually written (the temp fallback included).
pub fn path() -> PathBuf { OPENED.get().cloned().unwrap_or_else(|| dir().join("quiet-md.log")) }

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, m: &log::Metadata) -> bool { m.level() <= log::Level::Warn || m.target().starts_with("quiet_md") }
    fn log(&self, r: &log::Record) {
        if self.enabled(r.metadata()) { write(&format!("{} {}: {}", r.level(), r.target(), r.args())); }
    }
    fn flush(&self) {}
}

/// Opens the log (trimmed when large), captures `log` records (GPUI's
/// included) and panics. Falls back to the temp directory when the usual one
/// cannot be created; never fails the launch.
pub fn init() {
    let open = |dir: PathBuf| -> std::io::Result<File> {
        std::fs::create_dir_all(&dir)?;
        let p = dir.join("quiet-md.log");
        let big = std::fs::metadata(&p).is_ok_and(|m| m.len() > 1 << 20);
        if big { let _ = std::fs::rename(&p, dir.join("quiet-md.old.log")); }
        let f = OpenOptions::new().create(true).append(true).open(&p)?;
        let _ = OPENED.set(p);
        Ok(f)
    };
    if let Ok(f) = open(dir()).or_else(|_| open(std::env::temp_dir().join("quiet.md-logs"))) { let _ = FILE.set(Mutex::new(f)); }
    let _ = log::set_logger(&Logger).map(|()| log::set_max_level(log::LevelFilter::Info));
    let default = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        write(&format!("PANIC {info}\n{}", std::backtrace::Backtrace::force_capture()));
        default(info);
    }));
    let exe = std::env::current_exe().map(|p| p.display().to_string()).unwrap_or_default();
    write(&format!("start Odyssey {} ({exe})", env!("CARGO_PKG_VERSION")));
}

/// One line to the log file and stderr.
pub fn write(line: &str) {
    eprintln!("{line}");
    let Some(f) = FILE.get() else { return };
    let Ok(mut f) = f.lock() else { return };
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let _ = writeln!(f, "[{secs}] {line}");
    let _ = f.flush();
}

/// A launch that cannot continue: logged, shown (Windows has no console),
/// exit code 1.
pub fn fatal(what: &str, err: &anyhow::Error) -> ! {
    write(&format!("ERROR {what}: {err:#} (log: {})", path().display()));
    #[cfg(target_os = "windows")]
    {
        let text = format!("{what}\n\n{err:#}\n\n日志 / Log: {}", path().display());
        let wide = |s: &str| s.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
        #[link(name = "user32")]
        unsafe extern "system" { fn MessageBoxW(hwnd: *mut std::ffi::c_void, text: *const u16, caption: *const u16, kind: u32) -> i32; }
        // MB_OK | MB_ICONERROR
        unsafe { MessageBoxW(std::ptr::null_mut(), wide(&text).as_ptr(), wide("Odyssey").as_ptr(), 0x10); }
    }
    std::process::exit(1)
}
