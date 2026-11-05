//! Document → one self-contained HTML page (images inlined as data URIs,
//! equations as MathML). WebKit lays it out for 下载为 → PDF and 打印.

use crate::model::*;
use crate::theme;
use gpui::Hsla;
use std::fmt::Write;

pub struct Options<'a> {
    /// Titles of the document's child pages (what 子页面列表 shows).
    pub sub_pages: &'a [String],
}

const CSS: &str = r#"
* { box-sizing: border-box; }
html { -webkit-print-color-adjust: exact; print-color-adjust: exact; }
body { margin: 0; color: #1f2225; font: 15px/22px -apple-system, "SF Pro Text", "PingFang SC", sans-serif; word-wrap: break-word; }
.cover { height: 200px; margin-bottom: 24px; background-size: cover; background-position: center; border-radius: 12px; }
.icon { font-size: 48px; line-height: 56px; margin-bottom: 8px; }
h1.title { font-size: 21px; line-height: 27px; font-weight: 700; margin: 0 0 16px; padding-bottom: 16px; border-bottom: 1px solid rgba(0,0,0,0.045); }
.b { margin: 13px 0; min-height: 22px; }
.h { font-weight: 600; }
.h1 { font-size: 21px; line-height: 27px; font-weight: 700; margin-top: 48px; margin-bottom: 16px; }
.h2 { font-size: 19px; line-height: 25px; font-weight: 700; margin-top: 36px; margin-bottom: 16px; }
.h3 { font-size: 18px; line-height: 24px; margin-top: 28px; margin-bottom: 12px; }
.h4 { font-size: 16px; line-height: 25px; margin-top: 16px; margin-bottom: 16px; }
.h5, .h6, .h7, .h8, .h9 { font-size: 15px; margin-top: 13px; }
.hn { margin-right: 6px; color: #737477; }
.li { display: flex; margin: 13px 0; }
.mk { flex: none; min-width: 18px; margin-right: 6px; color: #1f2225; white-space: nowrap; }
.dot { flex: none; width: 24px; display: flex; padding-left: 5px; align-items: center; height: 22px; }
.dot i { display: block; width: 5px; height: 5px; border-radius: 50%; background: #1f2225; }
.dot i.d1 { background: none; border: 1px solid #1f2225; }
.dot i.d2 { border-radius: 1px; width: 4.5px; height: 4.5px; }
.box { flex: none; width: 17px; height: 17px; margin: 2.5px 8px 0 0; border: 1px solid rgba(0,0,0,0.22); border-radius: 5px; }
.box.on { background: #3a85f7; border-color: #3a85f7; position: relative; }
.box.on::after { content: ""; position: absolute; left: 5px; top: 2px; width: 4px; height: 8px; border: solid #fff; border-width: 0 2px 2px 0; transform: rotate(45deg); }
.done { color: rgba(31,34,37,0.5); text-decoration: line-through; }
.lb { flex: 1; min-width: 0; }
.lb > .li:first-child, .lb > .b:first-child { margin-top: 0; }
.kids { padding-left: 25px; }
.quote { border-left: 4px solid rgba(31,34,37,0.82); padding-left: 12px; margin: 13px 0; }
.callout { display: flex; border: 1px solid; border-radius: 4px; padding: 12px 14px; margin: 16px 0; }
.callout .em { flex: none; width: 22px; margin-right: 10px; font-size: 18px; }
.callout .cb { flex: 1; min-width: 0; }
.callout .cb > :first-child { margin-top: 0; } .callout .cb > :last-child { margin-bottom: 0; }
pre { margin: 16px 0; padding: 12px 16px; background: #fbfbfb; border: 1px solid #eaebeb; border-radius: 6px; font: 13px/20px ui-monospace, "SF Mono", Menlo, monospace; color: #4c4d50; white-space: pre-wrap; word-break: break-all; }
.cap { margin-top: -8px; margin-bottom: 13px; font-size: 13px; line-height: 20px; font-weight: 300; color: #7e8287; text-align: center; }
.lang { font-size: 12px; color: #737477; margin-bottom: 6px; font-family: -apple-system, "PingFang SC", sans-serif; }
hr { border: none; height: 2px; margin: 11px 0; background: radial-gradient(circle, rgba(31,34,37,0.5) 0.75px, transparent 1px) 0 0 / 5px 2px repeat-x; }
.img { margin: 13px 0; }
.img img { max-width: 100%; vertical-align: top; border-radius: 5px; }
.img .cap { margin-top: 6px; }
.crop { display: inline-block; overflow: hidden; max-width: 100%; vertical-align: top; border-radius: 5px; }
.crop img { max-width: none; border-radius: 0; }
table { border-collapse: separate; border-spacing: 0; margin: 16px 0; table-layout: fixed; max-width: 100%; }
td, th { border-right: 1px solid rgba(0,0,0,0.09); border-bottom: 1px solid rgba(0,0,0,0.09); padding: 9px 10px; font-size: 14px; line-height: 20px; vertical-align: top; text-align: left; font-weight: normal; }
tr:first-child > * { border-top: 1px solid rgba(0,0,0,0.09); }
tr > :first-child { border-left: 1px solid rgba(0,0,0,0.09); }
tr:first-child > :first-child { border-top-left-radius: 5px; } tr:first-child > :last-child { border-top-right-radius: 5px; }
tr:last-child > :first-child { border-bottom-left-radius: 5px; } tr:last-child > :last-child { border-bottom-right-radius: 5px; }
td > .b, th > .b, td > .li, th > .li { margin: 0; min-height: 20px; }
th { background: #f8f8f8; }
.grid { display: flex; gap: 16px; margin: 13px 0; }
.col { min-width: 0; }
.subpages { border: 1px solid rgba(0,0,0,0.08); border-radius: 5px; padding: 4px; margin: 13px 0; }
.subpages div { padding: 4px 8px; font-size: 14px; line-height: 22px; }
.countdown { display: flex; justify-content: center; align-items: flex-start; padding: 20px 0 16px; margin: 13px 0; border: 1px solid rgba(0,0,0,0.08); border-radius: 5px; color: var(--cd); }
.countdown span { display: flex; flex-direction: column; align-items: center; }
.countdown span b { min-width: 40px; padding: 0 6px; height: 52px; line-height: 52px; border-radius: 8px; background: var(--cd); color: #fff; font-size: 26px; text-align: center; }
.countdown span i { margin-top: 4px; font-style: normal; font-size: 13px; line-height: 20px; }
.countdown .sep { width: 16px; height: 54px; line-height: 46px; font-size: 28px; text-align: center; }
.raw { margin: 13px 0; padding: 8px 12px; background: #f7f7f7; border-radius: 5px; color: #737477; font-size: 13px; }
code { font-family: ui-monospace, "SF Mono", Menlo, monospace; font-size: 0.875em; background: #f8f8f8; border: 1px solid #e2e3e3; border-radius: 3px; padding: 0 3.5px; }
a { color: #0f5492; text-decoration: underline; text-underline-offset: 2px; }
.mathb { margin: 13px 0; text-align: center; }
.bgb { border-radius: 3px; margin: 0 -7px; padding: 9px 7px; }
"#;

/// The document as a standalone page, always in the light colours (it is printed or opened elsewhere).
pub fn to_html(doc: &Document, opts: &Options) -> String { crate::theme::light(|| page(doc, opts)) }

fn page(doc: &Document, opts: &Options) -> String {
    let title = crate::settings::display_title(&doc.title.text);
    let mut out = String::new();
    let _ = write!(out, "<!DOCTYPE html>\n<html lang=\"zh-CN\"><head><meta charset=\"utf-8\"><title>{}</title><style>{CSS}</style></head><body>", esc(&title));
    if let Some(c) = &doc.cover
        && let Some(uri) = data_uri(&c.src)
    {
        let _ = write!(out, "<div class=\"cover\" style=\"background-image:url('{uri}')\"></div>");
    }
    if let Some(i) = &doc.icon {
        let _ = write!(out, "<div class=\"icon\">{}</div>", esc(i));
    }
    let _ = write!(out, "<h1 class=\"title\">{}</h1>", esc(&title));
    let w = Writer { opts, headings: crate::editor::core::heading_orders(&doc.blocks) };
    w.blocks(&doc.blocks, 0, &mut out);
    out.push_str("</body></html>\n");
    out
}

struct Writer<'a> {
    opts: &'a Options<'a>,
    headings: rustc_hash::FxHashMap<BlockId, String>,
}

impl Writer<'_> {
    fn blocks(&self, list: &[Block], depth: usize, out: &mut String) {
        for (i, b) in list.iter().enumerate() {
            self.block(b, list, i, depth, out);
        }
    }

    fn block(&self, b: &Block, siblings: &[Block], idx: usize, depth: usize, out: &mut String) {
        // A block background wraps the block and its children.
        match b.bg.as_ref().filter(|_| b.kind.takes_bg()).and_then(|c| theme::block_bg(&c.0)) {
            Some(fill) => {
                let _ = write!(out, "<div class=\"bgb\" style=\"background:{}\">", css(fill));
                self.block_body(b, siblings, idx, depth, out);
                out.push_str("</div>");
            }
            None => self.block_body(b, siblings, idx, depth, out),
        }
    }

    fn block_body(&self, b: &Block, siblings: &[Block], idx: usize, depth: usize, out: &mut String) {
        let align = match b.align {
            Align::Left => "",
            Align::Center => " style=\"text-align:center\"",
            Align::Right => " style=\"text-align:right\"",
        };
        match &b.kind {
            BlockKind::Text => {
                // A paragraph holding just a display equation.
                if let Some(src) = lone_equation(&b.text)
                    && crate::editor::math::is_block(&src)
                {
                    let _ = write!(out, "<div class=\"mathb\">{}</div>", math(&src));
                } else {
                    let _ = write!(out, "<div class=\"b\"{align}>{}</div>", inline(&b.text));
                }
                self.kids(b, depth, out);
            }
            BlockKind::Heading { level, .. } => {
                let n = self.headings.get(&b.id).map(|n| format!("<span class=\"hn\">{}</span>", esc(n))).unwrap_or_default();
                let _ = write!(out, "<div class=\"b h h{}\"{align}>{n}{}</div>", level.clamp(&1, &9), inline(&b.text));
                self.kids(b, depth, out);
            }
            BlockKind::Bullet | BlockKind::Ordered { .. } | BlockKind::Todo { .. } => {
                let (marker, body_class) = match b.kind {
                    BlockKind::Bullet => (format!("<span class=\"dot\"><i class=\"d{}\"></i></span>", depth % 3), ""),
                    BlockKind::Ordered { .. } => (
                        format!(
                            "<span class=\"mk\">{}</span>",
                            crate::editor::core::ordered_marker(crate::editor::core::ordered_number(siblings, idx), depth)
                        ),
                        "",
                    ),
                    BlockKind::Todo { done } => (format!("<span class=\"box{}\"></span>", if done { " on" } else { "" }), if done { " done" } else { "" }),
                    _ => unreachable!(),
                };
                let _ = write!(out, "<div class=\"li\">{marker}<div class=\"lb\"><div class=\"{}\"{align}>{}</div>", body_class.trim(), inline(&b.text));
                if !b.children.is_empty() {
                    out.push_str("<div>");
                    self.blocks(&b.children, depth + 1, out);
                    out.push_str("</div>");
                }
                out.push_str("</div></div>");
            }
            BlockKind::Code { lang, caption, .. } => {
                out.push_str("<pre>");
                if lang != "Plain Text" {
                    let _ = write!(out, "<div class=\"lang\">{}</div>", esc(lang));
                }
                out.push_str(&code(lang, &b.text.text));
                out.push_str("</pre>");
                if let Some(c) = caption.as_ref().filter(|c| !c.is_empty()) {
                    let _ = write!(out, "<div class=\"cap\">{}</div>", esc(c));
                }
            }
            BlockKind::Quote => {
                out.push_str("<div class=\"quote\">");
                self.blocks(&b.children, depth, out);
                out.push_str("</div>");
            }
            BlockKind::Callout { emoji, bg, border, text_color } => {
                let fill = bg.as_ref().and_then(|c| theme::callout_bg(&c.0)).unwrap_or(theme::c(theme::craft::O50));
                let line = border.as_ref().and_then(|c| theme::callout_border(&c.0)).unwrap_or(theme::c(theme::craft::O200));
                let color = text_color.as_ref().and_then(|c| theme::text_color(&c.0)).map(|c| format!("color:{};", css(c))).unwrap_or_default();
                let _ = write!(
                    out,
                    "<div class=\"callout\" style=\"background:{};border-color:{};{color}\"><span class=\"em\">{}</span><div class=\"cb\">",
                    css(fill),
                    css(line),
                    esc(emoji)
                );
                self.blocks(&b.children, depth, out);
                out.push_str("</div></div>");
            }
            BlockKind::Divider => out.push_str("<hr>"),
            BlockKind::Image { src, width, height, caption, crop, .. } => {
                let _ = write!(out, "<div class=\"img\"{align}>");
                match data_uri(src) {
                    Some(uri) => out.push_str(&image(&uri, *width, *height, *crop)),
                    None => {
                        let _ = write!(out, "<div class=\"raw\">{}</div>", esc(src));
                    }
                }
                if let Some(c) = caption.as_ref().filter(|c| !c.is_empty()) {
                    let _ = write!(out, "<div class=\"cap\">{}</div>", esc(c));
                }
                out.push_str("</div>");
            }
            BlockKind::Table(t) => self.table(t, out),
            BlockKind::Grid => {
                out.push_str("<div class=\"grid\">");
                let total: f32 = b.children.iter().map(|c| if let BlockKind::Column { ratio } = c.kind { ratio } else { 1. }).sum::<f32>().max(0.01);
                for col in &b.children {
                    let ratio = if let BlockKind::Column { ratio } = col.kind { ratio } else { 1. };
                    let _ = write!(out, "<div class=\"col\" style=\"flex:{:.4} 1 0\">", ratio / total);
                    self.blocks(&col.children, depth, out);
                    out.push_str("</div>");
                }
                out.push_str("</div>");
            }
            BlockKind::Column { .. } => self.blocks(&b.children, depth, out),
            BlockKind::File(f) => {
                let path = (f.src_kind == "path").then(|| crate::store::image_path(&f.src)).flatten();
                let _ = write!(out, "<div class=\"b\"{align}>");
                match path {
                    Some(p) => {
                        let _ = write!(out, "<a href=\"file://{}\">{}</a>", esc(&p.to_string_lossy()), esc(&f.name));
                    }
                    None => out.push_str(&esc(&f.name)),
                }
                out.push_str("</div>");
            }
            BlockKind::Countdown(c) => {
                // The add-on's face as of the export.
                let left = c.remaining(crate::time::now_ms());
                let parts = [(left / 86_400, crate::settings::text("天")), (left / 3600 % 24, crate::settings::text("时")), (left / 60 % 60, crate::settings::text("分")), (left % 60, crate::settings::text("秒"))];
                let _ = write!(out, "<div class=\"countdown\" style=\"--cd:{}\">", esc(&c.color));
                for (i, (v, unit)) in parts.iter().enumerate() {
                    if i > 0 {
                        out.push_str("<b class=\"sep\">:</b>");
                    }
                    let _ = write!(out, "<span><b>{v:02}</b><i>{unit}</i></span>");
                }
                out.push_str("</div>");
            }
            BlockKind::SubPageList => {
                out.push_str("<div class=\"subpages\">");
                for t in self.opts.sub_pages {
                    let _ = write!(out, "<div>📄 {}</div>", esc(&crate::settings::display_title(t)));
                }
                out.push_str("</div>");
            }
            BlockKind::Raw { xml, .. } => match crate::docxml::raw_summary(xml) {
                (text, Some(link)) => {
                    let label = if text.is_empty() { &link } else { &text };
                    let _ = write!(out, "<div class=\"raw\"><a href=\"{}\">{}</a></div>", esc(&link), esc(label));
                }
                (text, None) if !text.is_empty() => {
                    let _ = write!(out, "<div class=\"raw\">{}</div>", esc(&text).replace('\n', "<br>"));
                }
                _ => {}
            },
        }
    }

    fn kids(&self, b: &Block, depth: usize, out: &mut String) {
        if b.children.is_empty() {
            return;
        }
        out.push_str("<div class=\"kids\">");
        self.blocks(&b.children, depth + 1, out);
        out.push_str("</div>");
    }

    fn table(&self, t: &Table, out: &mut String) {
        let total: f32 = t.col_widths.iter().sum();
        let _ = write!(out, "<table style=\"width:{total:.0}px\"><colgroup>");
        for w in &t.col_widths {
            let _ = write!(out, "<col style=\"width:{w:.0}px\">");
        }
        out.push_str("</colgroup>");
        for (r, row) in t.rows.iter().enumerate() {
            out.push_str("<tr>");
            for (c, cell) in row.iter().enumerate() {
                if cell.is_covered() {
                    continue;
                }
                let head = r < t.header_rows || (t.header_col && c == 0);
                let tag = if head { "th" } else { "td" };
                let mut style = String::new();
                if let Some(bg) = cell.bg.as_ref().and_then(|b| theme::block_bg(&b.0)) {
                    let _ = write!(style, "background:{};", css(bg));
                }
                if let Some(v) = &cell.valign {
                    let _ = write!(style, "vertical-align:{};", if v == "center" { "middle" } else { v.as_str() });
                }
                let _ = write!(out, "<{tag}");
                if cell.rowspan > 1 {
                    let _ = write!(out, " rowspan=\"{}\"", cell.rowspan);
                }
                if cell.colspan > 1 {
                    let _ = write!(out, " colspan=\"{}\"", cell.colspan);
                }
                if !style.is_empty() {
                    let _ = write!(out, " style=\"{style}\"");
                }
                out.push('>');
                self.blocks(&cell.blocks, 0, out);
                let _ = write!(out, "</{tag}>");
            }
            out.push_str("</tr>");
        }
        out.push_str("</table>");
    }
}

fn image(uri: &str, width: Option<f32>, height: Option<f32>, crop: Option<[f32; 4]>) -> String {
    match (crop, width, height) {
        (Some([l, t, r, b]), Some(w), Some(h)) if r > l && b > t => {
            let (fw, fh) = (w / (r - l), h / (b - t));
            format!(
                "<span class=\"crop\" style=\"width:{w:.0}px;height:{h:.0}px\"><img src=\"{uri}\" style=\"width:{fw:.1}px;height:{fh:.1}px;margin-left:{:.1}px;margin-top:{:.1}px\"></span>",
                -l * fw,
                -t * fh
            )
        }
        _ => {
            let size = width.map(|w| format!(" style=\"width:{w:.0}px\"")).unwrap_or_default();
            format!("<img src=\"{uri}\"{size}>")
        }
    }
}

/// An image source as a data URI: local files, and remote images already
/// downloaded for display. Other remote images stay links.
fn data_uri(src: &str) -> Option<String> {
    use base64::Engine;
    if src.starts_with("data:") {
        return Some(src.to_string());
    }
    let remote = src.starts_with("http://") || src.starts_with("https://");
    let path = if remote { crate::editor::image::cached_remote(src) } else { crate::store::image_path(src) };
    let Some(bytes) = path.and_then(|p| std::fs::read(p).ok()) else {
        return remote.then(|| src.replace('"', "%22"));
    };
    let mime = match image::guess_format(&bytes) {
        Ok(f) => f.to_mime_type(),
        Err(_) if bytes.starts_with(b"<svg") || bytes.starts_with(b"<?xml") => "image/svg+xml",
        Err(_) => "image/png",
    };
    Some(format!("data:{mime};base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes)))
}

fn code(lang: &str, text: &str) -> String {
    if lang == "Plain Text" {
        return esc(text);
    }
    let mut out = String::new();
    let mut at = 0;
    for (r, color, italic) in crate::highlight::spans(lang, text) {
        if r.start < at || r.end > text.len() {
            continue;
        }
        out.push_str(&esc(&text[at..r.start]));
        let _ = write!(out, "<span style=\"color:{}{}\">{}</span>", css(color), if italic { ";font-style:italic" } else { "" }, esc(&text[r.clone()]));
        at = r.end;
    }
    out.push_str(&esc(&text[at..]));
    out
}

fn lone_equation(t: &RichText) -> Option<String> {
    match t.runs.as_slice() {
        [run] if t.text == "\u{fffc}" => run.marks.inline.as_ref().and_then(Inline::equation).map(str::to_string),
        _ => None,
    }
}

fn math(src: &str) -> String {
    crate::editor::math::mathml(src).unwrap_or_else(|| format!("<code>{}</code>", esc(src)))
}

fn inline(t: &RichText) -> String {
    let mut out = String::new();
    for (r, m) in t.spans() {
        let s = &t.text[r];
        let mut x = match &m.inline {
            Some(Inline { kind: InlineKind::File(f), .. }) => s.chars().map(|_| esc(&f.name)).collect(),
            Some(obj) => match obj.equation() {
                Some(src) => s.chars().map(|_| math(src)).collect(),
                None => {
                    let (text, link) = crate::docxml::raw_summary(&obj.xml);
                    let x = match link.filter(|_| matches!(obj.kind, InlineKind::Button(_))) {
                        Some(l) => format!("<a href=\"{}\">{}</a>", esc(&l), esc(&text)),
                        None => esc(&text),
                    };
                    s.chars().map(|_| x.clone()).collect()
                }
            },
            None => esc(s).replace('\n', "<br>"),
        };
        if x.is_empty() {
            continue;
        }
        if m.code {
            x = format!("<code>{x}</code>");
        }
        let mut style = String::new();
        if let Some(c) = m.text_color.as_ref().and_then(|c| theme::text_color(&c.0)) {
            let _ = write!(style, "color:{};", css(c));
        }
        if let Some(c) = m.bg_color.as_ref().and_then(|c| theme::bg_color(&c.0)) {
            let _ = write!(style, "background:{};", css(c));
        }
        if !style.is_empty() {
            x = format!("<span style=\"{style}\">{x}</span>");
        }
        if m.underline {
            x = format!("<u>{x}</u>");
        }
        if m.strike {
            x = format!("<s>{x}</s>");
        }
        if m.italic {
            x = format!("<i>{x}</i>");
        }
        if m.bold {
            x = format!("<b>{x}</b>");
        }
        if let Some(url) = &m.link {
            x = format!("<a href=\"{}\">{x}</a>", esc(url));
        }
        out.push_str(&x);
    }
    if out.is_empty() { "<br>".into() } else { out }
}

fn css(c: Hsla) -> String {
    let c = c.to_rgb();
    let (r, g, b) = ((c.r * 255.).round() as u8, (c.g * 255.).round() as u8, (c.b * 255.).round() as u8);
    if c.a >= 0.999 { format!("#{r:02x}{g:02x}{b:02x}") } else { format!("rgba({r},{g},{b},{:.3})", c.a) }
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_blocks_and_marks() {
        let mut t = RichText::plain("a ");
        t.push("b", Marks { bold: true, text_color: Some(ColorName("red".into())), ..Default::default() });
        let doc = Document {
            title: RichText::plain("T"),
            blocks: vec![
                Block::new(BlockKind::Text, t),
                Block::new(BlockKind::Ordered { seq: None }, RichText::plain("x")),
                Block::new(BlockKind::Ordered { seq: None }, RichText::plain("y")),
                Block::new(BlockKind::SubPageList, RichText::default()),
                Block::new(BlockKind::Raw { tag: "iframe".into(), xml: "<iframe url=\"https://e.x\"/>".into() }, RichText::default()),
            ],
            ..Default::default()
        };
        let h = to_html(&doc, &Options { sub_pages: &["子页".into()] });
        assert!(h.contains("a <b><span style=\"color:#cb0424;\">b</span></b>"));
        assert!(h.contains(">2.</span>"));
        assert!(h.contains("📄 子页"));
        assert!(h.contains("href=\"https://e.x\""));
    }
}
