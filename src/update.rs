//! 自动更新. A copy installed from a release package reads
//! `https://download.quiet.md/latest/update-<platform>.json` (written last by
//! scripts/publish-downloads.sh) 10s after launch and every 12h while
//! 自动检查更新 is on; 检查更新… asks at once. Nothing is downloaded until the
//! user asks: the package is fetched with `curl`, its minisign signature checked
//! against packaging/update-key.pub (the trusted comment must be the package's
//! own file name, so an old signed package cannot pose as a newer version),
//! and on macOS the unpacked app must also carry this copy's code signature
//! team. 重启以更新 quits through the usual save path; a detached helper waits
//! for this process to exit, then swaps the bundle (macOS) or runs the Inno
//! installer silently (Windows), and starts the new version.
//!
//! `QUIET_MD_UPDATE_URL` points the check at another folder (testing; it also
//! enables the check in unsigned builds). The public key can be replaced at
//! build time with `QUIET_MD_UPDATE_PUBKEY`.

use gpui::*;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

const BASE: &str = "https://download.quiet.md/latest";
const KEY: &str = match option_env!("QUIET_MD_UPDATE_PUBKEY") { Some(k) => k, None => include_str!("../packaging/update-key.pub") };

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Manifest {
    pub version: String,
    pub url: String,
    pub size: u64,
    /// The `.minisig` file's text.
    pub signature: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum State {
    Idle,
    /// This copy cannot update itself (a development build, an unsigned app, another platform).
    Unsupported,
    Checking,
    Current,
    Available(Manifest),
    Downloading(Manifest),
    /// Verified and unpacked: the new `.app` (macOS) or the installer (Windows).
    Ready(Manifest, PathBuf),
    /// The helper waits for this process to exit (a failed save may have kept it open).
    Installing(Manifest),
    Failed(String),
}

/// `last`: when the latest automatic check started.
pub struct Updater { pub state: State, told: Option<String>, last: Option<std::time::Instant> }
impl Global for Updater {}

pub fn state(cx: &App) -> State { cx.try_global::<Updater>().map_or(State::Idle, |u| u.state.clone()) }

fn set(state: State, cx: &mut App) {
    cx.global_mut::<Updater>().state = state;
    cx.refresh_windows();
}

/// Starts the periodic check.
pub fn init(cx: &mut App) {
    cx.set_global(Updater { state: State::Idle, told: None, last: None });
    cx.spawn(async |cx| {
        cx.background_executor().timer(Duration::from_secs(10)).await;
        loop {
            if cx.update(|cx| if crate::settings::prefs().auto_update { check(false, cx) }).is_err() { break }
            cx.background_executor().timer(Duration::from_secs(12 * 3600)).await;
        }
    }).detach();
}

/// A window came to the front: check again if the last check is over an
/// hour old, so a copy left open learns of a release within hours, not 12.
pub fn check_if_stale(cx: &mut App) {
    let Some(u) = cx.try_global::<Updater>() else { return };
    // Before the first check (10s after launch) the timer has it.
    if !crate::settings::prefs().auto_update || u.last.is_none_or(|t| t.elapsed() < Duration::from_secs(3600)) { return }
    check(false, cx);
}

/// Asks the server for the newest version. `manual` (检查更新…) also reports
/// failures and an up-to-date copy; the automatic check stays silent except
/// for a toast, once per version, when there is something new.
pub fn check(manual: bool, cx: &mut App) {
    match state(cx) {
        // A finished download waits for 重启以更新.
        State::Checking | State::Downloading(_) | State::Ready(..) | State::Installing(_) => return,
        State::Unsupported if !manual => return,
        _ => {}
    }
    if manual { set(State::Checking, cx); } else { cx.global_mut::<Updater>().last = Some(std::time::Instant::now()); }
    let task = cx.background_executor().spawn(async { fetch_manifest() });
    cx.spawn(async move |cx| {
        let result = task.await;
        let _ = cx.update(|cx| {
            let state = match result {
                Ok(Some(m)) => State::Available(m),
                Ok(None) => State::Current,
                Err(Error::Unsupported) => State::Unsupported,
                Err(Error::Failed(e)) if manual => State::Failed(e),
                // A quiet retry at the next check; keep whatever was shown.
                Err(Error::Failed(_)) => return,
            };
            if let State::Available(m) = &state && !manual && cx.global::<Updater>().told.as_ref() != Some(&m.version) {
                cx.global_mut::<Updater>().told = Some(m.version.clone());
                announce(&m.version, cx);
            }
            set(state, cx);
        });
    }).detach();
}

/// A toast in the front window: the version and the sidebar's 更新 pill.
fn announce(version: &str, cx: &mut App) {
    let msg = crate::localized_format!("Quiet.md {} 已发布，点侧栏顶部的「更新」安装", "Quiet.md {} is available. Click Update at the top of the sidebar.", version);
    let window = cx.active_window().or_else(|| cx.windows().first().copied()).and_then(|w| w.downcast::<crate::app::Workspace>());
    if let Some(w) = window { let _ = w.update(cx, |ws, _, cx| ws.show_toast_kind(crate::ui::ToastKind::Info, &msg, cx)); }
}

/// How much of `m` has arrived, 0–1, read off the partial file.
pub fn progress(m: &Manifest) -> f32 {
    let Some(platform) = platform() else { return 0. };
    let part = updates_dir().join(format!("{}.part", package_name(&m.version, platform)));
    let got = std::fs::metadata(part).map_or(0, |d| d.len());
    if m.size == 0 { 0. } else { (got as f32 / m.size as f32).clamp(0., 1.) }
}

/// Downloads and verifies the version on offer.
pub fn download(cx: &mut App) {
    let State::Available(m) = state(cx) else { return };
    set(State::Downloading(m.clone()), cx);
    // Redraw the progress (read off the partial file) while the download runs.
    cx.spawn(async |cx| loop {
        cx.background_executor().timer(Duration::from_millis(300)).await;
        if !cx.update(|cx| if matches!(state(cx), State::Downloading(_)) { cx.refresh_windows(); true } else { false }).unwrap_or(false) { break }
    }).detach();
    let task = cx.background_executor().spawn({ let m = m.clone(); async move { fetch_package(&m) } });
    cx.spawn(async move |cx| {
        let result = task.await;
        let _ = cx.update(|cx| set(match result { Ok(path) => State::Ready(m, path), Err(e) => State::Failed(e) }, cx));
    }).detach();
}

/// Hands the downloaded version to a helper that installs it once this
/// process has exited, then quits (asking first if a document did not save).
pub fn install(cx: &mut App) {
    let State::Ready(m, path) = state(cx) else { return };
    match start_installer(&path) {
        // After the click's window update: the quit asks every window to save.
        Ok(()) => { set(State::Installing(m), cx); cx.defer(crate::app::quit); }
        Err(e) => set(State::Failed(e), cx),
    }
}

// ---- the server

enum Error { Unsupported, Failed(String) }

fn platform() -> Option<&'static str> {
    if cfg!(all(target_os = "macos", target_arch = "aarch64")) { Some("macos-arm64") }
    else if cfg!(all(target_os = "windows", target_arch = "x86_64")) { Some("windows-x64") }
    else { None }
}

fn package_name(version: &str, platform: &str) -> String {
    if platform.starts_with("windows") { format!("quiet-md-{version}-{platform}-setup.exe") } else { format!("quiet-md-{version}-{platform}.zip") }
}

fn override_url() -> Option<String> { std::env::var("QUIET_MD_UPDATE_URL").ok().filter(|u| !u.is_empty()) }

fn public_key() -> Option<minisign_verify::PublicKey> {
    minisign_verify::PublicKey::decode(KEY.trim()).ok().or_else(|| minisign_verify::PublicKey::from_base64(KEY.trim()).ok())
}

/// `a` is a later version than `b` (dotted numbers; a missing part is 0).
pub(crate) fn newer(a: &str, b: &str) -> bool {
    let parts = |v: &str| -> Option<Vec<u64>> { v.split('.').map(|p| p.parse().ok()).collect() };
    let (Some(mut a), Some(mut b)) = (parts(a), parts(b)) else { return false };
    let n = a.len().max(b.len());
    a.resize(n, 0);
    b.resize(n, 0);
    a > b
}

/// This copy can replace itself: an installed, release-signed build.
fn supported() -> bool {
    // Forks must never install an upstream Quiet.md release.
    if option_env!("ODYSSEY_ENABLE_UPDATER") != Some("1") { return false }
    if platform().is_none() || public_key().is_none() { return false }
    if override_url().is_some() { return true }
    if std::env::var_os("FDL_SCRIPT").is_some() { return false }
    #[cfg(target_os = "macos")]
    { app_bundle().is_some_and(|app| team(&app).is_some()) }
    #[cfg(target_os = "windows")]
    { std::env::current_exe().ok().and_then(|e| e.parent().map(|d| d.join("unins000.exe").is_file())).unwrap_or(false) }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    { false }
}

/// The newest version when it is later than this one.
fn fetch_manifest() -> Result<Option<Manifest>, Error> {
    if !supported() { return Err(Error::Unsupported) }
    let platform = platform().unwrap();
    let url = format!("{}/update-{platform}.json", override_url().as_deref().unwrap_or(BASE).trim_end_matches('/'));
    let body = curl(&["--max-time", "20", &url]).map_err(Error::Failed)?;
    let m: Manifest = serde_json::from_slice(&body).map_err(|e| Error::Failed(format!("{}: {e}", crate::settings::tr("更新信息无法识别", "Unrecognised update information"))))?;
    validate(&m, platform, override_url().is_some()).map_err(Error::Failed)?;
    Ok(newer(&m.version, env!("CARGO_PKG_VERSION")).then_some(m))
}

/// The manifest names this platform's package for its version, over HTTPS,
/// with a signature for that very file.
fn validate(m: &Manifest, platform: &str, testing: bool) -> Result<(), String> {
    let name = package_name(&m.version, platform);
    let bad = |what: &str| Err(format!("{}: {what}", crate::settings::tr("更新信息无效", "Invalid update information")));
    if m.version.split('.').any(|p| p.parse::<u64>().is_err()) { return bad("version") }
    if !(m.url.starts_with("https://") || testing) || !m.url.ends_with(&format!("/{name}")) { return bad("url") }
    if m.size == 0 || m.size > 1 << 30 { return bad("size") }
    match minisign_verify::Signature::decode(&m.signature) { Ok(sig) if sig.trusted_comment() == name => Ok(()), _ => bad("signature") }
}

/// `bytes` carry a signature by `key` whose trusted comment is the package name.
fn signed(bytes: &[u8], signature: &str, name: &str, key: &minisign_verify::PublicKey) -> bool {
    minisign_verify::Signature::decode(signature).is_ok_and(|sig| sig.trusted_comment() == name && key.verify(bytes, &sig, false).is_ok())
}

fn updates_dir() -> PathBuf {
    let base = dirs::cache_dir().unwrap_or_else(std::env::temp_dir);
    if cfg!(target_os = "macos") { base.join("com.s1dashu.quietmd").join("updates") } else { base.join("quiet.md").join("updates") }
}

/// Downloads, checks the signature and (macOS) unpacks; the path to install.
fn fetch_package(m: &Manifest) -> Result<PathBuf, String> {
    let platform = platform().ok_or("platform")?;
    let name = package_name(&m.version, platform);
    let dir = updates_dir();
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let file = dir.join(&name);
    let part = dir.join(format!("{name}.part"));
    // curl gets no path: a Windows curl.exe reads its arguments in the ANSI
    // code page, which cannot spell every user folder name (中文 included).
    let out = std::fs::File::create(&part).map_err(|e| e.to_string())?;
    curl_to(&["--max-time", "900", &m.url], Stdio::from(out))?;
    let bytes = std::fs::read(&part).map_err(|e| e.to_string())?;
    let failed = |what: &str| format!("{}: {what}", crate::settings::tr("更新包校验失败", "The update package failed verification"));
    if bytes.len() as u64 != m.size { return Err(failed("size")) }
    let key = public_key().ok_or_else(|| failed("key"))?;
    if !signed(&bytes, &m.signature, &name, &key) { return Err(failed("signature")) }
    drop(bytes);
    std::fs::rename(&part, &file).map_err(|e| e.to_string())?;
    #[cfg(target_os = "macos")]
    {
        let unpacked = dir.join("app");
        run(Command::new("/usr/bin/ditto").args(["-x", "-k"]).arg(&file).arg(&unpacked))?;
        let app = unpacked.join("quiet.md.app");
        run(Command::new("/usr/bin/codesign").args(["--verify", "--deep", "--strict"]).arg(&app)).map_err(|_| failed("codesign"))?;
        // Signed by the same team as this copy (unsigned only against unsigned, when testing).
        let mine = app_bundle().and_then(|a| team(&a));
        if team(&app) != mine || mine.is_none() && override_url().is_none() { return Err(failed("team")) }
        let _ = std::fs::remove_file(&file);
        Ok(app)
    }
    #[cfg(not(target_os = "macos"))]
    Ok(file)
}

fn curl(args: &[&str]) -> Result<Vec<u8>, String> { curl_to(args, Stdio::piped()) }

/// `stdout`: piped (returned) or a file to download into.
fn curl_to(args: &[&str], stdout: Stdio) -> Result<Vec<u8>, String> {
    let out = command("curl").args(["-fsSL", "--proto", "=https,http", "--connect-timeout", "15"]).args(args)
        .stdin(Stdio::null()).stdout(stdout).stderr(Stdio::piped()).output()
        .map_err(|e| format!("curl: {e}"))?;
    if out.status.success() { return Ok(out.stdout) }
    let msg = String::from_utf8_lossy(&out.stderr).trim().trim_start_matches("curl: ").to_string();
    Err(format!("{}: {msg}", crate::settings::tr("无法连接更新服务器", "Could not reach the update server")))
}

/// A command that opens no console window on Windows.
fn command(program: impl AsRef<std::ffi::OsStr>) -> Command {
    #[allow(unused_mut)]
    let mut c = Command::new(program);
    #[cfg(target_os = "windows")]
    { use std::os::windows::process::CommandExt; c.creation_flags(0x0800_0000); }
    c
}

#[cfg(target_os = "macos")]
fn run(c: &mut Command) -> Result<(), String> {
    let out = c.stdin(Stdio::null()).output().map_err(|e| e.to_string())?;
    if out.status.success() { Ok(()) } else { Err(String::from_utf8_lossy(&out.stderr).trim().to_string()) }
}

// ---- installing

/// The `.app` this process runs from.
#[cfg(target_os = "macos")]
fn app_bundle() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?.canonicalize().ok()?;
    let app = exe.parent()?.parent()?.parent()?;
    (app.extension()? == "app").then(|| app.to_path_buf())
}

/// The code signature's team, None for ad-hoc and unsigned code.
#[cfg(target_os = "macos")]
fn team(app: &Path) -> Option<String> {
    let out = Command::new("/usr/bin/codesign").args(["-d", "-v"]).arg(app).stdin(Stdio::null()).output().ok()?;
    let text = String::from_utf8_lossy(&out.stderr);
    let id = text.lines().find_map(|l| l.strip_prefix("TeamIdentifier="))?.trim();
    (id != "not set" && !id.is_empty()).then(|| id.to_string())
}

/// Waits for `pid` to exit, swaps the bundles (putting the old one back if the
/// new one cannot move in) and opens whichever is in place. `open` starts apps
/// without the caller's environment, so a copy run with its own library
/// (`QUIET_MD_HOME`) or update folder gets them back through `--env`.
#[cfg(target_os = "macos")]
const SWAP: &str = r#"pid=$1 app=$2 new=$3 old=$4
shift 4
while kill -0 "$pid" 2>/dev/null; do sleep 0.2; done
if mv "$app" "$old"; then
  if mv "$new" "$app"; then rm -rf "$old"; else mv "$old" "$app"; fi
fi
/usr/bin/open -n "$@" "$app""#;

#[cfg(target_os = "macos")]
fn start_installer(new: &Path) -> Result<(), String> {
    use std::os::unix::process::CommandExt;
    let app = app_bundle().ok_or_else(|| crate::settings::tr("找不到正在运行的应用", "Could not find the running app").to_string())?;
    let parent = app.parent().ok_or("parent")?;
    if app.to_string_lossy().contains("/AppTranslocation/") {
        return Err(crate::settings::tr("请先把 Quiet.md 移到「应用程序」文件夹，再更新", "Move Quiet.md to the Applications folder first, then update").into());
    }
    // The folder must take the new bundle (a standard user cannot write to /Applications).
    let probe = parent.join(format!(".quiet.md-update-{}", std::process::id()));
    if std::fs::create_dir(&probe).is_err() {
        return Err(crate::localized_format!("没有写入 {} 的权限，请从网站下载新版本", "No permission to write to {}. Download the new version from the website instead.", parent.display()));
    }
    let _ = std::fs::remove_dir(&probe);
    let old = parent.join(format!(".quiet.md-old-{}.app", std::process::id()));
    let env = ["QUIET_MD_HOME", "QUIET_MD_UPDATE_URL"].into_iter()
        .filter_map(|k| std::env::var(k).ok().map(|v| ["--env".to_string(), format!("{k}={v}")])).flatten();
    Command::new("/bin/sh").arg("-c").arg(SWAP).arg("sh")
        .arg(std::process::id().to_string()).arg(&app).arg(new).arg(&old).args(env)
        .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null())
        .process_group(0).spawn().map(drop).map_err(|e| e.to_string())
}

/// PowerShell waits for this process, then runs the installer silently;
/// `/UPDATE=1` makes it start the app again (packaging/windows/quiet-md.iss).
#[cfg(target_os = "windows")]
fn start_installer(setup: &Path) -> Result<(), String> {
    let quote = |s: &str| format!("'{}'", s.replace('\'', "''"));
    let script = format!(
        "Wait-Process -Id {} -ErrorAction SilentlyContinue; Start-Process -FilePath {} -ArgumentList '/VERYSILENT','/SUPPRESSMSGBOXES','/NORESTART','/CLOSEAPPLICATIONS','/UPDATE=1'",
        std::process::id(), quote(&setup.to_string_lossy()));
    command("powershell.exe").args(["-NoProfile", "-NonInteractive", "-WindowStyle", "Hidden", "-Command", &script])
        .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null())
        .spawn().map(drop).map_err(|e| e.to_string())
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn start_installer(_: &Path) -> Result<(), String> { Err("unsupported".into()) }

#[cfg(test)]
mod tests {
    use super::{newer, signed, validate, Manifest};

    #[test]
    fn versions_compare_numerically() {
        assert!(newer("0.1.10", "0.1.9"));
        assert!(newer("0.2", "0.1.9"));
        assert!(newer("1.0.0", "0.99.99"));
        assert!(!newer("0.1.3", "0.1.3"));
        assert!(!newer("0.1.3.0", "0.1.3"));
        assert!(!newer("0.1.2", "0.1.3"));
        assert!(!newer("0.1.x", "0.1.3"));
    }

    // A throwaway key (not the release key) and its signature of b"quiet.md" as quiet-md-0.2.0-macos-arm64.zip.
    const TEST_KEY: &str = "RWTh6tW62t9YuAQn7kot+dQgLqLst7Y75FmIEJTHVhcBtN0WXRpyTuPV";
    const TEST_SIG: &str = "untrusted comment: signature from minisign secret key
RUTh6tW62t9YuELfpasf2JJ5CdktflamMcMROwLwYH7Ps6KTTQ/OUvViqT/c+L9PB1Egk9FoQqb94hk+x00y66OvPsdZOwwMww4=
trusted comment: quiet-md-0.2.0-macos-arm64.zip
FuE4L1zQnqL1nA/5xffpEtZyK2ca882Q1joA5X11poZ1xezdAlEMfn/36MJc5C60clQFec4vQf66MoxPi+AZCA==
";

    fn manifest(version: &str, url: &str, comment: &str) -> Manifest {
        let signature = TEST_SIG.replace("trusted comment: quiet-md-0.2.0-macos-arm64.zip", &format!("trusted comment: {comment}"));
        Manifest { version: version.into(), url: url.into(), size: 8, signature }
    }

    #[test]
    fn packages_verify_only_under_their_own_name() {
        let key = minisign_verify::PublicKey::from_base64(TEST_KEY).unwrap();
        let name = "quiet-md-0.2.0-macos-arm64.zip";
        assert!(signed(b"quiet.md", TEST_SIG, name, &key));
        assert!(!signed(b"quiet.mD", TEST_SIG, name, &key));
        assert!(!signed(b"quiet.md", TEST_SIG, "quiet-md-0.9.0-macos-arm64.zip", &key));
        // Rewriting the trusted comment breaks the signature.
        let renamed = TEST_SIG.replace("0.2.0", "0.9.0");
        assert!(!signed(b"quiet.md", &renamed, "quiet-md-0.9.0-macos-arm64.zip", &key));
    }

    #[test]
    fn manifests_must_name_the_signed_package() {
        let ok = manifest("0.2.0", "https://download.quiet.md/v0.2.0/quiet-md-0.2.0-macos-arm64.zip", "quiet-md-0.2.0-macos-arm64.zip");
        assert_eq!(validate(&ok, "macos-arm64", false), Ok(()));
        // An older package's signature under a newer version number.
        let replay = manifest("0.9.0", "https://download.quiet.md/v0.2.0/quiet-md-0.2.0-macos-arm64.zip", "quiet-md-0.2.0-macos-arm64.zip");
        assert!(validate(&replay, "macos-arm64", false).is_err());
        let renamed = manifest("0.9.0", "https://download.quiet.md/v0.9.0/quiet-md-0.9.0-macos-arm64.zip", "quiet-md-0.2.0-macos-arm64.zip");
        assert!(validate(&renamed, "macos-arm64", false).is_err());
        let http = manifest("0.2.0", "http://download.quiet.md/v0.2.0/quiet-md-0.2.0-macos-arm64.zip", "quiet-md-0.2.0-macos-arm64.zip");
        assert!(validate(&http, "macos-arm64", false).is_err());
        assert_eq!(validate(&http, "macos-arm64", true), Ok(()));
        let windows = manifest("0.2.0", "https://download.quiet.md/v0.2.0/quiet-md-0.2.0-windows-x64-setup.exe", "quiet-md-0.2.0-windows-x64-setup.exe");
        assert_eq!(validate(&windows, "windows-x64", false), Ok(()));
        assert!(validate(&windows, "macos-arm64", false).is_err());
    }
}
