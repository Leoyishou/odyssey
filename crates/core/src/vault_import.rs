//! Copy-only Markdown import: read/preview first, publish a complete new vault last.
use crate::{docxml, model::*, paste, store::{Store, FORMAT}};
use anyhow::{Context, Result, ensure};
use serde::Serialize;
use std::{collections::BTreeMap, fs, path::{Component, Path, PathBuf}};

#[derive(Clone, Debug, Serialize)]
pub struct Warning { pub file: String, pub detail: String }
#[derive(Clone, Debug, Default, Serialize)]
pub struct Report {
    pub notes: usize, pub folders: usize, pub attachments: usize, pub resolved_links: usize,
    pub warnings: Vec<Warning>,
    pub paths: BTreeMap<String, String>,
}
struct Original { relative: PathBuf, bytes: Vec<u8> }
struct Note { relative: PathBuf, id: String, title: String, doc: Document }
#[derive(Clone)]
struct Token { key: String, raw: String, target: String, label: Option<String>, embed: bool }
pub struct Plan {
    pub source: PathBuf,
    pub report: Report,
    notes: Vec<Note>, folders: BTreeMap<PathBuf, String>, originals: Vec<Original>,
    assets: BTreeMap<PathBuf, Vec<u8>>,
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
fn discover(root: &Path, report: &mut Report) -> Result<Vec<PathBuf>> {
    let mut found = Vec::new(); let mut todo = vec![PathBuf::new()];
    while let Some(relative) = todo.pop() {
        for entry in fs::read_dir(root.join(&relative))? {
            let e = entry?; let path = relative.join(e.file_name()); let ft = e.file_type()?;
            if ft.is_symlink() { warning(report, &path, "符号链接已跳过，未读取其目标"); continue; }
            if e.file_name().to_string_lossy().starts_with('.') { continue; }
            if ft.is_dir() { todo.push(path); }
            else if ft.is_file() && is_markdown(&path) { found.push(path); }
            else if path.starts_with("docs") && path.extension().is_some_and(|e| e == "xml") { warning(report, &path, "已有 XML 文档保留在源目录；本次只复制导入 Markdown"); }
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
        let source = source.canonicalize()?; ensure!(source.is_dir(), "请选择 Markdown 文件夹");
        let mut plan = Self { source, report: Report::default(), notes: vec![], folders: BTreeMap::new(), originals: vec![], assets: BTreeMap::new() };
        let paths = discover(&plan.source, &mut plan.report)?;
        ensure!(!paths.is_empty(), "未找到可导入的 Markdown 文件；没有创建目标库");
        let mut total = 0;
        for relative in paths {
            let bytes = read_bounded(&plan.source.join(&relative), 16 * 1024 * 1024)?; total += bytes.len();
            ensure!(total <= 256 * 1024 * 1024, "Markdown 总量超过本次预览上限，尚未创建目标库");
            std::str::from_utf8(&bytes).with_context(|| format!("文件不是 UTF-8，停止导入以免损坏文字：{}", relative.display()))?;
            let id = uuid::Uuid::new_v4().to_string();
            plan.report.paths.insert(key(&relative), id.clone());
            let title = relative.file_stem().unwrap_or_default().to_string_lossy().to_string();
            plan.notes.push(Note { relative: relative.clone(), id, title, doc: Document::default() });
            let mut parent = relative.parent();
            while let Some(p) = parent.filter(|p| !p.as_os_str().is_empty()) { plan.folders.entry(p.to_path_buf()).or_insert_with(|| uuid::Uuid::new_v4().to_string()); parent = p.parent(); }
            plan.originals.push(Original { relative, bytes });
        }
        for i in 0..plan.notes.len() {
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
        plan.report.notes = plan.notes.len(); plan.report.folders = plan.folders.len(); plan.report.attachments = plan.assets.len();
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
                let bytes = read_bounded(&path, 64 * 1024 * 1024)?;
                ensure!(self.assets.values().map(Vec::len).sum::<usize>() + bytes.len() <= 512 * 1024 * 1024, "附件总量超过本次预览上限");
                self.assets.insert(relative.clone(), bytes);
            }
            return Ok(Some(FileBlock { src_kind: "path".into(), src: format!("assets/imported/{}", key(&relative)), name: relative.file_name().unwrap_or_default().to_string_lossy().to_string(), view: FileView::Card }));
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
        for original in &self.originals { ensure!(fs::read(self.source.join(&original.relative))? == original.bytes, "源文件已变化，请重新预览：{}", original.relative.display()); }
        for (relative, bytes) in &self.assets { ensure!(&fs::read(self.source.join(relative))? == bytes, "源附件已变化，请重新预览：{}", relative.display()); }
        let stage = parent.join(format!(".odyssey-import-{}", uuid::Uuid::new_v4())); fs::create_dir(&stage)?;
        let result = (|| -> Result<()> {
            let mut store = Store::open_at(stage.clone())?;
            for (folder, id) in &self.folders {
                let doc = Document { title: RichText::plain(folder.file_name().unwrap_or_default().to_string_lossy()), blocks: vec![Block::new(BlockKind::SubPageList, RichText::default())], ..Default::default() };
                store.save(id, &doc)?;
                if let Some(parent) = folder.parent().and_then(|p| self.folders.get(p)) { store.set_parent(id, Some(parent))?; }
            }
            for note in &self.notes {
                store.save(&note.id, &note.doc)?;
                if let Some(parent) = note.relative.parent().and_then(|p| self.folders.get(p)) { store.set_parent(&note.id, Some(parent))?; }
            }
            for original in &self.originals {
                let dest = stage.join(".odyssey-import/source").join(&original.relative); fs::create_dir_all(dest.parent().unwrap())?; fs::write(dest, &original.bytes)?;
            }
            for (relative, bytes) in &self.assets {
                let dest = stage.join("assets/imported").join(relative); fs::create_dir_all(dest.parent().unwrap())?; fs::write(dest, bytes)?;
            }
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
}
