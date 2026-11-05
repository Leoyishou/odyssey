//! Copy-only Markdown import: read/preview first, publish a complete new vault last.
use crate::{docxml, model::*, paste, store::{Store, FORMAT}};
use anyhow::{Context, Result, ensure};
use serde::Serialize;
use std::{collections::BTreeMap, fs, path::{Component, Path, PathBuf}};

#[derive(Clone, Debug, Serialize)]
pub struct Warning { pub file: String, pub detail: String }
#[derive(Clone, Debug, Default, Serialize)]
pub struct Report {
    pub mode: String,
    pub notes: usize, pub native_notes: usize, pub native_attachments: usize, pub native_trash: usize, pub folders: usize, pub attachments: usize, pub resolved_links: usize,
    pub warnings: Vec<Warning>,
    pub paths: BTreeMap<String, String>,
}
struct Original { relative: PathBuf, bytes: Vec<u8> }
struct Note { relative: PathBuf, id: String, title: String, doc: Document, native: bool }
#[derive(Clone)]
struct Token { key: String, raw: String, target: String, label: Option<String>, embed: bool }
pub struct Plan {
    pub source: PathBuf,
    pub report: Report,
    notes: Vec<Note>, folders: BTreeMap<PathBuf, String>, originals: Vec<Original>,
    assets: BTreeMap<PathBuf, Vec<u8>>,
    native: Option<crate::native_vault_snapshot::Snapshot>, asset_prefix: String,
}
fn key(p: &Path) -> String { p.to_string_lossy().replace('\\', "/") }
fn is_markdown(p: &Path) -> bool { p.extension().is_some_and(|e| e.eq_ignore_ascii_case("md") || e.eq_ignore_ascii_case("markdown")) }
fn warning(report: &mut Report, file: &Path, detail: impl Into<String>) {
    report.warnings.push(Warning { file: key(file), detail: detail.into() });
}
fn read_bounded(path: &Path, max: u64) -> Result<Vec<u8>> {
    ensure!(fs::metadata(path)?.len() <= max, "文件过大，停止预览：{}", path.display());
    fs::read(path).with_context(|| format!("无法读取：{}", path.display()))
}
pub(crate) fn read_source_file(root: &Path, relative: &Path, max: u64) -> Result<Vec<u8>> {
    let mut path = root.to_path_buf();
    for part in relative.components() {
        ensure!(matches!(part, Component::Normal(_)), "源文件路径无效：{}", relative.display());
        path.push(part);
        ensure!(!fs::symlink_metadata(&path)?.file_type().is_symlink(), "源文件路径包含符号链接，请重新预览：{}", relative.display());
    }
    ensure!(path.canonicalize()?.starts_with(root) && path.is_file(), "源文件越界或不是普通文件：{}", relative.display());
    read_bounded(&path, max)
}
fn normalized(base: &Path, target: &str) -> Option<PathBuf> {
    let p = Path::new(target); if p.is_absolute() || target.contains(':') { return None; }
    let mut out = base.to_path_buf();
    for c in p.components() {
        match c { Component::Normal(n) => out.push(n), Component::CurDir => {}, Component::ParentDir => { if !out.pop() { return None; } }, _ => return None }
    }
    Some(out)
}
fn decode(s: &str) -> String {
    let b = s.as_bytes(); let mut out = Vec::new(); let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() && let Ok(h) = std::str::from_utf8(&b[i + 1..i + 3]) && let Ok(v) = u8::from_str_radix(h, 16) { out.push(v); i += 3; }
        else { out.push(b[i]); i += 1; }
    }
    String::from_utf8(out).unwrap_or_else(|_| s.into())
}
fn discover(root: &Path, native: bool, report: &mut Report) -> Result<Vec<PathBuf>> {
    let mut found = Vec::new(); let mut todo = vec![PathBuf::new()];
    while let Some(relative) = todo.pop() {
        for entry in fs::read_dir(root.join(&relative))? {
            let e = entry?; let path = relative.join(e.file_name()); let ft = e.file_type()?;
            if ft.is_symlink() { warning(report, &path, "符号链接已跳过，未读取其目标"); continue; }
            if e.file_name().to_string_lossy().starts_with('.') { continue; }
            if native && relative.as_os_str().is_empty() && ["assets", "trash", "themes"].iter().any(|n| e.file_name() == *n) { continue; }
            if ft.is_dir() { todo.push(path); }
            else if ft.is_file() && is_markdown(&path) { found.push(path); }
            else if !native && path.starts_with("docs") && path.extension().is_some_and(|e| e == "xml") { warning(report, &path, "已有 XML 文档保留在源目录；本次只复制导入 Markdown"); }
        }
    }
    found.sort(); Ok(found)
}

/// Protect wiki pipes from the Markdown table parser. Code spans/fences are
/// restored as literal text after parsing, never converted to references.
fn tokenize(text: &str) -> (String, Vec<Token>) {
    let prefix = format!("ODYSSEYIMPORT{}T", uuid::Uuid::new_v4().simple());
    let mut out = String::new(); let mut tokens = Vec::new(); let mut at = 0;
    while at < text.len() {
        let rest = &text[at..]; let escaped = at > 0 && text.as_bytes()[at - 1] == b'\\';
        let embed = rest.starts_with("!["); let start = usize::from(embed);
        let tail = &rest[start..];
        let parsed = if !escaped && tail.starts_with("[[") {
            tail[2..].find("]]").map(|end| {
                let body = &tail[2..end + 2]; let (target, label) = body.split_once('|').map_or((body, None), |(t, l)| (t, Some(l.to_string())));
                (start + end + 4, target.trim().to_string(), label)
            })
        } else if !escaped && tail.starts_with('[') {
            tail.find("](").and_then(|close| tail[close + 2..].find(')').map(|end| {
                let label = tail[1..close].to_string(); let target = tail[close + 2..close + 2 + end].trim().trim_matches(['<', '>']);
                (start + close + 3 + end, target.to_string(), Some(label))
            }))
        } else { None };
        if let Some((len, target, label)) = parsed && !target.contains('\n') && !target.contains(':') {
            let raw = &rest[..len]; let token = format!("{prefix}{}END", tokens.len());
            out.push_str(&token); tokens.push(Token { key: token, raw: raw.into(), target: decode(&target), label, embed }); at += len;
        } else { let c = rest.chars().next().unwrap(); out.push(c); at += c.len_utf8(); }
    }
    (out, tokens)
}

impl Plan {
    /// No directory creation, Store mutation, networking or source writes.
    pub fn scan(source: &Path) -> Result<Self> {
        Self::scan_mode(source, true)
    }
    pub fn recover_native(source: &Path) -> Result<Self> {
        Self::scan_mode(source, false)
    }
    fn scan_mode(source: &Path, markdown: bool) -> Result<Self> {
        let source = source.canonicalize()?; ensure!(source.is_dir(), "请选择 Markdown 文件夹");
        let native = crate::native_vault_snapshot::Snapshot::read(&source)?;
        let asset_prefix = if native.is_some() { format!("imported-{}", uuid::Uuid::new_v4()) } else { "imported".into() };
        let mut plan = Self { source, report: Report::default(), notes: vec![], folders: BTreeMap::new(), originals: vec![], assets: BTreeMap::new(), native, asset_prefix };
        plan.report.mode = if markdown { "markdown_and_native" } else { "native_only" }.into();
        let paths = if markdown { discover(&plan.source, plan.native.is_some(), &mut plan.report)? } else { vec![] };
        ensure!(!paths.is_empty() || plan.native.as_ref().is_some_and(|n| !n.docs.is_empty()), "未找到可导入的 Markdown 文件；没有创建目标库");
        let mut total = 0;
        for relative in paths {
            let bytes = read_source_file(&plan.source, &relative, 16 * 1024 * 1024)?; total += bytes.len();
            ensure!(total <= 256 * 1024 * 1024, "Markdown 总量超过本次预览上限，尚未创建目标库");
            std::str::from_utf8(&bytes).with_context(|| format!("文件不是 UTF-8，停止导入以免损坏文字：{}", relative.display()))?;
            let id = uuid::Uuid::new_v4().to_string();
            plan.report.paths.insert(key(&relative), id.clone());
            let title = relative.file_stem().unwrap_or_default().to_string_lossy().to_string();
            plan.notes.push(Note { relative: relative.clone(), id, title, doc: Document::default(), native: false });
            let mut parent = relative.parent();
            while let Some(p) = parent.filter(|p| !p.as_os_str().is_empty()) { plan.folders.entry(p.to_path_buf()).or_insert_with(|| uuid::Uuid::new_v4().to_string()); parent = p.parent(); }
            plan.originals.push(Original { relative, bytes });
        }
        let markdown_count = plan.notes.len();
        if let Some(native) = &plan.native {
            plan.report.native_notes = native.docs.len(); plan.report.native_attachments = native.assets; plan.report.native_trash = native.trash;
            for (id, doc) in &native.docs {
                let relative = PathBuf::from(format!("docs/{id}.xml"));
                plan.report.paths.insert(key(&relative), id.clone());
                plan.notes.push(Note { relative, id: id.clone(), title: doc.title.text.clone(), doc: doc.clone(), native: true });
            }
            for detail in &native.warnings { warning(&mut plan.report, Path::new("native"), detail); }
        }
        for i in 0..markdown_count {
            let path = plan.notes[i].relative.clone(); let raw = std::str::from_utf8(&plan.originals[i].bytes)?.trim_start_matches('\u{feff}').to_string();
            let mut text = raw.clone();
            if let Some(after) = raw.strip_prefix("---\n") && let Some(end) = after.find("\n---") {
                text = format!("```yaml\n{}\n```{}", &after[..end], &after[end + 4..]);
                warning(&mut plan.report, &path, "YAML 属性保留为代码文本，未映射为文档属性或别名");
            }
            if raw.contains("> [!") || raw.contains("```dataview") || raw.contains("<div") || raw.contains("<table") {
                warning(&mut plan.report, &path, "包含 Obsidian callout、插件或 HTML 内容；仅保留基础文本，原始 Markdown 已归档");
            }
            let (text, tokens) = tokenize(&text);
            let mut blocks = paste::markdown_to_blocks(&text, paste::Off { file: true, ..Default::default() });
            plan.convert_blocks(&path, &mut blocks, &tokens)?;
            let title = plan.notes[i].title.clone();
            if blocks.first().is_some_and(|b| matches!(b.kind, BlockKind::Heading { level: 1, .. }) && b.text.text.trim() == title) { blocks.remove(0); }
            if blocks.is_empty() { blocks.push(Block::text("")); }
            plan.notes[i].doc = Document { title: RichText::plain(title), icon: None, cover: None, blocks };
            let xml = docxml::serialize(&plan.notes[i].doc);
            ensure!(!tokens.iter().any(|t| xml.contains(&t.key)), "无法可靠转换该文档的嵌套语法，未创建目标库：{}", path.display());
        }
        plan.report.notes = markdown_count; plan.report.folders = plan.folders.len(); plan.report.attachments = plan.assets.len();
        Ok(plan)
    }

    fn note_target(&self, source: &Path, target: &str) -> Result<Option<usize>, &'static str> {
        for base in [source.parent().unwrap_or(Path::new("")), Path::new("")] {
            if let Some(p) = normalized(base, target) {
                for (i, note) in self.notes.iter().enumerate() {
                    if note.relative == p || (p.extension().is_none() && note.relative.with_extension("") == p) { return Ok(Some(i)); }
                }
            }
        }
        let matches: Vec<_> = self.notes.iter().enumerate().filter(|(_, n)| n.title == target.trim_end_matches(".md")).map(|(i, _)| i).collect();
        match matches.as_slice() { [i] => Ok(Some(*i)), [] => Ok(None), _ => Err("同名目标有歧义，保留原始链接，未自动选择") }
    }
    fn asset(&mut self, source: &Path, target: &str) -> Result<Option<FileBlock>> {
        for base in [source.parent().unwrap_or(Path::new("")), Path::new("")] {
            let Some(relative) = normalized(base, target) else { continue };
            let path = self.source.join(&relative);
            if !path.is_file() { continue; }
            let canonical = path.canonicalize()?;
            ensure!(canonical.starts_with(&self.source), "附件越出源文件夹，停止导入：{}", relative.display());
            let metadata = fs::symlink_metadata(&path)?; ensure!(!metadata.file_type().is_symlink(), "附件是符号链接，停止导入：{}", relative.display());
            if !self.assets.contains_key(&relative) {
                let bytes = read_source_file(&self.source, &relative, 64 * 1024 * 1024)?;
                ensure!(self.assets.values().map(Vec::len).sum::<usize>() + bytes.len() <= 512 * 1024 * 1024, "附件总量超过本次预览上限");
                self.assets.insert(relative.clone(), bytes);
            }
            return Ok(Some(FileBlock { src_kind: "path".into(), src: format!("assets/{}/{}", self.asset_prefix, key(&relative)), name: relative.file_name().unwrap_or_default().to_string_lossy().to_string(), view: FileView::Card }));
        }
        Ok(None)
    }
    fn convert_blocks(&mut self, path: &Path, blocks: &mut [Block], tokens: &[Token]) -> Result<()> {
        for b in blocks {
            let code = matches!(b.kind, BlockKind::Code { .. });
            let whole = tokens.iter().find(|t| b.text.text.trim() == t.key && t.embed).cloned();
            if !code && !b.text.spans().any(|(_, m)| m.code) && let Some(t) = whole && !t.target.contains('#') && self.note_target(path, &t.target).ok().flatten().is_none() && let Some(file) = self.asset(path, &t.target)? {
                let image = Path::new(&file.name).extension().is_some_and(|e| ["png", "jpg", "jpeg", "gif", "webp", "svg", "bmp"].iter().any(|x| e.eq_ignore_ascii_case(x)));
                b.kind = if image { BlockKind::Image { src_kind: file.src_kind, src: file.src, width: None, height: None, caption: t.label, crop: None } } else { BlockKind::File(file) };
                b.text = RichText::default();
            } else {
                for run in &mut b.text.runs {
                    if let Some(inline) = &mut run.marks.inline && let InlineKind::Equation(src) = &mut inline.kind {
                        for t in tokens { *src = src.replace(&t.key, &t.raw); }
                        *inline = docxml::to_inline(InlineKind::Equation(src.clone()));
                    }
                }
                for t in tokens {
                    while let Some(at) = b.text.text.find(&t.key) {
                        let end = at + t.key.len(); let marks = b.text.spans().find(|(r, _)| r.contains(&at)).map(|(_, m)| m.clone()).unwrap_or_default();
                        let mut replacement = RichText::plain(t.raw.clone());
                        if !code && !marks.code {
                            if t.target.contains('#') || t.target.contains('^') { warning(&mut self.report, path, format!("标题/块锚点暂未转换，保留原文：{}", t.raw)); }
                            else {
                                match self.note_target(path, &t.target) {
                                    Ok(Some(i)) if !t.embed => {
                                        let note = &self.notes[i]; let inline = docxml::to_inline(InlineKind::DocLink(InlineDocLink { id: note.id.clone(), title: note.title.clone(), alias: t.label.clone(), pending: false }));
                                        replacement = RichText::default(); replacement.push("\u{fffc}", Marks { inline: Some(inline), ..marks.clone() }); self.report.resolved_links += 1;
                                    }
                                    Ok(Some(_)) => warning(&mut self.report, path, format!("文档嵌入尚不支持，保留原文：{}", t.raw)),
                                    Err(message) => warning(&mut self.report, path, format!("{message}：{}", t.raw)),
                                    Ok(None) => {
                                        if let Some(file) = self.asset(path, &t.target)? {
                                            replacement = RichText::default(); replacement.push("\u{fffc}", Marks { inline: Some(docxml::to_inline(InlineKind::File(file))), ..marks.clone() });
                                            if t.embed { warning(&mut self.report, path, format!("段落内媒体以附件显示：{}", t.raw)); }
                                        } else { warning(&mut self.report, path, format!("找不到链接/附件目标，保留原文：{}", t.raw)); }
                                    }
                                }
                            }
                        }
                        // Preserve formatting for literal fallback, including code spans.
                        if replacement.text != "\u{fffc}" { replacement = RichText::default(); replacement.push(&t.raw, marks); }
                        b.text.replace(at..end, &replacement);
                    }
                }
            }
            self.convert_blocks(path, &mut b.children, tokens)?;
            if let BlockKind::Table(table) = &mut b.kind { for cell in table.rows.iter_mut().flatten() { self.convert_blocks(path, &mut cell.blocks, tokens)?; } }
        }
        Ok(())
    }

    /// Requires a new path outside the source. Only the private staging folder
    /// is removed on failure; neither source nor an existing target is edited.
    pub fn write_new(&self, target: &Path) -> Result<PathBuf> {
        ensure!(!target.try_exists()?, "目标已存在，未覆盖：{}", target.display());
        crate::vault::validate_new_parent(target)?;
        let parent = target.parent().ok_or_else(|| anyhow::anyhow!("请选择目标父目录"))?.canonicalize()?;
        ensure!(!parent.starts_with(&self.source), "目标必须位于源库之外；源库未修改");
        ensure!(self.source.canonicalize()? == self.source, "源目录位置已变化，请重新预览");
        if self.report.mode != "native_only" {
            let paths = discover(&self.source, self.native.is_some(), &mut Report::default())?;
            ensure!(paths == self.originals.iter().map(|o| o.relative.clone()).collect::<Vec<_>>(), "Markdown 文件列表已变化，请重新预览");
        }
        ensure!(self.source.join("format.json").try_exists()? == self.native.is_some(), "源库格式已变化，请重新预览");
        for original in &self.originals { ensure!(read_source_file(&self.source, &original.relative, 16 * 1024 * 1024)? == original.bytes, "源文件已变化，请重新预览：{}", original.relative.display()); }
        for (relative, bytes) in &self.assets { ensure!(&read_source_file(&self.source, relative, 64 * 1024 * 1024)? == bytes, "源附件已变化，请重新预览：{}", relative.display()); }
        if let Some(native) = &self.native {
            native.verify_source(&self.source)?;
            ensure!(self.notes.iter().filter(|n| !n.native).all(|n| !native.has_id(&n.id)) && self.folders.values().all(|id| !native.has_id(id)), "新旧文档 ID 冲突，请重新预览");
        }
        let stage = parent.join(format!(".odyssey-import-{}", uuid::Uuid::new_v4())); fs::create_dir(&stage)?;
        let result = (|| -> Result<()> {
            if let Some(native) = &self.native { native.write_to(&stage)?; }
            let mut store = Store::open_at(stage.clone())?;
            for (folder, id) in &self.folders {
                let doc = Document { title: RichText::plain(folder.file_name().unwrap_or_default().to_string_lossy()), blocks: vec![Block::new(BlockKind::SubPageList, RichText::default())], ..Default::default() };
                store.save(id, &doc)?;
                if let Some(parent) = folder.parent().and_then(|p| self.folders.get(p)) { store.set_parent(id, Some(parent))?; }
            }
            for note in self.notes.iter().filter(|n| !n.native) {
                store.save(&note.id, &note.doc)?;
                if let Some(parent) = note.relative.parent().and_then(|p| self.folders.get(p)) { store.set_parent(&note.id, Some(parent))?; }
            }
            for original in &self.originals {
                let dest = stage.join(".odyssey-import/source").join(&original.relative); fs::create_dir_all(dest.parent().unwrap())?; fs::write(dest, &original.bytes)?;
            }
            for (relative, bytes) in &self.assets {
                let dest = stage.join("assets").join(&self.asset_prefix).join(relative); fs::create_dir_all(dest.parent().unwrap())?; fs::write(dest, bytes)?;
            }
            fs::create_dir_all(stage.join(".odyssey-import"))?;
            if let Some(native) = &self.native { native.finish(&stage)?; }
            fs::write(stage.join(".odyssey-import/report.json"), serde_json::to_vec_pretty(&self.report)?)?;
            ensure!(crate::vault::inspect(&stage)?.format == FORMAT, "目标格式验证失败");
            for note in &self.notes { let loaded = store.load(&note.id)?; ensure!(loaded.doc_links().len() == note.doc.doc_links().len(), "链接往返校验失败"); }
            ensure!(!target.try_exists()?, "目标在导入期间被创建，未覆盖");
            publish_new(&stage, target)?; Ok(())
        })();
        if result.is_err() { let _ = fs::remove_dir_all(&stage); }
        result?; target.canonicalize().map_err(Into::into)
    }
}

/// macOS RENAME_EXCL publishes atomically without replacing even an empty folder.
fn publish_new(from: &Path, to: &Path) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        use std::{ffi::CString, os::unix::ffi::OsStrExt};
        unsafe extern "C" { fn renamex_np(from: *const std::ffi::c_char, to: *const std::ffi::c_char, flags: u32) -> i32; }
        let from = CString::new(from.as_os_str().as_bytes())?; let to = CString::new(to.as_os_str().as_bytes())?;
        if unsafe { renamex_np(from.as_ptr(), to.as_ptr(), 0x00000004) } != 0 { return Err(std::io::Error::last_os_error().into()); }
        Ok(())
    }
    #[cfg(not(target_os = "macos"))]
    {
        ensure!(!to.try_exists()?, "目标已存在，未覆盖"); fs::rename(from, to)?; Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Temp(PathBuf);
    impl Temp {
        fn new() -> Self { let p = std::env::temp_dir().join(format!("odyssey-import-test-{}", uuid::Uuid::new_v4())); fs::create_dir(&p).unwrap(); Self(p) }
        fn put(&self, path: &str, value: &[u8]) { let p = self.0.join(path); fs::create_dir_all(p.parent().unwrap()).unwrap(); fs::write(p, value).unwrap(); }
    }
    impl Drop for Temp { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }

    #[test]
    fn import_preserves_sources_hierarchy_links_assets_and_code() {
        let t = Temp::new();
        let raw = b"# Plan\n\n[[../Reference/Design|design]]\n\n| Link | Value |\n| --- | --- |\n| [[../Reference/Design|cell]] | 1 |\n\n`[[../Reference/Design]]`\n\n```text\n[[literal]]\n```\n\n![[../media/picture.png]]\n\n[PDF](../media/guide.pdf)\n";
        t.put("src/Projects/Plan.md", raw); t.put("src/Reference/Design.md", b"# Design\n\nDetail");
        t.put("src/media/picture.png", b"png bytes"); t.put("src/media/guide.pdf", b"pdf bytes"); t.put("src/.obsidian/config.json", b"private settings");
        let plan = Plan::scan(&t.0.join("src")).unwrap();
        assert_eq!(plan.report.notes, 2); assert_eq!(plan.report.attachments, 2); assert_eq!(plan.report.resolved_links, 2);
        assert_eq!(plan.report.folders, 2); assert!(!t.0.join("dest").exists());
        let target = plan.write_new(&t.0.join("dest")).unwrap();
        assert_eq!(fs::read(t.0.join("src/Projects/Plan.md")).unwrap(), raw);
        assert_eq!(fs::read(target.join(".odyssey-import/source/Projects/Plan.md")).unwrap(), raw);
        assert_eq!(fs::read(target.join("assets/imported/media/picture.png")).unwrap(), b"png bytes");
        assert!(!target.join(".obsidian").exists());
        let mut store = Store::open_at(target).unwrap();
        let doc = store.load(&plan.report.paths["Projects/Plan.md"]).unwrap();
        assert!(doc.doc_links().iter().all(|l| l.target == plan.report.paths["Reference/Design.md"]));
        assert!(docxml::serialize(&doc).contains("[[literal]]"));
        assert!(docxml::serialize(&doc).contains("[[../Reference/Design]]"));
        let entries = store.list(); assert_eq!(entries.len(), 4);
        let plan_entry = entries.iter().find(|e| e.id == plan.report.paths["Projects/Plan.md"]).unwrap(); assert!(plan_entry.meta.parent.is_some());
    }
    #[test]
    fn ambiguous_missing_anchors_and_embeds_are_reported_not_mislinked() {
        let t = Temp::new();
        t.put("src/Start.md", b"[[Guide]] [[Missing]] [[A/Guide#Heading]] ![[A/Guide]]");
        t.put("src/A/Guide.md", b"A"); t.put("src/B/Guide.md", b"B");
        let plan = Plan::scan(&t.0.join("src")).unwrap();
        assert_eq!(plan.report.resolved_links, 0); assert_eq!(plan.report.warnings.len(), 4);
        let start = plan.notes.iter().find(|n| n.title == "Start").unwrap();
        assert!(docxml::serialize(&start.doc).contains("[[Guide]]"));
    }
    #[test]
    fn aliases_code_and_multilevel_hierarchy_survive_save_and_reopen() {
        let t = Temp::new();
        t.put("src/Projects/Deep/Start.md", b"[[../../Reference/Design|body alias]]\n\n| Link | Code |\n| --- | --- |\n| [[../../Reference/Design|cell alias]] | `[[../../Reference/Design]]` |\n\n`[[../../Reference/Design|literal alias]]` and `[literal](../../Reference/Design.md)`\n\n```text\n[[../../Reference/Design]]\n[literal](../../Reference/Design.md)\n```\n");
        t.put("src/Reference/Design.md", b"Target"); t.put("src/.obsidian/config", b"private");
        let before = snapshot(&t.0.join("src")); let plan = Plan::scan(&t.0.join("src")).unwrap();
        assert_eq!(plan.report.resolved_links, 2); assert!(plan.report.warnings.is_empty());
        let target = plan.write_new(&t.0.join("dest")).unwrap();
        let id = &plan.report.paths["Projects/Deep/Start.md"]; let reference = &plan.report.paths["Reference/Design.md"];
        {
            let mut store = Store::open_at(target.clone()).unwrap(); let doc = store.load(id).unwrap(); store.save(id, &doc).unwrap();
        }
        let mut store = Store::open_at(target).unwrap(); let doc = store.load(id).unwrap(); let mut links = vec![];
        doc.for_each_block(|b| { for run in &b.text.runs { if let Some(link) = run.marks.inline.as_ref().and_then(Inline::doc_link) { links.push((link.id.clone(), link.alias.clone())); } } });
        assert_eq!(links, vec![(reference.clone(), Some("body alias".into())), (reference.clone(), Some("cell alias".into()))]);
        let table = doc.blocks.iter().find_map(|b| if let BlockKind::Table(t) = &b.kind { Some(t) } else { None }).unwrap();
        assert!(table.rows.iter().flatten().flat_map(|c| &c.blocks).any(|b| b.text.runs.iter().any(|r| r.marks.inline.as_ref().and_then(Inline::doc_link).is_some_and(|l| l.alias.as_deref() == Some("cell alias")))));
        let xml = docxml::serialize(&doc); assert!(xml.contains("[[../../Reference/Design|literal alias]]")); assert!(xml.contains("[literal](../../Reference/Design.md)"));
        let entries = store.list(); let entry = entries.iter().find(|e| &e.id == id).unwrap();
        assert_eq!(entry.meta.parent.as_ref(), Some(&plan.folders[Path::new("Projects/Deep")]));
        let deep = entries.iter().find(|e| e.id == plan.folders[Path::new("Projects/Deep")]).unwrap();
        assert_eq!(deep.meta.parent.as_ref(), Some(&plan.folders[Path::new("Projects")]));
        assert_eq!(snapshot(&t.0.join("src")), before);
    }
    #[test]
    fn explicit_paths_disambiguate_duplicate_titles_and_decode_spaces() {
        let t = Temp::new();
        t.put("src/A/Start.md", b"[[Guide|local]] [[B/Guide|root path]] [[../B/Space%20Note|encoded]]");
        t.put("src/A/Guide.md", b"A"); t.put("src/B/Guide.md", b"B"); t.put("src/B/Space Note.md", b"Space");
        t.put("src/Start.md", b"[[Guide|ambiguous]]");
        let plan = Plan::scan(&t.0.join("src")).unwrap(); assert_eq!(plan.report.resolved_links, 3); assert_eq!(plan.report.warnings.len(), 1);
        let note = plan.notes.iter().find(|n| n.relative == Path::new("A/Start.md")).unwrap();
        let links = note.doc.doc_links(); assert_eq!(links.len(), 3);
        for (link, path) in links.iter().zip(["A/Guide.md", "B/Guide.md", "B/Space Note.md"]) { assert_eq!(link.target, plan.report.paths[path]); }
        let unresolved = plan.notes.iter().find(|n| n.relative == Path::new("Start.md")).unwrap(); assert!(docxml::serialize(&unresolved.doc).contains("[[Guide|ambiguous]]"));
    }
    #[test]
    fn added_notes_format_changes_and_changed_assets_invalidate_preview() {
        let t = Temp::new(); t.put("src/Note.md", b"![[media/picture.png]]"); t.put("src/media/picture.png", b"before");
        let plan = Plan::scan(&t.0.join("src")).unwrap();
        t.put("src/New.md", b"new since preview"); assert!(plan.write_new(&t.0.join("added")).is_err()); assert!(!t.0.join("added").exists());
        fs::remove_file(t.0.join("src/New.md")).unwrap();
        t.put("src/format.json", b"{}"); assert!(plan.write_new(&t.0.join("format-changed")).is_err()); assert!(!t.0.join("format-changed").exists());
        fs::remove_file(t.0.join("src/format.json")).unwrap();
        t.put("src/media/picture.png", b"changed"); let before = snapshot(&t.0.join("src"));
        assert!(plan.write_new(&t.0.join("asset-changed")).is_err()); assert!(!t.0.join("asset-changed").exists()); assert_eq!(snapshot(&t.0.join("src")), before);
    }
    #[cfg(unix)]
    #[test]
    fn attachment_parent_escape_and_symlink_components_never_get_copied() {
        use std::os::unix::fs::symlink;
        let t = Temp::new(); t.put("outside.png", b"private"); t.put("src/Note.md", b"![[../outside.png]]");
        let plan = Plan::scan(&t.0.join("src")).unwrap(); assert_eq!(plan.report.attachments, 0); assert_eq!(plan.report.warnings.len(), 1);
        assert!(docxml::serialize(&plan.notes[0].doc).contains("![[../outside.png]]"));
        t.put("src/real/picture.png", b"inside"); symlink("real", t.0.join("src/media")).unwrap();
        t.put("src/Note.md", b"![[media/picture.png]]"); assert!(Plan::scan(&t.0.join("src")).is_err());
        fs::remove_file(t.0.join("src/media")).unwrap(); symlink("real/picture.png", t.0.join("src/picture.png")).unwrap();
        t.put("src/Note.md", b"![[picture.png]]"); assert!(Plan::scan(&t.0.join("src")).is_err());
        fs::remove_file(t.0.join("src/picture.png")).unwrap(); t.put("src/picture.png", b"inside");
        let plan = Plan::scan(&t.0.join("src")).unwrap(); fs::remove_file(t.0.join("src/picture.png")).unwrap(); symlink("real/picture.png", t.0.join("src/picture.png")).unwrap();
        assert!(plan.write_new(&t.0.join("changed-link")).is_err()); assert!(!t.0.join("changed-link").exists());
        assert_eq!(fs::read(t.0.join("outside.png")).unwrap(), b"private");
    }
    #[test]
    fn source_changes_and_existing_or_nested_targets_do_not_publish() {
        let t = Temp::new(); t.put("src/Note.md", b"before");
        let plan = Plan::scan(&t.0.join("src")).unwrap();
        assert!(plan.write_new(&t.0.join("src/Nested")).is_err());
        fs::create_dir(t.0.join("occupied")).unwrap(); t.put("occupied/keep", b"user data");
        assert!(plan.write_new(&t.0.join("occupied")).is_err());
        assert_eq!(fs::read(t.0.join("occupied/keep")).unwrap(), b"user data");
        t.put("src/Note.md", b"changed"); assert!(plan.write_new(&t.0.join("dest")).is_err());
        assert!(!t.0.join("dest").exists());
        assert!(fs::read_dir(&t.0).unwrap().all(|e| !e.unwrap().file_name().to_string_lossy().starts_with(".odyssey-import-")));
    }
    #[test]
    fn invalid_utf8_and_external_assets_fail_before_target_writes() {
        let t = Temp::new(); t.put("src/Bad.md", &[0xff]); assert!(Plan::scan(&t.0.join("src")).is_err());
        fs::remove_file(t.0.join("src/Bad.md")).unwrap();
        t.put("src/Note.md", b"![[escape/file.png]]"); t.put("outside/file.png", b"external");
        #[cfg(unix)] {
            std::os::unix::fs::symlink(t.0.join("outside"), t.0.join("src/escape")).unwrap();
            assert!(Plan::scan(&t.0.join("src")).is_err());
        }
        assert!(!t.0.join("dest").exists());
    }
    #[test]
    fn atomic_publish_does_not_replace_an_existing_directory() {
        let t = Temp::new(); fs::create_dir(t.0.join("stage")).unwrap(); fs::create_dir(t.0.join("target")).unwrap();
        t.put("stage/content", b"stage");
        assert!(publish_new(&t.0.join("stage"), &t.0.join("target")).is_err());
        assert!(t.0.join("stage/content").exists()); assert!(t.0.join("target").is_dir());
    }

    fn snapshot(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
        let mut out = BTreeMap::new(); let mut todo = vec![root.to_path_buf()];
        while let Some(dir) = todo.pop() { for e in fs::read_dir(dir).unwrap() {
            let p = e.unwrap().path(); if p.is_dir() { todo.push(p); } else { out.insert(p.strip_prefix(root).unwrap().to_path_buf(), fs::read(p).unwrap()); }
        } }
        out
    }

    #[test]
    fn mixed_vault_preserves_twelve_native_documents_metadata_and_assets_without_source_writes() {
        let t = Temp::new(); let source = t.0.join("src"); fs::create_dir(&source).unwrap(); crate::vault::create(&source).unwrap();
        let mut index = serde_json::Map::new();
        for i in 0..12 {
            let xml = format!("<title>Native {i}</title><table><tbody><tr><td><p><cite type=\"doc\" doc-id=\"native-1\" file-type=\"docx\">Native 1</cite></p></td></tr></tbody></table><img path=\"assets/image.png\"/>");
            t.put(&format!("src/docs/native-{i}.xml"), xml.as_bytes());
            index.insert(format!("native-{i}"), serde_json::json!({"created":i,"modified":i+1,"parent":if i==0 {None} else {Some("native-0")},"order":i,"future_metadata":"retain"}));
        }
        t.put("src/index.json", &serde_json::to_vec(&index).unwrap());
        t.put("src/preferences.json", b"{\"english\":false}"); t.put("src/settings.json", b"{\"sidebar_width\":333}");
        t.put("src/assets/image.png", b"original native image"); t.put("src/assets/imported/guide.md", b"resource, not a note");
        t.put("src/trash/deleted.xml", b"<title>Deleted original</title>");
        t.put("src/trash/index.json", b"{\"deleted\":{\"meta\":{\"created\":1,\"modified\":2},\"deleted\":9999999999}}");
        t.put("src/.obsidian/config.json", b"do not touch"); t.put("src/.git/config", b"do not touch");
        t.put("src/New.md", b"[[Native 1|existing]] [[Missing]]");
        let before = snapshot(&source); let plan = Plan::scan(&source).unwrap();
        assert_eq!(plan.report.notes, 1); assert_eq!(plan.report.native_notes, 12); assert_eq!(plan.report.native_attachments, 2);
        assert_eq!(plan.report.native_trash, 1);
        assert_eq!(plan.report.resolved_links, 1); assert!(!plan.report.warnings.is_empty());
        let target = plan.write_new(&t.0.join("dest")).unwrap();
        assert_eq!(snapshot(&source), before); assert!(!source.join(".lock").exists());
        for i in 0..12 {
            let rel = format!("docs/native-{i}.xml");
            assert_eq!(fs::read(target.join(&rel)).unwrap(), before[Path::new(&rel)]);
            assert_eq!(fs::read(target.join(".odyssey-import/native").join(&rel)).unwrap(), before[Path::new(&rel)]);
        }
        let copied: serde_json::Value = serde_json::from_slice(&fs::read(target.join("index.json")).unwrap()).unwrap();
        for (id, meta) in index { assert_eq!(copied[&id], meta); }
        assert_eq!(fs::read(target.join("assets/image.png")).unwrap(), b"original native image");
        assert_eq!(fs::read(target.join("preferences.json")).unwrap(), before[Path::new("preferences.json")]);
        assert_eq!(fs::read(target.join("trash/deleted.xml")).unwrap(), before[Path::new("trash/deleted.xml")]);
        assert_eq!(fs::read(target.join("trash/index.json")).unwrap(), before[Path::new("trash/index.json")]);
        let store = Store::open_at(target).unwrap();
        assert_eq!(store.load(&plan.report.paths["New.md"]).unwrap().doc_links()[0].target, "native-1");
    }

    #[test]
    fn native_only_recovery_leaves_markdown_unread_and_rejects_changed_native_files() {
        let t = Temp::new(); let source = t.0.join("src"); fs::create_dir(&source).unwrap(); crate::vault::create(&source).unwrap();
        t.put("src/docs/keep.xml", b"<title>Keep</title><p>unchanged</p>");
        t.put("src/.obsidian/config", b"private"); t.put("src/invalid.md", &[0xff]);
        let before = snapshot(&source); let plan = Plan::recover_native(&source).unwrap();
        assert_eq!(plan.report.notes, 0); assert_eq!(plan.report.native_notes, 1);
        let target = plan.write_new(&t.0.join("native-copy")).unwrap();
        assert_eq!(snapshot(&source), before); assert!(!target.join("invalid.md").exists());
        assert_eq!(Store::open_at(target).unwrap().load("keep").unwrap().title.text, "Keep");
        t.put("src/docs/keep.xml", b"<title>Changed since preview</title>");
        assert!(plan.write_new(&t.0.join("stale")).is_err()); assert!(!t.0.join("stale").exists());
        t.put("src/index.json", b"corrupt"); assert!(Plan::recover_native(&source).is_err());
        assert_eq!(fs::read(source.join("index.json")).unwrap(), b"corrupt");
    }
    #[cfg(unix)]
    #[test]
    fn read_only_mixed_source_is_copied_and_native_symlinks_are_rejected() {
        use std::os::unix::fs::PermissionsExt;
        let t = Temp::new(); let source = t.0.join("src"); fs::create_dir(&source).unwrap(); crate::vault::create(&source).unwrap();
        t.put("src/docs/note.xml", b"<title>Read only</title>"); t.put("src/Note.md", b"[[Read only]]");
        t.put("src/.obsidian/config", b"untouched");
        let before = snapshot(&source);
        for rel in before.keys() { fs::set_permissions(source.join(rel), fs::Permissions::from_mode(0o444)).unwrap(); }
        let dirs = [source.clone(), source.join("docs"), source.join("assets"), source.join("trash"), source.join(".obsidian")];
        for d in &dirs { fs::set_permissions(d, fs::Permissions::from_mode(0o555)).unwrap(); }
        let result = Plan::scan(&source).and_then(|p| p.write_new(&t.0.join("copied")));
        for d in &dirs { fs::set_permissions(d, fs::Permissions::from_mode(0o755)).unwrap(); }
        assert!(result.is_ok(), "{result:?}"); assert_eq!(snapshot(&source), before);
        t.put("outside", b"private"); std::os::unix::fs::symlink(t.0.join("outside"), source.join("assets/escape")).unwrap();
        assert!(Plan::recover_native(&source).is_err()); assert!(!t.0.join("unexpected").exists());
    }
    #[test]
    fn native_absolute_assets_and_covers_survive_source_move_and_repeated_recovery() {
        let t = Temp::new(); let source = t.0.join("src"); fs::create_dir(&source).unwrap(); crate::vault::create(&source).unwrap();
        t.put("src/assets/image.png", b"independent asset");
        let asset = source.join("assets/image.png");
        let doc = Document { title: RichText::plain("Preserved"), cover: Some(Cover { src: asset.display().to_string(), x: 0., y: 0. }), blocks: vec![Block::new(BlockKind::Image { src_kind: "path".into(), src: asset.display().to_string(), width: None, height: None, caption: None, crop: None }, RichText::default())], ..Default::default() };
        let xml = docxml::serialize(&doc); t.put("src/docs/keep.xml", xml.as_bytes()); t.put("src/Keep.md", b"original markdown");
        let plan = Plan::recover_native(&source).unwrap(); assert!(plan.report.warnings.is_empty());
        let target = plan.write_new(&t.0.join("copy")).unwrap();
        fs::rename(&source, t.0.join("source-moved")).unwrap();
        assert_eq!(fs::read_to_string(target.join("docs/keep.xml")).unwrap(), xml);
        let resolved = crate::store::asset_path_at(&target, &asset.display().to_string()).unwrap();
        assert_eq!(resolved, target.join("assets/image.png")); assert_eq!(fs::read(resolved).unwrap(), b"independent asset");
        let second = Plan::recover_native(&target).unwrap().write_new(&t.0.join("second")).unwrap();
        fs::rename(&target, t.0.join("copy-moved")).unwrap();
        assert_eq!(fs::read(crate::store::asset_path_at(&second, &asset.display().to_string()).unwrap()).unwrap(), b"independent asset");
        assert_eq!(fs::read_to_string(second.join("docs/keep.xml")).unwrap(), xml);
    }
}
