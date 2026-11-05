//! 下载为 → Markdown: DocxXML document model to CommonMark/GFM text.

use crate::model::*;
use std::path::PathBuf;

/// 导出 Markdown 设置.
#[derive(Default)]
pub struct Options<'a> {
    /// Titles of the document's child pages (what 子页面列表 shows).
    pub sub_pages: &'a [String],
    /// 所有内容: images are kept, local ones copied into this folder next to
    /// the .md file. None is 仅文本（不含图片和附件）.
    pub assets: Option<&'a str>,
}

pub struct Export {
    pub text: String,
    /// Local image files to copy: (source, path relative to the .md file).
    pub files: Vec<(PathBuf, String)>,
}

#[cfg(test)]
pub fn to_markdown(doc: &Document) -> String {
    export(doc, &Options { assets: Some("assets"), ..Default::default() }).text
}

pub fn export(doc: &Document, opts: &Options) -> Export {
    let mut w = Writer { opts, files: vec![] };
    let mut out = String::new();
    let title = doc.title.text.trim();
    if !title.is_empty() {
        out.push_str(&format!("# {}\n\n", escape(title)));
    }
    out.push_str(&w.blocks(&doc.blocks));
    let trimmed = out.trim_end().len();
    out.truncate(trimmed);
    out.push('\n');
    Export { text: out, files: w.files }
}

struct Writer<'a> {
    opts: &'a Options<'a>,
    files: Vec<(PathBuf, String)>,
}

impl Writer<'_> {
    /// Blocks separated by blank lines; consecutive list items stay tight.
    fn blocks(&mut self, bs: &[Block]) -> String {
        let mut out = String::new();
        let mut prev_list = false;
        let mut seq = 0u32;
        for b in bs {
            let list = b.kind.is_list();
            match &b.kind {
                BlockKind::Ordered { seq: Some(n) } => seq = *n,
                BlockKind::Ordered { seq: None } => seq = if prev_list && seq > 0 { seq + 1 } else { 1 },
                _ => seq = 0,
            }
            let s = self.block(b, seq);
            if s.is_empty() && !list {
                continue;
            }
            if !out.is_empty() {
                out.push_str(if list && prev_list { "\n" } else { "\n\n" });
            }
            out.push_str(&s);
            prev_list = list;
        }
        out
    }

    fn block(&mut self, b: &Block, seq: u32) -> String {
        let text = inline(&b.text);
        let nested = |w: &mut Self, head: String| -> String {
            if b.children.is_empty() {
                return head;
            }
            format!("{head}\n{}", indent(&w.blocks(&b.children), "    "))
        };
        match &b.kind {
            BlockKind::Text => nested(self, text),
            BlockKind::Heading { level, .. } => nested(self, format!("{} {text}", "#".repeat((*level).clamp(1, 6) as usize))),
            BlockKind::Bullet => nested(self, format!("- {text}")),
            BlockKind::Ordered { .. } => nested(self, format!("{seq}. {text}")),
            BlockKind::Todo { done } => nested(self, format!("- [{}] {text}", if *done { "x" } else { " " })),
            BlockKind::Code { lang, .. } => {
                let lang = if lang.eq_ignore_ascii_case("plain text") { String::new() } else { lang.to_lowercase().replace(' ', "") };
                let fence = if b.text.text.contains("```") { "````" } else { "```" };
                format!("{fence}{lang}\n{}\n{fence}", b.text.text)
            }
            BlockKind::Quote => indent(&self.blocks(&b.children), "> "),
            BlockKind::Callout { emoji, .. } => {
                let body = self.blocks(&b.children);
                indent(&format!("{emoji} {body}"), "> ")
            }
            BlockKind::Divider => "---".into(),
            BlockKind::Image { src, caption, .. } => match self.opts.assets {
                Some(dir) => {
                    let alt = caption.clone().unwrap_or_default();
                    format!("![{}]({})", escape(&alt), self.image(src, dir).replace(' ', "%20"))
                }
                None => String::new(),
            },
            BlockKind::Table(t) => table(t),
            BlockKind::Grid | BlockKind::Column { .. } => self.blocks(&b.children),
            // An add-on block: Markdown has nothing for it.
            BlockKind::Countdown(_) => String::new(),
            BlockKind::File(f) => match self.opts.assets.filter(|_| f.src_kind == "path") {
                Some(dir) => format!("[{}]({})", escape(&f.name), self.image(&f.src, dir).replace(' ', "%20")),
                None => escape(&f.name),
            },
            BlockKind::SubPageList => {
                self.opts.sub_pages.iter().map(|t| format!("- {}", escape(&crate::model::display_title(t)))).collect::<Vec<_>>().join("\n")
            }
            BlockKind::Raw { xml, .. } => match crate::docxml::raw_summary(xml) {
                (text, Some(link)) => format!("[{}]({})", escape(if text.is_empty() { &link } else { &text }), link.replace(' ', "%20")),
                (text, None) => escape(&text).replace('\n', "<br>"),
            },
        }
    }

    /// The link for an image: remote ones stay remote, local files get a
    /// copy in the assets folder (same name, numbered on clashes).
    fn image(&mut self, src: &str, dir: &str) -> String {
        if src.starts_with("http://") || src.starts_with("https://") {
            return src.to_string();
        }
        let Some(path) = crate::store::image_path(src) else { return src.to_string() };
        if let Some((_, rel)) = self.files.iter().find(|(p, _)| *p == path) {
            return rel.clone();
        }
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("image").to_string();
        let ext = path.extension().and_then(|s| s.to_str()).map(|e| format!(".{e}")).unwrap_or_default();
        let mut rel = format!("{dir}/{stem}{ext}");
        let mut n = 1;
        while self.files.iter().any(|(_, r)| *r == rel) {
            n += 1;
            rel = format!("{dir}/{stem}-{n}{ext}");
        }
        self.files.push((path, rel.clone()));
        rel
    }
}

fn table(t: &Table) -> String {
    let cols = t.rows.iter().map(|r| r.len()).max().unwrap_or(0).max(1);
    let cell = |c: &TableCell| {
        let parts: Vec<String> = c.blocks.iter().map(|b| inline(&b.text)).filter(|s| !s.is_empty()).collect();
        parts.join("<br>").replace('|', "\\|")
    };
    let row = |r: &Vec<TableCell>| {
        let mut cells: Vec<String> = r.iter().map(cell).collect();
        cells.resize(cols, String::new());
        format!("| {} |", cells.join(" | "))
    };
    let mut lines = vec![];
    let mut rows = t.rows.iter();
    match rows.next() {
        Some(r) => lines.push(row(r)),
        None => lines.push(row(&vec![])),
    }
    lines.push(format!("|{}", " --- |".repeat(cols)));
    lines.extend(rows.map(row));
    lines.join("\n")
}

fn indent(s: &str, prefix: &str) -> String {
    s.lines()
        .map(|l| if l.is_empty() && prefix.trim().is_empty() { String::new() } else { format!("{prefix}{l}").trim_end().to_string() })
        .collect::<Vec<_>>()
        .join("\n")
}

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if matches!(c, '\\' | '*' | '_' | '`' | '[' | ']' | '<' | '~') {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

#[derive(PartialEq, Clone, Default)]
struct Md {
    bold: bool,
    italic: bool,
    strike: bool,
    underline: bool,
    code: bool,
    link: Option<String>,
}

fn inline(t: &RichText) -> String {
    // Merge runs that only differ in marks Markdown can't express (colors).
    let mut runs: Vec<(Md, String)> = vec![];
    for (r, m) in t.spans() {
        let s = match m.inline.as_ref().and_then(Inline::equation) {
            Some(src) => t.text[r].chars().map(|_| format!("${src}$")).collect(),
            None => match &m.inline {
                Some(Inline { kind: InlineKind::File(f), .. }) => t.text[r].chars().map(|_| f.name.clone()).collect(),
                Some(obj) => {
                    // An 打开超链接 button exports as a link.
                    let (text, link) = crate::docxml::raw_summary(&obj.xml);
                    let s = match link.filter(|_| matches!(obj.kind, InlineKind::Button(_))) {
                        Some(l) => format!("[{text}]({l})"),
                        None => text,
                    };
                    t.text[r].chars().map(|_| s.clone()).collect()
                }
                None => t.text[r].to_string(),
            },
        };
        let md = Md {
            bold: m.bold,
            italic: m.italic,
            strike: m.strike,
            underline: m.underline,
            code: m.code,
            link: m.link.clone(),
        };
        match runs.last_mut() {
            Some((last, text)) if *last == md => text.push_str(&s),
            _ => runs.push((md, s)),
        }
    }
    let mut out = String::new();
    for (m, s) in runs {
        if s.is_empty() {
            continue;
        }
        // Emphasis markers must hug the text: keep outer spaces outside.
        let body = s.trim();
        let lead = &s[..s.len() - s.trim_start().len()];
        let trail = &s[s.trim_end().len()..];
        if body.is_empty() {
            out.push_str(&s);
            continue;
        }
        let mut x = if m.code {
            let tick = if body.contains('`') { "``" } else { "`" };
            format!("{tick}{body}{tick}")
        } else {
            escape(body)
        };
        if m.underline {
            x = format!("<u>{x}</u>");
        }
        if m.strike {
            x = format!("~~{x}~~");
        }
        if m.italic {
            x = format!("*{x}*");
        }
        if m.bold {
            x = format!("**{x}**");
        }
        if let Some(url) = &m.link {
            x = format!("[{x}]({})", url.replace(' ', "%20").replace(')', "%29"));
        }
        out.push_str(lead);
        out.push_str(&x);
        out.push_str(trail);
    }
    out.replace('\n', "<br>")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exports_common_blocks() {
        let mut bold = RichText::plain("a ");
        bold.push("bold ", Marks { bold: true, ..Default::default() });
        bold.push("link", Marks { link: Some("https://x.y".into()), ..Default::default() });
        let doc = Document {
            title: RichText::plain("标题"),
            blocks: vec![
                Block::new(BlockKind::Heading { level: 2, seq_auto: false }, RichText::plain("小节")),
                Block::new(BlockKind::Text, bold),
                Block::new(BlockKind::Ordered { seq: None }, RichText::plain("one"))
                    .with_children(vec![Block::new(BlockKind::Bullet, RichText::plain("sub"))]),
                Block::new(BlockKind::Ordered { seq: None }, RichText::plain("two")),
                Block::new(BlockKind::Todo { done: true }, RichText::plain("done")),
                Block::new(BlockKind::Code { lang: "Rust".into(), caption: None, wrap: false }, RichText::plain("fn main() {}")),
                Block::new(BlockKind::Quote, RichText::default()).with_children(vec![Block::text("q")]),
                Block::new(BlockKind::Divider, RichText::default()),
            ],
            ..Default::default()
        };
        let md = to_markdown(&doc);
        assert_eq!(
            md,
            "# 标题\n\n## 小节\n\na **bold** [link](https://x.y)\n\n1. one\n    - sub\n2. two\n- [x] done\n\n```rust\nfn main() {}\n```\n\n> q\n\n---\n"
        );
    }

    #[test]
    fn exports_tables() {
        let cell = |s: &str| TableCell { blocks: vec![Block::text(s)], ..TableCell::empty() };
        let t = Table { col_widths: vec![], header_rows: 1, header_col: false, rows: vec![vec![cell("a"), cell("b|c")], vec![cell("1")]] };
        assert_eq!(table(&t), "| a | b\\|c |\n| --- | --- |\n| 1 |  |");
    }
}
