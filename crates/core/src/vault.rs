//! Explicit vault validation/creation. Inspecting a folder never repairs or writes it.

use anyhow::{Context, Result, bail, ensure};
use serde::Deserialize;
use std::{fs, path::{Path, PathBuf}};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VaultInfo {
    pub path: PathBuf,
    pub name: String,
    pub format: u32,
}

#[derive(Deserialize)]
struct Marker { format: u32 }

pub fn inspect(path: &Path) -> Result<VaultInfo> {
    ensure!(path.is_dir(), "文档库目录不存在或不是文件夹：{}", path.display());
    let entries = fs::read_dir(path).with_context(|| format!("无法读取文档库目录：{}", path.display()))?;
    let mut markdown = false;
    for entry in entries {
        let entry = entry?;
        markdown |= entry.file_name() == ".obsidian" || entry.path().extension().is_some_and(|e| e.eq_ignore_ascii_case("md"));
    }
    let marker = path.join("format.json");
    if !marker.try_exists()? {
        if markdown { bail!("这是 Markdown / Obsidian 文件夹，当前不能直接打开；源文件未修改") }
        bail!("不是 Odyssey 文档库：缺少 format.json；目录未修改")
    }
    let bytes = fs::read(&marker).with_context(|| format!("无法读取库格式：{}", marker.display()))?;
    let marker: Marker = serde_json::from_slice(&bytes).context("文档库格式标记损坏；目录未修改")?;
    ensure!(marker.format == crate::store::FORMAT, "不支持文档库格式版本 {}（当前支持 {}）；目录未修改", marker.format, crate::store::FORMAT);
    ensure!(path.join("docs").is_dir(), "文档库缺少 docs 目录；目录未修改");
    let path = path.canonicalize()?;
    let name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
    Ok(VaultInfo { path, name, format: marker.format })
}

/// Publish a complete empty vault by renaming a sibling staging directory.
/// On supported local filesystems rename cannot replace a nonempty directory.
pub fn create(path: &Path) -> Result<VaultInfo> {
    ensure!(path.file_name().is_some(), "请选择新文档库的文件夹名称");
    let parent = path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
    ensure!(parent.is_dir(), "父文件夹不存在：{}", parent.display());
    if path.try_exists()? {
        ensure!(!fs::symlink_metadata(path)?.file_type().is_symlink(), "不能在符号链接上创建文档库");
        ensure!(path.is_dir() && fs::read_dir(path)?.next().is_none(), "文件夹不是空目录，未覆盖：{}", path.display());
    }
    let staging = parent.join(format!(".odyssey-create-{}", uuid::Uuid::new_v4()));
    fs::create_dir(&staging)?;
    let result = (|| -> Result<()> {
        for dir in ["docs", "assets", "trash"] { fs::create_dir(staging.join(dir))?; }
        fs::write(staging.join("format.json"), format!("{{\"format\":{}}}\n", crate::store::FORMAT))?;
        fs::write(staging.join("index.json"), b"{}\n")?;
        fs::rename(&staging, path).with_context(|| format!("无法创建文档库，目标可能已被占用：{}", path.display()))?;
        Ok(())
    })();
    if result.is_err() { let _ = fs::remove_dir_all(&staging); }
    result?;
    inspect(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Temp(PathBuf);
    impl Temp {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!("odyssey-vault-test-{}", uuid::Uuid::new_v4()));
            fs::create_dir(&p).unwrap(); Self(p)
        }
    }
    impl Drop for Temp { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }

    #[test]
    fn create_open_and_refuse_nonempty_folder() {
        let t = Temp::new(); let path = t.0.join("中文库");
        let info = create(&path).unwrap();
        assert_eq!(info.name, "中文库");
        assert_eq!(inspect(&path).unwrap(), info);
        fs::write(path.join("docs/note.xml"), "<title>keep</title>").unwrap();
        assert!(create(&path).is_err());
        assert_eq!(fs::read_to_string(path.join("docs/note.xml")).unwrap(), "<title>keep</title>");
        assert!(fs::read_dir(&t.0).unwrap().all(|e| !e.unwrap().file_name().to_string_lossy().starts_with(".odyssey-create")));
    }

    #[test]
    fn rejected_vaults_are_not_modified() {
        let t = Temp::new();
        fs::write(t.0.join("note.md"), "user markdown").unwrap();
        assert!(inspect(&t.0).unwrap_err().to_string().contains("Markdown"));
        assert_eq!(fs::read_dir(&t.0).unwrap().count(), 1);
        for marker in ["broken json", "{\"format\":9999}"] {
            fs::write(t.0.join("format.json"), marker).unwrap();
            assert!(inspect(&t.0).is_err());
            assert_eq!(fs::read_to_string(t.0.join("format.json")).unwrap(), marker);
            assert!(!t.0.join("docs").exists());
            assert!(!t.0.join(".lock").exists());
        }
    }

    #[test]
    fn two_vaults_and_existing_empty_folder_are_independent() {
        let t = Temp::new(); let a = t.0.join("a"); let b = t.0.join("b");
        fs::create_dir(&b).unwrap(); create(&a).unwrap(); create(&b).unwrap();
        fs::write(a.join("docs/same-id.xml"), "A").unwrap();
        fs::write(b.join("docs/same-id.xml"), "B").unwrap();
        assert_ne!(inspect(&a).unwrap().path, inspect(&b).unwrap().path);
        assert_eq!(fs::read_to_string(a.join("docs/same-id.xml")).unwrap(), "A");
        assert_eq!(fs::read_to_string(b.join("docs/same-id.xml")).unwrap(), "B");
    }
}
