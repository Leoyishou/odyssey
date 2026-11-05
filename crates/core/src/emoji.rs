//! The emoji table (emoji-mart v5 ids, zh names and keywords): the picker in
//! the UI and the `data-emoji-id` attribute on paste both read it.

/// One line per emoji: category, id, char, zh name, en name, keywords, emoticons.
pub const TSV: &str = include_str!("emoji.tsv");

/// The character for an emoji-mart id (`chestnut` → 🌰).
pub fn by_id(id: &str) -> Option<&'static str> {
    TSV.lines().find_map(|l| {
        let mut f = l.split('\t').skip(1);
        (f.next()? == id).then(|| f.next()).flatten()
    })
}
