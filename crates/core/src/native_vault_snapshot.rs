//! Read-only snapshot of native files inside an explicitly selected import source.
use crate::{docxml, model::Document, store::{Meta, FORMAT}};
use anyhow::{Context, Result, ensure};
use std::{collections::{BTreeMap, HashMap}, fs, path::{Path, PathBuf}};

pub(crate) struct Snapshot {
    pub docs: Vec<(String, Document)>,
    pub assets: usize,
    pub trash: usize,
    pub warnings: Vec<String>,
    files: BTreeMap<PathBuf, Vec<u8>>,
    index: serde_json::Map<String, serde_json::Value>,
    origins: Vec<PathBuf>,
}

fn paths(root: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for name in ["format.json", "index.json", "index.json.bak", "settings.json", "preferences.json", ".odyssey-relocations.json"] {
        let p = root.join(name);
        if fs::symlink_metadata(&p).is_ok() { out.push(PathBuf::from(name)); }
    }
    for name in ["docs", "assets", "trash", "themes"] {
        let p = root.join(name); if !p.try_exists()? { continue; }
        ensure!(!fs::symlink_metadata(&p)?.file_type().is_symlink(), "原生目录为符号链接，未复制：{}", p.display());
        let mut todo = vec![PathBuf::from(name)];
        while let Some(dir) = todo.pop() {
            for e in fs::read_dir(root.join(&dir))? {
                let e = e?; let relative = dir.join(e.file_name()); let kind = e.file_type()?;
                ensure!(!kind.is_symlink(), "原生文件为符号链接，未复制：{}", relative.display());
                if kind.is_dir() { if name != "docs" { todo.push(relative); } }
                else if kind.is_file() && (name != "docs" || relative.extension().is_some_and(|e| e == "xml")) { out.push(relative); }
            }
        }
    }
    out.sort(); Ok(out)
}

impl Snapshot {
    pub fn read(root: &Path) -> Result<Option<Self>> {
        if !root.join("format.json").try_exists()? { return Ok(None); }
        let mut files = BTreeMap::new(); let mut total = 0;
        for relative in paths(root)? {
            let p = root.join(&relative); let meta = fs::symlink_metadata(&p)?;
            ensure!(meta.is_file() && !meta.file_type().is_symlink(), "原生文件不是普通文件：{}", relative.display());
            ensure!(p.canonicalize()?.starts_with(root), "原生文件越出源目录：{}", relative.display());
            ensure!(meta.len() <= 64 * 1024 * 1024, "原生文件超过 64 MiB：{}", relative.display());
            let bytes = fs::read(&p)?; total += bytes.len();
            ensure!(total <= 512 * 1024 * 1024, "原生文件总量超过 512 MiB，未创建目标库");
            files.insert(relative, bytes);
        }
        let marker: serde_json::Value = serde_json::from_slice(&files[Path::new("format.json")])?;
        ensure!(marker.get("format").and_then(|v| v.as_u64()) == Some(FORMAT as u64), "原生库格式不受支持；源目录未修改");
        ensure!(root.join("docs").is_dir(), "格式标记存在但缺少 docs；未创建目标库");
        let index = if let Some(bytes) = files.get(Path::new("index.json")) {
            let _: HashMap<String, Meta> = serde_json::from_slice(bytes).context("原生索引损坏，停止复制；源文件未修改")?;
            serde_json::from_slice::<serde_json::Value>(bytes)?.as_object().cloned().unwrap()
        } else { Default::default() };
        let mut origins: Vec<PathBuf> = files.get(Path::new(".odyssey-relocations.json")).map(|b| serde_json::from_slice(b)).transpose()?.unwrap_or_default();
        ensure!(origins.len() < 32 && origins.iter().all(|p| p.is_absolute()), "原生附件重定位记录无效，源库未修改");
        if !origins.iter().any(|p| p == root) { origins.push(root.to_path_buf()); }
        let mut snapshot = Self { docs: vec![], assets: 0, trash: 0, warnings: vec![], files, index, origins };
        for (relative, bytes) in &snapshot.files {
            if relative.starts_with("assets") { snapshot.assets += 1; }
            if relative.starts_with("trash") && relative.extension().is_some_and(|e| e == "xml") { snapshot.trash += 1; }
            if relative.starts_with("docs") {
                let id = relative.file_stem().and_then(|s| s.to_str()).ok_or_else(|| anyhow::anyhow!("无效 XML 文件名"))?.to_string();
                let doc = docxml::parse(std::str::from_utf8(bytes)?).with_context(|| format!("原生 XML 不能解析，停止复制：{}", relative.display()))?;
                snapshot.docs.push((id, doc));
            }
        }
        // Record proven aliases while the source exists (including case aliases
        // on macOS). Later resolution is lexical and works after source removal.
        for (_, doc) in &snapshot.docs {
            for asset in doc.asset_sources().into_iter().chain(doc.cover.iter().map(|c| c.src.clone())) {
                let p = Path::new(asset.strip_prefix("file://").unwrap_or(&asset));
                if p.is_absolute() && let Ok(canonical) = p.canonicalize()
                    && let Ok(relative) = canonical.strip_prefix(root)
                    && relative.starts_with("assets") && snapshot.files.contains_key(relative)
                    && let Some(origin) = p.ancestors().nth(relative.components().count())
                    && origin.canonicalize().is_ok_and(|p| p == root)
                    && !snapshot.origins.iter().any(|p| p == origin)
                { snapshot.origins.push(origin.to_path_buf()); }
            }
        }
        ensure!(snapshot.origins.len() <= 32, "附件重定位来源过多，未创建目标库");
        for (id, doc) in &snapshot.docs {
            for asset in doc.asset_sources().into_iter().chain(doc.cover.iter().map(|c| c.src.clone())) {
                let p = Path::new(asset.strip_prefix("file://").unwrap_or(&asset));
                let local = if p.is_absolute() { snapshot.origins.iter().find_map(|o| p.strip_prefix(o).ok().filter(|r| r.starts_with("assets"))) } else { Some(p) };
                if !local.is_some_and(|p| snapshot.files.contains_key(p)) {
                    snapshot.warnings.push(format!("docs/{id}.xml：附件不在已复制原生资源中，原引用保留：{asset}"));
                }
            }
            for link in doc.doc_links() {
                if !snapshot.docs.iter().any(|(id, _)| *id == link.target) {
                    snapshot.warnings.push(format!("docs/{id}.xml：文档引用目标未在原生 XML 中找到，保留 UUID：{}", link.target));
                }
            }
        }
        Ok(Some(snapshot))
    }

    pub fn has_id(&self, id: &str) -> bool { self.index.contains_key(id) || self.docs.iter().any(|(i, _)| i == id) }
    pub fn verify_source(&self, root: &Path) -> Result<()> {
        ensure!(paths(root)? == self.files.keys().cloned().collect::<Vec<_>>(), "原生文件列表已变化，请重新预览");
        for (rel, bytes) in &self.files { ensure!(fs::read(root.join(rel))? == *bytes, "原生文件已变化，请重新预览：{}", rel.display()); }
        Ok(())
    }
    pub fn write_to(&self, target: &Path) -> Result<()> {
        for (rel, bytes) in &self.files {
            let p = target.join(rel); fs::create_dir_all(p.parent().unwrap())?; fs::write(p, bytes)?;
            if !rel.starts_with("assets") {
                let archive = target.join(".odyssey-import/native").join(rel);
                fs::create_dir_all(archive.parent().unwrap())?; fs::write(archive, bytes)?;
            }
        }
        fs::write(target.join(".odyssey-relocations.json"), serde_json::to_vec_pretty(&self.origins)?)?;
        Ok(())
    }
    // Store adds the new Markdown entries; restore complete original metadata
    // values (including fields unknown to this build) without losing new entries.
    pub fn finish(&self, target: &Path) -> Result<()> {
        let path = target.join("index.json");
        let mut index: serde_json::Map<String, serde_json::Value> = if path.exists() { serde_json::from_slice(&fs::read(&path)?)? } else { Default::default() };
        for (id, value) in &self.index { index.insert(id.clone(), value.clone()); }
        fs::write(path, serde_json::to_vec_pretty(&index)?)?;
        for (rel, bytes) in &self.files {
            if rel == Path::new("index.json") || rel == Path::new("index.json.bak") || rel == Path::new(".odyssey-relocations.json") { continue; }
            ensure!(fs::read(target.join(rel))? == *bytes, "原生文件复制校验失败：{}", rel.display());
        }
        Ok(())
    }
}
