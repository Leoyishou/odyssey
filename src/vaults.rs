//! Each vault runs in its own process: preferences, assets and indexes never cross roots.
use crate::{app::Workspace, settings::tr, ui::ToastKind};
use gpui::*;
use serde::{Deserialize, Serialize};
use std::{fs, path::{Path, PathBuf}};

pub(crate) static STARTUP_NOTICE: std::sync::OnceLock<String> = std::sync::OnceLock::new();

#[derive(Default, Serialize, Deserialize)]
struct Registry { recent: Vec<PathBuf>, last: Option<PathBuf> }

fn state_dir() -> PathBuf {
    std::env::var_os("ODYSSEY_STATE_HOME").map(PathBuf::from).unwrap_or_else(|| {
        if std::env::var_os("FDL_SCRIPT").is_some() { return crate::store::Store::root_dir().join(".test-app-state"); }
        dirs::config_dir().unwrap_or_else(|| PathBuf::from(".")).join("Odyssey")
    })
}
fn read_at(dir: &Path) -> Registry {
    fs::read(dir.join("vaults.json")).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default()
}
pub fn recent() -> Vec<PathBuf> { read_at(&state_dir()).recent }
fn edit_registry(dir: &Path, f: impl FnOnce(&mut Registry)) -> anyhow::Result<()> {
    use std::io::Write;
    fs::create_dir_all(dir)?;
    let lock = fs::OpenOptions::new().create(true).truncate(false).write(true).open(dir.join("vaults.lock"))?;
    lock.lock()?;
    let mut r = read_at(dir); f(&mut r);
    let tmp = dir.join(format!("vaults-{}.tmp", uuid::Uuid::new_v4()));
    let result = (|| -> anyhow::Result<()> {
        let mut file = fs::File::create(&tmp)?;
        file.write_all(&serde_json::to_vec_pretty(&r)?)?; file.sync_all()?;
        fs::rename(&tmp, dir.join("vaults.json"))?; Ok(())
    })();
    if result.is_err() { let _ = fs::remove_file(tmp); }
    result
}
pub fn remember(path: &Path) -> anyhow::Result<()> {
    let path = path.canonicalize()?;
    edit_registry(&state_dir(), |r| {
        r.recent.retain(|p| p != &path); r.recent.insert(0, path.clone()); r.recent.truncate(12); r.last = Some(path);
    })
}
pub fn forget(path: &Path) -> anyhow::Result<()> {
    edit_registry(&state_dir(), |r| { r.recent.retain(|p| p != path); if r.last.as_deref() == Some(path) { r.last = None; } })
}

/// Called at the very start of main, before GPUI or worker threads exist.
pub fn startup() -> anyhow::Result<()> {
    let mut args = std::env::args_os().skip(1);
    let explicit = match args.next() {
        Some(a) if a == "--vault" => Some(PathBuf::from(args.next().ok_or_else(|| anyhow::anyhow!("--vault 缺少目录"))?)),
        _ => None,
    };
    let env_root = std::env::var_os("ODYSSEY_HOME").or_else(|| std::env::var_os("QUIET_MD_HOME"));
    let strict = explicit.is_some();
    let requested = strict || env_root.is_some();
    let selected = explicit.or_else(|| env_root.clone().map(PathBuf::from)).or_else(|| read_at(&state_dir()).last);
    if let Some(path) = selected {
        if !strict && env_root.is_some() { validate_initial_root(&path)?; if !path.join("format.json").exists() { return Ok(()); } }
        let info = match fdl_core::vault::inspect(&path) {
            Ok(info) => info,
            Err(e) if !requested => {
                let _ = STARTUP_NOTICE.set(format!("上次文档库不可用：{e}。已打开默认库，请到文档库设置重新定位。"));
                return validate_initial_root(&crate::store::Store::root_dir());
            },
            Err(e) => return Err(e),
        };
        // No threads exist yet. Never change this process's root after startup.
        unsafe { std::env::set_var("ODYSSEY_HOME", info.path); }
    }
    validate_initial_root(&crate::store::Store::root_dir())
}

fn validate_initial_root(path: &Path) -> anyhow::Result<()> {
    if !path.try_exists()? { return Ok(()); }
    if path.is_dir() && fs::read_dir(path)?.next().is_none() { return Ok(()); }
    fdl_core::vault::inspect(path).map(|_| ())
}

fn pick_folder(create: bool, cx: &mut App) -> impl std::future::Future<Output = Option<PathBuf>> + use<> {
    let picked = crate::devtools::take_pick();
    let open = (picked.is_none() && !create).then(|| cx.prompt_for_paths(PathPromptOptions {
        files: false, directories: true, multiple: false, prompt: Some(tr("打开文档库", "Open vault").into()), extensions: None,
    }));
    let new = (picked.is_none() && create).then(|| cx.prompt_for_new_path(&dirs::document_dir().unwrap_or_default(), Some("Odyssey Vault")));
    async move {
        if let Some(p) = picked { return p.into_iter().next(); }
        if let Some(p) = open { return p.await.ok().and_then(Result::ok).flatten().and_then(|p| p.into_iter().next()); }
        if let Some(p) = new { return p.await.ok().and_then(Result::ok).flatten(); }
        None
    }
}

impl Workspace {
    pub(crate) fn pick_vault(&mut self, create: bool, window: &mut Window, cx: &mut Context<Self>) {
        let pick = pick_folder(create, cx);
        cx.spawn_in(window, async move |this, cx| {
            if let Some(path) = pick.await {
                let _ = this.update_in(cx, |w, window, cx| w.open_vault(path, create, window, cx));
            }
        }).detach();
    }

    pub(crate) fn open_vault(&mut self, path: PathBuf, create: bool, window: &mut Window, cx: &mut Context<Self>) {
        let retry = path.clone();
        if !self.save_before(cx, move |w, window, cx| w.open_vault(retry, create, window, cx)) { return; }
        let result = (|| -> anyhow::Result<()> {
            let info = if create { fdl_core::vault::create(&path)? } else { fdl_core::vault::inspect(&path)? };
            if info.path == crate::store::Store::root_dir().canonicalize()? {
                self.show_toast(tr("该库已在当前窗口打开", "This vault is already open here"), cx); return Ok(());
            }
            if std::env::var_os("FDL_SCRIPT").is_some() {
                // Scripted UI tests exercise save/validation/selection without opening a foreground window.
                let dir = std::env::var_os("FDL_SHOTS").map(PathBuf::from).ok_or_else(|| anyhow::anyhow!("FDL_SHOTS missing"))?;
                fs::write(dir.join("vault-launch.json"), serde_json::to_vec(&info.path)?)?;
            } else {
                std::process::Command::new(std::env::current_exe()?).arg("--vault").arg(&info.path)
                    .env_remove("ODYSSEY_HOME").env_remove("QUIET_MD_HOME").spawn()?;
            }
            self.show_toast(tr("正在打开文档库新窗口，当前文档已保存", "Opening vault in a new window; current document saved"), cx);
            Ok(())
        })();
        if let Err(e) = result { self.show_toast_kind(ToastKind::Error, &e.to_string(), cx); }
        let _ = window; cx.notify();
    }

    pub(crate) fn forget_vault(&mut self, path: &Path, cx: &mut Context<Self>) {
        match forget(path) {
            Ok(()) => self.show_toast(tr("已从最近库移除，文件未删除", "Removed from recent vaults; files kept"), cx),
            Err(e) => self.show_toast_kind(ToastKind::Error, &e.to_string(), cx),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Registry, edit_registry, read_at};
    #[test]
    fn startup_rejects_non_vault_environment_paths_without_writes() {
        let dir = std::env::temp_dir().join(format!("odyssey-startup-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir(&dir).unwrap();
        assert!(super::validate_initial_root(&dir).is_ok());
        std::fs::write(dir.join("note.md"), "user content").unwrap();
        assert!(super::validate_initial_root(&dir).is_err());
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);
        assert_eq!(std::fs::read_to_string(dir.join("note.md")).unwrap(), "user content");
        std::fs::write(dir.join("format.json"), "{\"format\":999}").unwrap();
        assert!(super::validate_initial_root(&dir).is_err());
        assert!(!dir.join("docs").exists());
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn registry_serializes_without_deleting_vault_files() {
        let dir = std::env::temp_dir().join(format!("odyssey-registry-{}", uuid::Uuid::new_v4()));
        let vault = dir.join("vault"); std::fs::create_dir_all(&vault).unwrap();
        std::fs::write(vault.join("keep"), "note").unwrap();
        edit_registry(&dir, |r| { r.recent = vec![vault.clone()]; r.last = Some(vault.clone()); }).unwrap();
        assert_eq!(read_at(&dir).last, Some(vault.clone()));
        edit_registry(&dir, |r| *r = Registry::default()).unwrap();
        assert!(read_at(&dir).recent.is_empty()); assert_eq!(std::fs::read_to_string(vault.join("keep")).unwrap(), "note");
        std::fs::remove_dir_all(dir).unwrap();
    }
}
