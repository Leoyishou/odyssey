//! Emoji data and the state shared by the emoji pickers. Search, 最近 and
//! random picks all work off the zh names / keywords in `fdl_core::emoji::TSV`.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::ops::Range;
use std::sync::{LazyLock, Mutex};

pub struct Emoji {
    pub cat: usize,
    pub id: &'static str,
    pub char: &'static str,
    /// Tooltip name (zh).
    pub name: &'static str,
    /// Search string: `,id,name words,keywords,emoticons`.
    search: String,
}

/// Picker sections after 最近, in this order (no flags).
pub const CATEGORIES: [&str; 7] = ["表情符号", "动物", "食物", "活动", "地点", "物件", "符号"];

/// Emoji per row in the picker.
pub const PER_LINE: usize = 11;

/// The starting 最近 list, scored `PER_LINE - i` until something is picked.
const DEFAULT_FREQUENT: [&str; 15] = [
    "+1",
    "grinning",
    "kissing_heart",
    "heart_eyes",
    "laughing",
    "stuck_out_tongue_winking_eye",
    "sweat_smile",
    "joy",
    "scream",
    "disappointed",
    "unamused",
    "weary",
    "sob",
    "sunglasses",
    "heart",
];

/// `ccm_docx_random_pick_emoji_blocklist`: never drawn by 随机.
const RANDOM_BLOCKLIST: [&str; 43] = [
    "dizzy_face",
    "rage",
    "angry",
    "face_with_symbols_on_mouth",
    "mask",
    "nauseated_face",
    "face_vomiting",
    "smiling_imp",
    "imp",
    "japanese_ogre",
    "japanese_goblin",
    "skull_and_crossbones",
    "middle_finger",
    "anger",
    "bomb",
    "hocho",
    "crossed_swords",
    "gun",
    "coffin",
    "funeral_urn",
    "right_anger_bubble",
    "face_with_head_bandage",
    "face_with_thermometer",
    "persevere",
    "disappointed_relieved",
    "tired_face",
    "unamused",
    "sweat",
    "pensive",
    "confused",
    "white_frowning_face",
    "slightly_frowning_face",
    "disappointed",
    "worried",
    "triumph",
    "cry",
    "sob",
    "weary",
    "crying_cat_face",
    "-1",
    "broken_heart",
    "black_heart",
    "sos",
];

const MAX_RESULTS: usize = 90;

struct Data {
    all: Vec<Emoji>,
    ranges: Vec<Range<usize>>,
    by_id: HashMap<&'static str, usize>,
}

static DATA: LazyLock<Data> = LazyLock::new(|| {
    let mut all = vec![];
    for line in fdl_core::emoji::TSV.lines() {
        let f: Vec<&'static str> = line.split('\t').collect();
        let [cat, id, ch, name, _en, keywords, emoticons] = f[..] else { continue };
        let mut parts = vec![id.to_lowercase()];
        parts.extend(name.split(['-', '|', '_', ' ']).map(str::to_lowercase));
        parts.extend(keywords.split(',').map(str::to_lowercase));
        parts.extend(emoticons.split(',').map(str::to_lowercase));
        parts.retain(|p| !p.trim().is_empty());
        all.push(Emoji { cat: cat.parse().unwrap_or(0), id, char: ch, name, search: format!(",{}", parts.join(",")) });
    }
    let mut ranges = vec![0..0; CATEGORIES.len()];
    for (i, e) in all.iter().enumerate() {
        let r = &mut ranges[e.cat];
        if r.start == r.end {
            *r = i..i;
        }
        r.end = i + 1;
    }
    let by_id = all.iter().enumerate().map(|(i, e)| (e.id, i)).collect();
    Data { all, ranges, by_id }
});

pub fn category(i: usize) -> &'static [Emoji] {
    let d = &*DATA;
    &d.all[d.ranges[i].clone()]
}

pub fn by_id(id: &str) -> Option<&'static Emoji> {
    DATA.by_id.get(id).map(|i| &DATA.all[*i])
}

/// Search: every term must start a word of the
/// search string; score = summed match positions (an exact id scores 0),
/// ties by id, at most 90.
pub fn search(q: &str) -> Vec<&'static Emoji> {
    let q = q.to_lowercase();
    let mut terms: Vec<&str> = vec![];
    for t in q.split(|c: char| c.is_whitespace() || c == '|' || c == ',') {
        if !t.is_empty() && !terms.contains(&t) {
            terms.push(t);
        }
    }
    if terms.is_empty() {
        return vec![];
    }
    let mut pool: Vec<(usize, usize)> = (0..DATA.all.len()).map(|i| (i, 0)).collect();
    for t in terms {
        let needle = format!(",{t}");
        pool.retain_mut(|(i, score)| {
            let e = &DATA.all[*i];
            match e.search.find(&needle) {
                Some(at) => {
                    // JS `indexOf` counts UTF-16 units.
                    *score += if e.id == t { 0 } else { e.search[..at].encode_utf16().count() + 1 };
                    true
                }
                None => false,
            }
        });
    }
    pool.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| locale_key(DATA.all[a.0].id).cmp(&locale_key(DATA.all[b.0].id))));
    pool.into_iter().take(MAX_RESULTS).map(|(i, _)| &DATA.all[i]).collect()
}

/// `localeCompare` order for ids: punctuation, then digits, then letters.
fn locale_key(id: &str) -> Vec<(u8, char)> {
    id.chars()
        .map(|c| match c {
            '0'..='9' => (1, c),
            c if c.is_alphabetic() => (2, c.to_ascii_lowercase()),
            _ => (0, c),
        })
        .collect()
}

/// 随机: any emoji outside the blocklist.
pub fn random_pick() -> &'static Emoji {
    let d = &*DATA;
    loop {
        let e = &d.all[super::menus::random_index(d.all.len())];
        if !RANDOM_BLOCKLIST.contains(&e.id) {
            return e;
        }
    }
}

/// Last colours / emoji of a callout, applied to new callouts.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CalloutSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
}

/// `emoji.json` next to the library.
#[derive(Default, Serialize, Deserialize)]
struct State {
    /// Picks per emoji id; None until the first pick.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    frequently: Option<BTreeMap<String, u32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    last: Option<String>,
    #[serde(default)]
    callout: CalloutSetting,
}

fn state_path() -> std::path::PathBuf {
    crate::store::Store::root_dir().join("emoji.json")
}

static STATE: LazyLock<Mutex<State>> = LazyLock::new(|| {
    let s = if cfg!(test) { None } else { std::fs::read(state_path()).ok().and_then(|b| serde_json::from_slice(&b).ok()) };
    Mutex::new(s.unwrap_or_default())
});

fn save(s: &State) {
    if cfg!(test) {
        return;
    }
    if let Ok(b) = serde_json::to_vec_pretty(s) {
        let _ = std::fs::write(state_path(), b);
    }
}

pub fn add_frequent(id: &str) {
    let mut s = STATE.lock().unwrap();
    let f = s.frequently.get_or_insert_with(default_frequent);
    *f.entry(id.to_string()).or_insert(0) += 1;
    s.last = Some(id.to_string());
    save(&s);
}

fn default_frequent() -> BTreeMap<String, u32> {
    DEFAULT_FREQUENT.iter().take(PER_LINE).enumerate().map(|(i, id)| (id.to_string(), (PER_LINE - i) as u32)).collect()
}

/// One row: most picked first (ties by id); the last pick always stays in
/// the row. Counts that fall out of the row are forgotten.
pub fn frequent() -> Vec<&'static Emoji> {
    let mut s = STATE.lock().unwrap();
    let last = s.last.clone();
    let Some(f) = &mut s.frequently else {
        return DEFAULT_FREQUENT.iter().take(PER_LINE).filter_map(|id| by_id(id)).collect();
    };
    let mut ids: Vec<String> = f.keys().cloned().collect();
    ids.sort_by(|a, b| f[b].cmp(&f[a]).then_with(|| a.cmp(b)));
    if ids.len() > PER_LINE {
        for id in ids.split_off(PER_LINE) {
            if Some(&id) != last.as_ref() {
                f.remove(&id);
            }
        }
        if let Some(last) = last
            && !ids.contains(&last)
            && let Some(slot) = ids.last_mut()
        {
            f.remove(slot);
            *slot = last;
        }
        save(&s);
    }
    ids.iter().filter_map(|id| by_id(id)).collect()
}

pub fn callout_setting() -> CalloutSetting {
    STATE.lock().unwrap().callout.clone()
}

/// Remember a callout's colours, and its emoji when one was just picked.
pub fn save_callout(bg: Option<&str>, border: Option<&str>, emoji: Option<&str>) {
    let mut s = STATE.lock().unwrap();
    s.callout.bg = bg.map(str::to_string);
    s.callout.border = border.map(str::to_string);
    if let Some(e) = emoji {
        s.callout.emoji = Some(e.to_string());
    }
    save(&s);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chars(v: Vec<&Emoji>) -> String {
        v.iter().map(|e| e.char).collect()
    }

    #[test]
    fn data() {
        assert_eq!(DATA.all.len(), 1578);
        assert_eq!(category(0)[0].id, "grinning");
        assert_eq!(category(6).len(), 219);
        assert_eq!(by_id("heart").map(|e| e.char.trim_end_matches('\u{fe0f}')), Some("❤"));
    }

    #[test]
    fn search_like_feishu() {
        assert_eq!(chars(search("猫")), "🐱🦉🐈🐯😹😽😾🐾😸😼🙀😺🐈‍⬛😿😻");
        assert_eq!(search("cat")[0].char, "🐱");
        assert!(search("zzzz").is_empty());
        assert!(search("  ").is_empty());
    }

    #[test]
    fn default_frequent_row() {
        assert_eq!(chars(frequent()), "👍😀😘😍😆😜😅😂😱😞😒");
    }

    #[test]
    fn random_avoids_blocklist() {
        for _ in 0..200 {
            assert!(!RANDOM_BLOCKLIST.contains(&random_pick().id));
        }
    }
}
