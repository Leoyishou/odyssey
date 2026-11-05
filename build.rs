//! Embed every file under assets/ into the binary (see src/assets.rs).

use std::fmt::Write;
use std::path::Path;

fn main() {
    embed("assets", "assets.rs");
    embed("shared-mock/library", "guides.rs");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") { windows_resources(); }
}

fn windows_resources() {
    let root = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let icon = root.join("assets/branding/app-icon.ico").to_string_lossy().replace('\\', "/");
    let version = std::env::var("CARGO_PKG_VERSION").unwrap();
    let numbers = format!("{},0", version.replace('.', ","));
    let rc = format!(r#"1 ICON "{icon}"
1 VERSIONINFO
FILEVERSION {numbers}
PRODUCTVERSION {numbers}
FILEOS 0x40004
FILETYPE 0x1
BEGIN
  BLOCK "StringFileInfo"
  BEGIN
    BLOCK "040904B0"
    BEGIN
      VALUE "FileDescription", "Odyssey\0"
      VALUE "FileVersion", "{version}\0"
      VALUE "ProductName", "Odyssey\0"
      VALUE "ProductVersion", "{version}\0"
      VALUE "OriginalFilename", "quiet_md.exe\0"
    END
  END
  BLOCK "VarFileInfo"
  BEGIN
    VALUE "Translation", 0x409, 1200
  END
END
"#);
    let source = out.join("quiet-md.rc");
    let resource = out.join("quiet-md.res");
    std::fs::write(&source, rc).unwrap();
    let status = std::process::Command::new("rc.exe").arg("/nologo").arg("/fo").arg(&resource).arg(&source).status().expect("Windows SDK rc.exe is required; build from a Visual Studio developer shell");
    assert!(status.success(), "Windows resource compilation failed");
    println!("cargo:rustc-link-arg-bin=quiet_md={}", resource.display());
}

fn embed(source: &str, output: &str) {
    // Read at run time: a compile-time path goes stale when the checkout moves.
    let root = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join(source);
    println!("cargo:rerun-if-changed={}", root.display());
    let mut files = vec![];
    collect(&root, &root, &mut files);
    files.sort();
    let mut out = String::from("pub static FILES: &[(&str, &[u8])] = &[\n");
    for (rel, abs) in files {
        writeln!(out, "    ({rel:?}, include_bytes!({abs:?})),").unwrap();
    }
    out.push_str("];\n");
    let dest = Path::new(&std::env::var("OUT_DIR").unwrap()).join(output);
    std::fs::write(dest, out).unwrap();
}

fn collect(root: &Path, dir: &Path, out: &mut Vec<(String, String)>) {
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        let p = e.path();
        if dir.ends_with("branding") && p.file_name().is_some_and(|n| n != "app-icon.png" && n != "app-icon.ico") { continue; }
        if p.is_dir() {
            collect(root, &p, out);
        } else {
            let rel = p.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/");
            out.push((rel, p.to_string_lossy().into_owned()));
        }
    }
}
