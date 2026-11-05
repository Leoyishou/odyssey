//! Clipboard import: HTML (other apps, web pages) and Markdown to DocxXML
//! blocks. Reading the pasteboard itself is `clipboard.rs`.

use crate::model::*;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{Handle, NodeData, RcDom};

pub fn html_to_blocks(html: &str) -> Vec<Block> {
    let Ok(dom) = html5ever::parse_document(RcDom::default(), Default::default()).from_utf8().read_from(&mut html.as_bytes()) else {
        return vec![];
    };
    let root = find(&dom.document, "body").unwrap_or_else(|| dom.document.clone());
    let mut out = vec![];
    container(&root, &Ctx::default(), &mut out);
    // Markdown typed into another app's rich text still converts (inline).
    if !has_class(&dom.document, "lark-doc") {
        reparse_markdown(&mut out);
    }
    out
}

fn has_class(n: &Handle, class: &str) -> bool {
    has_own_class(n, class) || n.children.borrow().iter().any(|c| has_class(c, class))
}

fn has_own_class(n: &Handle, class: &str) -> bool {
    attr(n, "class").is_some_and(|c| c.split_whitespace().any(|c| c == class))
}

/// A callout: colours snap to the nearest swatch, the emoji id
/// (`data-emoji-id`) to its character.
fn callout(n: &Handle, ctx: &Ctx) -> Block {
    let st = style(n);
    let get = |k: &str| st.iter().find(|(p, _)| p == k).and_then(|(_, v)| parse_color(v));
    let near = |c: (u8, u8, u8, f32), names: &mut dyn Iterator<Item = &str>, f: fn(&str) -> Option<crate::palette::Rgba>| {
        names
            .filter_map(|n| Some((n, f(n)?)))
            .min_by_key(|(_, p)| {
                let d = |a: u8, b: u8| (a as i32 - b as i32).pow(2);
                d(c.0, p.r) + d(c.1, p.g) + d(c.2, p.b)
            })
            .map(|(n, _)| ColorName(n.into()))
    };
    let bg = get("background-color").and_then(|c| near(c, &mut crate::palette::CALLOUT_BG_COLORS.iter().flatten().copied(), crate::palette::callout_bg));
    let border = get("border-color").and_then(|c| near(c, &mut crate::palette::HUES.iter().copied(), crate::palette::callout_border));
    let emoji = n
        .parent
        .take()
        .and_then(|w| {
            let p = w.upgrade();
            n.parent.set(Some(w));
            p
        })
        .and_then(|p| attr(&p, "data-emoji-id"))
        .and_then(|id| crate::emoji::by_id(&id))
        .unwrap_or("💡");
    let mut kids = vec![];
    container(n, ctx, &mut kids);
    if kids.is_empty() {
        kids.push(Block::text(""));
    }
    Block::new(BlockKind::Callout { emoji: emoji.into(), bg, border, text_color: None }, RichText::default()).with_children(kids)
}

/// Unmarked runs of text blocks are read as inline Markdown.
fn reparse_markdown(blocks: &mut [Block]) {
    for b in blocks {
        reparse_markdown(&mut b.children);
        if let BlockKind::Table(t) = &mut b.kind {
            for cell in t.rows.iter_mut().flatten() {
                reparse_markdown(&mut cell.blocks);
            }
        }
        if !b.kind.has_text() || matches!(b.kind, BlockKind::Code { .. }) {
            continue;
        }
        let mut out = RichText::default();
        for (r, m) in b.text.spans() {
            if *m == Marks::default() {
                out.append(&md_inline(&b.text.text[r]));
            } else {
                out.append(&b.text.slice(r));
            }
        }
        b.text = out;
    }
}

/// Content that never becomes document text.
const SKIP: &[&str] = &[
    "head", "title", "meta", "link", "style", "script", "noscript", "template", "iframe", "object", "embed", "video", "audio",
    "canvas", "svg", "math", "button", "select", "option", "textarea", "input", "map", "area", "source", "track",
];

/// Elements that break the inline flow (their content forms new paragraphs).
const BLOCK: &[&str] = &[
    "p", "div", "section", "article", "header", "footer", "main", "aside", "nav", "figure", "figcaption", "center", "address",
    "dl", "dt", "dd", "form", "fieldset", "legend", "details", "summary", "hgroup", "html", "body", "tbody", "thead", "tfoot", "tr",
    "td", "th", "caption",
];

#[derive(Clone, Default)]
struct Ctx {
    marks: Marks,
    align: Option<Align>,
}

fn tag(n: &Handle) -> Option<String> {
    match &n.data {
        NodeData::Element { name, .. } => Some(name.local.to_string().to_ascii_lowercase()),
        _ => None,
    }
}

fn attr(n: &Handle, key: &str) -> Option<String> {
    match &n.data {
        NodeData::Element { attrs, .. } => attrs.borrow().iter().find(|a| &*a.name.local == key).map(|a| a.value.to_string()),
        _ => None,
    }
}

fn find(n: &Handle, name: &str) -> Option<Handle> {
    if tag(n).as_deref() == Some(name) {
        return Some(n.clone());
    }
    n.children.borrow().iter().find_map(|c| find(c, name))
}

/// Declarations of the inline `style` attribute, lowercased.
fn style(n: &Handle) -> Vec<(String, String)> {
    attr(n, "style")
        .unwrap_or_default()
        .split(';')
        .filter_map(|d| {
            let (k, v) = d.split_once(':')?;
            Some((k.trim().to_ascii_lowercase(), v.trim().to_ascii_lowercase()))
        })
        .collect()
}

fn font_px(st: &[(String, String)]) -> Option<f32> {
    let v = &st.iter().find(|(k, _)| k == "font-size")?.1;
    if let Some(n) = v.strip_suffix("px") {
        return n.trim().parse().ok();
    }
    v.strip_suffix("pt").and_then(|n| n.trim().parse::<f32>().ok()).map(|pt| pt * 4. / 3.)
}

fn hidden(st: &[(String, String)]) -> bool {
    st.iter().any(|(k, v)| (k == "display" && v.starts_with("none")) || (k == "visibility" && v == "hidden"))
}

fn align_of(n: &Handle, st: &[(String, String)]) -> Option<Align> {
    let v = st.iter().find(|(k, _)| k == "text-align").map(|(_, v)| v.clone()).or_else(|| attr(n, "align"))?;
    match v.trim().to_ascii_lowercase().as_str() {
        "center" | "-webkit-center" => Some(Align::Center),
        "right" | "end" | "-webkit-right" => Some(Align::Right),
        "left" | "start" | "justify" => Some(Align::Left),
        _ => None,
    }
}

/// `ctx` with the marks an inline element and its style add.
fn inline_ctx(n: &Handle, t: &str, st: &[(String, String)], ctx: &Ctx) -> Ctx {
    let mut c = ctx.clone();
    let m = &mut c.marks;
    match t {
        "b" | "strong" => m.bold = true,
        "i" | "em" | "cite" | "dfn" | "var" => m.italic = true,
        "u" | "ins" => m.underline = true,
        "s" | "strike" | "del" => m.strike = true,
        "code" | "kbd" | "samp" | "tt" => m.code = true,
        "a" => {
            if let Some(h) = attr(n, "href").map(|h| h.trim().to_string()).filter(|h| link_ok(h)) {
                m.link = Some(h);
            }
        }
        _ => {}
    }
    for (k, v) in st {
        match k.as_str() {
            "font-weight" => {
                m.bold = match v.as_str() {
                    "bold" | "bolder" => true,
                    "normal" | "lighter" => false,
                    n => n.parse::<u32>().map_or(m.bold, |w| w >= 700),
                }
            }
            "font-style" => m.italic = v.starts_with("italic") || v.starts_with("oblique"),
            "text-decoration" | "text-decoration-line" => {
                m.underline |= v.contains("underline");
                m.strike |= v.contains("line-through");
            }
            // Only colors of the palette survive.
            "color" => {
                if let Some(n) = parse_color(v).and_then(palette_text) {
                    m.text_color = n;
                }
            }
            "background-color" | "background" => {
                if let Some(n) = parse_color(v).and_then(palette_bg) {
                    m.bg_color = n;
                }
            }
            _ => {}
        }
    }
    c
}

/// `#rgb`, `#rrggbb`, `rgb(…)` / `rgba(…)` → (r, g, b, alpha).
fn parse_color(v: &str) -> Option<(u8, u8, u8, f32)> {
    let v = v.trim().trim_end_matches("!important").trim();
    if let Some(h) = v.strip_prefix('#') {
        let x = |s: &str| u8::from_str_radix(s, 16).ok();
        return match h.len() {
            3 => {
                let d: Vec<u8> = h.chars().map(|c| x(&c.to_string()).map(|n| n * 17)).collect::<Option<_>>()?;
                Some((d[0], d[1], d[2], 1.))
            }
            6 => Some((x(&h[0..2])?, x(&h[2..4])?, x(&h[4..6])?, 1.)),
            _ => None,
        };
    }
    let inner = v.strip_prefix("rgba(").or_else(|| v.strip_prefix("rgb("))?.strip_suffix(')')?;
    let parts: Vec<&str> = inner.split([',', ' ', '/']).filter(|p| !p.is_empty()).collect();
    let n = |i: usize| parts.get(i)?.trim().parse::<f32>().ok();
    Some((n(0)? as u8, n(1)? as u8, n(2)? as u8, parts.get(3).map_or(Some(1.), |_| n(3))?))
}

fn same(a: (u8, u8, u8, f32), b: (u8, u8, u8, f32)) -> bool {
    a.0 == b.0 && a.1 == b.1 && a.2 == b.2 && (a.3 - b.3).abs() < 0.02
}

/// Some(None) is the default text color.
fn palette_text(c: (u8, u8, u8, f32)) -> Option<Option<ColorName>> {
    crate::palette::FONT_COLORS
        .iter()
        .find(|(_, v)| same(c, ((v >> 16) as u8, (v >> 8) as u8, *v as u8, 1.)))
        .map(|(n, _)| n.map(|n| ColorName(n.into())))
}

fn palette_bg(c: (u8, u8, u8, f32)) -> Option<Option<ColorName>> {
    if c.3 == 0. {
        return Some(None);
    }
    crate::palette::BG_COLORS
        .iter()
        .flatten()
        .find(|n| crate::palette::bg_color(n).is_some_and(|p| same(c, (p.r, p.g, p.b, p.a))))
        .map(|n| Some(ColorName(n.to_string())))
}

fn link_ok(h: &str) -> bool {
    let l = h.to_ascii_lowercase();
    ["http://", "https://", "mailto:", "ftp://"].iter().any(|p| l.starts_with(p))
}

fn container(n: &Handle, ctx: &Ctx, out: &mut Vec<Block>) {
    let mut para = RichText::default();
    for c in n.children.borrow().iter() {
        node(c, ctx, out, &mut para);
    }
    flush(&mut para, ctx, out);
}

fn node(n: &Handle, ctx: &Ctx, out: &mut Vec<Block>, para: &mut RichText) {
    match &n.data {
        NodeData::Text { contents } => push_text(para, &contents.borrow(), &ctx.marks),
        NodeData::Element { .. } => {
            let t = tag(n).unwrap_or_default();
            let st = style(n);
            if SKIP.contains(&t.as_str()) || hidden(&st) {
                return;
            }
            let mut bctx = ctx.clone();
            if let Some(a) = align_of(n, &st) {
                bctx.align = Some(a);
            }
            match t.as_str() {
                "br" => {
                    trim_end_spaces(para);
                    para.push("\n", ctx.marks.clone());
                }
                // Superscript / subscript text is wrapped in parentheses.
                "sup" | "sub" => {
                    let c = inline_ctx(n, &t, &st, ctx);
                    para.push("(", c.marks.clone());
                    for ch in n.children.borrow().iter() {
                        node(ch, &c, out, para);
                    }
                    para.push(")", c.marks.clone());
                }
                "img" => {
                    flush(para, ctx, out);
                    out.extend(image(n));
                }
                "hr" => {
                    flush(para, ctx, out);
                    out.push(Block::new(BlockKind::Divider, RichText::default()));
                }
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                    flush(para, ctx, out);
                    let level = t[1..].parse().unwrap_or(1);
                    let mut kids = vec![];
                    container(n, &bctx, &mut kids);
                    let mut text = RichText::default();
                    for (i, k) in kids.iter().filter(|k| k.kind.has_text()).enumerate() {
                        if i > 0 {
                            text.push("\n", Marks::default());
                        }
                        text.append(&k.text);
                    }
                    if !text.is_empty() {
                        let mut b = Block::new(BlockKind::Heading { level, seq_auto: false }, text);
                        b.align = bctx.align.unwrap_or_default();
                        out.push(b);
                    }
                    out.extend(kids.into_iter().filter(|k| !k.kind.has_text()));
                }
                "ul" | "ol" | "menu" => {
                    flush(para, ctx, out);
                    list(n, &bctx, out, t == "ol");
                }
                "li" => {
                    flush(para, ctx, out);
                    out.push(item(n, &bctx, false, None));
                }
                "blockquote" => {
                    flush(para, ctx, out);
                    let mut kids = vec![];
                    container(n, &bctx, &mut kids);
                    if !kids.is_empty() {
                        out.push(Block::new(BlockKind::Quote, RichText::default()).with_children(kids));
                    }
                }
                "pre" if code_lang(n).is_some() || find(n, "code").is_some() => {
                    flush(para, ctx, out);
                    out.push(code(n));
                }
                // A `<pre>` that isn't code is text with its line breaks.
                "pre" => {
                    flush(para, ctx, out);
                    let mut text = String::new();
                    raw_text(n, &mut text);
                    for line in text.replace("\r\n", "\n").trim_end_matches('\n').split('\n') {
                        let mut b = Block::new(BlockKind::Text, RichText::default());
                        b.text.push(&line.replace('\t', " "), ctx.marks.clone());
                        b.align = bctx.align.unwrap_or_default();
                        out.push(b);
                    }
                }
                "table" => {
                    flush(para, ctx, out);
                    out.extend(table(n, &bctx));
                }
                // 高亮块: `.callout-block` carries the colours, the container
                // above it the emoji id.
                _ if has_own_class(n, "callout-block") => {
                    flush(para, ctx, out);
                    out.push(callout(n, &bctx));
                }
                _ if BLOCK.contains(&t.as_str()) => {
                    flush(para, ctx, out);
                    let before = out.len();
                    container(n, &bctx, out);
                    // Big text (a styled title) reads as a heading 1.
                    if font_px(&st).is_some_and(|px| px >= 20.) {
                        for b in &mut out[before..] {
                            if matches!(b.kind, BlockKind::Text) && b.children.is_empty() {
                                b.kind = BlockKind::Heading { level: 1, seq_auto: false };
                            }
                        }
                    }
                    // An empty paragraph is a blank line.
                    if out.len() == before && t == "p" {
                        let mut b = Block::text("");
                        b.align = bctx.align.unwrap_or_default();
                        out.push(b);
                    }
                }
                _ => {
                    let c = inline_ctx(n, &t, &st, ctx);
                    for ch in n.children.borrow().iter() {
                        node(ch, &c, out, para);
                    }
                }
            }
        }
        _ => {
            for ch in n.children.borrow().iter() {
                node(ch, ctx, out, para);
            }
        }
    }
}

/// HTML whitespace collapsing: runs become one space, none at line start.
fn push_text(para: &mut RichText, s: &str, marks: &Marks) {
    let mut t = String::with_capacity(s.len());
    let mut space = para.text.is_empty() || para.text.ends_with([' ', '\n']);
    for c in s.chars() {
        if c.is_ascii_whitespace() {
            if !space {
                t.push(' ');
                space = true;
            }
        } else {
            t.push(if c == '\u{a0}' { ' ' } else { c });
            space = false;
        }
    }
    if !t.is_empty() {
        para.push(&t, marks.clone());
    }
}

fn trim_end_spaces(t: &mut RichText) {
    let end = t.text.trim_end_matches(' ').len();
    if end < t.text.len() {
        *t = t.slice(0..end);
    }
}

fn flush(para: &mut RichText, ctx: &Ctx, out: &mut Vec<Block>) {
    let t = std::mem::take(para);
    let start = t.text.len() - t.text.trim_start_matches([' ', '\n']).len();
    let end = t.text.trim_end_matches([' ', '\n']).len();
    if end <= start {
        return;
    }
    // A `<br>` starts a new block.
    let t = t.slice(start..end);
    let mut at = 0;
    for line in t.text.split('\n') {
        let mut b = Block::new(BlockKind::Text, t.slice(at..at + line.len()));
        b.align = ctx.align.unwrap_or_default();
        out.push(b);
        at += line.len() + 1;
    }
}

fn list(n: &Handle, ctx: &Ctx, out: &mut Vec<Block>, ordered: bool) {
    let start: Option<u32> = attr(n, "start").and_then(|s| s.trim().parse().ok()).filter(|&s| s != 1);
    let mut items: Vec<Block> = vec![];
    for c in n.children.borrow().iter() {
        match tag(c).as_deref() {
            Some("li") => {
                let seq = if items.is_empty() { start } else { None };
                items.push(item(c, ctx, ordered, seq));
            }
            // A list nested straight in a list belongs to the item before it.
            Some(t @ ("ul" | "ol")) => {
                let mut kids = vec![];
                list(c, ctx, &mut kids, t == "ol");
                match items.last_mut() {
                    Some(last) => last.children.extend(kids),
                    None => items.extend(kids),
                }
            }
            Some(_) => {
                let mut kids = vec![];
                container(c, ctx, &mut kids);
                items.extend(kids);
            }
            None => {}
        }
    }
    out.extend(items);
}

/// One `<li>`: its first paragraph is the item's text, the rest children.
fn item(li: &Handle, ctx: &Ctx, ordered: bool, seq: Option<u32>) -> Block {
    let kind = match checkbox(li) {
        Some(done) => BlockKind::Todo { done },
        None if ordered => BlockKind::Ordered { seq: attr(li, "value").and_then(|v| v.trim().parse().ok()).or(seq) },
        None => BlockKind::Bullet,
    };
    let mut kids = vec![];
    container(li, ctx, &mut kids);
    let text = match kids.first() {
        Some(b) if matches!(b.kind, BlockKind::Text) && b.children.is_empty() => kids.remove(0).text,
        _ => RichText::default(),
    };
    Block::new(kind, text).with_children(kids)
}

/// `Some(checked)` when the item carries a checkbox of its own (GFM task lists).
fn checkbox(li: &Handle) -> Option<bool> {
    // `<li data-list="done">` (checked) / `"check"` (inferred, unchecked).
    match attr(li, "data-list").as_deref() {
        Some("done") => return Some(true),
        Some("check" | "undone" | "todo") => return Some(false),
        _ => {}
    }
    fn walk(n: &Handle) -> Option<bool> {
        for c in n.children.borrow().iter() {
            match tag(c).as_deref() {
                Some("ul" | "ol") => {}
                Some("input") if attr(c, "type").is_some_and(|t| t.eq_ignore_ascii_case("checkbox")) => {
                    return Some(attr(c, "checked").is_some());
                }
                Some(_) => {
                    if let Some(v) = walk(c) {
                        return Some(v);
                    }
                }
                None => {}
            }
        }
        None
    }
    walk(li)
}

fn raw_text(n: &Handle, out: &mut String) {
    match &n.data {
        NodeData::Text { contents } => out.push_str(&contents.borrow().replace('\u{a0}', " ")),
        NodeData::Element { .. } => match tag(n).as_deref() {
            Some("br") => out.push('\n'),
            Some(t) if SKIP.contains(&t) => {}
            Some(t) => {
                let block = matches!(t, "div" | "p" | "li" | "tr");
                if block && !out.is_empty() && !out.ends_with('\n') {
                    out.push('\n');
                }
                for c in n.children.borrow().iter() {
                    raw_text(c, out);
                }
            }
            None => {}
        },
        _ => {
            for c in n.children.borrow().iter() {
                raw_text(c, out);
            }
        }
    }
}

fn code(pre: &Handle) -> Block {
    let mut text = String::new();
    raw_text(pre, &mut text);
    let text = text.replace("\r\n", "\n");
    let text = text.strip_suffix('\n').unwrap_or(&text).to_string();
    let lang = code_lang(pre).map_or(crate::lang::from_token(""), |l| crate::lang::resolve(&l).to_string());
    Block::new(BlockKind::Code { lang, caption: None, wrap: false }, RichText::plain(text))
}

/// Language from `language-x` / `lang-x` / `highlight-source-x` classes or a
/// `lang` attribute on the `<pre>`, its `<code>` or a wrapping element.
fn code_lang(pre: &Handle) -> Option<String> {
    let mut nodes = vec![pre.clone()];
    nodes.extend(pre.children.borrow().iter().filter(|c| tag(c).as_deref() == Some("code")).cloned());
    if let Some(p) = pre.parent.take() {
        if let Some(p) = p.upgrade() {
            nodes.push(p);
        }
        pre.parent.set(Some(p));
    }
    for n in &nodes {
        for key in ["data-lark-language", "data-lang", "data-language"] {
            if let Some(v) = attr(n, key).filter(|v| !v.trim().is_empty()) {
                return Some(v);
            }
        }
        let classes = attr(n, "class").unwrap_or_default();
        let classes: Vec<&str> = classes.split_whitespace().collect();
        if classes.contains(&"hljs")
            && let Some(l) = classes.iter().find(|c| **c != "hljs" && !c.starts_with("hljs-"))
        {
            return Some(l.to_string());
        }
        for c in classes {
            for p in ["language-", "lang-", "highlight-source-", "data-lark-language-"] {
                if let Some(l) = c.strip_prefix(p).filter(|l| !l.is_empty()) {
                    return Some(l.to_string());
                }
            }
        }
    }
    None
}

fn image(n: &Handle) -> Option<Block> {
    let src = attr(n, "src")?.trim().to_string();
    let l = src.to_ascii_lowercase();
    let src = if l.starts_with("http://") || l.starts_with("https://") || l.starts_with("data:image/") {
        src
    } else {
        let p = src.strip_prefix("file://")?;
        percent_decode(p)
    };
    let mut b = Block::new(
        BlockKind::Image { src_kind: "src".into(), src, width: None, height: None, caption: None, crop: None },
        RichText::default(),
    );
    b.align = Align::Center;
    Some(b)
}

fn percent_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%'
            && i + 2 < b.len()
            && s.is_char_boundary(i + 3)
            && let Ok(v) = u8::from_str_radix(&s[i + 1..i + 3], 16)
        {
            out.push(v);
            i += 3;
        } else {
            out.push(b[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn width_px(n: &Handle) -> Option<f32> {
    let st = style(n);
    let v = st.iter().find(|(k, _)| k == "width").map(|(_, v)| v.clone()).or_else(|| attr(n, "width"))?;
    let v = v.trim();
    v.strip_suffix("px").unwrap_or(v).trim().parse::<f32>().ok().filter(|w| *w > 0.)
}

/// Column widths from `<col>`s, else the first row's single-column cells.
fn col_widths(table: &Handle, trs: &[(Handle, bool)], ncols: usize) -> Vec<f32> {
    let mut w: Vec<Option<f32>> = vec![None; ncols];
    let mut cols = vec![];
    for c in table.children.borrow().iter() {
        match tag(c).as_deref() {
            Some("colgroup") => cols.extend(c.children.borrow().iter().filter(|x| tag(x).as_deref() == Some("col")).cloned()),
            Some("col") => cols.push(c.clone()),
            _ => {}
        }
    }
    let mut i = 0;
    for col in &cols {
        let span: usize = attr(col, "span").and_then(|s| s.parse().ok()).unwrap_or(1);
        for _ in 0..span {
            if let Some(x) = w.get_mut(i) {
                *x = width_px(col);
            }
            i += 1;
        }
    }
    if let Some((tr, _)) = trs.first() {
        let mut c = 0;
        for td in tr.children.borrow().iter().filter(|x| matches!(tag(x).as_deref(), Some("td" | "th"))) {
            let span: usize = attr(td, "colspan").and_then(|s| s.trim().parse().ok()).unwrap_or(1).max(1);
            if span == 1
                && let Some(x) = w.get_mut(c)
                && x.is_none()
            {
                *x = width_px(td);
            }
            c += span;
        }
    }
    w.into_iter().map(|x| x.map_or(Table::DEFAULT_WIDTH, |x| x.round().max(Table::MIN_WIDTH))).collect()
}

fn table(n: &Handle, ctx: &Ctx) -> Option<Block> {
    fn rows_of(n: &Handle, out: &mut Vec<(Handle, bool)>, head: bool) {
        for c in n.children.borrow().iter() {
            match tag(c).as_deref() {
                Some("thead") => rows_of(c, out, true),
                Some("tbody" | "tfoot") => rows_of(c, out, head),
                Some("tr") => out.push((c.clone(), head)),
                _ => {}
            }
        }
    }
    let mut trs = vec![];
    rows_of(n, &mut trs, false);
    let mut rows: Vec<Vec<TableCell>> = vec![];
    let mut taken: Vec<Vec<bool>> = vec![];
    let mut all_th = vec![];
    for (r, (tr, _)) in trs.iter().enumerate() {
        if rows.len() <= r {
            rows.resize_with(r + 1, Vec::new);
            taken.resize_with(r + 1, Vec::new);
        }
        let mut c = 0;
        let mut th = true;
        for td in tr.children.borrow().iter() {
            let t = tag(td).unwrap_or_default();
            if t != "td" && t != "th" {
                continue;
            }
            th &= t == "th";
            while taken[r].get(c).copied().unwrap_or(false) {
                c += 1;
            }
            let colspan: usize = attr(td, "colspan").and_then(|v| v.trim().parse().ok()).unwrap_or(1).clamp(1, 64);
            let rowspan: usize = attr(td, "rowspan").and_then(|v| v.trim().parse().ok()).unwrap_or(1).clamp(1, trs.len() - r);
            for rr in r..r + rowspan {
                if rows.len() <= rr {
                    rows.resize_with(rr + 1, Vec::new);
                    taken.resize_with(rr + 1, Vec::new);
                }
                if taken[rr].len() < c + colspan {
                    taken[rr].resize(c + colspan, false);
                }
                if rows[rr].len() < c + colspan {
                    rows[rr].resize_with(c + colspan, TableCell::covered);
                }
                for cc in c..c + colspan {
                    taken[rr][cc] = true;
                }
            }
            let mut blocks = vec![];
            container(td, &Ctx { align: align_of(td, &style(td)).or(ctx.align), ..Ctx::default() }, &mut blocks);
            // Cells hold no nested tables.
            blocks.retain(|b| !matches!(b.kind, BlockKind::Table(_)));
            if blocks.is_empty() {
                blocks.push(Block::text(""));
            }
            rows[r][c] = TableCell { blocks, colspan: colspan as u32, rowspan: rowspan as u32, ..TableCell::empty() };
            c += colspan;
        }
        all_th.push(th && c > 0);
    }
    let ncols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    if ncols == 0 {
        return None;
    }
    for (r, row) in rows.iter_mut().enumerate() {
        row.resize_with(ncols, TableCell::covered);
        for (c, cell) in row.iter_mut().enumerate() {
            if cell.is_covered() && !taken[r].get(c).copied().unwrap_or(false) {
                *cell = TableCell::empty();
            }
        }
    }
    let head = trs.iter().take_while(|(_, h)| *h).count();
    let header_rows = if head > 0 { head } else { usize::from(all_th.first().copied().unwrap_or(false)) };
    let mut t = Table { col_widths: col_widths(n, &trs, ncols), header_rows, header_col: false, rows };
    t.repair();
    Some(Block::new(BlockKind::Table(t), RichText::default()))
}

// ---- Markdown (plain-text paste, behavior-spec §2.5)

/// Parses Markdown into blocks with the same rules as the live shortcuts plus
/// fences, nesting by indentation, pipe tables, images and links.
pub fn markdown_to_blocks(text: &str, off: Off) -> Vec<Block> {
    let text = text.replace("\r\n", "\n").replace('\r', "\n");
    let lines: Vec<&str> = text.trim_end_matches('\n').split('\n').collect();
    md_blocks(&lines, off)
}

/// Inline rules only (pasting into a non-empty block): one block per line.
pub fn markdown_inline_lines(text: &str) -> Vec<Block> {
    let text = text.replace("\r\n", "\n").replace('\r', "\n");
    text.trim_end_matches('\n').split('\n').map(|l| Block::new(BlockKind::Text, md_inline(&tabs(l)))).collect()
}

/// Runs of tabs become one space.
fn tabs(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c != '\t' {
            out.push(c);
        } else if !out.ends_with('\t') {
            out.push('\t');
        }
    }
    out.replace('\t', " ")
}

/// Rules switched off by the paste target (a list item doesn't re-detect its
/// own kind, a quote no quotes, a callout no code or dividers).
#[derive(Clone, Copy, Default)]
pub struct Off {
    pub bullet: bool,
    pub ordered: bool,
    pub todo: bool,
    pub quote: bool,
    pub code_divider: bool,
    /// Importing a whole file: blank lines only separate blocks, and a line
    /// holding just `![alt](src)` is an image.
    pub file: bool,
}

fn indent_of(line: &str) -> usize {
    let mut n = 0;
    for c in line.chars() {
        match c {
            ' ' => n += 1,
            '\t' => n += 4 - n % 4,
            _ => break,
        }
    }
    n
}

/// `line` without its first `n` columns of indentation.
fn dedent(line: &str, n: usize) -> &str {
    let mut col = 0;
    for (i, c) in line.char_indices() {
        if col >= n {
            return &line[i..];
        }
        match c {
            ' ' => col += 1,
            '\t' => col += 4 - col % 4,
            _ => return &line[i..],
        }
    }
    ""
}

enum Marker {
    Bullet,
    Ordered(u32),
}

/// A list item line: (marker, content column, content).
fn list_item(line: &str) -> Option<(Marker, usize, &str)> {
    let ind = indent_of(line);
    let rest = line.trim_start_matches([' ', '\t']);
    let skip = line.len() - rest.len();
    let (marker, mlen) = if let Some(c) = rest.chars().next().filter(|c| matches!(c, '-' | '*' | '+')) {
        (Marker::Bullet, c.len_utf8())
    } else {
        let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
        if digits == 0 || digits > 9 {
            return None;
        }
        let tail = &rest[digits..];
        let d = tail.chars().next().filter(|c| matches!(c, '.' | ')' | '。'))?;
        (Marker::Ordered(rest[..digits].parse().ok()?), digits + d.len_utf8())
    };
    let after = &rest[mlen..];
    let content = after.strip_prefix([' ', '\t'])?;
    // `***` / `- - -` style rules are dividers, not items.
    if divider(rest) {
        return None;
    }
    let _ = skip;
    Some((marker, ind + mlen + 1, content))
}

fn divider(line: &str) -> bool {
    let t: String = line.trim().chars().filter(|c| *c != ' ').collect();
    matches!(t.as_str(), "---" | "***" | "___") || (t.len() > 3 && t.len() < 6 && t.chars().all(|c| c == '-'))
}

fn fence(line: &str) -> Option<(char, usize, &str)> {
    let t = line.trim_start();
    let c = t.chars().next().filter(|c| matches!(c, '`' | '~'))?;
    let n = t.chars().take_while(|&x| x == c).count();
    (n >= 3).then(|| (c, n, t[n..].trim()))
}

fn todo_prefix(s: &str) -> Option<(bool, &str)> {
    for (p, done) in [("[ ] ", false), ("[] ", false), ("【】 ", false), ("[x] ", true), ("[X] ", true), ("【x】 ", true)] {
        if let Some(r) = s.strip_prefix(p) {
            return Some((done, r));
        }
    }
    None
}

fn md_blocks(lines: &[&str], off: Off) -> Vec<Block> {
    let mut out: Vec<Block> = vec![];
    let mut i = 0;
    // Ordered items continue the previous item's numbering.
    let mut last_list = false;
    while i < lines.len() {
        let line = lines[i];
        // Blank lines are empty paragraphs (none before the first block).
        if line.trim().is_empty() {
            if !out.is_empty() && !off.file {
                out.push(Block::text(""));
            }
            last_list = false;
            i += 1;
            continue;
        }
        let ind = indent_of(line);
        let body = line.trim();
        // Fenced code (closed or running to the end).
        if let Some((fc, n, info)) = fence(line).filter(|_| !off.code_divider) {
            let mut code = vec![];
            let mut j = i + 1;
            while j < lines.len() {
                if let Some((c2, n2, rest)) = fence(lines[j])
                    && c2 == fc
                    && n2 >= n
                    && rest.is_empty()
                {
                    break;
                }
                code.push(dedent(lines[j], ind));
                j += 1;
            }
            let lang = crate::lang::resolve(info.split_whitespace().next().unwrap_or("")).to_string();
            out.push(Block::new(BlockKind::Code { lang, caption: None, wrap: false }, RichText::plain(code.join("\n"))));
            i = j + 1;
            last_list = false;
            continue;
        }
        if let Some((marker, col, content)) = list_item(line).filter(|(m, _, c)| {
            let todo = todo_prefix(c).is_some();
            !(todo && off.todo || !todo && matches!(m, Marker::Bullet) && off.bullet || !todo && matches!(m, Marker::Ordered(_)) && off.ordered)
        }) {
            // The item's own lines: anything indented past its marker.
            let mut j = i + 1;
            let mut sub: Vec<&str> = vec![];
            while j < lines.len() {
                let l = lines[j];
                if l.trim().is_empty() {
                    // A blank line ends the item unless indented content follows.
                    if lines.get(j + 1).is_some_and(|n| !n.trim().is_empty() && indent_of(n) > ind) {
                        j += 1;
                        continue;
                    }
                    break;
                }
                if indent_of(l) <= ind {
                    break;
                }
                sub.push(dedent(l, col.min(indent_of(l))));
                j += 1;
            }
            let (kind, text) = match todo_prefix(content) {
                Some((done, rest)) => (BlockKind::Todo { done }, rest),
                None => match marker {
                    Marker::Bullet => (BlockKind::Bullet, content),
                    Marker::Ordered(n) => {
                        let cont = last_list && matches!(out.last().map(|b| &b.kind), Some(BlockKind::Ordered { .. }));
                        (BlockKind::Ordered { seq: (!cont && n != 1).then_some(n) }, content)
                    }
                },
            };
            out.push(Block::new(kind, md_inline(text.trim())).with_children(md_blocks(&sub, off)));
            last_list = true;
            i = j;
            continue;
        }
        last_list = false;
        if let Some((done, rest)) = todo_prefix(body).filter(|_| !off.todo) {
            out.push(Block::new(BlockKind::Todo { done }, md_inline(rest.trim())));
            i += 1;
            continue;
        }
        let hashes = body.bytes().take_while(|&b| b == b'#').count();
        if (1..=9).contains(&hashes) && body[hashes..].starts_with([' ', '\t']) {
            let t = body[hashes..].trim().trim_end_matches('#').trim_end();
            out.push(Block::new(BlockKind::Heading { level: hashes as u8, seq_auto: false }, md_inline(t)));
            i += 1;
            continue;
        }
        if divider(body) && !off.code_divider {
            out.push(Block::new(BlockKind::Divider, RichText::default()));
            i += 1;
            continue;
        }
        if quote_line(body).is_some() && !off.quote {
            let mut sub = vec![];
            while i < lines.len() {
                let Some(r) = quote_line(lines[i].trim()) else { break };
                sub.push(r);
                i += 1;
            }
            let kids = md_blocks(&sub, Off { quote: true, ..off });
            if !kids.is_empty() {
                out.push(Block::new(BlockKind::Quote, RichText::default()).with_children(kids));
            }
            continue;
        }
        if let Some((t, n)) = pipe_table(&lines[i..]) {
            out.push(t);
            i += n;
            continue;
        }
        if let Some(src) = image_line(body).filter(|_| off.file) {
            let mut b = Block::new(
                BlockKind::Image { src_kind: "src".into(), src: src.to_string(), width: None, height: None, caption: None, crop: None },
                RichText::default(),
            );
            b.align = crate::model::Align::Center;
            out.push(b);
            i += 1;
            continue;
        }
        // Leading spaces stay literal.
        out.push(Block::new(BlockKind::Text, md_inline(&tabs(line.trim_end()))));
        i += 1;
    }
    out
}

/// `![alt](src "title")` filling the whole line: its src.
fn image_line(s: &str) -> Option<&str> {
    let rest = s.strip_prefix("![")?;
    let (_, tail) = rest.split_once("](")?;
    let inner = tail.strip_suffix(')')?;
    let src = inner.split_once([' ', '\t']).map_or(inner, |(a, _)| a);
    let src = src.trim_start_matches('<').trim_end_matches('>');
    (!src.is_empty() && !inner.contains(')')).then_some(src)
}

fn quote_line(s: &str) -> Option<&str> {
    if s == ">" {
        return Some("");
    }
    if let Some(r) = s.strip_prefix('>') {
        return Some(r.strip_prefix(' ').unwrap_or(r));
    }
    s.strip_prefix("》 ")
}

fn table_cells(line: &str) -> Vec<String> {
    let t = line.trim();
    let t = t.strip_prefix('|').unwrap_or(t);
    let t = t.strip_suffix('|').filter(|x| !x.ends_with('\\')).unwrap_or(t);
    let mut cells = vec![];
    let mut cur = String::new();
    let mut chars = t.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' if chars.peek() == Some(&'|') => {
                cur.push('|');
                chars.next();
            }
            '|' => cells.push(std::mem::take(&mut cur).trim().to_string()),
            _ => cur.push(c),
        }
    }
    cells.push(cur.trim().to_string());
    cells
}

/// GFM pipe table at the start of `lines`: (table, lines used).
fn pipe_table(lines: &[&str]) -> Option<(Block, usize)> {
    let head = lines.first()?;
    let sep = lines.get(1)?;
    if !head.contains('|') {
        return None;
    }
    let seps = table_cells(sep);
    let ok = |c: &String| {
        let c = c.trim_matches(':');
        !c.is_empty() && c.chars().all(|x| x == '-')
    };
    if !seps.iter().all(ok) || !sep.contains('-') {
        return None;
    }
    let header = table_cells(head);
    let ncols = header.len();
    if ncols != seps.len() {
        return None;
    }
    let aligns: Vec<Align> = seps
        .iter()
        .map(|c| match (c.starts_with(':'), c.ends_with(':')) {
            (true, true) => Align::Center,
            (false, true) => Align::Right,
            _ => Align::Left,
        })
        .collect();
    let mut n = 2;
    let mut rows = vec![header];
    while let Some(l) = lines.get(n) {
        if l.trim().is_empty() || !l.contains('|') {
            break;
        }
        rows.push(table_cells(l));
        n += 1;
    }
    let rows = rows
        .into_iter()
        .map(|r| {
            (0..ncols)
                .map(|c| {
                    let mut b = Block::new(BlockKind::Text, md_inline(r.get(c).map_or("", String::as_str)));
                    b.align = aligns[c];
                    TableCell { blocks: vec![b], ..TableCell::empty() }
                })
                .collect()
        })
        .collect();
    let mut t = Table { col_widths: vec![200.; ncols], header_rows: 1, header_col: false, rows };
    t.repair();
    Some((Block::new(BlockKind::Table(t), RichText::default()), n))
}

fn is_boundary(c: Option<char>) -> bool {
    c.is_none_or(|c| c.is_whitespace())
}

/// Inline Markdown: code spans, equations, links, bare URLs and the emphasis
/// rules of the live shortcuts (`~x~` is underline). Paste turns escapes,
/// images and `<…>` autolinks off.
pub fn md_inline(s: &str) -> RichText {
    let mut out = RichText::default();
    let mut plain = String::new();
    let flush = |plain: &mut String, out: &mut RichText| {
        if !plain.is_empty() {
            out.push(plain, Marks::default());
            plain.clear();
        }
    };
    let mut i = 0;
    while i < s.len() {
        let rest = &s[i..];
        let c = rest.chars().next().unwrap();
        let prev = s[..i].chars().next_back();
        // Code span.
        if c == '`' {
            let n = rest.bytes().take_while(|&b| b == b'`').count();
            let delim = &rest[..n];
            if let Some(end) = find_run(&rest[n..], delim, b'`') {
                let inner = &rest[n..n + end];
                let inner = if inner.len() > 2 && inner.starts_with(' ') && inner.ends_with(' ') { &inner[1..inner.len() - 1] } else { inner };
                if !inner.is_empty() {
                    flush(&mut plain, &mut out);
                    out.push(inner, Marks { code: true, ..Marks::default() });
                    i += n + end + n;
                    continue;
                }
            }
            plain.push_str(delim);
            i += n;
            continue;
        }
        // Equations: `$$x$$` and `$x$` with whitespace/line boundaries.
        if c == '$' && is_boundary(prev) {
            let mut done = false;
            for delim in ["$$", "$"] {
                if !rest.starts_with(delim) || rest[delim.len()..].starts_with('$') {
                    continue;
                }
                let body = &rest[delim.len()..];
                let Some(end) = body.find(delim) else { continue };
                let inner = &body[..end];
                let after = body[end + delim.len()..].chars().next();
                // `$` before a digit is money, not math.
                let digit = delim == "$" && inner.starts_with(|c: char| c.is_ascii_digit());
                if digit || inner.trim().is_empty() || inner.starts_with(' ') || !is_boundary(after) || after == Some('$') {
                    continue;
                }
                flush(&mut plain, &mut out);
                out.push(crate::editor::core::EQUATION, Marks { inline: Some(crate::docxml::to_inline(InlineKind::Equation(inner.to_string()))), ..Marks::default() });
                i += delim.len() * 2 + end;
                done = true;
                break;
            }
            if done {
                continue;
            }
        }
        // Links `[text](url)`.
        if c == '[' {
            let open = 0;
            if let Some((label, url, used)) = link_at(&rest[open..]) {
                flush(&mut plain, &mut out);
                let mut t = md_inline(label);
                if t.is_empty() {
                    t = RichText::plain(url);
                }
                let len = t.len();
                t.update_marks(0..len, |m| m.link = Some(url.to_string()));
                out.append(&t);
                i += open + used;
                continue;
            }
        }
        // Bare URLs (they get links when drawn).
        if (c == 'h' || c == 'w') && !prev.is_some_and(|p| p.is_alphanumeric()) {
            let l = rest.get(..8).unwrap_or(rest).to_ascii_lowercase();
            if l.starts_with("https://") || l.starts_with("http://") || l.starts_with("www.") {
                let end = rest.find(|c: char| c.is_whitespace() || "<>\"'`".contains(c) || !c.is_ascii()).unwrap_or(rest.len());
                let url = rest[..end].trim_end_matches(|c: char| ".,;:!?)]}*_~".contains(c));
                let host = url.splitn(2, "://").last().unwrap_or("");
                if host.len() > 3 && host.contains('.') {
                    flush(&mut plain, &mut out);
                    let href = if l.starts_with("www.") { format!("https://{url}") } else { url.to_string() };
                    out.push(url, Marks { link: Some(href), ..Marks::default() });
                    i += url.len();
                    continue;
                }
            }
        }
        // Emphasis.
        if matches!(c, '*' | '_' | '~') {
            let n = rest.bytes().take_while(|&b| b == c as u8).count();
            let run = &rest[..n];
            let rule: Option<fn(&mut Marks)> = match (c, n) {
                ('*' | '_', 3) => Some(|m| {
                    m.bold = true;
                    m.italic = true
                }),
                ('*' | '_', 2) => Some(|m| m.bold = true),
                ('*' | '_', 1) => Some(|m| m.italic = true),
                ('~', 2) => Some(|m| m.strike = true),
                ('~', 1) => Some(|m| m.underline = true),
                _ => None,
            };
            let bounded = c == '_' || (c == '~' && n == 1);
            if let Some(set) = rule
                && (!bounded || is_boundary(prev))
                && let Some(end) = find_run(&rest[n..], run, c as u8)
            {
                let inner = &rest[n..n + end];
                let after = rest[n + end + n..].chars().next();
                if !inner.starts_with(char::is_whitespace) && !inner.ends_with(char::is_whitespace) && (!bounded || is_boundary(after)) {
                    flush(&mut plain, &mut out);
                    let mut t = md_inline(inner);
                    let len = t.len();
                    t.update_marks(0..len, set);
                    out.append(&t);
                    i += n + end + n;
                    continue;
                }
            }
            plain.push_str(run);
            i += n;
            continue;
        }
        plain.push(c);
        i += c.len_utf8();
    }
    flush(&mut plain, &mut out);
    out
}

/// Offset in `s` of a delimiter run exactly equal to `delim` (not part of a
/// longer run of `ch`), skipping a non-empty match only.
fn find_run(s: &str, delim: &str, ch: u8) -> Option<usize> {
    let b = s.as_bytes();
    let mut from = 0;
    while let Some(k) = s[from..].find(delim) {
        let at = from + k;
        let before_ok = at == 0 || b[at - 1] != ch;
        let after_ok = b.get(at + delim.len()) != Some(&ch);
        if at > 0 && before_ok && after_ok {
            return Some(at);
        }
        from = at + delim.len().max(1);
        // Skip the rest of a longer run.
        while b.get(from) == Some(&ch) {
            from += 1;
        }
    }
    None
}

/// `[label](url "title")` at the start of `s`: (label, url, bytes used).
fn link_at(s: &str) -> Option<(&str, &str, usize)> {
    let b = s.as_bytes();
    if b.first() != Some(&b'[') {
        return None;
    }
    let mut depth = 0;
    let mut close = None;
    for (k, &c) in b.iter().enumerate() {
        match c {
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    close = Some(k);
                    break;
                }
            }
            _ => {}
        }
    }
    let close = close?;
    let rest = &s[close + 1..];
    let inner = rest.strip_prefix('(')?;
    let end = inner.find(')')?;
    let target = inner[..end].trim();
    let url = target.split_whitespace().next().unwrap_or("").trim_matches(['<', '>']);
    if url.is_empty() || !(link_ok(url) || url.starts_with("www.")) {
        return None;
    }
    Some((&s[1..close], url, close + 1 + 1 + end + 1))
}

#[cfg(test)]
mod tests {
    use super::html_to_blocks;
    use crate::docxml;
    use crate::model::{Document, RichText};

    fn xml(html: &str) -> String {
        docxml::serialize(&Document { blocks: html_to_blocks(html), ..Default::default() })
    }

    #[test]
    fn blocks_and_marks() {
        let x = xml(
            "<meta charset='utf-8'><h2>Title</h2><p>a <b>bold</b> and <a href=\"https://x.y/\">link</a></p>\
             <ul><li>one<ul><li>sub</li></ul></li><li><input type=checkbox checked> done</li></ul>\
             <ol start=3><li>three</li></ol><blockquote><p>q</p></blockquote><hr><pre><code class=\"language-rust\">fn x() {}\n</code></pre>",
        );
        assert!(x.contains("<h2>Title</h2>"), "{x}");
        assert!(x.contains("<p>a <b>bold</b> and <a href=\"https://x.y/\">link</a></p>"), "{x}");
        assert!(x.contains("<li>one"), "{x}");
        assert!(x.contains("sub"), "{x}");
        assert!(x.contains("<checkbox done=\"true\">done</checkbox>"), "{x}");
        assert!(x.contains("<li seq=\"3\">three</li>"), "{x}");
        assert!(x.contains("<blockquote>"), "{x}");
        assert!(x.contains("<hr/>"), "{x}");
        assert!(x.contains("lang=\"rust\""), "{x}");
        assert!(x.contains("<code>fn x() {}</code></pre>"), "{x}");
    }

    #[test]
    fn feishu_todo_and_callout() {
        let x = xml("<ul class=\"list-done1\"><li data-list=\"done\"><div>t</div></li></ul>\
             <div class=\"zoneType-calloutBlock\"><div class=\"callout-container\" data-emoji-id=\"chestnut\">\
             <div class=\"callout-block\" style=\"background-color:rgb(255,245,235);border-color:rgb(254,212,164);\"><div>c</div></div></div></div>");
        assert!(x.contains("<checkbox done=\"true\">t</checkbox>"), "{x}");
        assert!(x.contains("<callout emoji=\"🌰\" background-color=\"light-orange\" border-color=\"orange\">"), "{x}");
    }

    #[test]
    fn google_docs_wrapper_is_not_bold() {
        let x = xml("<b style=\"font-weight:normal;\" id=\"docs-internal-guid-1\"><p dir=\"ltr\"><span style=\"font-weight:700\">B</span><span> plain</span></p></b>");
        assert!(x.contains("<p><b>B</b> plain</p>"), "{x}");
    }

    #[test]
    fn whitespace_and_breaks() {
        let x = xml("<div>\n  line one <br>\n  line   two\n</div><p></p><div>x</div>");
        assert!(x.contains("<p>line one</p>\n<p>line two</p>\n<p></p>\n<p>x</p>"), "{x}");
    }

    #[test]
    fn markdown() {
        let x = docxml::serialize(&Document {
            title: RichText::default(),
            blocks: super::markdown_to_blocks(
                "# H1\n\nSome **bold** and *it* and ~u~ and `c` [l](https://a.b/c)\n- a\n  - b\n- [x] t\n3. three\n4. four\n> q\n\n---\n```rust\nfn main() {}\n```\n| A | B |\n|---|:-:|\n| 1 | 2 |\n",
                Default::default(),
            ),
            ..Default::default()
        });
        assert!(x.contains("<h1>H1</h1>"), "{x}");
        assert!(x.contains("<b>bold</b>"), "{x}");
        assert!(x.contains("<em>it</em>"), "{x}");
        assert!(x.contains("<u>u</u>"), "{x}");
        assert!(x.contains("<code>c</code>"), "{x}");
        assert!(x.contains("<a href=\"https://a.b/c\">l</a>"), "{x}");
        assert!(x.contains("<li>a"), "{x}");
        assert!(x.contains("<checkbox done=\"true\">t</checkbox>"), "{x}");
        assert!(x.contains("<li seq=\"3\">three</li>"), "{x}");
        assert!(x.contains("<li>four</li>"), "{x}");
        assert!(x.contains("<blockquote>"), "{x}");
        assert!(x.contains("<hr/>"), "{x}");
        assert!(x.contains("fn main() {}"), "{x}");
        assert!(x.contains("<table"), "{x}");
    }

    #[test]
    fn markdown_literals() {
        let t = super::md_inline("a_b_c snake_case_name ** x** costs $5 and $6");
        assert!(t.runs.iter().all(|r| !r.marks.bold && !r.marks.italic), "{:?}", t);
    }

    #[test]
    fn tables() {
        let x = xml("<table><tr><th>a</th><th>b</th></tr><tr><td colspan=2>c</td></tr></table>");
        assert!(x.contains("<table"), "{x}");
        assert!(x.contains("colspan=\"2\""), "{x}");
    }
}
