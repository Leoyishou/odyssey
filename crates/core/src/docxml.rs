//! DocxXML (Feishu/Lark document XML) ⇄ [`Document`].
//!
//! Parsing goes through a tiny DOM so unknown elements can be re-emitted
//! verbatim as [`BlockKind::Raw`] / [`Inline`].

use crate::model::*;
use anyhow::{Result, anyhow};
use quick_xml::Reader;
use quick_xml::events::Event;

#[derive(Clone, Debug)]
enum Node {
    El(Element),
    Text(String),
}

#[derive(Clone, Debug)]
struct Element {
    name: String,
    attrs: Vec<(String, String)>,
    children: Vec<Node>,
}

impl Element {
    fn attr(&self, k: &str) -> Option<&str> {
        self.attrs.iter().find(|(a, _)| a == k).map(|(_, v)| v.as_str())
    }
}

fn parse_dom(xml: &str) -> Result<Vec<Node>> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(false);
    let mut stack: Vec<Element> = vec![Element { name: "#root".into(), attrs: vec![], children: vec![] }];
    loop {
        match reader.read_event()? {
            Event::Start(e) => {
                stack.push(Element { name: name_of(e.name().as_ref()), attrs: attrs_of(&e)?, children: vec![] });
            }
            Event::Empty(e) => {
                let el = Element { name: name_of(e.name().as_ref()), attrs: attrs_of(&e)?, children: vec![] };
                stack.last_mut().unwrap().children.push(Node::El(el));
            }
            Event::End(_) => {
                let el = stack.pop().ok_or_else(|| anyhow!("unbalanced xml"))?;
                stack.last_mut().ok_or_else(|| anyhow!("unbalanced xml"))?.children.push(Node::El(el));
            }
            Event::Text(t) => {
                let s = t.unescape()?.into_owned();
                stack.last_mut().unwrap().children.push(Node::Text(s));
            }
            Event::CData(t) => {
                let s = String::from_utf8_lossy(&t.into_inner()).into_owned();
                stack.last_mut().unwrap().children.push(Node::Text(s));
            }
            Event::Eof => break,
            _ => {}
        }
    }
    if stack.len() != 1 {
        return Err(anyhow!("unclosed element <{}>", stack.last().unwrap().name));
    }
    Ok(stack.pop().unwrap().children)
}

fn name_of(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}

fn attrs_of(e: &quick_xml::events::BytesStart) -> Result<Vec<(String, String)>> {
    let mut out = vec![];
    for a in e.attributes() {
        let a = a?;
        out.push((name_of(a.key.as_ref()), a.unescape_value()?.into_owned()));
    }
    Ok(out)
}

// ---------------------------------------------------------------- parsing

pub fn parse(xml: &str) -> Result<Document> {
    let nodes = parse_dom(xml)?;
    let mut doc = Document { blocks: vec![], ..Default::default() };
    let mut top = nodes;
    // Only known document wrappers may be unwrapped; unknown blocks keep their attributes.
    loop {
        let els: Vec<&Element> = top.iter().filter_map(|n| if let Node::El(e) = n { Some(e) } else { None }).collect();
        if els.len() == 1 && matches!(els[0].name.as_str(), "document" | "doc" | "fragment")
            && top.iter().all(|n| !matches!(n, Node::Text(t) if !t.trim().is_empty())) {
            top = els[0].children.clone();
        } else {
            break;
        }
    }
    for n in &top {
        if let Node::El(e) = n
            && e.name == "title"
        {
            doc.title = inline_text(&e.children);
            doc.icon = e.attr("icon").filter(|v| !v.is_empty()).map(str::to_string);
            doc.cover = e.attr("cover").filter(|v| !v.is_empty()).map(|src| Cover {
                src: src.to_string(),
                x: e.attr("cover-x").and_then(|v| v.parse().ok()).unwrap_or(0.),
                y: e.attr("cover-y").and_then(|v| v.parse().ok()).unwrap_or(0.),
            });
            continue;
        }
        parse_block_node(n, &mut doc.blocks);
    }
    if doc.blocks.is_empty() {
        doc.blocks.push(Block::text(""));
    }
    Ok(doc)
}

fn is_block_tag(name: &str) -> bool {
    matches!(
        name,
        "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "h7" | "h8" | "h9" | "ul" | "ol" | "li"
            | "checkbox" | "blockquote" | "hr" | "pre" | "img" | "table" | "grid" | "column"
            | "callout" | "whiteboard" | "source" | "figure" | "sheet" | "task" | "chat_card"
            | "bookmark" | "html5-block" | "okr" | "sub-page-list" | "countdown"
    )
}

/// Inline objects that may also stand alone as a top-level element (each
/// then becomes a paragraph of its own).
fn is_inline_object(name: &str) -> bool {
    name == "button"
}

fn parse_align(e: &Element) -> Align {
    match e.attr("align") {
        Some("center") => Align::Center,
        Some("right") => Align::Right,
        _ => Align::Left,
    }
}

fn parse_block_node(n: &Node, out: &mut Vec<Block>) {
    match n {
        Node::Text(t) => {
            // Stray top-level text: keep it as a paragraph unless whitespace.
            if !t.trim().is_empty() {
                out.push(Block::text(t.trim()));
            }
        }
        Node::El(e) => parse_block(e, out),
    }
}

fn parse_blocks(children: &[Node]) -> Vec<Block> {
    let mut out = vec![];
    // Runs of inline content directly inside a container become a paragraph.
    let mut pending: Vec<Node> = vec![];
    let flush = |pending: &mut Vec<Node>, out: &mut Vec<Block>| {
        if pending.iter().any(|n| match n {
            Node::Text(t) => !t.trim().is_empty(),
            Node::El(_) => true,
        }) {
            let mut b = Block::new(BlockKind::Text, inline_text(pending));
            trim_rich(&mut b.text);
            out.push(b);
        }
        pending.clear();
    };
    for n in children {
        match n {
            Node::El(e) if is_block_tag(&e.name) || e.name == "title" => {
                flush(&mut pending, &mut out);
                parse_block(e, &mut out);
            }
            other => pending.push(other.clone()),
        }
    }
    flush(&mut pending, &mut out);
    out
}

fn parse_block(e: &Element, out: &mut Vec<Block>) {
    let name = e.name.as_str();
    let mut block = match name {
        // Blocks nested in a paragraph or heading are its (indented) children.
        "p" => {
            let (inline, nested) = split_inline_and_blocks(&e.children);
            Block::new(BlockKind::Text, inline_text(&inline)).with_children(nested)
        }
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "h7" | "h8" | "h9" => {
            let level = name[1..].parse().unwrap_or(1);
            let (inline, nested) = split_inline_and_blocks(&e.children);
            Block::new(BlockKind::Heading { level, seq_auto: e.attr("seq") == Some("auto") }, inline_text(&inline))
                .with_children(nested)
        }
        "ul" | "ol" => {
            for c in &e.children {
                if let Node::El(li) = c {
                    if li.name == "li" {
                        out.push(parse_li(li, name == "ol"));
                    } else {
                        parse_block(li, out);
                    }
                }
            }
            return;
        }
        "li" => {
            out.push(parse_li(e, false));
            return;
        }
        "checkbox" => {
            let (inline, nested) = split_inline_and_blocks(&e.children);
            Block::new(BlockKind::Todo { done: e.attr("done") == Some("true") }, inline_text(&inline))
                .with_children(nested)
        }
        "blockquote" => Block::new(BlockKind::Quote, RichText::default()).with_children(non_empty(parse_blocks(&e.children))),
        "hr" => Block::new(BlockKind::Divider, RichText::default()),
        "sub-page-list" => Block::new(BlockKind::SubPageList, RichText::default()),
        "countdown" => {
            let d = Countdown::default();
            let num = |k: &str| e.attr(k).and_then(|v| v.parse::<u32>().ok()).unwrap_or(0);
            let cd = Countdown {
                duration: e.attr("duration").and_then(|v| v.parse().ok()).unwrap_or(0.),
                start: e.attr("start-time").and_then(|v| v.parse().ok()).unwrap_or(0),
                notify: e.attr("notify") != Some("false"),
                color: e.attr("color").filter(|c| COUNTDOWN_COLORS.iter().any(|k| k.eq_ignore_ascii_case(c))).map(|c| c.to_ascii_uppercase()).unwrap_or(d.color),
                by_date: e.attr("timing") == Some("date"),
                dhms: [num("days"), num("hours"), num("minutes"), num("seconds")],
                date: e.attr("date").unwrap_or_default().to_string(),
                time: e.attr("time").unwrap_or_default().to_string(),
            };
            Block::new(BlockKind::Countdown(cd), RichText::default())
        }
        "pre" => {
            let code_el = e.children.iter().find_map(|n| match n {
                Node::El(c) if c.name == "code" => Some(c),
                _ => None,
            });
            let text = match code_el {
                Some(c) => code_text(&c.children),
                None => code_text(&e.children),
            };
            Block::new(
                BlockKind::Code {
                    lang: crate::lang::from_token(e.attr("lang").unwrap_or("")),
                    caption: e.attr("caption").map(str::to_string),
                    wrap: e.attr("wrap") == Some("true"),
                },
                RichText::plain(text),
            )
        }
        "img" => {
            let (kind, src) = ["path", "href", "src"]
                .iter()
                .find_map(|k| e.attr(k).map(|v| (k.to_string(), v.to_string())))
                .unwrap_or(("src".into(), String::new()));
            Block::new(
                BlockKind::Image {
                    src_kind: kind,
                    src,
                    width: e.attr("width").and_then(|v| v.parse().ok()),
                    height: e.attr("height").and_then(|v| v.parse().ok()),
                    caption: e.attr("caption").filter(|c| !c.is_empty()).map(str::to_string),
                    crop: e.attr("crop").and_then(parse_crop),
                },
                RichText::default(),
            )
        }
        "callout" => Block::new(
            BlockKind::Callout {
                emoji: e.attr("emoji").unwrap_or("💡").to_string(),
                bg: e.attr("background-color").map(|s| ColorName(s.into())),
                border: e.attr("border-color").map(|s| ColorName(s.into())),
                text_color: e.attr("text-color").map(|s| ColorName(s.into())),
            },
            RichText::default(),
        )
        .with_children(non_empty(parse_blocks(&e.children))),
        "grid" => {
            let cols: Vec<Block> = e
                .children
                .iter()
                .filter_map(|n| match n {
                    Node::El(c) if c.name == "column" => Some(c),
                    _ => None,
                })
                .map(|c| {
                    let ratio = c.attr("width-ratio").and_then(|v| v.parse().ok()).unwrap_or(0.5);
                    Block::new(BlockKind::Column { ratio }, RichText::default())
                        .with_children(non_empty(parse_blocks(&c.children)))
                })
                .collect();
            Block::new(BlockKind::Grid, RichText::default()).with_children(cols)
        }
        "table" => Block::new(BlockKind::Table(parse_table(e)), RichText::default()),
        "source" => Block::new(BlockKind::File(file_of(e, FileView::Card)), RichText::default()),
        "figure" if figure_source(e).is_some() => {
            let view = if e.attr("view-type").is_some_and(|v| v.eq_ignore_ascii_case("preview")) { FileView::Preview } else { FileView::Card };
            Block::new(BlockKind::File(file_of(figure_source(e).unwrap_or(e), view)), RichText::default())
        }
        _ if is_inline_object(name) => Block::new(BlockKind::Text, inline_text(std::slice::from_ref(&Node::El(e.clone())))),
        _ => Block::new(BlockKind::Raw { tag: e.name.clone(), xml: serialize_el(e) }, RichText::default()),
    };
    block.align = parse_align(e);
    // Images are centred by default.
    if e.name == "img" && e.attr("align").is_none() {
        block.align = Align::Center;
    }
    block.folded = e.attr("folded") == Some("true");
    block.bg = parse_bg(e, &block.kind);
    out.push(block);
}

fn non_empty(mut v: Vec<Block>) -> Vec<Block> {
    if v.is_empty() {
        v.push(Block::text(""));
    }
    v
}

fn parse_li(li: &Element, ordered: bool) -> Block {
    let (inline, mut nested) = split_inline_and_blocks(&li.children);
    // A <li> whose only content is a <p> uses the paragraph's text.
    let mut text = inline_text(&inline);
    if text.is_empty()
        && let Some(first) = nested.first()
        && matches!(first.kind, BlockKind::Text)
    {
        text = nested.remove(0).text;
    }
    let kind = if ordered {
        let seq = li.attr("seq").and_then(|s| s.parse().ok());
        BlockKind::Ordered { seq }
    } else {
        BlockKind::Bullet
    };
    let mut b = Block::new(kind, text);
    b.align = parse_align(li);
    b.folded = li.attr("folded") == Some("true");
    b.bg = parse_bg(li, &b.kind);
    b.children = nested;
    b
}

/// A block background; a callout's own `background-color` is its fill.
fn parse_bg(e: &Element, kind: &BlockKind) -> Option<ColorName> {
    e.attr("background-color").filter(|_| kind.takes_bg()).map(|s| ColorName(s.into()))
}

fn bg_attr(b: &Block) -> String {
    match &b.bg {
        Some(c) if b.kind.takes_bg() => format!(" background-color=\"{}\"", esc_attr(&c.0)),
        _ => String::new(),
    }
}

fn split_inline_and_blocks(children: &[Node]) -> (Vec<Node>, Vec<Block>) {
    // A `<source>` inside a paragraph is an inline attachment (文本视图).
    let is_block = |n: Option<&Node>| matches!(n, Some(Node::El(e)) if is_block_tag(&e.name) && e.name != "source");
    let mut inline = vec![];
    let mut blocks = vec![];
    for (i, n) in children.iter().enumerate() {
        match n {
            Node::El(e) if is_block(Some(n)) => parse_block(e, &mut blocks),
            // Spaces before or between nested blocks are layout: table cells
            // keep the indentation but not the newlines.
            Node::Text(t) if is_block(children.get(i + 1)) => {
                inline.push(Node::Text(t.trim_end_matches([' ', '\t']).to_string()));
            }
            Node::Text(t) if !blocks.is_empty() && t.trim().is_empty() => {}
            other => inline.push(other.clone()),
        }
    }
    (inline, blocks)
}

fn parse_table(e: &Element) -> Table {
    let mut col_widths = vec![];
    let mut rows: Vec<Vec<TableCell>> = vec![];
    let mut header_rows;
    fn rows_of<'a>(e: &'a Element, out: &mut Vec<(&'a Element, bool)>, head: bool) {
        for n in &e.children {
            if let Node::El(c) = n {
                match c.name.as_str() {
                    "thead" => rows_of(c, out, true),
                    "tbody" | "tfoot" => rows_of(c, out, false),
                    "tr" => out.push((c, head)),
                    _ => {}
                }
            }
        }
    }
    for n in &e.children {
        if let Node::El(c) = n
            && c.name == "colgroup"
        {
            for col in &c.children {
                if let Node::El(col) = col {
                    let w = col.attr("width").and_then(|v| v.trim_end_matches("px").parse().ok()).unwrap_or(100.0);
                    let span: usize = col.attr("span").and_then(|v| v.parse().ok()).unwrap_or(1);
                    for _ in 0..span.max(1) {
                        col_widths.push(w);
                    }
                }
            }
        }
    }
    let mut trs = vec![];
    rows_of(e, &mut trs, false);
    // Place cells by HTML occupancy: each cell takes the next free slot of
    // its row; the slots its spans cover become `TableCell::covered`.
    let mut taken: Vec<Vec<bool>> = vec![];
    // Per row: whether it sits in <thead>, is all <th>, starts with <th>.
    let mut kinds: Vec<(bool, bool, bool)> = vec![];
    for (r, (tr, head)) in trs.iter().enumerate() {
        if rows.len() <= r {
            rows.resize_with(r + 1, Vec::new);
            taken.resize_with(r + 1, Vec::new);
        }
        // A row whose first slot a cell above covers counts as starting with <th>
        // when that cell does (checked through the row that holds it).
        let (mut all_th, mut any) = (true, false);
        let mut first_th = taken[r].first().copied().unwrap_or(false);
        let mut c = 0;
        for n in &tr.children {
            if let Node::El(td) = n
                && (td.name == "td" || td.name == "th")
            {
                while taken[r].get(c).copied().unwrap_or(false) {
                    c += 1;
                }
                if c == 0 {
                    first_th = td.name == "th";
                }
                all_th &= td.name == "th";
                any = true;
                let colspan: usize = td.attr("colspan").and_then(|v| v.parse().ok()).unwrap_or(1).max(1);
                let rowspan: usize = td.attr("rowspan").and_then(|v| v.parse().ok()).unwrap_or(1).clamp(1, trs.len() - r);
                for rr in r..r + rowspan {
                    if rows.len() <= rr {
                        taken.resize_with(rr + 1, Vec::new);
                        rows.resize_with(rr + 1, Vec::new);
                    }
                    for cc in c..c + colspan {
                        if taken[rr].len() <= cc {
                            taken[rr].resize(cc + 1, false);
                        }
                        taken[rr][cc] = true;
                        if rows[rr].len() <= cc {
                            rows[rr].resize_with(cc + 1, TableCell::covered);
                        }
                    }
                }
                rows[r][c] = TableCell {
                    blocks: non_empty(parse_blocks(&td.children)),
                    bg: td.attr("background-color").map(|s| ColorName(s.into())),
                    valign: td.attr("vertical-align").map(str::to_string),
                    colspan: colspan as u32,
                    rowspan: rowspan as u32,
                };
                c += colspan;
            }
        }
        kinds.push((*head, any && all_th, first_th));
    }
    header_rows = kinds.iter().take_while(|k| k.0).count();
    if header_rows == 0 && kinds.first().is_some_and(|k| k.1) {
        header_rows = 1;
    }
    let body = &kinds[header_rows.min(kinds.len())..];
    let header_col = !body.is_empty() && body.iter().all(|k| k.2);
    let ncols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    while col_widths.len() < ncols {
        col_widths.push(Table::DEFAULT_WIDTH);
    }
    // Slots nothing claimed (ragged rows) become empty cells.
    for (r, row) in rows.iter_mut().enumerate() {
        for (c, cell) in row.iter_mut().enumerate() {
            if cell.is_covered() && !taken[r].get(c).copied().unwrap_or(false) {
                *cell = TableCell::empty();
            }
        }
    }
    let mut t = Table { col_widths, header_rows, header_col, rows };
    t.repair();
    t
}

/// The `<source>` of a `<figure>`.
fn figure_source(e: &Element) -> Option<&Element> {
    e.children.iter().find_map(|n| match n {
        Node::El(c) if c.name == "source" => Some(c),
        _ => None,
    })
}

fn file_of(e: &Element, view: FileView) -> FileBlock {
    let (src_kind, src) = ["path", "token"]
        .iter()
        .find_map(|k| e.attr(k).map(|v| (k.to_string(), v.to_string())))
        .unwrap_or(("path".into(), String::new()));
    let name = e.attr("name").map(str::to_string).unwrap_or_else(|| {
        // A local path names itself.
        if src_kind == "path" { src.rsplit('/').next().unwrap_or_default().to_string() } else { String::new() }
    });
    FileBlock { src_kind, src, name, view }
}

fn source_xml(f: &FileBlock) -> String {
    let kind = if f.src_kind == "token" { "token" } else { "path" };
    format!("<source {kind}=\"{}\" name=\"{}\"/>", esc_attr(&f.src), esc_attr(&f.name))
}

/// An inline object of `kind`, with its canonical element.
pub fn to_inline(kind: InlineKind) -> Inline {
    let xml = match &kind {
        InlineKind::Equation(src) => format!("<latex>{}</latex>", esc(src)),
        InlineKind::File(f) => source_xml(f),
        InlineKind::Button(b) => {
            let mut xml = format!("<button action=\"{}\"", b.action.name());
            if b.action == ButtonAction::OpenLink && !b.src.is_empty() {
                xml.push_str(&format!(" src=\"{}\"", esc_attr(&b.src)));
            }
            if !b.color.is_empty() {
                xml.push_str(&format!(" background-color=\"{}\"", esc_attr(&b.color)));
            }
            xml.push_str(&format!(">{}</button>", esc(&b.text)));
            xml
        }
        InlineKind::Reminder(r) => {
            let mut xml = format!("<time expire-time=\"{}\" notify-time=\"{}\" should-notify=\"{}\"", r.expire, r.notify, r.should_notify);
            if r.whole_day {
                xml.push_str(" is-whole-day=\"true\"");
            }
            xml.push_str(&format!(">{}</time>", esc(&r.label(" "))));
            xml
        }
        InlineKind::DocLink(l) => format!("<cite type=\"doc\" doc-id=\"{}\" file-type=\"docx\"{}{}>{}</cite>", esc_attr(&l.id), if l.pending { " pending=\"true\"" } else { "" }, l.alias.as_ref().map(|a| format!(" alias=\"{}\"", esc_attr(a))).unwrap_or_default(), esc(&l.title)),
        InlineKind::Other { tag } => format!("<{tag}/>"),
    };
    // Read back, so that both halves are exactly what a reload will give.
    match parse_dom(&xml).ok().and_then(|n| n.into_iter().next()) {
        Some(Node::El(e)) => inline_of(&e),
        _ => Inline { kind, xml },
    }
}

/// An inline element read from a file: kept verbatim, understood if we can.
fn inline_of(e: &Element) -> Inline {
    let kind = match e.name.as_str() {
        "latex" => Some(InlineKind::Equation(code_text(&e.children))),
        // The attachment, as a card (its view once turned back into a block).
        "source" => Some(InlineKind::File(file_of(e, FileView::Card))),
        "button" => {
            let action = match e.attr("action") {
                Some("DuplicatePage") => ButtonAction::DuplicatePage,
                Some("FollowPage") => ButtonAction::FollowPage,
                _ => ButtonAction::OpenLink,
            };
            Some(InlineKind::Button(InlineButton {
                text: inline_text(&e.children).plain_text(),
                action,
                src: e.attr("src").unwrap_or_default().to_string(),
                color: e.attr("background-color").unwrap_or(BUTTON_DEFAULT_COLOR).to_string(),
            }))
        }
        "time" => reminder_of(e).map(InlineKind::Reminder),
        "cite" if e.attr("type") == Some("doc") => {
            e.attr("doc-id").map(|id| InlineKind::DocLink(InlineDocLink { id: id.to_string(), title: code_text(&e.children), alias: e.attr("alias").map(str::to_string), pending: e.attr("pending") == Some("true") }))
        }
        _ => None,
    };
    Inline { kind: kind.unwrap_or_else(|| InlineKind::Other { tag: e.name.clone() }), xml: serialize_el(e) }
}

fn reminder_of(e: &Element) -> Option<InlineReminder> {
    let expire: i64 = e.attr("expire-time")?.trim().parse().ok()?;
    let notify = e.attr("notify-time").and_then(|v| v.trim().parse().ok()).unwrap_or(expire);
    let whole_day = match e.attr("is-whole-day") {
        Some(v) => v == "true",
        None => {
            let (_, _, _, h, mi) = crate::time::civil(expire.max(0) as u64 / 1000);
            (h, mi) == (23, 59)
        }
    };
    Some(InlineReminder { expire, notify, should_notify: e.attr("should-notify") != Some("false"), whole_day })
}

/// What an export can show for an element we keep verbatim: its text
/// content, and the first link-like attribute (`href`, `url`, `src`, ...).
pub fn raw_summary(xml: &str) -> (String, Option<String>) {
    let Ok(nodes) = parse_dom(xml) else { return (String::new(), None) };
    let link = nodes.iter().find_map(|n| match n {
        Node::El(e) => ["href", "url", "src", "link"].iter().find_map(|k| e.attr(k)).map(str::to_string),
        Node::Text(_) => None,
    });
    (code_text(&nodes).trim().to_string(), link)
}

fn code_text(nodes: &[Node]) -> String {
    let mut s = String::new();
    for n in nodes {
        match n {
            Node::Text(t) => s.push_str(t),
            Node::El(e) if e.name == "br" => s.push('\n'),
            Node::El(e) => s.push_str(&code_text(&e.children)),
        }
    }
    s
}

fn inline_text(nodes: &[Node]) -> RichText {
    let mut out = RichText::default();
    walk_inline(nodes, &Marks::default(), &mut out);
    out
}

fn walk_inline(nodes: &[Node], marks: &Marks, out: &mut RichText) {
    for n in nodes {
        match n {
            Node::Text(t) => {
                // Source formatting (a newline plus indentation) is not
                // content; <br/> is, and so are plain edge spaces.
                // Trailing: from the first newline after the text on.
                let ws = |c: char| c.is_ascii_whitespace();
                let body_end = t.trim_end_matches(ws).len();
                let t = &t[..t[body_end..].find('\n').map_or(t.len(), |k| body_end + k)];
                // Leading: up to the last newline before the text.
                let body_start = t.len() - t.trim_start_matches(ws).len();
                let t = t[t[..body_start].rfind('\n').map_or(0, |k| k + 1)..].replace(['\n', '\r'], "");
                out.push(&t, marks.clone());
            }
            Node::El(e) => {
                let mut m = marks.clone();
                match e.name.as_str() {
                    "br" => {
                        out.push("\n", marks.clone());
                        continue;
                    }
                    "b" | "strong" => m.bold = true,
                    "em" | "i" => m.italic = true,
                    "u" => m.underline = true,
                    "del" | "s" => m.strike = true,
                    "code" => m.code = true,
                    "a" if e.attr("type") != Some("url-preview") || e.attr("href").is_some() => {
                        m.link = e.attr("href").map(str::to_string);
                    }
                    "span" => {
                        if let Some(c) = e.attr("text-color") {
                            m.text_color = Some(ColorName(c.into()));
                        }
                        if let Some(c) = e.attr("background-color") {
                            m.bg_color = Some(ColorName(c.into()));
                        }
                    }
                    _ => {
                        // Inline objects (equations, mentions, ...) are one U+FFFC
                        // each, carrying their element verbatim.
                        m.inline = Some(inline_of(e));
                        out.push("\u{fffc}", m);
                        continue;
                    }
                }
                walk_inline(&e.children, &m, out);
            }
        }
    }
}

fn trim_rich(t: &mut RichText) {
    let start = t.text.len() - t.text.trim_start_matches([' ', '\t']).len();
    let end = t.text.trim_end_matches([' ', '\t']).len();
    if start > 0 || end < t.text.len() {
        *t = t.slice(start..end.max(start));
    }
}

// ---------------------------------------------------------------- serialization

pub fn serialize(doc: &Document) -> String {
    let mut s = String::new();
    s.push_str("<title");
    if let Some(icon) = &doc.icon {
        s.push_str(&format!(" icon=\"{}\"", esc_attr(icon)));
    }
    if let Some(c) = &doc.cover {
        s.push_str(&format!(" cover=\"{}\"", esc_attr(&c.src)));
        if c.x != 0. {
            s.push_str(&format!(" cover-x=\"{}\"", c.x));
        }
        if c.y != 0. {
            s.push_str(&format!(" cover-y=\"{}\"", c.y));
        }
    }
    s.push('>');
    write_inline(&doc.title, &mut s);
    s.push_str("</title>\n");
    write_blocks(&doc.blocks, &mut s, 0);
    s
}

thread_local! {
    /// Writing a table cell, whose blocks go on one line: content newlines
    /// (code, raw elements) are written as `&#10;` so that dropping the
    /// layout newlines afterwards leaves them alone.
    static IN_CELL: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    let in_cell = IN_CELL.get();
    for c in s.chars() {
        match c {
            '<' => o.push_str("&lt;"),
            '>' => o.push_str("&gt;"),
            '&' => o.push_str("&amp;"),
            '\n' if in_cell => o.push_str("&#10;"),
            _ => o.push(c),
        }
    }
    o
}

fn esc_attr(s: &str) -> String {
    esc(s).replace('"', "&quot;")
}

fn align_attr(a: Align) -> &'static str {
    match a {
        Align::Left => "",
        Align::Center => " align=\"center\"",
        Align::Right => " align=\"right\"",
    }
}

fn indent(s: &mut String, depth: usize) {
    // A cell's blocks go on one line, unindented (older files indented them; the reader still drops that).
    if IN_CELL.get() {
        return;
    }
    for _ in 0..depth {
        s.push_str("  ");
    }
}

fn write_blocks(blocks: &[Block], s: &mut String, depth: usize) {
    let mut i = 0;
    while i < blocks.len() {
        let b = &blocks[i];
        match &b.kind {
            BlockKind::Bullet | BlockKind::Ordered { .. } => {
                let ordered = matches!(b.kind, BlockKind::Ordered { .. });
                let tag = if ordered { "ol" } else { "ul" };
                indent(s, depth);
                s.push_str(&format!("<{tag}>\n"));
                while i < blocks.len() && matches!(blocks[i].kind, BlockKind::Ordered { .. }) == ordered && blocks[i].kind.is_list() && !matches!(blocks[i].kind, BlockKind::Todo { .. }) {
                    let li = &blocks[i];
                    indent(s, depth + 1);
                    let seq = match li.kind {
                        BlockKind::Ordered { seq: Some(n) } => format!(" seq=\"{n}\""),
                        _ => String::new(),
                    };
                    s.push_str(&format!("<li{seq}{}{}{}>", align_attr(li.align), bg_attr(li), folded_attr(li.folded)));
                    write_inline(&li.text, s);
                    // An empty item's first child paragraph would be read
                    // back as the item's own text (parse_li); spell it out.
                    if li.text.is_empty() && li.children.first().is_some_and(|c| matches!(c.kind, BlockKind::Text)) {
                        s.push_str("<p></p>");
                    }
                    if !li.children.is_empty() {
                        s.push('\n');
                        write_blocks(&li.children, s, depth + 2);
                        indent(s, depth + 1);
                    }
                    s.push_str("</li>\n");
                    i += 1;
                }
                indent(s, depth);
                s.push_str(&format!("</{tag}>\n"));
                continue;
            }
            _ => write_block(b, s, depth),
        }
        i += 1;
    }
}

/// Text, heading, list and todo blocks remember being folded.
fn folded_attr(folded: bool) -> &'static str {
    if folded { " folded=\"true\"" } else { "" }
}

/// Inline content, then nested children, then the closing tag.
fn write_text_with_children(b: &Block, close: &str, s: &mut String, depth: usize) {
    write_inline(&b.text, s);
    if !b.children.is_empty() {
        s.push('\n');
        write_blocks(&b.children, s, depth + 1);
        indent(s, depth);
    }
    s.push_str(close);
    s.push('\n');
}

fn write_block(b: &Block, s: &mut String, depth: usize) {
    indent(s, depth);
    let al = format!("{}{}", align_attr(b.align), bg_attr(b));
    let fo = folded_attr(b.folded && b.kind.has_text());
    match &b.kind {
        BlockKind::Text => {
            s.push_str(&format!("<p{al}{fo}>"));
            write_text_with_children(b, "</p>", s, depth);
        }
        BlockKind::Heading { level, seq_auto } => {
            let seq = if *seq_auto { " seq=\"auto\"" } else { "" };
            s.push_str(&format!("<h{level}{seq}{al}{fo}>"));
            write_text_with_children(b, &format!("</h{level}>"), s, depth);
        }
        BlockKind::Todo { done } => {
            s.push_str(&format!("<checkbox done=\"{done}\"{al}{fo}>"));
            write_inline(&b.text, s);
            if !b.children.is_empty() {
                s.push('\n');
                write_blocks(&b.children, s, depth + 1);
                indent(s, depth);
            }
            s.push_str("</checkbox>\n");
        }
        BlockKind::Bullet | BlockKind::Ordered { .. } => unreachable!("handled by write_blocks"),
        BlockKind::Code { lang, caption, wrap } => {
            s.push_str(&format!("<pre lang=\"{}\"", esc_attr(&crate::lang::to_token(lang))));
            if let Some(c) = caption {
                s.push_str(&format!(" caption=\"{}\"", esc_attr(c)));
            }
            if *wrap {
                s.push_str(" wrap=\"true\"");
            }
            s.push_str("><code>");
            s.push_str(&esc(&b.text.text));
            s.push_str("</code></pre>\n");
        }
        BlockKind::Quote => {
            s.push_str(&format!("<blockquote{}>\n", bg_attr(b)));
            write_blocks(&b.children, s, depth + 1);
            indent(s, depth);
            s.push_str("</blockquote>\n");
        }
        BlockKind::Callout { emoji, bg, border, text_color } => {
            s.push_str(&format!("<callout emoji=\"{}\"", esc_attr(emoji)));
            if let Some(c) = bg {
                s.push_str(&format!(" background-color=\"{}\"", esc_attr(&c.0)));
            }
            if let Some(c) = border {
                s.push_str(&format!(" border-color=\"{}\"", esc_attr(&c.0)));
            }
            if let Some(c) = text_color {
                s.push_str(&format!(" text-color=\"{}\"", esc_attr(&c.0)));
            }
            s.push_str(">\n");
            write_blocks(&b.children, s, depth + 1);
            indent(s, depth);
            s.push_str("</callout>\n");
        }
        BlockKind::Divider => s.push_str("<hr/>\n"),
        BlockKind::SubPageList => s.push_str("<sub-page-list/>\n"),
        BlockKind::Countdown(c) => {
            s.push_str(&format!(
                "<countdown duration=\"{}\" start-time=\"{}\" notify=\"{}\" color=\"{}\" timing=\"{}\"",
                fmt_num(c.duration as f32),
                c.start,
                c.notify,
                esc_attr(&c.color),
                if c.by_date { "date" } else { "duration" }
            ));
            if c.dhms != [0; 4] {
                let [d, h, m, sec] = c.dhms;
                s.push_str(&format!(" days=\"{d}\" hours=\"{h}\" minutes=\"{m}\" seconds=\"{sec}\""));
            }
            if !c.date.is_empty() {
                s.push_str(&format!(" date=\"{}\" time=\"{}\"", esc_attr(&c.date), esc_attr(&c.time)));
            }
            s.push_str("/>\n");
        }
        BlockKind::File(f) => {
            let view = if f.view == FileView::Preview { "Preview" } else { "Card" };
            s.push_str(&format!("<figure view-type=\"{view}\"{al}>{}</figure>\n", source_xml(f)));
        }
        BlockKind::Image { src_kind, src, width, height, caption, crop } => {
            s.push_str(&format!("<img {}=\"{}\"", src_kind, esc_attr(src)));
            if let Some(w) = width {
                s.push_str(&format!(" width=\"{}\"", fmt_num(*w)));
            }
            if let Some(h) = height {
                s.push_str(&format!(" height=\"{}\"", fmt_num(*h)));
            }
            if let Some(c) = caption.as_ref().filter(|c| !c.is_empty()) {
                s.push_str(&format!(" caption=\"{}\"", esc_attr(c)));
            }
            if let Some(c) = crop {
                let v: Vec<String> = c.iter().map(|x| format!("{}", (x * 10000.).round() / 10000.)).collect();
                s.push_str(&format!(" crop=\"{}\"", v.join(",")));
            }
            // Written even when left: a missing align reads as centred.
            s.push_str(match b.align {
                Align::Left => " align=\"left\"",
                Align::Center => " align=\"center\"",
                Align::Right => " align=\"right\"",
            });
            s.push_str("/>\n");
        }
        BlockKind::Table(t) => write_table(t, s, depth),
        BlockKind::Grid => {
            s.push_str("<grid>\n");
            for col in &b.children {
                let ratio = if let BlockKind::Column { ratio } = col.kind { ratio } else { 0.5 };
                indent(s, depth + 1);
                s.push_str(&format!("<column width-ratio=\"{}\">\n", fmt_num(ratio)));
                write_blocks(&col.children, s, depth + 2);
                indent(s, depth + 1);
                s.push_str("</column>\n");
            }
            indent(s, depth);
            s.push_str("</grid>\n");
        }
        BlockKind::Column { ratio } => {
            s.push_str(&format!("<column width-ratio=\"{}\">\n", fmt_num(*ratio)));
            write_blocks(&b.children, s, depth + 1);
            indent(s, depth);
            s.push_str("</column>\n");
        }
        BlockKind::Raw { xml, .. } => {
            if IN_CELL.get() {
                s.push_str(&xml.replace('\n', "&#10;"));
            } else {
                s.push_str(xml);
            }
            s.push('\n');
        }
    }
}

/// `crop="l,t,r,b"`: fractions of the full image, ordered and in 0..1.
fn parse_crop(v: &str) -> Option<[f32; 4]> {
    let n: Vec<f32> = v.split(',').filter_map(|x| x.trim().parse().ok()).collect();
    let c: [f32; 4] = n.try_into().ok()?;
    let ok = c.iter().all(|x| (0. ..=1.).contains(x)) && c[0] < c[2] && c[1] < c[3];
    (ok && c != [0., 0., 1., 1.]).then_some(c)
}

fn fmt_num(v: f32) -> String {
    if v.fract() == 0.0 { format!("{}", v as i64) } else { format!("{}", (v * 1000.0).round() / 1000.0) }
}

fn write_table(t: &Table, s: &mut String, depth: usize) {
    s.push_str("<table>\n");
    indent(s, depth + 1);
    s.push_str("<colgroup>");
    for w in &t.col_widths {
        s.push_str(&format!("<col width=\"{}\"/>", fmt_num(*w)));
    }
    s.push_str("</colgroup>\n");
    let write_rows = |rows: &[Vec<TableCell>], head: bool, s: &mut String| {
        for row in rows {
            indent(s, depth + 2);
            s.push_str("<tr>");
            for (c, cell) in row.iter().enumerate() {
                if cell.is_covered() {
                    continue;
                }
                let tag = if head || (c == 0 && t.header_col) { "th" } else { "td" };
                s.push_str(&format!("<{tag}"));
                if let Some(c) = &cell.bg {
                    s.push_str(&format!(" background-color=\"{}\"", esc_attr(&c.0)));
                }
                if let Some(v) = &cell.valign {
                    s.push_str(&format!(" vertical-align=\"{}\"", esc_attr(v)));
                }
                if cell.colspan > 1 {
                    s.push_str(&format!(" colspan=\"{}\"", cell.colspan));
                }
                if cell.rowspan > 1 {
                    s.push_str(&format!(" rowspan=\"{}\"", cell.rowspan));
                }
                s.push('>');
                let mut inner = String::new();
                let outer = IN_CELL.replace(true);
                write_blocks(&cell.blocks, &mut inner, 0);
                IN_CELL.set(outer);
                s.push_str(&inner.replace('\n', ""));
                s.push_str(&format!("</{tag}>"));
            }
            s.push_str("</tr>\n");
        }
    };
    let h = t.header_rows.min(t.rows.len());
    if h > 0 {
        indent(s, depth + 1);
        s.push_str("<thead>\n");
        write_rows(&t.rows[..h], true, s);
        indent(s, depth + 1);
        s.push_str("</thead>\n");
    }
    indent(s, depth + 1);
    s.push_str("<tbody>\n");
    write_rows(&t.rows[h..], false, s);
    indent(s, depth + 1);
    s.push_str("</tbody>\n");
    indent(s, depth);
    s.push_str("</table>\n");
}

/// Inline wrappers for one run, outermost first. Runs that share a prefix of
/// wrappers share the elements, so a colored paragraph is one `<span>`.
fn wrappers(m: &Marks) -> Vec<(String, &'static str)> {
    let mut w = vec![];
    if m.text_color.is_some() || m.bg_color.is_some() {
        let mut open = "<span".to_string();
        if let Some(c) = &m.text_color {
            open.push_str(&format!(" text-color=\"{}\"", esc_attr(&c.0)));
        }
        if let Some(c) = &m.bg_color {
            open.push_str(&format!(" background-color=\"{}\"", esc_attr(&c.0)));
        }
        open.push('>');
        w.push((open, "</span>"));
    }
    if let Some(href) = &m.link {
        w.push((format!("<a href=\"{}\">", esc_attr(href)), "</a>"));
    }
    for (on, o, c) in [
        (m.bold, "<b>", "</b>"),
        (m.italic, "<em>", "</em>"),
        (m.underline, "<u>", "</u>"),
        (m.strike, "<del>", "</del>"),
        (m.code, "<code>", "</code>"),
    ] {
        if on {
            w.push((o.to_string(), c));
        }
    }
    w
}

fn write_inline(t: &RichText, s: &mut String) {
    let mut stack: Vec<(String, &'static str)> = vec![];
    let close_to = |stack: &mut Vec<(String, &'static str)>, n: usize, s: &mut String| {
        while stack.len() > n {
            s.push_str(stack.pop().unwrap().1);
        }
    };
    for (range, m) in t.spans() {
        if let Some(obj) = &m.inline {
            close_to(&mut stack, 0, s);
            // Adjacent identical objects share one run.
            for _ in t.text[range].chars() {
                s.push_str(&obj.xml);
            }
            continue;
        }
        let want = wrappers(m);
        let keep = stack.iter().zip(&want).take_while(|(a, b)| a == b).count();
        close_to(&mut stack, keep, s);
        for w in &want[keep..] {
            s.push_str(&w.0);
            stack.push(w.clone());
        }
        let mut first = true;
        for part in t.text[range].split('\n') {
            if !first {
                s.push_str("<br/>");
            }
            first = false;
            s.push_str(&esc(part));
        }
    }
    close_to(&mut stack, 0, s);
}

fn serialize_el(e: &Element) -> String {
    let mut s = format!("<{}", e.name);
    for (k, v) in &e.attrs {
        s.push_str(&format!(" {}=\"{}\"", k, esc_attr(v)));
    }
    if e.children.is_empty() {
        s.push_str("/>");
        return s;
    }
    s.push('>');
    for c in &e.children {
        match c {
            Node::Text(t) => s.push_str(&esc(t)),
            Node::El(c) => s.push_str(&serialize_el(c)),
        }
    }
    s.push_str(&format!("</{}>", e.name));
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"<title>示例 <b>文档</b></title>
<h1 seq="auto">一级标题</h1>
<p>普通 <b>加粗</b> <em>斜体</em> <span text-color="red" background-color="light-yellow">彩色</span> <a href="https://a.com/?x=1&amp;y=2">链接</a> <code>code</code><br/>第二行</p>
<ul>
  <li>一<ul><li>一.一</li></ul></li>
  <li>二</li>
</ul>
<ol><li seq="3">三</li><li>四</li></ol>
<checkbox done="true">完成</checkbox>
<blockquote><p>引用</p></blockquote>
<pre lang="rust" caption="例"><code>fn main() {
    println!("&lt;hi&gt;");
}</code></pre>
<hr/>
<callout emoji="💡" background-color="light-orange" border-color="orange"><p>提示</p></callout>
<grid><column width-ratio="0.5"><p>左</p></column><column width-ratio="0.5"><p>右</p></column></grid>
<table><colgroup><col width="120"/><col width="80"/></colgroup><thead><tr><th background-color="light-gray"><p>A</p></th><th><p>B</p></th></tr></thead><tbody><tr><td><p>1</p></td><td><p>2</p></td></tr></tbody></table>
<img href="https://x/y.png" width="300" caption="图"/>
<whiteboard type="blank"></whiteboard>
<p>公式 <latex>E=mc^2</latex> 与 <cite type="user" user-id="ou_1"/></p>
"#;

    #[test]
    fn icon_and_cover_round_trip() {
        let x = "<title icon=\"🐱\" cover=\"/a/b &amp; c.jpg\" cover-y=\"0.125\">T</title>\n<p>x</p>\n";
        let doc = parse(x).unwrap();
        assert_eq!(doc.icon.as_deref(), Some("🐱"));
        assert_eq!(doc.cover, Some(Cover { src: "/a/b & c.jpg".into(), x: 0., y: 0.125 }));
        assert_eq!(serialize(&doc), x);
        assert_eq!(serialize(&parse("<title>T</title>").unwrap()), "<title>T</title>\n<p></p>\n");
    }

    #[test]
    fn parse_sample() {
        let doc = parse(SAMPLE).unwrap();
        assert_eq!(doc.title.text, "示例 文档");
        let kinds: Vec<String> = doc.blocks.iter().map(|b| format!("{:?}", std::mem::discriminant(&b.kind))).collect();
        assert_eq!(doc.blocks.len(), 16, "{kinds:?}");
        let p = &doc.blocks[1];
        assert_eq!(p.text.text, "普通 加粗 斜体 彩色 链接 code\n第二行");
        assert!(matches!(doc.blocks[2].kind, BlockKind::Bullet));
        assert_eq!(doc.blocks[2].children.len(), 1);
        assert!(matches!(doc.blocks[4].kind, BlockKind::Ordered { seq: Some(3) }));
        assert!(matches!(doc.blocks[6].kind, BlockKind::Todo { done: true }));
        if let BlockKind::Code { lang, .. } = &doc.blocks[8].kind {
            assert_eq!(lang, "Rust");
        }
        assert!(doc.blocks[8].text.text.contains("println!(\"<hi>\");"));
        if let BlockKind::Table(t) = &doc.blocks[12].kind {
            assert_eq!(t.header_rows, 1);
            assert_eq!(t.col_widths, vec![120.0, 80.0]);
        } else {
            panic!("table expected");
        }
    }

    #[test]
    fn nested_text_children_and_folds_round_trip() {
        let src = "<title>T</title><p folded=\"true\">p<p>child</p></p><h2>h<ul><li>x</li></ul></h2><ul><li folded=\"true\">a<ul><li>b</li></ul></li></ul><checkbox done=\"false\" folded=\"true\">t<p>c</p></checkbox>";
        let doc = parse(src).unwrap();
        assert_eq!(doc.blocks.len(), 4);
        assert!(doc.blocks.iter().all(|b| !b.children.is_empty()));
        assert_eq!(doc.blocks.iter().map(|b| b.folded).collect::<Vec<_>>(), vec![true, false, true, true]);
        let xml = serialize(&doc);
        assert!(xml.contains("<p folded=\"true\">p\n  <p>child</p>\n</p>") || xml.contains("<p folded=\"true\">p<p>child</p></p>"), "{xml}");
        assert_eq!(serialize(&parse(&xml).unwrap()), xml);
    }

    #[test]
    fn block_background_round_trips() {
        let src = "<title>T</title><p background-color=\"light-blue\">p</p><ul><li background-color=\"light-red\">a</li></ul><blockquote background-color=\"light-gray\"><p>q</p></blockquote><hr background-color=\"light-red\"/>";
        let doc = parse(src).unwrap();
        let bgs: Vec<_> = doc.blocks.iter().map(|b| b.bg.as_ref().map(|c| c.0.clone())).collect();
        assert_eq!(bgs, vec![Some("light-blue".into()), Some("light-red".into()), Some("light-gray".into()), None]);
        let xml = serialize(&doc);
        assert!(xml.contains("<p background-color=\"light-blue\">p</p>"), "{xml}");
        assert!(xml.contains("<blockquote background-color=\"light-gray\">"), "{xml}");
        assert_eq!(serialize(&parse(&xml).unwrap()), xml);
    }

    /// `FDL_PERF_DOC=<file.xml> cargo test --release -- --ignored parse_and_serialize_speed --nocapture`
    #[test]
    #[ignore]
    fn parse_and_serialize_speed() {
        let path = std::env::var("FDL_PERF_DOC").expect("FDL_PERF_DOC");
        let xml = std::fs::read_to_string(path).unwrap();
        let t = std::time::Instant::now();
        let doc = parse(&xml).unwrap();
        let parsed = t.elapsed();
        let t = std::time::Instant::now();
        let out = serialize(&doc);
        let serialized = t.elapsed();
        let t = std::time::Instant::now();
        let copy = doc.clone();
        let cloned = t.elapsed();
        eprintln!("{} bytes: parse {parsed:?}, serialize {serialized:?}, clone {cloned:?}", out.len());
        drop(copy);
    }

    #[test]
    fn cell_content_keeps_its_newlines() {
        let src = "<title>T</title><table><colgroup><col width=\"100\"/></colgroup><tbody><tr><td><pre lang=\"rust\"><code>fn a() {\n    1\n}</code></pre><p>x<br/>y</p><chat_card id=\"c\">a\nb</chat_card></td></tr></tbody></table><pre lang=\"rust\"><code>top\nlevel</code></pre>";
        let doc = parse(src).unwrap();
        let xml = serialize(&doc);
        let doc2 = parse(&xml).unwrap();
        let BlockKind::Table(t) = &doc2.blocks[0].kind else { panic!("{xml}") };
        let cell = &t.rows[0][0].blocks;
        assert_eq!(cell[0].text.text, "fn a() {\n    1\n}", "{xml}");
        assert_eq!(cell[1].text.text, "x\ny", "{xml}");
        assert!(matches!(&cell[2].kind, BlockKind::Raw { xml, .. } if xml.contains("a\nb")), "{xml}");
        // Outside tables code keeps literal newlines.
        assert!(xml.contains("<code>top\nlevel</code>"), "{xml}");
        assert_eq!(serialize(&doc2), xml);
    }

    #[test]
    fn round_trip_is_stable() {
        let doc = parse(SAMPLE).unwrap();
        let xml = serialize(&doc);
        let doc2 = parse(&xml).unwrap();
        let xml2 = serialize(&doc2);
        assert_eq!(xml, xml2);
        assert_eq!(doc.plain_text(), doc2.plain_text());
        assert!(xml.contains("<latex>E=mc^2</latex>"));
        assert!(xml.contains("<cite type=\"user\" user-id=\"ou_1\"/>"));
        assert!(xml.contains("<whiteboard type=\"blank\"/>"));
        assert!(xml.contains("<span text-color=\"red\" background-color=\"light-yellow\">彩色</span>"));
    }

    /// The object `o` written out and read back.
    fn reread(o: &Inline) -> InlineKind {
        let doc = parse(&format!("<title>T</title><p>{}</p>", o.xml)).unwrap();
        doc.blocks[0].text.runs[0].marks.inline.clone().unwrap().kind
    }

    #[test]
    fn equations_are_single_objects() {
        let doc = parse("<title>T</title><p>a<latex>x&lt;1</latex><latex>x&lt;1</latex>b</p>").unwrap();
        let t = &doc.blocks[0].text;
        assert_eq!(t.text, "a\u{fffc}\u{fffc}b");
        let (_, m) = t.spans().nth(1).unwrap();
        assert_eq!(m.inline.as_ref().unwrap().equation(), Some("x<1"));
        assert!(serialize(&doc).contains("a<latex>x&lt;1</latex><latex>x&lt;1</latex>b"));
        assert_eq!(reread(&to_inline(InlineKind::Equation("a&b<c>".into()))), InlineKind::Equation("a&b<c>".into()));
        assert_eq!(doc.plain_text(), "ax<1x<1b");
    }

    #[test]
    fn buttons_are_inline_objects() {
        let doc = parse("<title>T</title><p>a<button action=\"OpenLink\" src=\"https://x.cn/?a=1&amp;b\" background-color=\"deep-red\">去 &lt;看&gt;</button>b</p><button action=\"DuplicatePage\">复制</button>").unwrap();
        assert_eq!(doc.blocks.len(), 2);
        let t = &doc.blocks[0].text;
        assert_eq!(t.text, "a\u{fffc}b");
        let (_, m) = t.spans().nth(1).unwrap();
        let b = m.inline.as_ref().unwrap().button().unwrap().clone();
        assert_eq!((b.text.as_str(), b.action, b.src.as_str(), b.color.as_str()), ("去 <看>", ButtonAction::OpenLink, "https://x.cn/?a=1&b", "deep-red"));
        assert_eq!(reread(&to_inline(InlineKind::Button(b.clone()))), InlineKind::Button(b));
        let (_, m) = doc.blocks[1].text.spans().next().unwrap();
        assert_eq!(m.inline.as_ref().unwrap().button().unwrap().action, ButtonAction::DuplicatePage);
        let xml = serialize(&doc);
        assert_eq!(serialize(&parse(&xml).unwrap()), xml);
    }

    #[test]
    fn doc_links_round_trip() {
        let doc = parse("<title>T</title><p>见 <cite type=\"doc\" doc-id=\"abc\" file-type=\"docx\">A &amp; B</cite></p>").unwrap();
        let (_, m) = doc.blocks[0].text.spans().find(|(_, m)| m.inline.is_some()).unwrap();
        let l = m.inline.as_ref().unwrap().doc_link().unwrap().clone();
        assert_eq!(l, InlineDocLink { id: "abc".into(), title: "A & B".into(), alias: None, pending: false });
        assert_eq!(reread(&to_inline(InlineKind::DocLink(l.clone()))), InlineKind::DocLink(l));
        assert!(serialize(&doc).contains("<cite type=\"doc\" doc-id=\"abc\" file-type=\"docx\">A &amp; B</cite>"));
    }

    #[test]
    fn doc_link_alias_survives_xml_and_table_round_trip() {
        let l = InlineDocLink { id: "stable-target".into(), title: "目标 & 标题".into(), alias: Some("显示 <别名>".into()), pending: false };
        let obj = to_inline(InlineKind::DocLink(l.clone()));
        assert_eq!(reread(&obj), InlineKind::DocLink(l));
        let xml = format!("<title>T</title><table><tbody><tr><td><p>{}</p></td></tr></tbody></table>", obj.xml);
        let doc = parse(&xml).unwrap();
        let saved = serialize(&doc);
        assert_eq!(serialize(&parse(&saved).unwrap()), saved);
        assert_eq!(doc.doc_links()[0].target, "stable-target");
        assert!(saved.contains("alias=\"显示 &lt;别名&gt;\""));
    }

    #[test]
    fn pending_doc_link_is_not_lost_on_restart() {
        let l = InlineDocLink { id: "reserved-id".into(), title: "尚未创建".into(), alias: Some("别名".into()), pending: true };
        let obj = to_inline(InlineKind::DocLink(l.clone()));
        assert_eq!(reread(&obj), InlineKind::DocLink(l));
        let doc = parse(&format!("<title>T</title><p>{}</p>", obj.xml)).unwrap();
        assert_eq!(doc.doc_links()[0].pending_title.as_deref(), Some("尚未创建"));
    }

    #[test]
    fn reminders_round_trip() {
        let doc = parse("<title>T</title><p>交付 <time expire-time=\"1761200000000\" notify-time=\"1761199700000\" should-notify=\"false\">x</time> 前</p>").unwrap();
        let (_, m) = doc.blocks[0].text.spans().find(|(_, m)| m.inline.is_some()).unwrap();
        let r = *m.inline.as_ref().unwrap().reminder().unwrap();
        assert_eq!((r.expire, r.notify, r.should_notify), (1761200000000, 1761199700000, false));
        assert_eq!(reread(&to_inline(InlineKind::Reminder(r))), InlineKind::Reminder(r));
        let w = to_inline(InlineKind::Reminder(InlineReminder { whole_day: true, should_notify: true, ..r }));
        assert!(w.xml.contains("is-whole-day=\"true\""));
        assert_eq!(reread(&w), w.kind);
        let xml = serialize(&doc);
        assert_eq!(serialize(&parse(&xml).unwrap()), xml);
    }

    #[test]
    fn adjacent_runs_share_wrappers() {
        let xml = "<title>T</title><p><span text-color=\"red\">a <b>b</b> <a href=\"https://x\">c</a></span><b>d<em>e</em></b></p>";
        let out = serialize(&parse(xml).unwrap());
        assert!(out.contains("<p><span text-color=\"red\">a <b>b</b> <a href=\"https://x\">c</a></span><b>d<em>e</em></b></p>"), "{out}");
    }

    #[test]
    fn table_spans_and_header_column() {
        let x = "<title>T</title>\n<table>\n  <colgroup><col width=\"120\"/><col width=\"100\"/><col width=\"100\"/></colgroup>\n  <thead>\n    <tr><th colspan=\"2\"><p>a</p></th><th><p>c</p></th></tr>\n  </thead>\n  <tbody>\n    <tr><th rowspan=\"2\"><p>d</p></th><td><p>e</p></td><td><p>f</p></td></tr>\n    <tr><td><p>h</p></td><td><p>i</p></td></tr>\n  </tbody>\n</table>\n";
        let doc = parse(x).unwrap();
        let BlockKind::Table(t) = &doc.blocks[0].kind else { panic!() };
        assert_eq!((t.header_rows, t.header_col, t.ncols()), (1, true, 3));
        assert!(t.rows[0][1].is_covered() && t.rows[2][0].is_covered());
        assert_eq!(t.rows[2][1].blocks[0].text.text, "h");
        let out = serialize(&doc);
        assert!(out.contains("<tr><th rowspan=\"2\"><p>d</p></th><td><p>e</p></td><td><p>f</p></td></tr>"), "{out}");
        assert!(out.contains("<tr><th colspan=\"2\"><p>a</p></th><th><p>c</p></th></tr>"), "{out}");
        let again = parse(&out).unwrap();
        let BlockKind::Table(t2) = &again.blocks[0].kind else { panic!() };
        assert_eq!((t2.header_rows, t2.header_col, t2.rows.len()), (1, true, 3));
    }

    #[test]
    fn files_round_trip() {
        let x = "<title>T</title>\n<figure view-type=\"Preview\" align=\"center\"><source path=\"/a/报告 &amp; 1.docx\" name=\"报告 &amp; 1.docx\"/></figure>\n<figure view-type=\"Card\"><source token=\"boxcnX\" name=\"b.zip\"/></figure>\n<p>见 <source path=\"/a/c.pdf\" name=\"c.pdf\"/> 附件</p>\n";
        let doc = parse(x).unwrap();
        let BlockKind::File(f) = &doc.blocks[0].kind else { panic!("file expected") };
        assert_eq!((f.src.as_str(), f.name.as_str(), f.view), ("/a/报告 & 1.docx", "报告 & 1.docx", FileView::Preview));
        assert_eq!(doc.blocks[0].align, Align::Center);
        let BlockKind::File(f) = &doc.blocks[1].kind else { panic!("file expected") };
        assert_eq!((f.src_kind.as_str(), f.view), ("token", FileView::Card));
        let (_, m) = doc.blocks[2].text.spans().find(|(_, m)| m.inline.is_some()).unwrap();
        assert_eq!(m.inline.as_ref().unwrap().file().unwrap().name, "c.pdf");
        assert_eq!(serialize(&doc), x);
        // A bare `<source>` block is a card named after its path.
        let doc = parse("<title>T</title><source path=\"/x/y.mp4\"/>").unwrap();
        let BlockKind::File(f) = &doc.blocks[0].kind else { panic!("file expected") };
        assert_eq!((f.name.as_str(), f.view), ("y.mp4", FileView::Card));
    }

    /// Every block kind, in the canonical saved form: loading and saving it changes nothing.
    #[test]
    fn fixture_round_trips() {
        let x = include_str!("../tests/fixtures/all-blocks.xml");
        let doc = parse(x).unwrap();
        assert_eq!(serialize(&doc), x);
        let text = doc.plain_text();
        for s in ["被折叠的段落", "尾随空格 \n", "\n \n", "fn main() {\n    println!(\"<hi> & 'q'\");\n\n}", "a = 1\nb = 2", "\\frac{1}{2}", "空项下的段落"] {
            assert!(text.contains(s), "{s:?} missing from {text:?}");
        }
    }

    /// Older versions indented the blocks inside a cell; that layout is not content.
    #[test]
    fn indented_cells_from_older_versions() {
        let doc = parse("<table><tbody><tr><td><ul>  <li>a    <p>b</p>  </li></ul><p>c  <p>d</p></p></td></tr></tbody></table>").unwrap();
        let x = serialize(&doc);
        assert!(x.contains("<td><ul><li>a<p>b</p></li></ul><p>c<p>d</p></p></td>"), "{x}");
    }

    /// Round-trips every document of a real library: FDL_ROUNDTRIP_DIR=<library>/docs cargo test -- --ignored
    #[test]
    #[ignore]
    fn library_round_trips() {
        let dir = std::env::var("FDL_ROUNDTRIP_DIR").expect("FDL_ROUNDTRIP_DIR");
        for e in std::fs::read_dir(dir).unwrap() {
            let path = e.unwrap().path();
            if path.extension().is_none_or(|x| x != "xml") {
                continue;
            }
            let src = std::fs::read_to_string(&path).unwrap();
            let doc = parse(&src).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            let x = serialize(&doc);
            let again = parse(&x).unwrap();
            assert_eq!(serialize(&again), x, "{} is not stable", path.display());
            assert_eq!(again.plain_text(), doc.plain_text(), "{} lost text", path.display());
        }
    }
}
