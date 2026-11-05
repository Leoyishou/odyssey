//! Document themes are CSS. Each theme is one stylesheet: `:root` holds the
//! name, the base style (`--theme-base: craft | feishu`, which decides how
//! blocks are drawn) and the named colours; rules on `.page`, `h1`, `code`,
//! `blockquote` … set the rest (docs/THEMES.md). Built-in themes ship as
//! complete files (`assets/themes/*.css`); the user's own live in
//! `<library>/themes/*.css`, and one named after a built-in replaces it.

use crate::editor_theme::TOKENS;
use gpui::Hsla;
use std::collections::HashMap;
#[cfg(test)]
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::atomic::{AtomicPtr, AtomicU64, Ordering};
use std::sync::{LazyLock, RwLock};

// ---- what a theme can set

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Kind {
    Len,
    Weight,
    Color,
    Font,
    /// `text-decoration`: underline / none.
    Underline,
    /// `text-decoration`: line-through / none.
    Strike,
    /// `display`: block / none.
    Display,
    /// `font-style`: italic / normal.
    Italic,
    /// `border-style`: dotted / solid / dashed.
    LineStyle,
}

/// A property's value in a base style.
#[derive(Clone, Copy, Debug)]
pub enum Def {
    N(f32),
    /// The colour token of that name (setting the property overrides it).
    Tok(&'static str),
    /// A colour of its own.
    Rgb(u32),
    /// No colour of its own: the surrounding text colour.
    Inherit,
    Family(&'static str),
    B(bool),
    Line(LineStyle),
    /// That base's drawing doesn't use it.
    Unused,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LineStyle { Dotted, Solid, Dashed }

pub struct Prop {
    pub sel: &'static str,
    pub prop: &'static str,
    pub key: String,
    pub kind: Kind,
    pub craft: Def,
    pub feishu: Def,
}

/// The system families generic CSS names resolve to.
pub const SYSTEM: &str = ".SystemUIFont";
pub const SYSTEM_MONO: &str = ".AppleSystemUIFontMonospaced";

pub static PROPS: LazyLock<Vec<Prop>> = LazyLock::new(|| {
    use Def::*;
    use Kind::*;
    let p = |sel: &'static str, prop: &'static str, key: &str, kind: Kind, craft: Def, feishu: Def| Prop { sel, prop, key: key.to_string(), kind, craft, feishu };
    let mut v = vec![
        p(".page", "background-color", "page.bg", Color, Tok("bg_body"), Tok("bg_body")),
        p(".page", "color", "page.color", Color, Tok("text"), Tok("text")),
        p(".page", "font-family", "page.font", Font, Family(SYSTEM), Family(SYSTEM)),
        p(".page", "font-size", "page.size", Len, N(crate::theme::BODY_SIZE), N(16.)),
        p(".page", "line-height", "page.line", Len, N(22.), N(26.)),
        p(".page", "max-width", "page.width", Len, N(820.), N(820.)),
        p(".page", "caret-color", "page.caret", Color, Tok("caret"), Tok("caret")),
        p("::selection", "background-color", "selection", Color, Tok("selection"), Tok("selection")),
        p("p", "margin-bottom", "p.gap", Len, N(crate::theme::BLOCK_GAP), N(8.)),
        p(".title", "font-family", "title.font", Font, Family(SYSTEM), Family(SYSTEM)),
        p(".title", "font-size", "title.size", Len, N(crate::theme::TITLE.size), N(crate::theme::TITLE.size)),
        p(".title", "line-height", "title.line", Len, N(crate::theme::TITLE.line_height), N(crate::theme::TITLE.line_height)),
        p(".title", "font-weight", "title.weight", Weight, N(700.), N(500.)),
        p(".title", "color", "title.color", Color, Inherit, Inherit),
    ];
    const H: [&str; 9] = ["h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9"];
    for (i, h) in H.iter().enumerate() {
        let l = i as u8 + 1;
        let (c, f) = (crate::theme::heading_metrics(l), crate::feishu_theme::heading_metrics(l));
        v.extend([
            p(h, "font-family", &format!("{h}.font"), Font, Family(SYSTEM), Unused),
            p(h, "font-size", &format!("{h}.size"), Len, N(c.size), N(f.size)),
            p(h, "line-height", &format!("{h}.line"), Len, N(c.line_height), N(f.line_height)),
            p(h, "font-weight", &format!("{h}.weight"), Weight, N(crate::theme::heading_weight(l).0), N(500.)),
            p(h, "color", &format!("{h}.color"), Color, Inherit, Unused),
            p(h, "margin-top", &format!("{h}.mt"), Len, N(c.mt), N(f.mt)),
            p(h, "margin-bottom", &format!("{h}.mb"), Len, N(c.mb), N(f.mb)),
        ]);
    }
    v.extend([
        p("strong", "font-weight", "strong.weight", Weight, N(700.), Unused),
        p("strong", "color", "strong.color", Color, Inherit, Unused),
        p("a", "color", "a.color", Color, Tok("link"), Tok("link")),
        p("a", "text-decoration", "a.underline", Underline, B(false), Unused),
        p("a", "text-decoration-color", "a.underline-color", Color, Inherit, Unused),
        p("a:hover", "color", "a.hover", Color, Tok("link_hover"), Tok("link_hover")),
        p("code", "color", "code.color", Color, Inherit, Unused),
        p("code", "background-color", "code.bg", Color, Tok("code_chip"), Unused),
        p("code", "border-color", "code.border", Color, Tok("code_chip_border"), Unused),
        p("code", "border-radius", "code.radius", Len, N(3.), Unused),
        p("code", "padding-left", "code.pad", Len, N(4.), Unused),
        p("pre", "font-family", "pre.font", Font, Family(SYSTEM_MONO), Family(crate::feishu_theme::MONO_FONT)),
        p("pre", "font-size", "pre.size", Len, N(13.), Unused),
        p("pre", "line-height", "pre.line", Len, N(20.), Unused),
        p("pre", "color", "pre.color", Color, Tok("code_text"), Tok("code_text")),
        p("pre", "background-color", "pre.bg", Color, Tok("code_bg"), Tok("code_bg")),
        p("pre", "border-color", "pre.border", Color, Tok("code_border"), Tok("code_border")),
        p("pre", "border-radius", "pre.radius", Len, N(6.), Unused),
        p(".line-numbers", "display", "pre.numbers", Display, B(true), Unused),
        p(".line-numbers", "color", "pre.number-color", Color, Tok("code_line_number"), Tok("code_line_number")),
        p(".line-numbers", "background-color", "pre.gutter", Color, Tok("code_gutter"), Tok("code_gutter")),
        p("blockquote", "border-left-width", "quote.bar", Len, N(4.), Unused),
        p("blockquote", "border-left-color", "quote.bar-color", Color, Tok("quote_bar"), Tok("quote_bar")),
        p("blockquote", "padding-left", "quote.indent", Len, N(16.), Unused),
        p("blockquote", "color", "quote.color", Color, Inherit, Unused),
        p("blockquote", "font-style", "quote.italic", Italic, B(false), Unused),
        p("ul", "padding-left", "list.indent", Len, N(25.), Unused),
        p("ul li::marker", "color", "bullet.color", Color, Inherit, Unused),
        p("ol li::marker", "color", "number.color", Color, Inherit, Unused),
        p(".task", "width", "task.size", Len, N(17.), Unused),
        p(".task", "border-radius", "task.radius", Len, N(5.), Unused),
        p(".task", "border-color", "task.border", Color, Tok("todo_box"), Tok("todo_box")),
        p(".task:checked", "background-color", "task.checked", Color, Tok("primary"), Unused),
        p(".task:checked", "color", "task.check", Color, Rgb(0xffffff), Unused),
        p(".task-done", "color", "done.color", Color, Tok("todo_done"), Tok("todo_done")),
        p(".task-done", "text-decoration", "done.strike", Strike, B(true), Unused),
        p("hr", "border-top-style", "hr.style", LineStyle, Line(self::LineStyle::Dotted), Unused),
        p("hr", "border-top-width", "hr.width", Len, N(1.5), Unused),
        p("hr", "border-top-color", "hr.color", Color, Tok("separator_dots"), Tok("separator_dots")),
        p("table", "font-size", "table.size", Len, N(14.), Unused),
        p("table", "line-height", "table.line-height", Len, N(20.), Unused),
        p("table", "border-color", "table.line", Color, Tok("table_line"), Tok("table_line")),
        p("table", "border-radius", "table.radius", Len, N(0.), Unused),
        p("th", "background-color", "th.bg", Color, Tok("table_header"), Tok("table_header")),
        p(".callout", "border-radius", "callout.radius", Len, N(4.), Unused),
        p(".callout", "border-width", "callout.border", Len, N(1.), Unused),
        p(".callout", "padding-top", "callout.pad-y", Len, N(12.), Unused),
        p(".callout", "padding-left", "callout.pad-x", Len, N(14.), Unused),
    ]);
    // Code highlighting, in Prism's class names.
    for (sel, key, rgb) in [
        (".token.comment", "syntax.comment", 0x8d9096), (".token.keyword", "syntax.keyword", 0xb2565c), (".token.tag", "syntax.tag", 0xb2565c),
        (".token.string", "syntax.string", 0x89713a), (".token.number", "syntax.number", 0x9759b3), (".token.boolean", "syntax.literal", 0x9759b3),
        (".token.class-name", "syntax.type", 0x3878b1), (".token.attr-name", "syntax.attr", 0x3878b1), (".token.function", "syntax.function", 0x5f7e43),
    ] {
        v.push(p(sel, "color", key, Color, Rgb(rgb), Rgb(rgb)));
    }
    v
});

static BY_KEY: LazyLock<HashMap<&'static str, usize>> = LazyLock::new(|| PROPS.iter().enumerate().map(|(i, p)| (p.key.as_str(), i)).collect());

/// Tokens a rule sets (the rest are listed as `--token-*` in full files).
#[cfg(test)]
static RULE_TOKENS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    PROPS.iter().flat_map(|p| [p.craft, p.feishu]).filter_map(|d| if let Def::Tok(t) = d { Some(t) } else { None }).collect()
});

/// Selector spellings that mean the same element.
fn canon_sel(s: &str) -> String {
    let s = s.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase();
    match s.as_str() {
        "body" | ".markdown" | ".editor" => ".page".into(),
        "b" => "strong".into(),
        ".inline-code" | "p code" | ":not(pre) > code" => "code".into(),
        ".code-block" | "pre code" => "pre".into(),
        ".quote" => "blockquote".into(),
        ".divider" => "hr".into(),
        "ul ::marker" | "ul li::marker" => "ul li::marker".into(),
        "ol ::marker" | "ol li::marker" => "ol li::marker".into(),
        "input[type=checkbox]" | "input[type=\"checkbox\"]" | ".todo" => ".task".into(),
        "input[type=checkbox]:checked" | "input[type=\"checkbox\"]:checked" | ".todo:checked" | ".task.checked" => ".task:checked".into(),
        ".todo-done" | ".task.done" => ".task-done".into(),
        "td" | "th, td" => "table".into(),
        _ => s,
    }
}

// ---- a parsed theme

#[derive(Default)]
pub struct Style {
    pub feishu: bool,
    pub tokens: Vec<Option<Hsla>>,
    values: HashMap<&'static str, Val>,
    /// Named colours: `color.red`, `background.light-red`, `callout.red`, `highlight.red`, `border.red`.
    palette: HashMap<String, Hsla>,
}

#[derive(Clone, Copy, Debug)]
enum Val { N(f32), C(Hsla), F(&'static str), B(bool), Line(LineStyle), Inherit }

impl Style {
    pub fn plain(feishu: bool) -> Style { Style { feishu, tokens: vec![None; TOKENS.len()], ..Default::default() } }
    fn def(&self, key: &str) -> Def {
        BY_KEY.get(key).map(|&i| if self.feishu { PROPS[i].feishu } else { PROPS[i].craft }).unwrap_or(Def::Unused)
    }
    pub fn num(&self, key: &str) -> f32 {
        match self.values.get(key) {
            Some(Val::N(n)) => *n,
            _ => match self.def(key) {
                Def::N(n) => n,
                // Unused by this base: fall back to the craft base's value.
                _ => BY_KEY.get(key).and_then(|&i| if let Def::N(n) = PROPS[i].craft { Some(n) } else { None }).unwrap_or(0.),
            },
        }
    }
    /// A colour property: its own value, its token's, or `None` (inherit).
    pub fn color(&self, key: &str) -> Option<Hsla> {
        match self.values.get(key) {
            Some(Val::C(c)) => Some(crate::theme::adapt(*c)),
            Some(Val::Inherit) => None,
            _ => match self.def(key) {
                Def::Tok(t) => Some(self.token(t)),
                Def::Rgb(v) => Some(crate::theme::c(v)),
                _ => None,
            },
        }
    }
    pub fn flag(&self, key: &str) -> bool {
        match self.values.get(key) {
            Some(Val::B(b)) => *b,
            _ => match self.def(key) { Def::B(b) => b, _ => BY_KEY.get(key).is_some_and(|&i| matches!(PROPS[i].craft, Def::B(true))) },
        }
    }
    pub fn font(&self, key: &str) -> &'static str {
        match self.values.get(key) {
            Some(Val::F(f)) => f,
            _ => match self.def(key) { Def::Family(f) => f, _ => SYSTEM },
        }
    }
    pub fn line(&self, key: &str) -> LineStyle {
        match self.values.get(key) {
            Some(Val::Line(l)) => *l,
            _ => match self.def(key) { Def::Line(l) => l, _ => LineStyle::Dotted },
        }
    }
    pub fn token(&self, name: &str) -> Hsla {
        let i = TOKENS.iter().position(|t| *t == name).unwrap_or(0);
        self.tokens[i].map(crate::theme::adapt).unwrap_or_else(|| crate::editor_theme::base_color(i, self.feishu))
    }
    /// A named colour the theme sets (`kind` is color / background / callout / highlight / border).
    pub fn palette(&self, kind: &str, name: &str) -> Option<Hsla> {
        if self.palette.is_empty() { return None }
        self.palette.get(&format!("{kind}.{name}")).copied().map(if kind == "highlight" { crate::theme::mark } else { crate::theme::adapt })
    }
    /// The named colour this style draws: its own, else its base's.
    pub fn named(&self, kind: &str, name: &str) -> Option<Hsla> { self.palette(kind, name).or_else(|| base_named(kind, name, self.feishu)) }
}

pub struct Theme {
    pub id: String,
    pub name: String,
    pub name_en: Option<String>,
    // Keep stylesheet metadata even though the picker only shows theme names.
    #[allow(dead_code)]
    pub author: Option<String>,
    #[allow(dead_code)]
    pub description: Option<String>,
    /// The file it was read from (`None`: the bundled copy).
    pub path: Option<PathBuf>,
    /// The bundled stylesheet of this id, if it is a built-in one.
    pub bundled: Option<&'static str>,
    pub style: &'static Style,
    /// Rules the app doesn't know (shown, not fatal).
    pub warnings: Vec<String>,
    /// A built-in theme whose file the user has changed.
    pub edited: bool,
}

impl Theme {
    pub fn name(&self) -> &str { if crate::settings::english() { self.name_en.as_deref().unwrap_or(&self.name) } else { &self.name } }
    /// A built-in theme the user has edited (their copy replaces it).
    pub fn customized(&self) -> bool { self.edited }
}

pub const BUNDLED: [(&str, &str); 4] = [
    ("craft", include_str!("../assets/themes/craft.css")),
    ("feishu", include_str!("../assets/themes/feishu.css")),
    ("notion", include_str!("../assets/themes/notion.css")),
    ("obsidian", include_str!("../assets/themes/obsidian.css")),
];

static THEMES: RwLock<Vec<Theme>> = RwLock::new(Vec::new());
static ERRORS: RwLock<Vec<String>> = RwLock::new(Vec::new());
static ACTIVE: AtomicPtr<Style> = AtomicPtr::new(std::ptr::null_mut());
static GENERATION: AtomicU64 = AtomicU64::new(0);
static PLAIN: LazyLock<Style> = LazyLock::new(|| Style::plain(false));

/// The theme being drawn.
pub fn active() -> &'static Style {
    let p = ACTIVE.load(Ordering::Relaxed);
    if p.is_null() { &PLAIN } else { unsafe { &*p } }
}
pub fn color(i: usize) -> Option<Hsla> { active().tokens.get(i).copied().flatten().map(crate::theme::adapt) }
/// Bumped whenever what is drawn may change (the editor re-measures its rows).
pub fn generation() -> u64 { GENERATION.load(Ordering::Relaxed) }
/// Something every theme draws changed (light / dark).
pub fn touch() { GENERATION.fetch_add(1, Ordering::Relaxed); }
pub fn themes() -> std::sync::RwLockReadGuard<'static, Vec<Theme>> { THEMES.read().unwrap() }
/// Files that could not be read at all, from the last load.
pub fn errors() -> Vec<String> { ERRORS.read().unwrap().clone() }
pub fn dir() -> PathBuf { crate::store::Store::root_dir().join("themes") }

// ---- CSS

struct Rule { sels: Vec<String>, decls: Vec<(String, String)> }

/// `selectors { name: value; … }`, comments removed; at-rules skipped whole.
fn rules(css: &str) -> Vec<Rule> {
    let mut s = String::with_capacity(css.len());
    let mut rest = css;
    while let Some(i) = rest.find("/*") {
        s.push_str(&rest[..i]);
        rest = rest[i + 2..].find("*/").map_or("", |j| &rest[i + 2 + j + 2..]);
    }
    s.push_str(rest);
    let chars: Vec<char> = s.chars().collect();
    let mut out = vec![];
    let (mut i, mut start) = (0, 0);
    while i < chars.len() {
        if chars[i] != '{' { i += 1; continue }
        let sel: String = chars[start..i].iter().collect();
        let (mut depth, mut j) = (0, i);
        while j < chars.len() {
            match chars[j] { '{' => depth += 1, '}' => { depth -= 1; if depth == 0 { break } } _ => {} }
            j += 1;
        }
        let body: String = chars[i + 1..j.min(chars.len())].iter().collect();
        i = j + 1;
        start = i;
        let sel = sel.trim();
        if sel.starts_with('@') || body.contains('{') { continue }
        let mut decls = vec![];
        let (mut paren, mut quote, mut from) = (0, None::<char>, 0);
        for (k, ch) in body.char_indices() {
            match ch {
                '"' | '\'' if quote == Some(ch) => quote = None,
                '"' | '\'' if quote.is_none() => quote = Some(ch),
                '(' if quote.is_none() => paren += 1,
                ')' if quote.is_none() => paren -= 1,
                ';' if quote.is_none() && paren == 0 => { decls.push(&body[from..k]); from = k + 1; }
                _ => {}
            }
        }
        decls.push(&body[from..]);
        let decls = decls.into_iter().filter_map(|d| {
            let (n, v) = d.split_once(':')?;
            let v = v.trim().trim_end_matches("!important").trim();
            (!n.trim().is_empty() && !v.is_empty()).then(|| (n.trim().to_lowercase(), v.to_string()))
        }).collect();
        out.push(Rule { sels: sel.split(',').map(canon_sel).filter(|s| !s.is_empty()).collect(), decls });
    }
    out
}

/// `var(--x)` / `var(--x, fallback)`, recursively.
fn subst(v: &str, vars: &HashMap<String, String>, depth: u8) -> String {
    let Some(i) = v.find("var(") else { return v.to_string() };
    if depth > 8 { return v.to_string() }
    let open = i + 4;
    let (mut d, mut end) = (1, v.len());
    for (k, ch) in v[open..].char_indices() {
        match ch { '(' => d += 1, ')' => { d -= 1; if d == 0 { end = open + k; break } } _ => {} }
    }
    let inner = &v[open..end];
    let (name, fallback) = match inner.split_once(',') { Some((n, f)) => (n.trim(), Some(f.trim())), None => (inner.trim(), None) };
    let rep = vars.get(name).cloned().or(fallback.map(str::to_string)).unwrap_or_default();
    let rep = subst(&rep, vars, depth + 1);
    subst(&format!("{}{}{}", &v[..i], rep, &v[(end + 1).min(v.len())..]), vars, depth + 1)
}

fn unquote(s: &str) -> String { s.trim().trim_matches(|c| c == '"' || c == '\'').to_string() }

pub fn parse_color(v: &str) -> Option<Hsla> {
    let v = v.trim().to_lowercase();
    let rgba = |r: f32, g: f32, b: f32, a: f32| -> Hsla { gpui::Rgba { r: r / 255., g: g / 255., b: b / 255., a }.into() };
    match v.as_str() {
        "transparent" => return Some(rgba(0., 0., 0., 0.)),
        "white" => return Some(rgba(255., 255., 255., 1.)),
        "black" => return Some(rgba(0., 0., 0., 1.)),
        _ => {}
    }
    if let Some(h) = v.strip_prefix('#') {
        let x = |s: &str| u8::from_str_radix(s, 16).ok().map(f32::from);
        let c: Vec<char> = h.chars().collect();
        let d = |c: char| x(&c.to_string()).map(|n| n * 17.);
        return match c.len() {
            3 | 4 => Some(rgba(d(c[0])?, d(c[1])?, d(c[2])?, if c.len() == 4 { d(c[3])? / 255. } else { 1. })),
            6 | 8 => Some(rgba(x(&h[0..2])?, x(&h[2..4])?, x(&h[4..6])?, if c.len() == 8 { x(&h[6..8])? / 255. } else { 1. })),
            _ => None,
        };
    }
    let (f, inner) = v.split_once('(')?;
    let inner = inner.strip_suffix(')')?;
    let parts: Vec<&str> = inner.split([',', ' ', '/']).filter(|p| !p.is_empty()).collect();
    if parts.len() < 3 { return None }
    let alpha = |s: Option<&&str>| -> Option<f32> { match s { None => Some(1.), Some(s) => s.strip_suffix('%').map_or_else(|| s.parse().ok(), |p| p.parse::<f32>().ok().map(|n| n / 100.)) } };
    match f.trim() {
        "rgb" | "rgba" => {
            let ch = |s: &str| -> Option<f32> { s.strip_suffix('%').map_or_else(|| s.parse().ok(), |p| p.parse::<f32>().ok().map(|n| n * 2.55)) };
            Some(rgba(ch(parts[0])?, ch(parts[1])?, ch(parts[2])?, alpha(parts.get(3))?))
        }
        "hsl" | "hsla" => {
            let h: f32 = parts[0].trim_end_matches("deg").parse().ok()?;
            let s: f32 = parts[1].trim_end_matches('%').parse::<f32>().ok()? / 100.;
            let l: f32 = parts[2].trim_end_matches('%').parse::<f32>().ok()? / 100.;
            Some(gpui::hsla(h.rem_euclid(360.) / 360., s, l, alpha(parts.get(3))?))
        }
        _ => None,
    }
}

/// A length in pt: px / pt as written, `em` of `em`, `rem` of 16, `%` and
/// unitless numbers of `unitless` (line heights) when given.
fn parse_len(v: &str, em: f32, unitless: Option<f32>) -> Option<f32> {
    let v = v.trim().to_lowercase();
    let num = |s: &str| s.trim().parse::<f32>().ok();
    if let Some(n) = v.strip_suffix("px").or_else(|| v.strip_suffix("pt")) { return num(n) }
    if let Some(n) = v.strip_suffix("rem") { return num(n).map(|n| n * 16.) }
    if let Some(n) = v.strip_suffix("em") { return num(n).map(|n| n * em) }
    if let Some(n) = v.strip_suffix('%') { return num(n).map(|n| n / 100. * unitless.unwrap_or(em)) }
    if v == "0" { return Some(0.) }
    num(&v).and_then(|n| unitless.map(|u| n * u))
}

fn parse_weight(v: &str) -> Option<f32> {
    match v.trim() { "normal" => Some(400.), "bold" => Some(700.), n => n.parse().ok().filter(|n| (1. ..=1000.).contains(n)) }
}

/// The first usable family of a `font-family` list.
fn parse_font<'a>(v: &str, fonts: &mut dyn FnMut() -> &'a [String]) -> Option<&'static str> {
    for f in v.split(',').map(unquote) {
        let alias = match f.to_lowercase().as_str() {
            "system-ui" | "-apple-system" | "blinkmacsystemfont" | "ui-sans-serif" | "sans-serif" => Some(SYSTEM),
            "ui-monospace" | "monospace" | "sfmono-regular" | "sf mono" => Some(SYSTEM_MONO),
            "ui-serif" | "serif" => Some("Times New Roman"),
            _ => None,
        };
        if let Some(a) = alias { return Some(a) }
        if f.starts_with('.') { return Some(Box::leak(f.into_boxed_str())); }
        let fonts = fonts();
        if fonts.is_empty() || fonts.iter().any(|x| x.eq_ignore_ascii_case(&f)) {
            return Some(Box::leak(f.into_boxed_str()));
        }
    }
    None
}

/// A border shorthand → (width, style, colour).
fn border(v: &str) -> (Option<String>, Option<&'static str>, Option<String>) {
    let (mut w, mut s, mut c) = (None, None, None);
    let mut parts = vec![];
    let (mut depth, mut cur) = (0, String::new());
    for ch in v.chars() {
        match ch {
            '(' => { depth += 1; cur.push(ch) }
            ')' => { depth -= 1; cur.push(ch) }
            ' ' if depth == 0 => { if !cur.is_empty() { parts.push(std::mem::take(&mut cur)) } }
            _ => cur.push(ch),
        }
    }
    if !cur.is_empty() { parts.push(cur) }
    for p in parts {
        match p.as_str() {
            "solid" => s = Some("solid"),
            "dotted" => s = Some("dotted"),
            "dashed" => s = Some("dashed"),
            "none" => w = Some("0".into()),
            p if p.chars().next().is_some_and(|c| c.is_ascii_digit() || c == '.') => w = Some(p.to_string()),
            p => c = Some(p.to_string()),
        }
    }
    (w, s, c)
}

pub struct Meta { pub name: Option<String>, pub name_en: Option<String>, pub author: Option<String>, pub description: Option<String> }

fn palette_names(kind: &str) -> Vec<&'static str> {
    match kind {
        "color" | "border" => crate::palette::HUES.to_vec(),
        "highlight" => crate::palette::BG_COLORS.iter().flatten().copied().collect(),
        _ => crate::palette::CALLOUT_BG_COLORS.iter().flatten().copied().collect(),
    }
}

/// Parse a stylesheet. Rules the app doesn't know become warnings; only an
/// unknown base fails.
pub fn parse(css: &str, fonts: &[String]) -> Result<(Style, Meta, Vec<String>), String> {
    parse_with_fonts(css, &mut || fonts)
}

fn parse_with_fonts<'a>(css: &str, fonts: &mut dyn FnMut() -> &'a [String]) -> Result<(Style, Meta, Vec<String>), String> {
    let rules = rules(css);
    let mut vars: HashMap<String, String> = HashMap::new();
    for r in rules.iter().filter(|r| r.sels.iter().any(|s| s == ":root")) {
        for (n, v) in &r.decls {
            if n.starts_with("--") { vars.insert(n.clone(), v.clone()); }
        }
    }
    let var = |n: &str| vars.get(n).map(|v| unquote(&subst(v, &vars, 0)));
    let feishu = match var("--theme-base").as_deref() { None | Some("craft") => false, Some("feishu") => true, Some(b) => return Err(format!("--theme-base: {b}")) };
    let meta = Meta { name: var("--theme-name"), name_en: var("--theme-name-en"), author: var("--theme-author"), description: var("--theme-description") };
    let mut st = Style::plain(feishu);
    let mut warn = vec![];
    for (n, v) in &vars {
        let v = subst(v, &vars, 0);
        for kind in ["color", "background", "callout", "highlight", "border"] {
            if let Some(name) = n.strip_prefix(&format!("--{kind}-")).filter(|name| palette_names(kind).contains(name)) {
                match parse_color(&v) { Some(c) => { st.palette.insert(format!("{kind}.{name}"), c); } None => warn.push(format!("{n}: {v}")) }
            }
        }
        if let Some(t) = n.strip_prefix("--token-") {
            let t = t.replace('-', "_");
            match (TOKENS.iter().position(|x| *x == t), parse_color(&v)) {
                (Some(i), Some(c)) => st.tokens[i] = Some(c),
                _ => warn.push(format!("{n}: {v}")),
            }
        }
    }
    // Declarations in order (later wins); lengths are resolved against the
    // element's font size once all sizes are known.
    let mut raw: Vec<(&'static Prop, String)> = vec![];
    for r in &rules {
        for sel in &r.sels {
            if sel == ":root" { continue }
            for (n, v) in &r.decls {
                let v = subst(v, &vars, 0);
                let mut set = |prop: &str, val: String| {
                    let targets: Vec<&str> = if sel == "li::marker" { vec!["ul li::marker", "ol li::marker"] } else { vec![sel.as_str()] };
                    let mut hit = false;
                    for t in targets {
                        if let Some(p) = PROPS.iter().find(|p| p.sel == t && p.prop == prop) { raw.push((p, val.clone())); hit = true; }
                    }
                    if !hit { warn.push(format!("{sel} {{ {prop} }}")) }
                };
                match n.as_str() {
                    "border-left" | "border-top" | "border" => {
                        let side = if n == "border" && sel == "hr" { "border-top" } else { n.as_str() };
                        let (w, s, c) = border(&v);
                        if let Some(w) = w { set(&format!("{side}-width"), w) }
                        if let Some(s) = s.filter(|_| side == "border-top") { set("border-top-style", s.into()) }
                        if let Some(c) = c { set(&format!("{side}-color"), c) }
                    }
                    "background" => set("background-color", v),
                    "padding" => {
                        let p: Vec<&str> = v.split_whitespace().collect();
                        let (top, left) = match p.len() { 1 => (p[0], p[0]), 2 | 3 => (p[0], p[1]), _ => (p[0], p[3]) };
                        set("padding-top", top.into());
                        set("padding-left", left.into());
                    }
                    "height" if sel == ".task" => set("width", v),
                    "text-decoration" | "text-decoration-line" => {
                        let mut parts = v.split_whitespace();
                        let first = parts.next().unwrap_or("").to_string();
                        let rest: Vec<&str> = parts.collect();
                        set("text-decoration", first);
                        if !rest.is_empty() && sel == "a" { set("text-decoration-color", rest.join(" ")) }
                    }
                    "border-style" if sel == "hr" => set("border-top-style", v),
                    "border-width" if sel == "hr" => set("border-top-width", v),
                    "border-color" if sel == "hr" => set("border-top-color", v),
                    _ => set(n, v),
                }
            }
        }
    }
    let base = Style::plain(feishu);
    let last = |key: &str| raw.iter().rev().find(|(p, _)| p.key == key).map(|(_, v)| v.clone());
    let body = last("page.size").and_then(|v| parse_len(&v, 16., None)).unwrap_or_else(|| base.num("page.size"));
    let size_of = |el: &str| -> f32 {
        let key = format!("{el}.size");
        last(&key).and_then(|v| parse_len(&v, body, None)).unwrap_or_else(|| base.num(&key))
    };
    for (p, v) in &raw {
        let el = p.key.split('.').next().unwrap_or("");
        let em = match el { "page" | "p" | "list" | "quote" | "task" | "code" | "callout" | "hr" | "strong" | "a" | "done" | "bullet" | "number" | "selection" | "th" => body, e => size_of(e) };
        let unitless = (p.prop == "line-height").then_some(em);
        let val = match p.kind {
            Kind::Len => parse_len(v, if p.prop == "font-size" { body } else { em }, unitless).filter(|n| (0. ..2000.).contains(n)).map(Val::N),
            Kind::Weight => parse_weight(v).map(Val::N),
            Kind::Color if matches!(v.trim().to_lowercase().as_str(), "inherit" | "currentcolor") => Some(Val::Inherit),
            Kind::Color => parse_color(v).map(Val::C),
            Kind::Font => parse_font(v, fonts).map(Val::F),
            Kind::Underline => match v.trim() { "underline" => Some(Val::B(true)), "none" => Some(Val::B(false)), _ => None },
            Kind::Strike => match v.trim() { "line-through" => Some(Val::B(true)), "none" => Some(Val::B(false)), _ => None },
            Kind::Display => match v.trim() { "none" => Some(Val::B(false)), "block" | "inline" | "flex" => Some(Val::B(true)), _ => None },
            Kind::Italic => match v.trim() { "italic" | "oblique" => Some(Val::B(true)), "normal" => Some(Val::B(false)), _ => None },
            Kind::LineStyle => match v.trim() { "dotted" => Some(Val::Line(LineStyle::Dotted)), "solid" => Some(Val::Line(LineStyle::Solid)), "dashed" => Some(Val::Line(LineStyle::Dashed)), _ => None },
        };
        let Some(val) = val else { warn.push(format!("{} {{ {}: {v} }}", p.sel, p.prop)); continue };
        let def = if feishu { p.feishu } else { p.craft };
        if matches!(def, Def::Unused) {
            warn.push(format!("{} {{ {} }} — {}", p.sel, p.prop, crate::settings::tr("飞书基础样式不使用", "unused by the feishu base")));
            continue;
        }
        // A colour property backed by a token sets the token too.
        if let (Def::Tok(t), Val::C(c)) = (def, val) {
            st.tokens[TOKENS.iter().position(|x| *x == t).unwrap()] = Some(c);
        }
        st.values.insert(p.key.as_str(), val);
    }
    warn.sort();
    warn.dedup();
    Ok((st, meta, warn))
}

// ---- loading

fn theme_of(id: String, css: &str, path: Option<PathBuf>, bundled: Option<&'static str>, parsed: (Style, Meta, Vec<String>)) -> Theme {
    let (mut st, m, w) = parsed;
    let edited = bundled.is_some_and(|b| b != css);
    // Untouched, the bundled craft.css / feishu.css are the code's own values
    // written out: draw those from the code, so they match it exactly (no rounding).
    if !edited && matches!(id.as_str(), "craft" | "feishu") {
        st = Style::plain(st.feishu);
    }
    Theme { name: m.name.unwrap_or_else(|| id.clone()), id, name_en: m.name_en, author: m.author, description: m.description, path, bundled, style: Box::leak(Box::new(st)), warnings: w, edited }
}

static ALL_LOADED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn all_loaded() -> bool { ALL_LOADED.load(Ordering::Relaxed) }

/// Startup reads only the selected theme. The picker requests the full catalog.
/// Enumerate installed fonts only if a user stylesheet names a concrete family.
pub fn load(selected: &str, all: bool, font_source: impl FnOnce() -> Vec<String>) {
    let fonts = std::cell::OnceCell::new();
    let mut source = Some(font_source);
    let mut resolve_fonts = || fonts.get_or_init(|| source.take().unwrap()()).as_slice();
    let mut list: Vec<Theme> = vec![];
    for (id, css) in BUNDLED.iter().copied().filter(|(id, _)| all || *id == selected) {
        match parse(css, &[]) {
            Ok(p) => list.push(theme_of(id.into(), css, None, Some(css), p)),
            Err(e) => eprintln!("bundled theme {id}: {e}"),
        }
    }
    let mut errors = vec![];
    let mut files: Vec<PathBuf> = if all {
        std::fs::read_dir(dir()).map(|rd| rd.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "css")).collect()).unwrap_or_default()
    } else if !selected.is_empty() && std::path::Path::new(selected).file_name().is_some_and(|n| n == selected) {
        vec![dir().join(format!("{selected}.css"))]
    } else { vec![] };
    files.sort();
    for p in files {
        if !all && !p.exists() { continue; }
        let id = p.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
        let file = p.file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
        match std::fs::read_to_string(&p).map_err(|e| e.to_string()).and_then(|t| parse_with_fonts(&t, &mut resolve_fonts).map(|parsed| (t, parsed))) {
            Ok((text, parsed)) => {
                let bundled = BUNDLED.iter().find(|(b, _)| *b == id).map(|(_, css)| *css);
                let t = theme_of(id.clone(), &text, Some(p), bundled, parsed);
                // The user's copy of a built-in theme takes its place.
                match list.iter().position(|x| x.id == id) { Some(i) => list[i] = t, None => list.push(t) }
            }
            Err(e) => errors.push(format!("{file}: {e}")),
        }
    }
    list.sort_by_key(|t| t.id != "feishu");
    if list.is_empty() {
        let (id, css) = BUNDLED[0];
        if let Ok(p) = parse(css, &[]) { list.push(theme_of(id.into(), css, None, Some(css), p)); }
        // Preserve the user's craft.css override when the selected theme disappeared.
        let p = dir().join("craft.css");
        if selected != "craft" && p.exists() {
            match std::fs::read_to_string(&p).map_err(|e| e.to_string()).and_then(|t| parse_with_fonts(&t, &mut resolve_fonts).map(|parsed| (t, parsed))) {
                Ok((text, parsed)) => list[0] = theme_of(id.into(), &text, Some(p), Some(css), parsed),
                Err(e) => errors.push(format!("craft.css: {e}")),
            }
        }
    }
    ALL_LOADED.store(all, Ordering::Relaxed);
    *THEMES.write().unwrap() = list;
    *ERRORS.write().unwrap() = errors;
    select(selected);
}

/// Draw with theme `id` (Craft when it is gone); returns the id in use.
pub fn select(id: &str) -> String {
    let list = THEMES.read().unwrap();
    let Some(t) = list.iter().find(|t| t.id == id).or_else(|| list.first()) else { return "craft".into() };
    ACTIVE.store(t.style as *const Style as *mut Style, Ordering::Relaxed);
    GENERATION.fetch_add(1, Ordering::Relaxed);
    crate::settings::set_base(t.style.feishu);
    t.id.clone()
}

/// Modification stamps of the user's theme files (live reload compares them).
pub fn stamp() -> Vec<(PathBuf, std::time::SystemTime, u64)> {
    let mut v: Vec<_> = std::fs::read_dir(dir()).map(|rd| rd.flatten().filter(|e| e.path().extension().is_some_and(|x| x == "css"))
        .filter_map(|e| { let m = e.metadata().ok()?; Some((e.path(), m.modified().ok()?, m.len())) }).collect()).unwrap_or_default();
    v.sort();
    v
}

/// The stylesheet of theme `id` as a file the user can edit: their own file,
/// or for a built-in theme a full copy of it (written now) that replaces it.
pub fn editable_file(id: &str) -> std::io::Result<PathBuf> {
    let list = THEMES.read().unwrap();
    let t = list.iter().find(|t| t.id == id).ok_or_else(|| std::io::Error::other("no such theme"))?;
    if let Some(p) = &t.path { return Ok(p.clone()) }
    let d = dir();
    std::fs::create_dir_all(&d)?;
    let p = d.join(format!("{id}.css"));
    std::fs::write(&p, t.bundled.unwrap_or_default())?;
    Ok(p)
}

/// A new theme file copied from `id`, under a new id and name.
pub fn duplicate(id: &str) -> std::io::Result<(String, PathBuf)> {
    let list = THEMES.read().unwrap();
    let t = list.iter().find(|t| t.id == id).ok_or_else(|| std::io::Error::other("no such theme"))?;
    let src = match &t.path { Some(p) => std::fs::read_to_string(p)?, None => t.bundled.unwrap_or_default().to_string() };
    let d = dir();
    std::fs::create_dir_all(&d)?;
    let (new_id, p) = (1..).map(|n| { let s = if n == 1 { format!("{id}-copy") } else { format!("{id}-copy-{n}") }; let p = d.join(format!("{s}.css")); (s, p) })
        .find(|(s, p)| !p.exists() && !BUNDLED.iter().any(|(b, _)| b == s)).unwrap();
    let name = format!("{} {}", t.name, crate::settings::tr("副本", "copy"));
    let mut out = String::with_capacity(src.len());
    for line in src.lines() {
        let l = line.trim_start();
        if l.starts_with("--theme-name:") { out.push_str(&format!("  --theme-name: \"{name}\";\n")); continue }
        if l.starts_with("--theme-name-en:") { continue }
        out.push_str(line);
        out.push('\n');
    }
    std::fs::write(&p, out)?;
    Ok((new_id, p))
}

/// Drop the user's copy of a built-in theme (or delete a theme of their own).
pub fn remove(id: &str) -> std::io::Result<()> {
    let p = THEMES.read().unwrap().iter().find(|t| t.id == id).and_then(|t| t.path.clone());
    match p { Some(p) => std::fs::remove_file(p), None => Ok(()) }
}

// ---- writing a complete stylesheet

#[cfg(test)]
fn css_color(c: Hsla) -> String {
    let c = gpui::Rgba::from(c);
    let b = |v: f32| (v * 255.).round() as u8;
    if b(c.a) == 255 { format!("#{:02x}{:02x}{:02x}", b(c.r), b(c.g), b(c.b)) } else { format!("#{:02x}{:02x}{:02x}{:02x}", b(c.r), b(c.g), b(c.b), b(c.a)) }
}

#[cfg(test)]
fn css_font(f: &str) -> String {
    match f { SYSTEM => "system-ui".into(), SYSTEM_MONO => "ui-monospace".into(), f if f.contains(' ') => format!("\"{f}\""), f => f.into() }
}

#[cfg(test)]
fn css_num(n: f32) -> String { if n.fract() == 0. { format!("{n:.0}") } else { format!("{}", (n * 100.).round() / 100.) } }

/// The base style's named colour.
fn base_named(kind: &str, name: &str, feishu: bool) -> Option<Hsla> {
    match (kind, feishu) {
        ("color", false) => crate::theme::text_color(name),
        ("color", true) => crate::feishu_theme::text_color(name),
        ("background", false) => crate::theme::block_bg(name),
        ("background", true) => crate::feishu_theme::block_bg(name),
        ("callout", false) => crate::theme::callout_bg(name),
        ("callout", true) => crate::feishu_theme::callout_bg(name),
        ("highlight", false) => crate::theme::bg_color(name),
        ("highlight", true) => crate::feishu_theme::bg_color(name),
        ("border", false) => crate::theme::callout_border(name),
        ("border", true) => crate::feishu_theme::callout_border(name),
        _ => None,
    }
}

/// Every property of a base style at its value, as a stylesheet. The bundled
/// craft.css / feishu.css are exactly this (a test keeps them so).
#[cfg(test)]
fn base_css(feishu: bool, name: &str, name_en: &str, description: &str) -> String {
    let st = Style::plain(feishu);
    let mut out = format!("/*\n * quiet.md 文档主题：{name}\n *\n * 每个属性都写着当前的值。改完保存，应用里立即生效。\n * 选择器与属性的说明见 docs/THEMES.md。\n */\n\n:root {{\n");
    out += &format!("  --theme-name: \"{name}\";\n  --theme-name-en: \"{name_en}\";\n  --theme-author: \"quiet.md\";\n  --theme-description: \"{description}\";\n  /* 块的画法：craft 或 feishu */\n  --theme-base: {};\n", if feishu { "feishu" } else { "craft" });
    for (kind, title) in [("color", "文字颜色"), ("highlight", "文字高亮"), ("background", "块背景"), ("callout", "高亮块底色"), ("border", "高亮块边框")] {
        out += &format!("\n  /* {title} */\n");
        for n in palette_names(kind) {
            if let Some(c) = base_named(kind, n, feishu) { out += &format!("  --{kind}-{n}: {};\n", css_color(c)); }
        }
    }
    out += "\n  /* 其它界面细节（文档内的选中、悬停、表格工具等） */\n";
    for (i, t) in TOKENS.iter().enumerate() {
        // Window material depends on the running OS, not the document theme.
        if *t != "window" && !RULE_TOKENS.contains(t) { out += &format!("  --token-{}: {};\n", t.replace('_', "-"), css_color(crate::editor_theme::base_color(i, feishu))); }
    }
    out += "}\n";
    let mut sel = "";
    for p in PROPS.iter() {
        let def = if feishu { p.feishu } else { p.craft };
        if matches!(def, Def::Unused) { continue }
        if p.sel != sel {
            if !sel.is_empty() { out += "}\n" }
            out += &format!("\n{} {{\n", p.sel);
            sel = p.sel;
        }
        let v = match p.kind {
            Kind::Len => format!("{}px", css_num(st.num(&p.key))),
            Kind::Weight => css_num(st.num(&p.key)),
            Kind::Color => st.color(&p.key).map_or_else(|| "inherit".into(), css_color),
            Kind::Font => css_font(st.font(&p.key)),
            Kind::Underline => if st.flag(&p.key) { "underline" } else { "none" }.into(),
            Kind::Strike => if st.flag(&p.key) { "line-through" } else { "none" }.into(),
            Kind::Display => if st.flag(&p.key) { "block" } else { "none" }.into(),
            Kind::Italic => if st.flag(&p.key) { "italic" } else { "normal" }.into(),
            Kind::LineStyle => match st.line(&p.key) { LineStyle::Dotted => "dotted", LineStyle::Solid => "solid", LineStyle::Dashed => "dashed" }.into(),
        };
        out += &format!("  {}: {v};\n", p.prop);
    }
    out += "}\n";
    out
}

#[cfg(test)]
const CRAFT_DESCRIPTION: &str = "Craft for macOS 的文档样式：SF Pro 15 / 22，虚点分隔线，蓝色强调。";
#[cfg(test)]
const FEISHU_DESCRIPTION: &str = "飞书文档的正文样式：16 / 26，标题 Medium，蓝色链接。";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_themes_are_complete_and_clean() {
        for (id, css) in BUNDLED {
            let (_, m, warn) = parse(css, &[]).unwrap_or_else(|e| panic!("{id}: {e}"));
            assert!(m.name.is_some(), "{id}: name");
            assert!(warn.is_empty(), "{id}: {warn:?}");
            let feishu = id == "feishu";
            let rules = rules(css);
            for p in PROPS.iter().filter(|p| !matches!(if feishu { p.feishu } else { p.craft }, Def::Unused)) {
                assert!(rules.iter().any(|r| r.sels.iter().any(|s| s == p.sel) && r.decls.iter().any(|(n, _)| n == p.prop)), "{id} lacks {} {{ {} }}", p.sel, p.prop);
            }
        }
    }

    #[test]
    fn base_files_match_the_code() {
        for (i, feishu, name, name_en, description) in [(0, false, "Craft", "Craft", CRAFT_DESCRIPTION), (1, true, "飞书", "Feishu", FEISHU_DESCRIPTION)] {
            let expected = base_css(feishu, name, name_en, description);
            let actual: Vec<_> = BUNDLED[i].1.lines().collect();
            let generated: Vec<_> = expected.lines().collect();
            for line in 0..actual.len().max(generated.len()) {
                assert_eq!(actual.get(line), generated.get(line), "{} theme line {} differs; regenerate with cargo test --release write_base_css -- --ignored", BUNDLED[i].0, line + 1);
            }
        }
    }

    /// `cargo test --release write_base_css -- --ignored` rewrites the two base files.
    #[test]
    #[ignore]
    fn write_base_css() {
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/themes");
        std::fs::write(format!("{dir}/craft.css"), base_css(false, "Craft", "Craft", CRAFT_DESCRIPTION)).unwrap();
        std::fs::write(format!("{dir}/feishu.css"), base_css(true, "飞书", "Feishu", FEISHU_DESCRIPTION)).unwrap();
    }

    #[test]
    fn css_values() {
        let css = r#"
            /* comment */
            :root { --theme-name: "T"; --accent: hsl(254, 80%, 68%); --color-red: #d44c47; --token-block-hover: rgba(0,0,0,.05); --mine: 1px; }
            .page, body { font-size: 16px; line-height: 1.5; font-family: "No Such", -apple-system, sans-serif; }
            h1 { font-size: 1.875em; line-height: 1.3; margin-top: 2em; font-weight: 600; }
            blockquote { border-left: 3px solid var(--accent); }
            a { text-decoration: underline rgba(55, 53, 47, .4); }
            hr { border-top: 1px solid #e0e0e0; }
            .callout { padding: 16px 16px 16px 12px; }
            @media (prefers-color-scheme: dark) { .page { color: white; } }
            .made-up { color: red; }
        "#;
        let (st, m, warn) = parse(css, &["Inter".into()]).unwrap();
        assert_eq!(m.name.as_deref(), Some("T"));
        assert_eq!(st.num("page.line"), 24.);
        assert_eq!(st.font("page.font"), SYSTEM);
        assert_eq!((st.num("h1.size"), st.num("h1.line"), st.num("h1.mt")), (30., 39., 60.));
        assert_eq!(st.num("quote.bar"), 3.);
        assert!(st.color("quote.bar-color").is_some());
        assert!(st.flag("a.underline") && st.color("a.underline-color").is_some());
        assert_eq!(st.line("hr.style"), LineStyle::Solid);
        assert_eq!((st.num("callout.pad-y"), st.num("callout.pad-x")), (16., 12.));
        assert!(st.palette("color", "red").is_some());
        assert!(st.tokens[TOKENS.iter().position(|t| *t == "block_hover").unwrap()].is_some());
        assert_eq!(warn, vec![".made-up { color }".to_string()]);
    }

    #[test]
    fn copies_keep_their_values() {
        let (a, ..) = parse(BUNDLED[2].1, &[]).unwrap();
        let renamed = BUNDLED[2].1.replacen("--theme-name: \"Notion\"", "--theme-name: \"X\"", 1);
        let (b, m, _) = parse(&renamed, &[]).unwrap();
        assert_eq!(m.name.as_deref(), Some("X"));
        assert_eq!(a.num("h1.size"), b.num("h1.size"));
    }
}
