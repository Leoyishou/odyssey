//! Static menu data: slash items with their search keywords, turn-into
//! targets, code languages and callout emoji (behavior-spec §3, §8, §9).

use crate::model::BlockKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashKind {
    Text,
    Heading(u8),
    Ordered,
    Bullet,
    Code,
    Quote,
    Divider,
    Link,
    Todo,
    Image,
    Table,
    Grid,
    Callout,
    Formula,
    SubPageList,
    Countdown,
    Button,
    File,
    Reminder,
}

pub struct SlashItem {
    pub kind: SlashKind,
    pub label: &'static str,
    pub icon: &'static str,
    /// Slash-menu icons are tinted per block family.
    pub color: u32,
    pub group: usize,
    pub keywords: &'static [&'static str],
}

pub struct SlashGroup {
    pub title: &'static str,
    pub keywords: &'static [&'static str],
}

/// Icons per row in the basic group's icon grid.
pub const GRID_COLS: usize = 6;
pub const GROUPS: [SlashGroup; 4] = [
    SlashGroup { title: "基础", keywords: &["基础", "basic", "jichu", "jc"] },
    SlashGroup { title: "常用", keywords: &["常用", "common", "changyong", "cy"] },
    SlashGroup { title: "团队协作", keywords: &["团队协作", "collaboration", "tuanduixiezuo", "tdxz"] },
    SlashGroup { title: "小应用", keywords: &["小应用", "miniapp", "xiaoyingyong", "xyy"] },
];

pub const BLUE: u32 = 0x336df4;
pub const INDIGO: u32 = 0x5b65f5;
pub const GREEN: u32 = 0x35bd4b;
pub const ORANGE: u32 = 0xff811a;
pub const YELLOW: u32 = 0xffc60a;
pub const TEAL: u32 = 0x2dbeab;
pub const NEUTRAL: u32 = 0x2b2f36;
pub const WATHET: u32 = 0x1295ca;

pub const SLASH_ITEMS: &[SlashItem] = &[
    SlashItem {
        kind: SlashKind::Text,
        label: "文本",
        icon: "type", color: BLUE,
        group: 0,
        keywords: &["正文", "文字", "text", "zhengwen", "zw", "wenben", "wb", "wenzi", "wz"],
    },
    SlashItem {
        kind: SlashKind::Heading(1),
        label: "一级标题",
        icon: "heading-1", color: BLUE,
        group: 0,
        keywords: &["yijibiaoti", "yjbt", "大标题", "标题1", "heading1", "header1", "h1"],
    },
    SlashItem {
        kind: SlashKind::Heading(2),
        label: "二级标题",
        icon: "heading-2", color: BLUE,
        group: 0,
        keywords: &["erjibiaoti", "ejbt", "中标题", "标题2", "heading2", "header2", "h2"],
    },
    SlashItem {
        kind: SlashKind::Heading(3),
        label: "三级标题",
        icon: "heading-3", color: BLUE,
        group: 0,
        keywords: &["sanjibiaoti", "sjbt", "小标题", "标题3", "heading3", "header3", "h3"],
    },
    SlashItem {
        kind: SlashKind::Heading(4),
        label: "四级标题",
        icon: "heading-4", color: BLUE,
        group: 0,
        keywords: &["sijibiaoti", "sjbt", "标题4", "heading4", "header4", "h4"],
    },
    SlashItem {
        kind: SlashKind::Heading(5),
        label: "五级标题",
        icon: "heading-5", color: BLUE,
        group: 0,
        keywords: &["wujibiaoti", "wjbt", "标题5", "heading5", "header5", "h5"],
    },
    SlashItem {
        kind: SlashKind::Ordered,
        label: "有序列表",
        icon: "list-ordered", color: INDIGO,
        group: 0,
        keywords: &["youxuliebiao", "yxlb", "numbered", "ordered", "list"],
    },
    SlashItem {
        kind: SlashKind::Bullet,
        label: "无序列表",
        icon: "list", color: INDIGO,
        group: 0,
        keywords: &["wuxuliebiao", "wxlb", "bullet", "unordered", "list"],
    },
    SlashItem { kind: SlashKind::Todo, label: "任务", icon: "todo", color: INDIGO, group: 0, keywords: &["todo", "renwu", "rw"] },
    SlashItem { kind: SlashKind::Code, label: "代码块", icon: "braces", color: GREEN, group: 0, keywords: &["code", "daimakuai", "dmk"] },
    SlashItem {
        kind: SlashKind::Quote,
        label: "引用",
        icon: "quote", color: BLUE,
        group: 0,
        keywords: &["quote", "yinyong", "yy", "cite"],
    },
    SlashItem { kind: SlashKind::Callout, label: "高亮块", icon: "callout", color: ORANGE, group: 0, keywords: &["callout", "gaoliangkuai", "glk", "highlight"] },
    SlashItem {
        kind: SlashKind::Divider,
        label: "分隔线",
        icon: "divider", color: ORANGE,
        group: 0,
        keywords: &["分割线", "divider", "fengexian", "fgx", "line"],
    },
    SlashItem {
        kind: SlashKind::Link,
        label: "链接",
        icon: "link", color: BLUE,
        group: 0,
        keywords: &["lianjie", "lj", "link", "url", "hyperlink"],
    },
    SlashItem { kind: SlashKind::Todo, label: "任务", icon: "todo", color: INDIGO, group: 1, keywords: &["todo", "renwu", "rw"] },
    SlashItem { kind: SlashKind::Image, label: "图片", icon: "image", color: YELLOW, group: 1, keywords: &["image", "tupian", "tp"] },
    SlashItem {
        kind: SlashKind::File,
        label: "视频或文件",
        icon: "paperclip", color: WATHET,
        group: 1,
        keywords: &["视频", "文件", "附件", "shipinhuowenjian", "sphwj", "shipin", "wenjian", "fujian", "file", "video", "attachment"],
    },
    SlashItem { kind: SlashKind::Table, label: "表格", icon: "table", color: TEAL, group: 1, keywords: &["table", "biaoge", "bg"] },
    SlashItem {
        kind: SlashKind::Grid,
        label: "分栏",
        icon: "columns-2", color: INDIGO,
        group: 1,
        keywords: &["column", "fenlan", "fl", "grid", "buju"],
    },
    SlashItem {
        kind: SlashKind::Callout,
        label: "高亮块",
        icon: "callout", color: ORANGE,
        group: 1,
        keywords: &["callout", "gaoliangkuai", "glk", "highlight"],
    },
    SlashItem {
        kind: SlashKind::Formula,
        label: "公式",
        icon: "sigma", color: NEUTRAL,
        group: 1,
        keywords: &["gongshi", "gs", "latex", "formula", "equation", "math"],
    },
    SlashItem { kind: SlashKind::Button, label: "按钮", icon: "button", color: BLUE, group: 1, keywords: &["按钮", "anniu", "an", "button"] },
    SlashItem {
        kind: SlashKind::SubPageList,
        label: "子页面列表",
        icon: "list-tree", color: BLUE,
        group: 1,
        keywords: &["ziyemianliebiao", "zymlb", "子页面", "子文档", "subpage", "catalog", "mulu"],
    },
    SlashItem {
        kind: SlashKind::Reminder,
        label: "日期提醒",
        icon: "alarm-clock", color: ORANGE,
        group: 2,
        keywords: &["日期提醒", "riqitixing", "rqtx", "reminder", "tixing", "date"],
    },
    SlashItem { kind: SlashKind::Countdown, label: "倒计时", icon: "timer", color: ORANGE, group: 3, keywords: &["倒计时", "djs", "timer", "daojishi", "countdown"] },
];

/// Whether a basic-group item shows in the icon grid. 文本 shows only where
/// the new block isn't already a text line (在下方添加, "/" after text), and
/// 任务 takes its place on an empty line; 任务 and 高亮块 also sit in 常用, so
/// a filtered list keeps just those copies.
pub fn basic_visible(kind: SlashKind, grid: bool, with_text: bool) -> bool {
    match kind {
        SlashKind::Text => !grid || with_text,
        SlashKind::Todo => grid && !with_text,
        SlashKind::Callout => grid,
        _ => true,
    }
}

/// Shortcut and Markdown hint for a basic item's grid tooltip.
pub fn basic_tip(kind: SlashKind) -> (Option<&'static str>, Option<&'static str>) {
    let turn = |label: &str| TURN_ITEMS.iter().find(|t| t.label == label).map(|t| (t.keys.map(|k| crate::shortcuts::hint(t.action, k)), t.md));
    match kind {
        SlashKind::Text => turn("正文"),
        SlashKind::Heading(l) => turn(["一级标题", "二级标题", "三级标题", "四级标题", "五级标题"][(l.clamp(1, 5) - 1) as usize]),
        SlashKind::Ordered => turn("有序列表"),
        SlashKind::Bullet => turn("无序列表"),
        SlashKind::Code => turn("代码块"),
        SlashKind::Quote => turn("引用"),
        SlashKind::Divider => Some((Some(crate::shortcuts::hint("editor::InsertDivider", "⌘ + ⌥ + S")), Some("---"))),
        SlashKind::Link => Some((Some(crate::shortcuts::hint("editor::Link", "⌘ + K")), None)),
        _ => None,
    }
    .unwrap_or((None, None))
}
/// Rank: item prefix > group prefix > item contains > group contains; ties
/// keep table order. Returns indices into `SLASH_ITEMS`.
pub fn filter_slash(query: &str) -> Vec<usize> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return (0..SLASH_ITEMS.len()).collect();
    }
    let prefix = |words: &[&str]| words.iter().any(|w| w.to_lowercase().starts_with(&q));
    let contains = |words: &[&str]| words.iter().any(|w| w.to_lowercase().contains(&q));
    let mut ranked: Vec<(u8, usize)> = vec![];
    for (i, it) in SLASH_ITEMS.iter().enumerate() {
        let item_words: Vec<&str> = [it.label, crate::settings::localize(it.label, true)].into_iter().chain(it.keywords.iter().copied()).collect();
        let group_words = GROUPS[it.group].keywords;
        let rank = if prefix(&item_words) {
            0
        } else if prefix(group_words) {
            1
        } else if contains(&item_words) {
            2
        } else if contains(group_words) {
            3
        } else {
            continue;
        };
        ranked.push((rank, i));
    }
    ranked.sort_by_key(|(r, _)| *r);
    ranked.into_iter().map(|(_, i)| i).collect()
}

/// Turn-into grid of the block menu, in order, with the shortcut shown in tooltips.
pub struct TurnItem {
    pub label: &'static str,
    /// The action its shortcut runs (for the hint once the user rebinds it).
    pub action: &'static str,
    pub icon: &'static str,
    pub keys: Option<&'static str>,
    /// Tooltip: "名称 (keys)" and a "Markdown: …" line.
    pub tip: &'static str,
    pub md: Option<&'static str>,
    pub kind: fn() -> BlockKind,
}

pub const TURN_ITEMS: &[TurnItem] = &[
    TurnItem { label: "正文", action: "editor::TurnText", icon: "type", keys: Some("⌘ + ⌥ + 0"), tip: "正文", md: None, kind: || BlockKind::Text },
    TurnItem { label: "一级标题", action: "editor::TurnH1", icon: "heading-1", keys: Some("⌘ + ⌥ + 1"), tip: "一级标题", md: Some("# 空格"), kind: || heading(1) },
    TurnItem { label: "二级标题", action: "editor::TurnH2", icon: "heading-2", keys: Some("⌘ + ⌥ + 2"), tip: "二级标题", md: Some("## 空格"), kind: || heading(2) },
    TurnItem { label: "三级标题", action: "editor::TurnH3", icon: "heading-3", keys: Some("⌘ + ⌥ + 3"), tip: "三级标题", md: Some("### 空格"), kind: || heading(3) },
    TurnItem { label: "四级标题", action: "editor::TurnH4", icon: "heading-4", keys: Some("⌘ + ⌥ + 4"), tip: "四级标题", md: Some("#### 空格"), kind: || heading(4) },
    TurnItem { label: "五级标题", action: "editor::TurnH5", icon: "heading-5", keys: Some("⌘ + ⌥ + 5"), tip: "五级标题", md: Some("##### 空格"), kind: || heading(5) },
    TurnItem { label: "有序列表", action: "editor::TurnOrdered", icon: "list-ordered", keys: Some("⌘ + Shift + 7"), tip: "有序列表", md: Some("1. 空格"), kind: || BlockKind::Ordered { seq: None } },
    TurnItem { label: "无序列表", action: "editor::TurnBullet", icon: "list", keys: Some("⌘ + Shift + 8"), tip: "无序列表", md: Some("- 空格 或 * 空格"), kind: || BlockKind::Bullet },
    TurnItem { label: "任务", action: "editor::TurnTodo", icon: "todo", keys: Some("⌘ + ⌥ + T"), tip: "任务列表", md: Some("[] 空格"), kind: || BlockKind::Todo { done: false } },
    TurnItem { label: "代码块", action: "editor::TurnCode", icon: "braces", keys: Some("⌘ + ⌥ + C"), tip: "代码块", md: Some("``` 空格 或 ``` 代码语言 空格"), kind: || code_block("Plain Text") },
    TurnItem { label: "引用", action: "editor::TurnQuote", icon: "quote", keys: Some("⌘ + Shift + >"), tip: "引用", md: Some("> 空格"), kind: || BlockKind::Quote },
    TurnItem { label: "高亮块", action: "", icon: "callout", keys: None, tip: "高亮块", md: None, kind: || callout(None) },
];

/// Which variant of the handle menu a block gets.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MenuShape {
    /// Text, headings, lists, todos, quotes: full grid, 缩进和对齐, 颜色.
    Text,
    /// Code blocks: grid without 高亮块, indent only.
    Code,
    /// Callouts: grid acts on the content (no 代码块 / 引用), indent only.
    Callout,
    /// Dividers, images, grids…: indent only.
    Other,
}

impl MenuShape {
    pub fn of(kind: &BlockKind) -> Self {
        match kind {
            BlockKind::Text
            | BlockKind::Heading { .. }
            | BlockKind::Ordered { .. }
            | BlockKind::Bullet
            | BlockKind::Todo { .. }
            | BlockKind::Quote => MenuShape::Text,
            BlockKind::Code { .. } => MenuShape::Code,
            BlockKind::Callout { .. } => MenuShape::Callout,
            _ => MenuShape::Other,
        }
    }

    pub fn turn_items(self) -> Vec<&'static TurnItem> {
        let keep = |t: &&TurnItem| match self {
            MenuShape::Text => true,
            MenuShape::Code => t.label != "高亮块",
            MenuShape::Callout => !matches!(t.label, "代码块" | "引用"),
            MenuShape::Other => false,
        };
        TURN_ITEMS.iter().filter(keep).collect()
    }
}

pub fn heading(level: u8) -> BlockKind {
    BlockKind::Heading { level, seq_auto: false }
}

pub fn code_block(lang: &str) -> BlockKind {
    BlockKind::Code { lang: lang.to_string(), caption: None, wrap: false }
}

pub fn callout(emoji: Option<&str>) -> BlockKind {
    BlockKind::Callout {
        emoji: emoji.map(str::to_string).unwrap_or_else(random_emoji),
        bg: Some(crate::model::ColorName("light-orange".into())),
        border: Some(crate::model::ColorName("orange".into())),
        text_color: None,
    }
}

/// True when `kind` is the same turn-into target as `current` (for the active state).
pub fn same_kind(a: &BlockKind, b: &BlockKind) -> bool {
    match (a, b) {
        (BlockKind::Heading { level: x, .. }, BlockKind::Heading { level: y, .. }) => x == y,
        _ => std::mem::discriminant(a) == std::mem::discriminant(b),
    }
}

pub const CALLOUT_EMOJI: [&str; 30] = [
    "💡", "👍", "🌟", "🎉", "📌", "📚", "✏️", "✍️", "❤️", "📍", "🥖", "⚽", "🎹", "🎼", "⛱️", "🎨", "🥇", "🏆", "🎁", "🌅",
    "🏕️", "🏖️", "🏝️", "🚅", "🥛", "🌰", "🍰", "🍞", "🦄", "🐵",
];

pub fn random_index(n: usize) -> usize {
    let r = uuid::Uuid::new_v4().as_u128();
    (r % n as u128) as usize
}

pub fn random_emoji() -> String {
    CALLOUT_EMOJI[random_index(CALLOUT_EMOJI.len())].to_string()
}

use crate::lang::LANGS;

pub fn filter_languages(q: &str) -> Vec<&'static str> {
    let q = q.trim().to_lowercase();
    if q.is_empty() {
        return LANGS.iter().map(|l| l.0).collect();
    }
    let names = || LANGS.iter().map(|l| l.0);
    let mut prefix: Vec<&str> = names().filter(|l| l.to_lowercase().starts_with(&q)).collect();
    let rest = names().filter(|l| !l.to_lowercase().starts_with(&q) && l.to_lowercase().contains(&q));
    prefix.extend(rest);
    prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slash_ranking() {
        let r = filter_slash("bg");
        assert_eq!(SLASH_ITEMS[r[0]].label, "表格");
        let r = filter_slash("cy");
        assert!(r.iter().all(|i| SLASH_ITEMS[*i].group == 1));
        assert_eq!(r.len(), 9);
        assert!(filter_slash("zzz").is_empty());
        assert_eq!(SLASH_ITEMS[filter_slash("djs")[0]].label, "倒计时");
        let r = filter_slash("list");
        assert_eq!(SLASH_ITEMS[r[0]].label, "有序列表");
    }
}
