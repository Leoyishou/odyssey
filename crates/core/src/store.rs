//! On-disk library: one DocxXML file per document plus a small JSON index
//! (pinned flag, timestamps) under `~/Documents/Quiet.md`.

use crate::docxml;
use crate::model::Document;
use anyhow::{Context, Result};
use std::io::Write;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::time::now;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Meta {
    pub created: u64,
    pub modified: u64,
    #[serde(default)]
    pub pinned: bool,
    /// Last time the document was opened, for 最近访问.
    #[serde(default)]
    pub opened: u64,
    /// 页宽设置, remembered per document.
    #[serde(default, skip_serializing_if = "PageWidth::is_standard")]
    pub width: PageWidth,
    /// Catalogue visibility is remembered independently of document headings.
    #[serde(default = "catalogue_closed_default")]
    pub catalogue_closed: bool,
    /// 父文档: the page this one hangs under in the 我的文档库 tree.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// Position among its siblings (milliseconds of the last move); 0 falls
    /// back to `created`.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub order: u64,
    /// 收藏时间 (0: not starred).
    #[serde(default, skip_serializing_if = "is_zero")]
    pub starred: u64,
    /// 阅读模式, remembered per document like the page width.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub reading: bool,
    /// The 文档信息 row under the title.
    #[serde(default, skip_serializing_if = "InfoRow::is_default")]
    pub info: InfoRow,
    /// 隐藏引用列表 (the "…" menu's 显示引用列表 switch off).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub refs_hidden: bool,
}

fn catalogue_closed_default() -> bool { true }

impl Default for Meta {
    fn default() -> Self {
        Self {
            created: 0, modified: 0, pinned: false, opened: 0,
            width: PageWidth::default(), catalogue_closed: true,
            parent: None, order: 0, starred: 0, reading: false,
            info: InfoRow::default(), refs_hidden: false,
        }
    }
}

/// The 文档信息 row under the title: which items it shows (选择要展示的文档信息)
/// and whether it is hidden altogether (隐藏文档信息).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct InfoRow {
    pub hidden: bool,
    pub created: bool,
    pub modified: bool,
}

impl Default for InfoRow {
    fn default() -> Self {
        InfoRow { hidden: false, created: false, modified: true }
    }
}

impl InfoRow {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

fn is_zero(n: &u64) -> bool {
    *n == 0
}

impl Meta {
    /// Sort key among siblings: oldest first, moved pages last.
    pub fn sibling_key(&self) -> u64 {
        if self.order > 0 { self.order } else { self.created * 1000 }
    }
}

/// Three page widths: 默认 (820px column), 较宽 (1020px), 全宽.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PageWidth {
    #[default]
    Standard,
    Wide,
    Full,
}

impl PageWidth {
    fn is_standard(&self) -> bool {
        *self == PageWidth::Standard
    }

    /// Max width of the content column; `None` fills the page.
    pub fn max(self) -> Option<f32> {
        match self {
            PageWidth::Standard => Some(820.),
            PageWidth::Wide => Some(1020.),
            PageWidth::Full => None,
        }
    }
}

/// Window-level UI state kept next to the library (`settings.json`).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quick_note_size: Option<[f32; 2]>,
    /// Dragged sidebar width; `None` is the default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sidebar_width: Option<f32>,
    /// 隐藏引用列表's 不再提示.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub refs_hide_no_ask: bool,
    /// 主页: list instead of grid, and the sort order ("visited" …).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub home_list: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub home_sort: Option<String>,
    /// Main window size, remembered like the quick note window's.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window_size: Option<[f32; 2]>,
}

/// A document in the 回收站, as kept in `trash/index.json`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trashed {
    pub meta: Meta,
    pub deleted: u64,
    /// The page whose deletion took this one along (`None`: deleted itself).
    /// Such pages are listed, restored and purged with that page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<String>,
}

/// A row of the 回收站 page: a deleted page and how many went with it.
#[derive(Clone, Debug)]
pub struct TrashEntry {
    pub id: String,
    pub title: String,
    pub icon: Option<String>,
    pub deleted: u64,
    /// Where it was: its parent page, `None` for the top of 我的文档库.
    pub parent: Option<String>,
}

/// The 回收站 is emptied after 30 days.
pub const TRASH_DAYS: u64 = 30;

#[derive(Clone, Debug)]
pub struct Entry {
    pub id: String,
    pub meta: Meta,
    pub title: String,
    /// 文档图标 emoji.
    pub icon: Option<String>,
    /// Plain text body, used for search and the list snippet.
    pub body: String,
    /// Its 日期提醒, for the system notifications.
    pub reminders: Vec<crate::model::InlineReminder>,
    /// Its @文档 mentions, for the 反向引用 of the documents they point at.
    pub links: Vec<crate::model::DocLink>,
}

pub struct Store {
    loaded: std::cell::RefCell<HashMap<String, String>>,
    root: PathBuf,
    index: HashMap<String, Meta>,
    trash: HashMap<String, Trashed>,
    /// Problems found while opening the library (a damaged index recovered
    /// from its backup …), for the window to tell the user once.
    pub warnings: Vec<String>,
}

/// One write to the indexes: several copies of the app may share a library
/// (builds tested side by side), so this is a short mutex, not a
/// single-instance guard. Released when the write is done.
struct WriteLock(std::fs::File);

impl Drop for WriteLock {
    fn drop(&mut self) { let _ = self.0.unlock(); }
}

/// Version of the on-disk layout, kept in `format.json`. Bump it when a
/// change would make older builds misread the library.
pub const FORMAT: u32 = 1;

#[derive(Serialize, Deserialize)]
struct FormatMarker {
    format: u32,
}

/// Write `bytes` to `path` through a temporary file, so a crash leaves
/// either the old or the new contents.
fn write_atomic(path: &std::path::Path, bytes: &[u8]) -> Result<()> {
    let mut tmp = path.as_os_str().to_owned();
    tmp.push(".tmp");
    let mut file = std::fs::File::create(&tmp)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

fn with_suffix(path: &std::path::Path, suffix: &str) -> PathBuf {
    let mut p = path.as_os_str().to_owned();
    p.push(suffix);
    PathBuf::from(p)
}

/// Read a JSON index written by `save_json`. A file that exists but does not
/// parse is set aside as `<name>.corrupt-<time>` (never overwritten) and the
/// previous version, `<name>.bak`, is used instead.
fn load_json<T: serde::de::DeserializeOwned + Default>(path: &std::path::Path, what: &str, warnings: &mut Vec<String>) -> T {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return T::default(),
        Err(e) => {
            warnings.push(format!("无法读取{what}：{e}"));
            return T::default();
        }
    };
    if let Ok(v) = serde_json::from_slice(&bytes) {
        return v;
    }
    let aside = with_suffix(path, &format!(".corrupt-{}", now()));
    let _ = std::fs::rename(path, &aside);
    let name = aside.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    match std::fs::read(with_suffix(path, ".bak")).ok().and_then(|b| serde_json::from_slice(&b).ok()) {
        Some(v) => {
            warnings.push(format!("{what}已损坏，已从备份恢复（原文件保留为 {name}）"));
            v
        }
        None => {
            warnings.push(format!("{what}已损坏且没有可用备份，部分文档的置顶、层级等信息已重置（原文件保留为 {name}）"));
            T::default()
        }
    }
}

/// A JSON file as another copy of the app left it, if it can be read right now.
fn reread<T: serde::de::DeserializeOwned>(path: &std::path::Path) -> Option<T> {
    serde_json::from_slice(&std::fs::read(path).ok()?).ok()
}

/// Atomically replace a JSON index, keeping the version it replaces as
/// `<name>.bak` for `load_json`.
fn save_json<T: Serialize>(path: &std::path::Path, v: &T) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(v)?;
    if path.exists() {
        write_atomic(&with_suffix(path, ".bak"), &std::fs::read(path)?)?;
    }
    write_atomic(path, &bytes)
}

/// Image path for an image block source (only local files are displayed).
pub fn image_path(src: &str) -> Option<PathBuf> {
    let p = PathBuf::from(src.strip_prefix("file://").unwrap_or(src));
    if p.is_absolute() {
        return Some(p);
    }
    // Relative to the library (`assets/…`); not a URI or a cloud token.
    (!src.is_empty() && !src.contains(':') && src.contains('/')).then(|| Store::root_dir().join(p))
}

impl Store {
    /// Explicit roots win; the default lives in the app's own data directory.
    /// Never assume Documents/Odyssey is free: case-insensitive volumes may
    /// resolve it to an existing Documents/odyssey Obsidian vault.
    pub fn root_dir() -> PathBuf {
        std::env::var_os("ODYSSEY_HOME").or_else(|| std::env::var_os("QUIET_MD_HOME"))
            .map(PathBuf::from)
            .unwrap_or_else(|| dirs::data_local_dir().or_else(|| dirs::home_dir().map(|h| h.join(".local/share"))).unwrap_or_else(|| PathBuf::from(".")).join("Odyssey").join("Vaults").join("Default"))
    }

    /// Where pasted / inserted images are copied to.
    pub fn assets_dir() -> PathBuf {
        let d = Self::root_dir().join("assets");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    pub fn open() -> Result<Self> {
        Self::open_at(Self::root_dir())
    }

    pub fn open_at(root: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(root.join("docs"))?;
        let mut warnings = vec![];
        let marker = root.join("format.json");
        match std::fs::read(&marker).ok().and_then(|b| serde_json::from_slice::<FormatMarker>(&b).ok()) {
            Some(m) if m.format > FORMAT => warnings.push("这个文档库由更新版本的 Quiet.md 创建，部分内容可能无法正确显示，请升级后再编辑".into()),
            Some(_) => {}
            None => write_atomic(&marker, &serde_json::to_vec_pretty(&FormatMarker { format: FORMAT })?)?,
        }
        let index = load_json(&root.join("index.json"), "文档索引", &mut warnings);
        let trash = load_json(&root.join("trash").join("index.json"), "回收站索引", &mut warnings);
        Ok(Store { root, index, trash, warnings, loaded: Default::default() })
    }

    fn path(&self, id: &str) -> PathBuf {
        self.root.join("docs").join(format!("{id}.xml"))
    }

    /// Take the library's write lock and adopt what other copies wrote since
    /// this one last looked. Every change is written through, so memory holds
    /// nothing the disk does not; without locks (an odd filesystem) the write
    /// goes ahead unguarded, as it did before.
    fn begin_write(&mut self) -> Option<WriteLock> {
        let f = std::fs::OpenOptions::new().create(true).truncate(false).write(true).open(self.root.join(".lock")).ok()?;
        f.lock().ok()?;
        // An unreadable index is not judged here (that is `open`'s job): the
        // version in memory is written back unchanged.
        if let Some(index) = reread(&self.root.join("index.json")) { self.index = index; }
        if let Some(trash) = reread(&self.root.join("trash").join("index.json")) { self.trash = trash; }
        Some(WriteLock(f))
    }

    fn save_index(&self) -> Result<()> {
        save_json(&self.root.join("index.json"), &self.index)
    }

    /// All documents found on disk. A `.xml` file dropped into `docs/` by hand
    /// is picked up too.
    pub fn list(&mut self) -> Vec<Entry> {
        let _lock = self.begin_write();
        let mut out = vec![];
        let Ok(rd) = std::fs::read_dir(self.root.join("docs")) else { return out };
        let mut dirty = false;
        for e in rd.flatten() {
            let p = e.path();
            if p.extension().and_then(|s| s.to_str()) != Some("xml") {
                continue;
            }
            let Some(id) = p.file_stem().and_then(|s| s.to_str()).map(str::to_string) else { continue };
            let Ok(xml) = std::fs::read_to_string(&p) else { continue };
            let doc = docxml::parse(&xml).unwrap_or_default();
            let meta = self.index.entry(id.clone()).or_insert_with(|| {
                dirty = true;
                let m = e
                    .metadata()
                    .ok()
                    .and_then(|m| m.modified().ok())
                    .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                    .map(|d| d.as_secs())
                    .unwrap_or_else(now);
                Meta { created: m, modified: m, ..Default::default() }
            });
            out.push(Entry { id, meta: meta.clone(), title: doc.title.text.clone(), icon: doc.icon.clone(), body: doc.plain_text(), reminders: doc.reminders(), links: doc.doc_links() });
        }
        if dirty {
            let _ = self.save_index();
        }
        out
    }

    pub fn load(&self, id: &str) -> Result<Document> {
        docxml::parse(&std::fs::read_to_string(self.path(id))?)
    }

    /// Only opening an editor establishes a new save baseline; previews must
    /// not hide external changes from an already open editor.
    pub fn load_for_edit(&self, id: &str, shared: bool) -> Result<Document> {
        let xml = std::fs::read_to_string(self.path(id))?;
        let doc = docxml::parse(&xml)?;
        if !shared { self.loaded.borrow_mut().insert(id.to_string(), xml); }
        Ok(doc)
    }

    pub fn save(&mut self, id: &str, doc: &Document) -> Result<Meta> {
        let _lock = self.begin_write();
        self.save_now(id, doc)
    }

    /// `save` for callers that already hold the write lock.
    fn save_now(&mut self, id: &str, doc: &Document) -> Result<Meta> {
        let xml = docxml::serialize(doc);
        if let Some(previous) = self.loaded.borrow().get(id) {
            let disk = std::fs::read_to_string(self.path(id))?;
            anyhow::ensure!(&disk == previous || disk == xml, "文档已被另一个实例修改，未覆盖磁盘内容。请先将当前内容导出，再重新打开文档");
        }
        write_atomic(&self.path(id), xml.as_bytes())?;
        self.loaded.borrow_mut().insert(id.to_string(), xml);
        let t = now();
        let m = self.index.entry(id.to_string()).or_insert(Meta { created: t, modified: t, ..Default::default() });
        m.modified = t;
        let m = m.clone();
        self.save_index()?;
        Ok(m)
    }

    /// Materialize a reserved link identity exactly once; never overwrite a document or trash.
    pub fn create_pending(&mut self, id: &str, title: &str) -> Result<()> {
        uuid::Uuid::parse_str(id)?;
        let _lock = self.begin_write();
        if self.path(id).try_exists()? { return Ok(()) }
        anyhow::ensure!(!self.trash_path(id).try_exists()?, "目标位于回收站，请恢复原文档");
        let mut doc = Document::default();
        doc.title = crate::model::RichText::plain(title);
        self.save_now(id, &doc)?;
        Ok(())
    }

    pub fn create(&mut self) -> Result<(String, Document)> {
        let doc = Document::default();
        let id = self.create_from(&doc)?;
        Ok((id, doc))
    }

    /// Save `doc` as a new document (创建副本, import).
    pub fn create_from(&mut self, doc: &Document) -> Result<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let _lock = self.begin_write();
        self.save_now(&id, doc)?;
        Ok(id)
    }

    /// Size of the document on disk, plus the local images and files it references.
    pub fn size_of(&self, id: &str, doc: &Document) -> u64 {
        let mut n = std::fs::metadata(self.path(id)).map(|m| m.len()).unwrap_or(0);
        for src in doc.asset_sources() {
            if let Some(p) = image_path(&src) {
                n += std::fs::metadata(p).map(|m| m.len()).unwrap_or(0);
            }
        }
        n
    }

    /// Move a document to the 回收站; `root` is the page whose deletion
    /// takes it along (`None`: deleted itself).
    pub fn delete(&mut self, id: &str, root: Option<&str>) -> Result<()> {
        let _lock = self.begin_write();
        let target = self.trash_path(id);
        anyhow::ensure!(!target.try_exists()?, "回收站已存在同名文档，未覆盖：{id}");
        std::fs::create_dir_all(self.root.join("trash"))?;
        // Persist destination metadata before moving content. An interrupted
        // move leaves an extra index entry, never an unindexed trashed file.
        let mut trash = self.trash.clone();
        let meta = self.index.get(id).cloned().unwrap_or_default();
        trash.insert(id.to_string(), Trashed { meta, deleted: now(), root: root.map(str::to_string) });
        save_json(&self.root.join("trash/index.json"), &trash)?;
        self.trash = trash;
        std::fs::rename(self.path(id), target)?;
        self.index.remove(id);
        // Content has moved: cleanup failure must not report deletion as failed.
        if let Err(e) = self.save_index() {
            self.warnings.push(format!("文档已移入回收站，文档索引清理失败：{e}"));
        }
        Ok(())
    }

    pub fn trash_dir(&self) -> PathBuf {
        let d = self.root.join("trash");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    fn trash_path(&self, id: &str) -> PathBuf {
        self.root.join("trash").join(format!("{id}.xml"))
    }

    fn save_trash(&self) -> Result<()> {
        save_json(&self.trash_dir().join("index.json"), &self.trash)
    }

    /// `id` and the pages deleted along with it.
    fn trash_group(&self, id: &str) -> Vec<String> {
        let mut out = vec![id.to_string()];
        out.extend(self.trash.iter().filter(|(k, t)| t.root.as_deref() == Some(id) && k.as_str() != id).map(|(k, _)| k.clone()));
        out
    }

    /// The 回收站, newest first: one row per deleted page (its child pages
    /// ride along). Files trashed by older versions are adopted, and pages
    /// past 30 days are removed for good.
    pub fn list_trash(&mut self) -> Vec<TrashEntry> {
        let _lock = self.begin_write();
        let dir = self.trash_dir();
        let mut dirty = false;
        let mut on_disk = std::collections::HashSet::new();
        // Unreadable folder: list what the index knows, and leave it alone.
        let listed = std::fs::read_dir(&dir);
        let mut readable = listed.is_ok();
        if let Ok(rd) = listed {
            for entry in rd {
                let e = match entry {
                    Ok(e) => e,
                    Err(e) => {
                        readable = false;
                        self.warnings.push(format!("回收站目录读取不完整：{e}"));
                        continue;
                    }
                };
                let p = e.path();
                if p.extension().and_then(|s| s.to_str()) != Some("xml") {
                    continue;
                }
                let Some(id) = p.file_stem().and_then(|s| s.to_str()).map(str::to_string) else { continue };
                if !self.trash.contains_key(&id) {
                    let t = e
                        .metadata()
                        .ok()
                        .and_then(|m| m.modified().ok())
                        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                        .map(|d| d.as_secs())
                        .unwrap_or_else(now);
                    // Modification time is not deletion time. Recovered orphans
                    // get a full retention period instead of immediate expiry.
                    let meta = self.index.get(&id).cloned().unwrap_or(Meta { created: t, modified: t, ..Default::default() });
                    self.trash.insert(id.clone(), Trashed { meta, deleted: now(), root: None });
                    dirty = true;
                }
                on_disk.insert(id);
            }
        }
        if readable {
            let before = self.trash.len();
            self.trash.retain(|id, _| on_disk.contains(id));
            dirty |= self.trash.len() != before;
        }
        let cutoff = now().saturating_sub(TRASH_DAYS * 86_400);
        let expired: Vec<String> = self.trash.iter().filter(|(_, t)| t.deleted <= cutoff).map(|(k, _)| k.clone()).collect();
        for id in expired {
            match std::fs::remove_file(self.trash_path(&id)) {
                Ok(()) => { self.trash.remove(&id); dirty = true; }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => { self.trash.remove(&id); dirty = true; }
                Err(e) => self.warnings.push(format!("无法清理过期回收站文档 {id}：{e}")),
            }
        }
        if dirty && let Err(e) = self.save_trash() {
            self.warnings.push(format!("回收站索引保存失败：{e}"));
        }
        let mut out: Vec<TrashEntry> = self
            .trash
            .iter()
            .filter(|(_, t)| t.root.as_ref().is_none_or(|r| !self.trash.contains_key(r)))
            .map(|(id, t)| {
                let doc = std::fs::read_to_string(self.trash_path(id)).ok().and_then(|x| docxml::parse(&x).ok()).unwrap_or_default();
                TrashEntry {
                    id: id.clone(),
                    title: doc.title.text.clone(),
                    icon: doc.icon.clone(),
                    deleted: t.deleted,
                    parent: t.meta.parent.clone(),
                }
            })
            .collect();
        out.sort_by(|a, b| b.deleted.cmp(&a.deleted).then_with(|| a.id.cmp(&b.id)));
        out
    }

    /// 恢复: put a deleted page (and the pages that went with it) back. True
    /// when its parent page is gone, so it landed at the top of 我的文档库.
    pub fn restore(&mut self, id: &str) -> Result<bool> {
        let _lock = self.begin_write();
        let group = self.trash_group(id);
        let mut index = self.index.clone();
        let parent = self.trash.get(id).and_then(|t| t.meta.parent.clone());
        let moved = parent.is_some_and(|p| !self.index_has(&p));
        for k in &group {
            let Some(t) = self.trash.get(k) else { continue };
            anyhow::ensure!(!self.path(k).try_exists()?, "文档库已存在同名文档，未覆盖：{k}");
            let mut meta = t.meta.clone();
            if k == id && moved { meta.parent = None; }
            index.insert(k.clone(), meta);
        }
        // Stage the destination metadata before the first move. On partial
        // failure, both sides retain enough metadata to retry after reopening.
        save_json(&self.root.join("index.json"), &index)?;
        self.index = index;
        for k in &group {
            if !self.trash.contains_key(k) { continue; }
            std::fs::rename(self.trash_path(k), self.path(k)).with_context(|| format!("恢复文档 {k} 失败，已恢复的文档保留在文档库"))?;
            self.trash.remove(k);
        }
        if let Err(e) = self.save_trash() {
            self.warnings.push(format!("文档已恢复，回收站索引清理失败：{e}"));
        }
        Ok(moved)
    }

    fn index_has(&self, id: &str) -> bool {
        self.index.contains_key(id) && self.path(id).exists()
    }

    /// 彻底删除 a page from the 回收站, with the pages that went with it.
    pub fn purge(&mut self, id: &str) -> Result<()> {
        let _lock = self.begin_write();
        for k in self.trash_group(id) {
            match std::fs::remove_file(self.trash_path(&k)) {
                Ok(()) => {}
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => return Err(e.into()),
            }
            self.trash.remove(&k);
        }
        self.save_trash()
    }

    /// Copy a DocxXML file into the library under a fresh id.
    pub fn import(&mut self, path: &std::path::Path) -> Result<String> {
        let xml = std::fs::read_to_string(path)?;
        self.create_from(&docxml::parse(&xml)?)
    }

    pub fn set_width(&mut self, id: &str, width: PageWidth) -> Result<()> {
        let _lock = self.begin_write();
        if let Some(m) = self.index.get_mut(id) {
            m.width = width;
        }
        self.save_index()
    }

    pub fn update_meta(&mut self, id: &str, f: impl FnOnce(&mut Meta)) -> Result<()> {
        let _lock = self.begin_write();
        if let Some(m) = self.index.get_mut(id) {
            f(m);
        }
        self.save_index()
    }

    pub fn set_catalogue(&mut self, id: &str, pinned: bool) -> Result<()> {
        let _lock = self.begin_write();
        if let Some(m) = self.index.get_mut(id) {
            m.catalogue_closed = !pinned;
        }
        self.save_index()
    }

    pub fn settings(&self) -> Settings {
        std::fs::read(self.root.join("settings.json"))
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or_default()
    }

    pub fn save_settings(&self, s: &Settings) -> Result<()> {
        write_atomic(&self.root.join("settings.json"), &serde_json::to_vec_pretty(s)?)
    }

    /// 重命名 a document that isn't open: rewrite its title on disk.
    pub fn rename(&mut self, id: &str, title: &str) -> Result<Meta> {
        let _lock = self.begin_write();
        let mut doc = self.load(id)?;
        doc.title = crate::model::RichText::plain(title);
        self.save_now(id, &doc)
    }

    /// Note that `id` was just opened.
    pub fn touch(&mut self, id: &str) -> Option<Meta> {
        let _lock = self.begin_write();
        let m = self.index.get_mut(id)?;
        m.opened = now();
        let m = m.clone();
        let _ = self.save_index();
        Some(m)
    }

    /// 移动到: hang `id` under `parent` (`None` = top level of 我的文档库), as its last child.
    pub fn set_parent(&mut self, id: &str, parent: Option<&str>) -> Result<Meta> {
        let _lock = self.begin_write();
        let m = self.index.get_mut(id).ok_or_else(|| anyhow::anyhow!("文档不存在"))?;
        m.parent = parent.map(str::to_string);
        m.order = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        let m = m.clone();
        self.save_index()?;
        Ok(m)
    }

    /// 收藏 / 取消收藏; returns the new 收藏时间 (0: not starred).
    pub fn set_starred(&mut self, id: &str, on: bool) -> Result<u64> {
        let _lock = self.begin_write();
        let t = if on { now() } else { 0 };
        if let Some(m) = self.index.get_mut(id) {
            m.starred = t;
        }
        self.save_index()?;
        Ok(t)
    }

    pub fn set_pinned(&mut self, id: &str, pinned: bool) -> Result<()> {
        let _lock = self.begin_write();
        if let Some(m) = self.index.get_mut(id) {
            m.pinned = pinned;
        }
        self.save_index()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A library in a fresh temporary folder, removed on drop.
    struct Lib(PathBuf);

    impl Lib {
        fn new(name: &str) -> Self {
            let d = std::env::temp_dir().join(format!("fdl-test-{name}-{}", uuid::Uuid::new_v4()));
            Lib(d)
        }
        fn open(&self) -> Store {
            Store::open_at(self.0.clone()).unwrap()
        }
    }

    impl Drop for Lib {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn doc(title: &str) -> Document {
        Document { title: crate::model::RichText::plain(title), ..Default::default() }
    }

    #[test]
    fn pending_identity_is_idempotent_and_never_resurrects_trash() {
        let lib = Lib::new("pending");
        let mut s = lib.open();
        let id = uuid::Uuid::new_v4().to_string();
        s.create_pending(&id, "新目标").unwrap();
        s.create_pending(&id, "不可覆盖").unwrap();
        assert_eq!(s.list().len(), 1);
        assert_eq!(s.load(&id).unwrap().title.text, "新目标");
        s.delete(&id, None).unwrap();
        assert!(s.create_pending(&id, "不可复活").is_err());
        assert!(s.create_pending("../escape", "invalid").is_err());
        assert!(s.list().is_empty());
    }

    #[test]
    fn save_list_and_reopen() {
        let lib = Lib::new("basic");
        let mut s = lib.open();
        assert!(s.warnings.is_empty());
        let id = s.create_from(&doc("一")).unwrap();
        s.set_pinned(&id, true).unwrap();
        let mut s = lib.open();
        let list = s.list();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].title, "一");
        assert!(list[0].meta.pinned);
        assert!(lib.0.join("format.json").is_file());
    }

    #[test]
    fn corrupt_index_falls_back_to_backup() {
        let lib = Lib::new("corrupt");
        let mut s = lib.open();
        let id = s.create_from(&doc("一")).unwrap();
        s.set_pinned(&id, true).unwrap();
        // The second write left the first (pinned: false) as the backup;
        // one more makes the backup pinned too.
        s.set_starred(&id, true).unwrap();
        std::fs::write(lib.0.join("index.json"), b"{ not json").unwrap();
        let mut s = lib.open();
        assert_eq!(s.warnings.len(), 1, "{:?}", s.warnings);
        assert!(s.warnings[0].contains("已从备份恢复"));
        assert!(s.list()[0].meta.pinned);
        // The damaged file is kept for inspection, not overwritten.
        let kept = std::fs::read_dir(&lib.0).unwrap().flatten().filter(|e| e.file_name().to_string_lossy().starts_with("index.json.corrupt-")).count();
        assert_eq!(kept, 1);
    }

    #[test]
    fn corrupt_index_without_backup_still_lists_documents() {
        let lib = Lib::new("nobak");
        let mut s = lib.open();
        s.create_from(&doc("一")).unwrap();
        std::fs::write(lib.0.join("index.json"), b"garbage").unwrap();
        let _ = std::fs::remove_file(lib.0.join("index.json.bak"));
        let mut s = lib.open();
        assert!(s.warnings[0].contains("没有可用备份"));
        assert_eq!(s.list().len(), 1);
    }

    #[test]
    fn newer_format_is_reported() {
        let lib = Lib::new("format");
        drop(lib.open());
        std::fs::write(lib.0.join("format.json"), format!("{{\"format\": {}}}", FORMAT + 1)).unwrap();
        assert_eq!(lib.open().warnings.len(), 1);
    }

    #[test]
    fn trash_round_trip() {
        let lib = Lib::new("trash");
        let mut s = lib.open();
        let a = s.create_from(&doc("父")).unwrap();
        let b = s.create_from(&doc("子")).unwrap();
        s.set_parent(&b, Some(&a)).unwrap();
        s.delete(&b, Some(&a)).unwrap();
        s.delete(&a, None).unwrap();
        assert!(s.list().is_empty());
        let t = s.list_trash();
        assert_eq!(t.len(), 1, "the child rides along with its parent");
        assert_eq!(t[0].title, "父");
        let mut s = lib.open();
        assert!(!s.restore(&a).unwrap());
        let list = s.list();
        assert_eq!(list.len(), 2);
        assert_eq!(list.iter().find(|e| e.id == b).unwrap().meta.parent.as_deref(), Some(a.as_str()));
        assert!(s.list_trash().is_empty());
    }

    #[test]
    fn unreadable_trash_folder_keeps_its_index() {
        let lib = Lib::new("trashdir");
        let mut s = lib.open();
        let a = s.create_from(&doc("一")).unwrap();
        s.delete(&a, None).unwrap();
        // Swap the folder for a file: read_dir fails.
        let trash = lib.0.join("trash");
        let index = std::fs::read(trash.join("index.json")).unwrap();
        std::fs::rename(&trash, lib.0.join("trash-away")).unwrap();
        std::fs::write(&trash, b"").unwrap();
        let mut s = Store { root: lib.0.clone(), index: HashMap::new(), trash: serde_json::from_slice(&index).unwrap(), warnings: vec![], loaded: Default::default() };
        assert_eq!(s.list_trash().len(), 1);
        assert_eq!(s.trash.len(), 1);
    }

    /// Two copies of the app on one library: neither writes back a stale
    /// index over what the other just changed.
    #[test]
    fn two_instances_keep_each_others_entries() {
        let lib = Lib::new("two");
        let mut a = lib.open();
        let mut b = lib.open();
        let one = a.create_from(&doc("一")).unwrap();
        let two = b.create_from(&doc("二")).unwrap();
        a.set_pinned(&one, true).unwrap();
        b.set_starred(&two, true).unwrap();
        let mut c = lib.open();
        let list = c.list();
        assert_eq!(list.len(), 2);
        assert!(list.iter().find(|e| e.id == one).unwrap().meta.pinned, "b kept a's entry");
        assert!(list.iter().find(|e| e.id == two).unwrap().meta.starred > 0, "a kept b's entry");
    }

    #[test]
    fn an_instance_does_not_undo_another_ones_delete() {
        let lib = Lib::new("two-del");
        let mut a = lib.open();
        let mut b = lib.open();
        let id = a.create_from(&doc("一")).unwrap();
        let two = a.create_from(&doc("二")).unwrap();
        a.delete(&id, None).unwrap();
        // `b` still has `id` in memory and changes it: it must not come back.
        b.set_pinned(&id, true).unwrap();
        let mut c = lib.open();
        assert_eq!(c.list().iter().map(|e| e.id.as_str()).collect::<Vec<_>>(), vec![two.as_str()]);
        assert_eq!(c.list_trash().len(), 1);
    }

    #[test]
    fn failed_save_leaves_the_old_file() {
        let lib = Lib::new("savefail");
        let mut s = lib.open();
        let id = s.create_from(&doc("旧")).unwrap();
        std::fs::create_dir(lib.0.join("docs").join(format!("{id}.xml.tmp"))).unwrap();
        assert!(s.save(&id, &doc("新")).is_err());
        assert_eq!(s.load(&id).unwrap().title.text, "旧");
    }
}
