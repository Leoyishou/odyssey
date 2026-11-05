//! Bundled first-launch library; existing libraries are never reseeded.
use std::path::Path;
use std::fs;

include!(concat!(env!("OUT_DIR"), "/guides.rs"));

/// Any entry but the lock file means an existing library, even if all
/// documents were deleted; a pending directory is a first launch to finish.
pub(crate) fn seeded(root: &Path) -> std::io::Result<bool> {
    Ok(!root.join(".guides-pending").exists() && fs::read_dir(root)?.any(|e| e.map_or(true, |e| e.file_name() != ".lock")))
}

/// Mac-only passages of the guides and what Windows reads instead; key
/// combinations go through `shortcuts::windows_keys` afterwards.
const WINDOWS_TEXT: &[(&str, &str)] = &[
    ("every document is a file on your Mac", "every document is a file on your computer"),
    ("Back up the whole folder with Time Machine or copy it to another Mac.", "Back up the whole folder with your backup tool or copy it to another computer."),
    ("On macOS, <code>⌘ ⌃ ⇧ 4</code> captures a selection to the clipboard.", "<code>Win+Shift+S</code> captures a selection to the clipboard."),
    ("Drag an image from Finder into the document.", "Drag an image from File Explorer into the document."),
    ("a project banner from your Mac.", "a project banner from your computer."),
    ("<h2>Move to another Mac</h2><ol><li>Quit Quiet.md on the old Mac with", "<h2>Move to another computer</h2><ol><li>Quit Quiet.md on the old computer with"),
    ("Copy the complete Quiet.md folder into Documents on the new Mac.", "Copy the complete Quiet.md folder into Documents on the new computer."),
    ("Press a new combination containing Command, Control or Option.", "Press a new combination containing Ctrl or Alt."),
    // The system-wide quick-note shortcut exists on macOS only.
    ("<p>A global shortcut can open a small note window while you are using another app. Write the thought down, then close the window; it is saved with your documents.</p><ul><li>Enable the global quick-note shortcut in Settings › Shortcuts.</li><li>The default <code>⌘ Space</code> commonly conflicts with Spotlight. Choose another combination, such as <code>⌥ ⇧ Space</code>, if needed. Registration failures are shown in settings.</li>",
     "<p>A note window is a small window of its own for writing a thought down. Close it when you are done; the note is saved with your documents.</p><ul>"),
];

/// A guide document as the Windows build seeds it: Ctrl / Alt keys, no Mac-only steps.
pub fn for_windows(xml: &str) -> String {
    let text = WINDOWS_TEXT.iter().fold(xml.to_string(), |s, (mac, win)| s.replace(mac, win));
    crate::shortcuts::windows_keys(&text)
}

pub fn initialize(root: &Path) -> anyhow::Result<()> {
    fs::create_dir_all(root)?;
    if seeded(root)? { return Ok(()); }
    // Copies of the app may share a library: two started together on a new
    // one seed it in turn. Released when `lock` drops.
    let lock = fs::OpenOptions::new().create(true).truncate(false).write(true).open(root.join(".lock"))?;
    lock.lock()?;
    if seeded(root)? { return Ok(()); }
    let pending = root.join(".guides-pending");
    fs::create_dir_all(&pending)?;
    // Stage complete files before publishing. The pending directory lets a
    // failed first launch resume without mistaking it for an existing library.
    for (name, bytes) in FILES {
        let path = pending.join(name);
        fs::create_dir_all(path.parent().unwrap())?;
        let windows = cfg!(target_os = "windows") && name.starts_with("docs/") && name.ends_with(".xml");
        match std::str::from_utf8(bytes) {
            Ok(xml) if windows => fs::write(path, for_windows(xml).replace("Quiet.md", "Odyssey"))?,
            Ok(xml) if name.starts_with("docs/") && name.ends_with(".xml") => fs::write(path, xml.replace("Quiet.md", "Odyssey"))?,
            _ => fs::write(path, bytes)?,
        }
    }
    for name in ["assets", "docs", "index.json", "format.json"] {
        let dest = root.join(name);
        if !dest.exists() { fs::rename(pending.join(name), dest)?; }
    }
    fs::remove_dir_all(pending)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    /// Guides go only into an empty folder: an existing library, even one whose
    /// documents were all deleted, is never refilled or overwritten.
    #[test]
    fn existing_library_is_never_reseeded() {
        let root = std::env::temp_dir().join(format!("qmd-guides-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        super::initialize(&root).unwrap();
        let docs = root.join("docs");
        let first = std::fs::read_dir(&docs).unwrap().next().unwrap().unwrap().path();
        std::fs::write(&first, "<title>mine</title>").unwrap();
        super::initialize(&root).unwrap();
        assert_eq!(std::fs::read_to_string(&first).unwrap(), "<title>mine</title>");
        for e in std::fs::read_dir(&docs).unwrap() { std::fs::remove_file(e.unwrap().path()).unwrap(); }
        super::initialize(&root).unwrap();
        assert_eq!(std::fs::read_dir(&docs).unwrap().count(), 0);
        std::fs::remove_dir_all(&root).unwrap();
    }

    /// Every Mac-only passage still exists, and the Windows guides keep no Mac key.
    #[test]
    fn windows_guides_use_windows_keys() {
        let docs: Vec<&str> = super::FILES.iter().filter(|(n, _)| n.starts_with("docs/") && n.ends_with(".xml")).map(|(_, b)| std::str::from_utf8(b).unwrap()).collect();
        for (mac, _) in super::WINDOWS_TEXT { assert!(docs.iter().any(|d| d.contains(mac)), "stale WINDOWS_TEXT entry: {mac}"); }
        let all: String = docs.iter().map(|d| super::for_windows(d)).collect();
        assert!(all.contains("<code>Ctrl+B</code>") && all.contains("<code>Ctrl+Alt+1</code>") && all.contains("<code>Ctrl+Shift+C</code>"));
        let mac = |l: &&str| l.contains(['⌘', '⌥', '⇧', '⌃']);
        assert!(!all.lines().any(|l| mac(&l)), "{}", all.lines().find(mac).unwrap_or_default());
    }

    /// Copies launched together on a new library: one seeds it, none fails.
    #[test]
    fn simultaneous_first_launches() {
        let root = std::env::temp_dir().join(format!("fdl-guides-{}", uuid::Uuid::new_v4()));
        let runs: Vec<_> = (0..4).map(|_| { let root = root.clone(); std::thread::spawn(move || super::initialize(&root)) }).collect();
        for r in runs { r.join().unwrap().unwrap(); }
        let docs = super::FILES.iter().filter(|(n, _)| n.starts_with("docs/")).count();
        assert!(docs > 0);
        assert_eq!(std::fs::read_dir(root.join("docs")).unwrap().count(), docs);
        assert!(!root.join(".guides-pending").exists());
        // A library with content is never reseeded.
        std::fs::remove_dir_all(root.join("docs")).unwrap();
        super::initialize(&root).unwrap();
        assert!(!root.join("docs").exists());
        let _ = std::fs::remove_dir_all(&root);
    }
}
