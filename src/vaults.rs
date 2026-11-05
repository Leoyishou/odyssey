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
    let recent = read_at(&state_dir()).last;
    let automatic = if !requested {
        let (path, notice) = select_automatic_root(recent, dirs::document_dir().map(|p| p.join("Odyssey")));
        if let Some(notice) = notice { let _ = STARTUP_NOTICE.set(notice); }
        path
    } else { None };
    let selected = explicit.or_else(|| env_root.clone().map(PathBuf::from)).or(automatic);
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

/// Read-only selection, separately testable without changing HOME or process state.
fn select_automatic_root(recent: Option<PathBuf>, legacy: Option<PathBuf>) -> (Option<PathBuf>, Option<String>) {
    let mut notices = vec![];
    for (path, required) in recent.into_iter().map(|p| (p, true)).chain(legacy.into_iter().map(|p| (p, false))) {
        if !required && !path.exists() { continue; }
        match fdl_core::vault::inspect(&path) {
            Ok(info) => return (Some(info.path), (!notices.is_empty()).then(|| notices.join("\n"))),
            Err(e) => notices.push(format!("未打开文档库 {}：{e}。没有删除或搬迁原文件；可重新定位，或使用下方“只复制原生文档”将原 XML 复制到独立新库。", path.display())),
        }
    }
    (None, (!notices.is_empty()).then(|| notices.join("\n")))
}

fn validate_initial_root(path: &Path) -> anyhow::Result<()> {
    if !path.try_exists()? { return fdl_core::vault::validate_new_parent(path); }
    if path.is_dir() && fs::read_dir(path)?.next().is_none() {
        anyhow::ensure!(!fs::symlink_metadata(path)?.file_type().is_symlink(), "空目录是符号链接，未初始化文档库：{}", path.display());
        return fdl_core::vault::validate_new_parent(path);
    }
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


impl Workspace {
    pub(crate) fn preview_vault_import(&mut self, native_only: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.vault_import_busy { return; }
        let pick = pick_folder(false, cx);
        cx.spawn_in(window, async move |this, cx| {
            let Some(source) = pick.await else { return };
            let _ = this.update(cx, |w, cx| { w.vault_import_busy = true; cx.notify(); });
            let result = cx.background_executor().spawn(async move { if native_only { fdl_core::vault_import::Plan::recover_native(&source) } else { fdl_core::vault_import::Plan::scan(&source) } }).await;
            let _ = this.update(cx, |w, cx| {
                w.vault_import_busy = false;
                match result {
                    Ok(plan) => w.vault_import = Some(std::sync::Arc::new(plan)),
                    Err(e) => w.show_toast_kind(ToastKind::Error, &e.to_string(), cx),
                }
                cx.notify();
            });
        }).detach();
    }

    pub(crate) fn confirm_vault_import(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.vault_import_busy { return; }
        if !self.save_before(cx, |w, window, cx| w.confirm_vault_import(window, cx)) { return; }
        let Some(plan) = self.vault_import.clone() else { return };
        let pick = pick_folder(true, cx);
        cx.spawn_in(window, async move |this, cx| {
            let Some(target) = pick.await else { return };
            let _ = this.update(cx, |w, cx| { w.vault_import_busy = true; cx.notify(); });
            let result = cx.background_executor().spawn(async move { plan.write_new(&target) }).await;
            let _ = this.update_in(cx, |w, window, cx| {
                w.vault_import_busy = false;
                match result {
                    Ok(path) => { w.vault_import = None; w.open_vault(path, false, window, cx); },
                    Err(e) => w.show_toast_kind(ToastKind::Error, &e.to_string(), cx),
                }
                cx.notify();
            });
        }).detach();
    }

    pub(crate) fn render_vault_import(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        if self.confirm.is_some() { return None; }
        let plan = self.vault_import.as_ref()?;
        let report = &plan.report; let busy = self.vault_import_busy;
        let summary = format!("{} Markdown · {} XML · {} {} · {} {} · {} {} (+{} {}) · {} {}", report.notes, report.native_notes,
            report.folders, tr("层级目录页", "folder pages"), report.resolved_links, tr("已解析双链", "resolved links"), report.attachments, tr("本地附件", "local attachments"), report.native_attachments, tr("原生", "native"), report.native_trash, tr("回收站文档", "trashed documents"));
        let body = crate::ui::dialog("vault-import-dialog").w(px(620.)).max_w_full().max_h_full().overflow_y_scroll().p(px(24.)).flex().flex_col().gap(px(12.))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(div().text_size(px(16.)).font_weight(FontWeight::SEMIBOLD).child(tr("复制导入预览", "Copy import preview")))
            .child(div().text_size(px(12.)).text_color(crate::theme::Theme::text_caption()).child(plan.source.display().to_string()))
            .child(div().text_size(px(14.)).child(summary))
            .child(div().text_size(px(13.)).child(tr("源目录不会修改。原生 XML 保留 UUID 和字节，索引层级、附件、设置与回收站复制到新库；XML 和元数据原件在 .odyssey-import/native。Markdown 原件在 .odyssey-import/source，完整报告为 .odyssey-import/report.json。", "The source is unchanged. Native XML retains IDs and bytes; metadata, assets, settings and trash are copied. Native originals are archived under .odyssey-import/native; Markdown originals under .odyssey-import/source, with report.json beside them.")))
            .child(div().text_size(px(13.)).child(tr("只转换基础 Markdown、可解析的文档链接及本地附件。属性、插件、文档嵌入和标题/块锚点不保证保真；原文保留供核对。", "Only basic Markdown, resolvable document links and local assets are converted. Properties, plugins, note embeds and heading/block anchors are not guaranteed; originals are retained for comparison.")))
            .child(div().id("vault-import-warnings").max_h(px(240.)).overflow_y_scroll().flex().flex_col().gap(px(6.))
                .children(report.warnings.iter().take(200).map(|w| div().text_size(px(12.)).text_color(crate::theme::Theme::text_caption()).child(format!("{}: {}", w.file, w.detail)))))
            .child(div().text_size(px(12.)).child(format!("{} {}", report.warnings.len(), tr("条转换提示（此处最多显示200条，完整报告将写入新库）", "conversion notices (up to 200 shown; full report saved in the new vault)"))))
            .child(div().flex().justify_end().gap(px(12.))
                .child(crate::ui::secondary_button("vault-import-cancel", tr("取消", "Cancel")).on_click(cx.listener(|w, _, _, cx| { if !w.vault_import_busy { w.vault_import = None; cx.notify(); } })))
                .child(crate::ui::primary_button("vault-import-confirm", if busy { tr("正在复制…", "Copying…") } else { tr("选择新目录并复制导入", "Choose new location and copy") }).on_click(cx.listener(|w, _, window, cx| w.confirm_vault_import(window, cx)))));
        Some(deferred(div().absolute().inset_0().flex().items_center().justify_center().p(px(16.)).bg(crate::ui::scrim()).child(body)).with_priority(5).into_any_element())
    }
}

#[cfg(test)]
mod tests {
    use super::{Registry, edit_registry, read_at};
    #[test]
    fn pure_legacy_default_remains_discoverable_and_opens_in_place() {
        let dir = std::env::temp_dir().join(format!("odyssey-legacy-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir(&dir).unwrap(); let old = dir.join("Odyssey");
        fdl_core::vault::create(&old).unwrap();
        let xml = "<title>Existing note</title><p>Original content</p>";
        std::fs::write(old.join("docs/keep.xml"), xml).unwrap();
        let (selected, notice) = super::select_automatic_root(None, Some(old.clone()));
        assert_eq!(selected, Some(old.canonicalize().unwrap())); assert!(notice.is_none());
        let store = fdl_core::store::Store::open_at(selected.unwrap()).unwrap();
        assert_eq!(store.load("keep").unwrap().title.text, "Existing note");
        assert_eq!(std::fs::read_to_string(old.join("docs/keep.xml")).unwrap(), xml);
        assert!(!old.join(".lock").exists());
        let (fallback, notice) = super::select_automatic_root(Some(dir.join("moved")), Some(old.clone()));
        assert_eq!(fallback, Some(old.canonicalize().unwrap())); assert!(notice.is_some());
        std::fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn mixed_legacy_is_declined_without_hiding_a_valid_recent_vault_or_writing_source() {
        let dir = std::env::temp_dir().join(format!("odyssey-legacy-mixed-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir(&dir).unwrap(); let old = dir.join("odyssey"); let recent = dir.join("recent");
        fdl_core::vault::create(&old).unwrap(); fdl_core::vault::create(&recent).unwrap();
        std::fs::write(old.join("note.md"), "untouched markdown").unwrap();
        std::fs::write(old.join("docs/keep.xml"), "<title>Keep</title>").unwrap();
        let (selected, notice) = super::select_automatic_root(None, Some(old.clone()));
        assert!(selected.is_none()); assert!(notice.unwrap().contains("只复制原生文档"));
        assert_eq!(std::fs::read_to_string(old.join("note.md")).unwrap(), "untouched markdown");
        assert_eq!(std::fs::read_to_string(old.join("docs/keep.xml")).unwrap(), "<title>Keep</title>");
        assert!(!old.join(".lock").exists());
        let (selected, notice) = super::select_automatic_root(Some(recent.clone()), Some(old));
        assert_eq!(selected, Some(recent.canonicalize().unwrap())); assert!(notice.is_none());
        std::fs::remove_dir_all(dir).unwrap();
    }
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

    #[cfg(unix)]
    #[test]
    fn empty_root_symlink_is_not_initialized() {
        let dir = std::env::temp_dir().join(format!("odyssey-empty-alias-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir(&dir).unwrap();
        let target = dir.join("original"); std::fs::create_dir(&target).unwrap();
        let alias = dir.join("alias"); std::os::unix::fs::symlink(&target, &alias).unwrap();
        assert!(super::validate_initial_root(&alias).is_err());
        assert_eq!(std::fs::read_dir(&target).unwrap().count(), 0);
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
