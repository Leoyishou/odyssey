//! LaTeX equations. katex-rs builds KaTeX's HTML for the source, a small
//! emulation of katex.css (0.16) lays that tree out — glyph runs in the
//! bundled KaTeX fonts, rules from borders, and KaTeX's SVG paths — and the
//! result is painted with GPUI. Every equation renders at a fixed 18px with
//! `\displaystyle` prepended.

use gpui::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{LazyLock, Mutex};

/// `.katex { font-size: 18px }`.
pub const FONT_PX: f32 = 18.;

/// Asset path prefix for the SVG pieces generated during layout.
pub const SVG_PREFIX: &str = "katex-svg/";

const SIZES: [f32; 11] = [0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.2, 1.44, 1.728, 2.074, 2.488];

/// The test for rendering as a block (display) equation:
/// `/\\begin\n*\{(equation|split|align|gather|alignat|CD)\}|\\tag/`.
pub fn is_block(src: &str) -> bool {
    if src.contains("\\tag") {
        return true;
    }
    let mut rest = src;
    while let Some(i) = rest.find("\\begin") {
        let after = rest[i + 6..].trim_start_matches('\n');
        if let Some(body) = after.strip_prefix('{') {
            for env in ["equation", "split", "align", "gather", "alignat", "CD"] {
                if body.strip_prefix(env).is_some_and(|r| r.starts_with('}')) {
                    return true;
                }
            }
        }
        rest = &rest[i + 6..];
    }
    false
}

/// Register the KaTeX fonts (MIT, bundled under assets/fonts/katex).
pub fn load_fonts(cx: &App) {
    let fonts = crate::assets::FILES
        .iter()
        .filter(|(p, _)| p.starts_with("fonts/katex/") && p.ends_with(".ttf"))
        .map(|(_, b)| std::borrow::Cow::Borrowed(*b))
        .collect();
    let _ = cx.text_system().add_fonts(fonts);
}

static SVGS: LazyLock<Mutex<HashMap<String, Vec<u8>>>> = LazyLock::new(Default::default);

pub fn svg_asset(path: &str) -> Option<Vec<u8>> {
    SVGS.lock().ok()?.get(path).cloned()
}

fn register_svg(markup: String) -> SharedString {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    markup.hash(&mut h);
    let key = format!("{SVG_PREFIX}{:016x}.svg", h.finish());
    if let Ok(mut m) = SVGS.lock() {
        m.entry(key.clone()).or_insert_with(|| markup.into_bytes());
    }
    key.into()
}

#[derive(Clone)]
enum Item {
    Text { x: f32, y: f32, line: ShapedLine },
    Rect { x: f32, y: f32, w: f32, h: f32, color: Hsla },
    Svg { x: f32, y: f32, w: f32, h: f32, path: SharedString, color: Hsla },
}

impl Item {
    fn shift(&mut self, dx: f32, dy: f32) {
        match self {
            Item::Text { x, y, .. } | Item::Rect { x, y, .. } | Item::Svg { x, y, .. } => {
                *x += dx;
                *y += dy;
            }
        }
    }
}

/// A laid-out equation. Coordinates are px with the origin at the left end
/// of the first line's baseline.
pub struct Math {
    pub width: f32,
    pub ascent: f32,
    pub depth: f32,
    items: Vec<Item>,
    /// `\tag{..}` of a display equation, right-aligned in the row.
    pub tag: Option<Box<Math>>,
}

impl Math {
    pub fn paint(&self, origin: Point<Pixels>, window: &mut Window, cx: &mut App) {
        for it in &self.items {
            match it {
                Item::Text { x, y, line } => {
                    let lh = line.ascent + line.descent;
                    let _ = line.paint(point(origin.x + px(*x), origin.y + px(*y) - line.ascent), lh, window, cx);
                }
                Item::Rect { x, y, w, h, color } => {
                    window.paint_quad(fill(
                        Bounds::new(point(origin.x + px(*x), origin.y + px(*y)), size(px(*w), px(*h))),
                        *color,
                    ));
                }
                Item::Svg { x, y, w, h, path, color } => {
                    let b = Bounds::new(point(origin.x + px(*x), origin.y + px(*y)), size(px(*w), px(*h)));
                    let _ = window.paint_svg(b, path.clone(), TransformationMatrix::unit(), *color, cx);
                }
            }
        }
    }
}

type Key = (String, bool, u32, u32, u32);

thread_local! {
    static CTX: katex::KatexContext = katex::KatexContext::default();
    static CACHE: RefCell<HashMap<Key, Option<Rc<Math>>>> = RefCell::new(HashMap::new());
    static NUMBERS: RefCell<HashMap<String, u32>> = RefCell::new(HashMap::new());
}

/// How many `(n)` equation numbers `src` shows (`\begin{equation}`,
/// numbered `align` rows, ...), for the document-wide `katexEqnNo` counter.
pub fn numbers_in(src: &str) -> u32 {
    if !src.contains("\\begin") {
        return 0;
    }
    if let Some(n) = NUMBERS.with(|c| c.borrow().get(src).copied()) {
        return n;
    }
    let n = render_html(src, is_block(src)).map_or(0, |h| h.matches("eqn-num").count() as u32);
    NUMBERS.with(|c| {
        let mut c = c.borrow_mut();
        if c.len() > 2000 {
            c.clear();
        }
        c.insert(src.to_string(), n);
    });
    n
}

/// Lay out `src` at `font_px`; `None` when KaTeX rejects it (无效公式).
/// Equation numbers count up from `eqno + 1` (`counter-reset: katexEqnNo eqno`).
pub fn layout(src: &str, display: bool, font_px: f32, color: Hsla, eqno: u32, window: &Window) -> Option<Rc<Math>> {
    let key = (src.to_string(), display, font_px.to_bits(), u32::from(Rgba::from(color)), eqno);
    if let Some(hit) = CACHE.with(|c| c.borrow().get(&key).cloned()) {
        return hit;
    }
    let math = render_html(src, display).and_then(|html| {
        let root = parse(&html)?;
        let mut eng = Engine { ts: window.text_system(), eqno };
        Some(Rc::new(eng.root(&root, display, font_px, color)))
    });
    CACHE.with(|c| {
        let mut c = c.borrow_mut();
        if c.len() > 2000 {
            c.clear();
        }
        c.insert(key, math.clone());
    });
    math
}

fn render_html(src: &str, display: bool) -> Option<String> {
    let mut settings = katex::types::Settings::default();
    settings.display_mode = display;
    settings.output = katex::types::OutputFormat::Html;
    settings.throw_on_error = true;
    let expr = format!("\\displaystyle {}", src.trim());
    CTX.with(|ctx| {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| katex::core::render_to_string(ctx, &expr, &settings)))
            .ok()?
            .ok()
    })
}

/// The equation as MathML (`<math>…</math>`), for HTML export.
pub fn mathml(src: &str) -> Option<String> {
    let mut settings = katex::types::Settings::default();
    settings.display_mode = is_block(src);
    settings.output = katex::types::OutputFormat::Mathml;
    settings.throw_on_error = true;
    let expr = format!("\\displaystyle {}", src.trim());
    CTX.with(|ctx| {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| katex::core::render_to_string(ctx, &expr, &settings)))
            .ok()?
            .ok()
    })
}

// ------------------------------------------------------------------ DOM

struct El {
    name: String,
    class: Vec<String>,
    style: Vec<(String, String)>,
    attrs: Vec<(String, String)>,
    kids: Vec<N>,
}

enum N {
    El(El),
    Text(String),
}

impl El {
    fn has(&self, c: &str) -> bool {
        self.class.iter().any(|x| x == c)
    }
    fn any(&self, cs: &[&str]) -> bool {
        cs.iter().any(|c| self.has(c))
    }
    fn style(&self, k: &str) -> Option<&str> {
        self.style.iter().rev().find(|(a, _)| a == k).map(|(_, v)| v.as_str())
    }
    fn attr(&self, k: &str) -> Option<&str> {
        self.attrs.iter().find(|(a, _)| a == k).map(|(_, v)| v.as_str())
    }
    fn els(&self) -> impl Iterator<Item = &El> {
        self.kids.iter().filter_map(|k| if let N::El(e) = k { Some(e) } else { None })
    }
    fn find(&self, c: &str) -> Option<&El> {
        if self.has(c) {
            return Some(self);
        }
        self.els().find_map(|e| e.find(c))
    }
}

fn parse(html: &str) -> Option<El> {
    use quick_xml::events::{BytesStart, Event};
    let mut reader = quick_xml::Reader::from_str(html);
    reader.config_mut().trim_text(false);
    let new_el = |e: &BytesStart| -> El {
        let mut el = El {
            name: String::from_utf8_lossy(e.name().as_ref()).into_owned(),
            class: vec![],
            style: vec![],
            attrs: vec![],
            kids: vec![],
        };
        for a in e.attributes().flatten() {
            let k = String::from_utf8_lossy(a.key.as_ref()).into_owned();
            let v = a.unescape_value().map(|v| v.into_owned()).unwrap_or_default();
            match k.as_str() {
                // katex-rs prefixes generic class names (`katex-base`, ...).
                "class" => {
                    el.class = v
                        .split_whitespace()
                        .map(|c| match c.strip_prefix("katex-") {
                            Some(r) if !matches!(r, "html" | "display" | "mathml" | "version") => r.to_string(),
                            _ => c.to_string(),
                        })
                        .collect()
                }
                "style" => {
                    el.style = v
                        .split(';')
                        .filter_map(|d| d.split_once(':'))
                        .map(|(a, b)| (a.trim().to_string(), b.trim().to_string()))
                        .collect()
                }
                _ => el.attrs.push((k, v)),
            }
        }
        el
    };
    let mut stack = vec![El { name: "#root".into(), class: vec![], style: vec![], attrs: vec![], kids: vec![] }];
    loop {
        match reader.read_event().ok()? {
            Event::Start(e) => stack.push(new_el(&e)),
            Event::Empty(e) => {
                let el = new_el(&e);
                stack.last_mut()?.kids.push(N::El(el));
            }
            Event::End(_) => {
                let el = stack.pop()?;
                stack.last_mut()?.kids.push(N::El(el));
            }
            Event::Text(t) => {
                let s = t.unescape().ok()?.into_owned();
                stack.last_mut()?.kids.push(N::Text(s));
            }
            Event::Eof => break,
            _ => {}
        }
    }
    if stack.len() != 1 {
        return None;
    }
    stack.pop()
}

// --------------------------------------------------------------- layout

#[derive(Default)]
struct Bx {
    /// Outer (margin-box) width.
    w: f32,
    /// Extent above / below the baseline.
    h: f32,
    d: f32,
    items: Vec<Item>,
    /// Creates a line box in its container (text or an atomic inline).
    lines: bool,
    /// Depends on the containing block width (`width: 100%`).
    pct: bool,
}

impl Bx {
    fn append(&mut self, mut b: Bx) {
        for it in &mut b.items {
            it.shift(self.w, 0.);
        }
        self.items.append(&mut b.items);
        self.w += b.w;
        self.h = self.h.max(b.h);
        self.d = self.d.max(b.d);
        self.lines |= b.lines;
        self.pct |= b.pct;
    }
}

#[derive(Clone)]
struct Ctx {
    fs: f32,
    family: &'static str,
    bold: bool,
    italic: bool,
    color: Hsla,
    /// Inherited text-align: 0 left, .5 center, 1 right.
    align: f32,
    /// Containing block width, once known.
    cb: Option<f32>,
}

/// (classes, family, bold, italic) in katex.css order.
const FONT_RULES: &[(&[&str], Option<&str>, Option<bool>, Option<bool>)] = &[
    (&["textbf"], None, Some(true), None),
    (&["textit"], None, None, Some(true)),
    (&["textrm"], Some("KaTeX_Main"), None, None),
    (&["textsf"], Some("KaTeX_SansSerif"), None, None),
    (&["texttt"], Some("KaTeX_Typewriter"), None, None),
    (&["mathnormal"], Some("KaTeX_Math"), None, Some(true)),
    (&["mathit"], Some("KaTeX_Main"), None, Some(true)),
    (&["mathrm"], None, None, Some(false)),
    (&["mathbf"], Some("KaTeX_Main"), Some(true), None),
    (&["boldsymbol"], Some("KaTeX_Math"), Some(true), Some(true)),
    (&["amsrm", "mathbb", "textbb"], Some("KaTeX_AMS"), None, None),
    (&["mathcal"], Some("KaTeX_Caligraphic"), None, None),
    (&["mathfrak", "textfrak"], Some("KaTeX_Fraktur"), None, None),
    (&["mathboldfrak", "textboldfrak"], Some("KaTeX_Fraktur"), Some(true), None),
    (&["mathtt"], Some("KaTeX_Typewriter"), None, None),
    (&["mathscr", "textscr"], Some("KaTeX_Script"), None, None),
    (&["mathsf", "textsf"], Some("KaTeX_SansSerif"), None, None),
    (&["mathboldsf", "textboldsf"], Some("KaTeX_SansSerif"), Some(true), None),
    (&["mathsfit", "mathitsf", "textitsf"], Some("KaTeX_SansSerif"), None, Some(true)),
    (&["mainrm"], Some("KaTeX_Main"), None, Some(false)),
];

const INLINE_BLOCK: &[&str] = &[
    "base",
    "strut",
    "mspace",
    "nulldelimiter",
    "frac-line",
    "rule",
    "overline-line",
    "underline-line",
    "hline",
    "hdashline",
    "fix",
    "vertical-separator",
    "arraycolsep",
    "cd-vert-arrow",
    "overlay",
    "stretchy",
    "vbox",
    "hbox",
    "thinbox",
];

const FULL_WIDTH: &[&str] =
    &["frac-line", "overline-line", "underline-line", "hline", "hdashline", "stretchy", "hide-tail", "hbox"];

const ABS: &[&str] = &["halfarrow-left", "halfarrow-right", "brace-left", "brace-center", "brace-right"];

fn len(v: &str, fs: f32, cb: Option<f32>) -> Option<f32> {
    let v = v.trim();
    if let Some(n) = v.strip_suffix("em") {
        n.parse::<f32>().ok().map(|n| n * fs)
    } else if let Some(n) = v.strip_suffix("px") {
        n.parse().ok()
    } else if let Some(n) = v.strip_suffix('%') {
        Some(n.parse::<f32>().ok()? / 100. * cb?)
    } else {
        v.parse().ok()
    }
}

/// Expand a 1–4 value box shorthand into [top, right, bottom, left].
fn sides(v: &str) -> [&str; 4] {
    let p: Vec<&str> = v.split_whitespace().collect();
    match p.len() {
        1 => [p[0]; 4],
        2 => [p[0], p[1], p[0], p[1]],
        3 => [p[0], p[1], p[2], p[1]],
        4 => [p[0], p[1], p[2], p[3]],
        _ => ["0"; 4],
    }
}

/// Chrome rounds border widths down to whole device pixels (min 1).
fn snap(w: f32) -> f32 {
    if w <= 0. { 0. } else { (w * 2.).floor().max(1.) / 2. }
}

fn css_color(v: &str) -> Option<Hsla> {
    let v = v.trim();
    if let Some(h) = v.strip_prefix('#') {
        let n = u32::from_str_radix(h, 16).ok()?;
        let rgb = match h.len() {
            3 => ((n >> 8 & 0xf) * 0x11) << 16 | ((n >> 4 & 0xf) * 0x11) << 8 | ((n & 0xf) * 0x11),
            6 => n,
            _ => return None,
        };
        return Some(rgb_u32(rgb));
    }
    let rgb = match v.to_ascii_lowercase().as_str() {
        "black" => 0x000000,
        "white" => 0xffffff,
        "red" => 0xff0000,
        "green" => 0x008000,
        "blue" => 0x0000ff,
        "yellow" => 0xffff00,
        "cyan" | "aqua" => 0x00ffff,
        "magenta" | "fuchsia" => 0xff00ff,
        "gray" | "grey" => 0x808080,
        "orange" => 0xffa500,
        "purple" => 0x800080,
        "brown" => 0xa52a2a,
        "pink" => 0xffc0cb,
        "lime" => 0x00ff00,
        "navy" => 0x000080,
        "teal" => 0x008080,
        "olive" => 0x808000,
        "maroon" => 0x800000,
        "silver" => 0xc0c0c0,
        "gold" => 0xffd700,
        "violet" => 0xee82ee,
        _ => return None,
    };
    Some(rgb_u32(rgb))
}

fn rgb_u32(n: u32) -> Hsla {
    rgb(n).into()
}

struct Engine<'a> {
    ts: &'a WindowTextSystem,
    /// The `katexEqnNo` counter.
    eqno: u32,
}

impl Engine<'_> {
    fn root(&mut self, root: &El, display: bool, fs: f32, color: Hsla) -> Math {
        let cx = Ctx { fs, family: "KaTeX_Main", bold: false, italic: false, color, align: 0., cb: None };
        // The root strut of `.katex` (KaTeX_Main, line-height 1.2).
        let (asc_min, desc_min) = ((0.903 + 0.0125) * fs, (0.272 + 0.0125) * fs);
        let Some(html) = root.find("katex-html") else {
            return Math { width: 0., ascent: asc_min, depth: desc_min, items: vec![], tag: None };
        };
        let mut lines = vec![Bx::default()];
        let mut tag = None;
        for k in html.els() {
            if k.has("newline") {
                lines.push(Bx::default());
            } else if k.has("tag") {
                let b = self.el(k, &cx, &html.class, false);
                tag = Some(Box::new(Math { width: b.w, ascent: b.h, depth: b.d, items: b.items, tag: None }));
            } else {
                let b = self.el(k, &cx, &html.class, false);
                lines.last_mut().unwrap().append(b);
            }
        }
        let width = lines.iter().fold(0f32, |a, l| a.max(l.w));
        let align = if display { 0.5 } else { 0. };
        let mut items = vec![];
        let mut y = 0.;
        let mut ascent = 0.;
        let mut depth = 0.;
        for (i, mut l) in lines.into_iter().enumerate() {
            let (h, d) = (l.h.max(asc_min), l.d.max(desc_min));
            if i == 0 {
                ascent = h;
            } else {
                y += depth + h;
            }
            depth = d;
            let dx = align * (width - l.w);
            for it in &mut l.items {
                it.shift(dx, y);
            }
            items.append(&mut l.items);
        }
        Math { width, ascent, depth: y + depth, items, tag }
    }

    fn apply(&self, e: &El, parent: &[String], cx: &mut Ctx) {
        if e.has("sizing") || e.has("fontsize-ensurer") {
            let num = |p: &str| e.class.iter().find_map(|c| c.strip_prefix(p)?.parse::<usize>().ok());
            if let (Some(a), Some(b)) = (num("reset-size"), num("size"))
                && (1..=11).contains(&a)
                && (1..=11).contains(&b)
            {
                cx.fs *= SIZES[b - 1] / SIZES[a - 1];
            }
        }
        for (cls, fam, bold, italic) in FONT_RULES {
            if e.any(cls) {
                if let Some(f) = fam {
                    cx.family = f;
                }
                if let Some(b) = bold {
                    cx.bold = *b;
                }
                if let Some(i) = italic {
                    cx.italic = *i;
                }
            }
        }
        if e.has("delimsizing") {
            for (c, f) in [("size1", "KaTeX_Size1"), ("size2", "KaTeX_Size2"), ("size3", "KaTeX_Size3"), ("size4", "KaTeX_Size4")]
            {
                if e.has(c) {
                    cx.family = f;
                }
            }
        }
        if parent.iter().any(|c| c == "delim-size1") {
            cx.family = "KaTeX_Size1";
        } else if parent.iter().any(|c| c == "delim-size4") {
            cx.family = "KaTeX_Size4";
        }
        if e.has("op-symbol") {
            if e.has("small-op") {
                cx.family = "KaTeX_Size1";
            } else if e.has("large-op") {
                cx.family = "KaTeX_Size2";
            }
        }
        if let Some(c) = e.style("color").and_then(css_color) {
            cx.color = c;
        }
        if e.any(&["msupsub", "svg-align", "cd-label-left"]) {
            cx.align = 0.;
        } else if e.any(&["x-arrow", "mover", "munder"]) {
            cx.align = 0.5;
        } else if e.has("cd-label-right") {
            cx.align = 1.;
        }
        if e.has("vlist-t") {
            let p = |c: &str| parent.iter().any(|x| x == c);
            if p("mfrac") || p("op-limits") || p("accent") || p("col-align-c") {
                cx.align = 0.5;
            } else if p("col-align-l") {
                cx.align = 0.;
            } else if p("col-align-r") {
                cx.align = 1.;
            }
        }
    }

    fn text(&mut self, t: &str, cx: &Ctx, out: &mut Bx) {
        if t.is_empty() {
            return;
        }
        let font = Font {
            family: cx.family.into(),
            features: FontFeatures::default(),
            fallbacks: Some(FontFallbacks::from_fonts(vec!["PingFang SC".into()])),
            weight: if cx.bold { FontWeight::BOLD } else { FontWeight::NORMAL },
            style: if cx.italic { FontStyle::Italic } else { FontStyle::Normal },
        };
        let run = TextRun {
            len: t.len(),
            font,
            color: cx.color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let line = self.ts.shape_line(SharedString::from(t.to_string()), px(cx.fs), &[run], None);
        let (asc, desc) = (f32::from(line.ascent), f32::from(line.descent));
        // The inline box of `line-height: 1.2`.
        let half = (1.2 * cx.fs - asc - desc) / 2.;
        let w = f32::from(line.width);
        out.items.push(Item::Text { x: out.w, y: 0., line });
        out.w += w;
        out.h = out.h.max(asc + half);
        out.d = out.d.max(desc + half);
        out.lines = true;
    }

    fn el(&mut self, e: &El, pcx: &Ctx, parent: &[String], atomic: bool) -> Bx {
        let mut cx = pcx.clone();
        self.apply(e, parent, &mut cx);
        if e.name == "svg" || e.has("vlist-s") || e.has("mathml") {
            return Bx::default();
        }
        if e.has("vlist-t") {
            return self.vlist(e, &cx);
        }
        let fs = cx.fs;
        let ib = atomic || e.any(INLINE_BLOCK);
        let block = e.has("overlay");

        // Margins, padding and borders.
        let (mut ml, mut mr) = (0., 0.);
        if e.has("root") && parent.iter().any(|c| c == "sqrt") {
            (ml, mr) = (0.277_777_8 * fs, -0.555_555_6 * fs);
        }
        if e.has("angl") {
            mr = 0.03889 * fs;
        }
        if e.has("cancel-lap") {
            (ml, mr) = (-0.2 * fs, -0.2 * fs);
        }
        if let Some(m) = e.style("margin") {
            let s = sides(m);
            ml = len(s[3], fs, cx.cb).unwrap_or(0.);
            mr = len(s[1], fs, cx.cb).unwrap_or(0.);
        }
        if let Some(v) = e.style("margin-left").and_then(|v| len(v, fs, cx.cb)) {
            ml = v;
        }
        if let Some(v) = e.style("margin-right").and_then(|v| len(v, fs, cx.cb)) {
            mr = v;
        }
        let (mut pl, mut pr) = (0., 0.);
        for (c, l, r) in [
            ("x-arrow-pad", 0.5, 0.5),
            ("cd-arrow-pad", 0.27778, 0.55556),
            ("boxpad", 0.3, 0.3),
            ("cancel-pad", 0.2, 0.2),
            ("anglpad", 0.03889, 0.03889),
        ] {
            if e.has(c) {
                (pl, pr) = (l * fs, r * fs);
            }
        }
        if let Some(v) = e.style("padding-left").and_then(|v| len(v, fs, cx.cb)) {
            pl = v;
        }
        if let Some(v) = e.style("padding-right").and_then(|v| len(v, fs, cx.cb)) {
            pr = v;
        }
        // [top, right, bottom, left] widths and styles (0 none, 1 solid, 2 dashed).
        let mut bw = [0f32; 4];
        let mut bs = [0u8; 4];
        let mut border_box = false;
        if e.any(&["frac-line", "overline-line", "underline-line", "hline"]) {
            bs[2] = 1;
        }
        if e.has("hdashline") {
            bs[2] = 2;
        }
        if e.has("rule") {
            bs = [1; 4];
        }
        if e.any(&["fbox", "fcolorbox"]) {
            bw = [0.04 * fs; 4];
            bs = [1; 4];
            border_box = true;
        }
        if e.has("sout") {
            bw[2] = 0.08 * fs;
            bs[2] = 1;
        }
        if e.has("angl") {
            bw[0] = 0.049 * fs;
            bw[1] = 0.049 * fs;
            bs[0] = 1;
            bs[1] = 1;
            border_box = true;
        }
        let style_of = |v: &str| match v {
            "solid" => 1,
            "dashed" => 2,
            _ => 0,
        };
        if let Some(v) = e.style("border-style") {
            let s = sides(v);
            for i in 0..4 {
                bs[i] = style_of(s[i]);
            }
        }
        if let Some(v) = e.style("border-width") {
            let s = sides(v);
            for i in 0..4 {
                bw[i] = len(s[i], fs, None).unwrap_or(0.);
            }
        }
        let mut bcolor = cx.color;
        if let Some(c) = e.style("border-color").and_then(css_color) {
            bcolor = c;
        }
        for (i, side) in ["top", "right", "bottom", "left"].iter().enumerate() {
            if let Some(v) = e.style(&format!("border-{side}-width")).and_then(|v| len(v, fs, None)) {
                bw[i] = v;
            }
            if let Some(v) = e.style(&format!("border-{side}-style")) {
                bs[i] = style_of(v);
            }
        }
        for i in 0..4 {
            bw[i] = if bs[i] == 0 { 0. } else { snap(bw[i]) };
        }

        // In-flow children.
        let lap = e.any(&["llap", "rlap", "clap"]);
        let mut flow = Bx::default();
        if e.has("eqn-num") {
            // `.eqn-num::before { counter-increment: katexEqnNo; content: "(" counter(katexEqnNo) ")" }`
            self.eqno += 1;
            self.text(&format!("({})", self.eqno), &cx, &mut flow);
        }
        for k in &e.kids {
            match k {
                N::Text(t) => self.text(t, &cx, &mut flow),
                N::El(c) => {
                    if c.name == "svg" || c.any(ABS) || (lap && c.has("inner")) {
                        continue;
                    }
                    let b = self.el(c, &cx, &e.class, false);
                    let is_block = c.has("overlay");
                    let lines = flow.lines;
                    flow.append(b);
                    if is_block {
                        flow.lines = lines;
                    }
                }
            }
        }
        let content_lines = flow.lines;
        if lap && let Some(inner) = e.els().find(|c| c.has("inner")) {
            let mut b = self.el(inner, &cx, &e.class, false);
            let dx = if e.has("llap") {
                -b.w
            } else if e.has("clap") {
                -b.w / 2.
            } else {
                0.
            };
            for it in &mut b.items {
                it.shift(flow.w + dx, 0.);
            }
            flow.items.append(&mut b.items);
            flow.h = flow.h.max(b.h);
            flow.d = flow.d.max(b.d);
        }

        // Width.
        let mut pct = flow.pct;
        let mut cw = flow.w;
        if ib || block {
            let hpad = pl + pr + bw[1] + bw[3];
            let mut width = e.style("width").and_then(|v| len(v, fs, cx.cb));
            if e.style("width").is_some_and(|v| v.ends_with('%')) && cx.cb.is_none() {
                pct = true;
            }
            if width.is_none() && e.any(FULL_WIDTH) {
                match cx.cb {
                    Some(w) => width = Some(if border_box { w } else { w - hpad }),
                    None => pct = true,
                }
            }
            if e.has("thinbox") {
                width = Some(0.);
            }
            if e.has("nulldelimiter") {
                width = Some(0.12 * fs);
            }
            if let Some(w) = width {
                cw = if border_box && !e.any(FULL_WIDTH) { w - hpad } else { w };
                if border_box && e.any(FULL_WIDTH) {
                    cw = w - hpad;
                }
            } else if pct && e.any(FULL_WIDTH) {
                cw = 0.;
            }
            let mut minw = e.style("min-width").and_then(|v| len(v, fs, cx.cb)).unwrap_or(0.);
            if e.has("vertical-separator") {
                minw = minw.max(1.);
            }
            cw = cw.max(minw);
        }
        if e.has("accent-body") && !e.has("accent-full") {
            cw = 0.;
        }
        if lap || (e.has("mtr-glue") && ib) {
            cw = 0.;
        }

        // Vertical extent.
        let (mut top, mut bottom) = if (ib || block) && !content_lines {
            let mut hgt = e.style("height").and_then(|v| len(v, fs, None)).unwrap_or(flow.h);
            if e.any(&["frac-line", "overline-line", "underline-line", "hline", "hdashline", "rule"]) {
                hgt = hgt.max(1.);
            }
            let total = if border_box { hgt.max(bw[0] + bw[2]) } else { hgt + bw[0] + bw[2] };
            (-total, 0.)
        } else {
            (-flow.h, flow.d)
        };
        let x0 = ml;
        let x1 = ml + bw[3] + pl + cw + pr + bw[1];
        let content_x = ml + bw[3] + pl;
        let mut items = vec![];
        if let Some(c) = e.style("background-color").and_then(css_color) {
            items.push(Item::Rect { x: x0, y: top, w: x1 - x0, h: bottom - top, color: c });
        }
        for mut it in flow.items {
            it.shift(content_x, 0.);
            items.push(it);
        }
        self.borders(&mut items, [x0, top, x1, bottom], bw, bs, bcolor);

        // Absolutely positioned children: SVGs and stretchy pieces.
        let pad_box = [ml + bw[3], top + bw[0], x1 - bw[1], bottom - bw[2]];
        let clip = e.any(&["hide-tail", "stretchy"]).then_some(pad_box);
        let own_h = e.style("height").and_then(|v| len(v, fs, None));
        for c in e.els() {
            if c.name == "svg" {
                self.svg(c, &cx, pad_box, own_h, clip, &mut items);
            } else if c.any(ABS) {
                let pw = pad_box[2] - pad_box[0];
                let (f, at) = if c.has("halfarrow-left") {
                    (0.502, 0.)
                } else if c.has("halfarrow-right") {
                    (0.502, 1.)
                } else if c.has("brace-left") {
                    (0.251, 0.)
                } else if c.has("brace-center") {
                    (0.5, -1.)
                } else {
                    (0.251, 1.)
                };
                let w = pw * f;
                let x = if at < 0. { pad_box[0] + pw * 0.25 } else { pad_box[0] + at * (pw - w) };
                let mut ccx = cx.clone();
                self.apply(c, &e.class, &mut ccx);
                let h = c.style("height").and_then(|v| len(v, ccx.fs, None));
                let r = [x, pad_box[1], x + w, pad_box[1] + h.unwrap_or(0.)];
                for s in c.els().filter(|s| s.name == "svg") {
                    self.svg(s, &ccx, r, h, Some(r), &mut items);
                }
            }
        }

        // vertical-align shifts the baseline; position: relative offsets paint only.
        let mut v = e.style("vertical-align").and_then(|v| len(v, fs, None)).unwrap_or(0.);
        if !ib && !block && e.style("vertical-align").is_none() {
            v = 0.;
        }
        let dx = e
            .style("left")
            .and_then(|v| len(v, fs, cx.cb))
            .or_else(|| e.style("right").and_then(|v| len(v, fs, cx.cb)).map(|r| -r))
            .unwrap_or(0.);
        let dy = e
            .style("top")
            .and_then(|v| len(v, fs, None))
            .or_else(|| e.style("bottom").and_then(|v| len(v, fs, None)).map(|b| -b))
            .unwrap_or(0.);
        if v != 0. || dx != 0. || dy != 0. {
            for it in &mut items {
                it.shift(dx, dy - v);
            }
        }
        top -= v;
        bottom -= v;
        Bx {
            w: x1 + mr,
            h: -top,
            d: bottom,
            items,
            lines: ib || block || content_lines || ml != 0. || mr != 0. || pl != 0. || pr != 0.,
            pct,
        }
    }

    fn borders(&self, items: &mut Vec<Item>, [x0, y0, x1, y1]: [f32; 4], bw: [f32; 4], bs: [u8; 4], color: Hsla) {
        let w = x1 - x0;
        let h = y1 - y0;
        let rects = [
            (x0, y0, w, bw[0]),
            (x1 - bw[1], y0, bw[1], h),
            (x0, y1 - bw[2], w, bw[2]),
            (x0, y0, bw[3], h),
        ];
        for (i, (x, y, rw, rh)) in rects.into_iter().enumerate() {
            if bw[i] <= 0. || rw <= 0. || rh <= 0. {
                continue;
            }
            if bs[i] == 2 {
                // Dashes three border-widths long.
                let horizontal = i % 2 == 0;
                let total = if horizontal { rw } else { rh };
                let dash = (bw[i] * 3.).max(2.);
                let mut t = 0.;
                while t < total {
                    let l = dash.min(total - t);
                    let (dx, dy, dw, dh) = if horizontal { (t, 0., l, rh) } else { (0., t, rw, l) };
                    items.push(Item::Rect { x: x + dx, y: y + dy, w: dw, h: dh, color });
                    t += dash * 2.;
                }
            } else {
                items.push(Item::Rect { x, y, w: rw, h: rh, color });
            }
        }
    }

    /// `.katex svg { position: absolute; width: 100%; height: inherit }`.
    fn svg(&self, s: &El, cx: &Ctx, cb: [f32; 4], parent_h: Option<f32>, clip: Option<[f32; 4]>, items: &mut Vec<Item>) {
        let fs = cx.fs;
        let cbw = cb[2] - cb[0];
        let w = s.style("width").and_then(|v| len(v, fs, Some(cbw))).unwrap_or(cbw);
        let h = parent_h.or_else(|| s.attr("height").and_then(|v| len(v, fs, None))).unwrap_or(cb[3] - cb[1]);
        if w <= 0. || h <= 0. {
            return;
        }
        let (x, y) = (cb[0], cb[1]);
        let vb: Option<Vec<f32>> = s.attr("viewBox").map(|v| v.split_whitespace().filter_map(|n| n.parse().ok()).collect());
        let mut body = String::new();
        for k in s.els() {
            body.push('<');
            body.push_str(&k.name);
            for (a, v) in &k.attrs {
                // Lengths in em (stroke widths) resolve against the svg font size.
                let v = match len(v, fs, None) {
                    Some(n) if v.ends_with("em") => format!("{n}"),
                    _ => v.clone(),
                };
                body.push_str(&format!(" {a}=\"{}\"", v.replace('"', "&quot;")));
            }
            if k.name == "line" {
                body.push_str(" stroke=\"black\"");
            }
            body.push_str("/>");
        }
        let (rect, view) = match vb {
            Some(vb) if vb.len() == 4 && vb[2] > 0. && vb[3] > 0. => {
                let par = s.attr("preserveAspectRatio").unwrap_or("xMidYMid meet");
                let (sx, sy, ox, oy) = if par.starts_with("none") {
                    (w / vb[2], h / vb[3], 0., 0.)
                } else {
                    let sc = if par.contains("slice") { (w / vb[2]).max(h / vb[3]) } else { (w / vb[2]).min(h / vb[3]) };
                    let fx = if par.contains("xMin") {
                        0.
                    } else if par.contains("xMax") {
                        1.
                    } else {
                        0.5
                    };
                    let fy = if par.contains("YMin") {
                        0.
                    } else if par.contains("YMax") {
                        1.
                    } else {
                        0.5
                    };
                    (sc, sc, fx * (w - vb[2] * sc), fy * (h - vb[3] * sc))
                };
                // Visible part: the svg viewport, clipped by an overflow:hidden parent.
                let mut r = [x, y, x + w, y + h];
                if let Some(c) = clip {
                    r = [r[0].max(c[0]), r[1].max(c[1]), r[2].min(c[2]), r[3].min(c[3])];
                }
                if r[2] - r[0] <= 0.01 || r[3] - r[1] <= 0.01 {
                    return;
                }
                let ux = vb[0] + (r[0] - x - ox) / sx;
                let uy = vb[1] + (r[1] - y - oy) / sy;
                let view = [ux, uy, (r[2] - r[0]) / sx, (r[3] - r[1]) / sy];
                (r, view)
            }
            _ => ([x, y, x + w, y + h], [0., 0., w, h]),
        };
        let markup = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"{} {} {} {}\" preserveAspectRatio=\"none\">{body}</svg>",
            rect[2] - rect[0],
            rect[3] - rect[1],
            view[0],
            view[1],
            view[2],
            view[3],
        );
        items.push(Item::Svg {
            x: rect[0],
            y: rect[1],
            w: rect[2] - rect[0],
            h: rect[3] - rect[1],
            path: register_svg(markup),
            color: cx.color,
        });
    }

    /// `.vlist-t`: an inline-table whose first row's cell (`.vlist`, bottom
    /// aligned, height H) holds zero-height blocks shifted by `top`; each
    /// block's baseline sits `top + pstrut` below the table baseline.
    fn vlist(&mut self, e: &El, cx: &Ctx) -> Bx {
        let rows: Vec<&El> = e.els().filter(|r| r.has("vlist-r")).collect();
        fn cell(r: &El, fs: f32) -> Option<(Option<f32>, &El)> {
            r.els().find(|c| c.has("vlist")).map(|c| (c.style("height").and_then(|v| len(v, fs, None)), c))
        }
        let mut out = Bx { lines: true, ..Default::default() };
        let Some((h, cell1)) = rows.first().and_then(|r| cell(r, cx.fs)) else { return out };
        out.h = h.unwrap_or(0.);
        out.d = rows.get(1).and_then(|r| cell(r, cx.fs)).and_then(|(d, _)| d).unwrap_or(0.);
        struct Entry {
            ml: f32,
            mr: f32,
            y: f32,
            cx: Ctx,
            b: Bx,
        }
        let lay = |eng: &mut Self, w: &El, wcx: &Ctx| -> (f32, Bx) {
            let mut flow = Bx::default();
            let mut pstrut = 0.;
            for k in &w.kids {
                match k {
                    N::El(c) if c.has("pstrut") => {
                        pstrut = c.style("height").and_then(|v| len(v, wcx.fs, None)).unwrap_or(0.);
                    }
                    N::El(c) => {
                        let b = eng.el(c, wcx, &w.class, true);
                        flow.append(b);
                    }
                    N::Text(t) => eng.text(t, wcx, &mut flow),
                }
            }
            (pstrut, flow)
        };
        let mut entries = vec![];
        let wrappers: Vec<&El> = cell1.els().collect();
        for w in &wrappers {
            let mut wcx = cx.clone();
            self.apply(w, &cell1.class, &mut wcx);
            let fs = wcx.fs;
            let top = w.style("top").and_then(|v| len(v, fs, None)).unwrap_or(0.);
            let ml = w.style("margin-left").and_then(|v| len(v, fs, None)).unwrap_or(0.);
            let mr = w.style("margin-right").and_then(|v| len(v, fs, None)).unwrap_or(0.);
            let (pstrut, b) = lay(self, w, &wcx);
            entries.push(Entry { ml, mr, y: top + pstrut, cx: wcx, b });
        }
        let cell_w = entries.iter().fold(0f32, |a, en| a.max(en.ml + en.b.w + en.mr));
        for (en, w) in entries.iter_mut().zip(&wrappers) {
            if en.b.pct {
                en.cx.cb = Some(cell_w - en.ml - en.mr);
                en.b = lay(self, w, &en.cx).1;
            }
            let x = en.ml + en.cx.align * (cell_w - en.ml - en.mr - en.b.w);
            for mut it in std::mem::take(&mut en.b.items) {
                it.shift(x, en.y);
                out.items.push(it);
            }
        }
        out.w = cell_w;
        out
    }
}

#[cfg(test)]
mod tests {
    use super::{is_block, parse, render_html};

    #[test]
    fn block_detection() {
        assert!(is_block("\\begin{align}a&=b\\end{align}"));
        assert!(is_block("\\begin\n{CD}A @>>> B\\end{CD}"));
        assert!(is_block("x\\tag{1}"));
        assert!(!is_block("\\begin{align*}a\\end{align*}"));
        assert!(!is_block("\\frac{a}{b}"));
    }

    #[test]
    fn katex_html_parses() {
        let html = render_html("\\frac{a}{b}+\\sqrt{x}", false).unwrap();
        let root = parse(&html).unwrap();
        assert!(root.find("katex-html").is_some());
        assert!(root.find("frac-line").is_some());
        assert!(render_html("\\frac{a", false).is_none());
    }
}
