//! The document editor entity: state, keyboard and mouse handling, and the
//! platform input handler (typing / IME). Rendering lives in `render.rs` and
//! the floating panels in `overlays.rs`.

use super::core::{self, Core, Pos, Selection, TITLE};
use super::keymap as k;
use super::menus::{self, SlashKind};
use super::text::{LayoutCache, hit_test};
use crate::docxml;
use crate::theme;
use crate::input::{TextField, TextFieldEvent};
use crate::model::*;
use crate::store::PageWidth;
use gpui::*;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::rc::Rc;
use std::time::Duration;

pub enum EditorEvent {
    Changed,
    /// ⌘S: write to disk now instead of waiting for the autosave debounce.
    Save,
    /// Toolbar 更多 → 文档库内搜索: the search dialog with this query.
    Search(String),
    /// 展开目录 / 收起目录, remembered per document.
    Catalogue(bool),
    /// A 子页面列表 item or an @文档 link was clicked.
    OpenDoc(String),
    OpenPending(String, String),
    /// A 反向引用 line: that document, scrolled to its nth block.
    OpenDocAt(String, usize),
    /// The reference list's close button.
    HideRefs,
    /// Switch to 阅读模式 (true) / 编辑模式 (false), e.g. "E" while reading.
    Mode(bool),
    /// 选择要展示的文档信息 / 隐藏文档信息 changed the info row.
    Info(crate::store::InfoRow),
    /// The editor showed a toast (the workspace drops its own).
    Toast,
    /// A 创建副本 button was clicked.
    Duplicate,
}

/// A child page shown by 子页面列表 blocks.
#[derive(Clone, Debug, PartialEq, Hash)]
pub struct SubPage {
    pub id: String,
    pub title: String,
    pub icon: Option<String>,
}

/// Open slash menu: `trigger` is the byte offset of `/` (or `、`), `start`
/// the offset right after it; the query is `text[start..caret]`.
#[derive(Clone, Debug)]
pub struct Slash {
    pub block: BlockId,
    pub trigger: usize,
    pub start: usize,
    pub selected: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockSub {
    Indent,
    Color,
    /// 在下方添加: the insert panel, with its highlighted item.
    Add(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Menu {
    /// "+" on an empty line: slash items without a trigger.
    Add { block: BlockId, selected: usize },
    /// Handle click.
    Block { block: BlockId, sub: Option<BlockSub> },
    /// Toolbar dropdowns.
    TurnInto,
    OrderedNumber { block: BlockId, pos: Point<Pixels>, edit: bool, depth: usize },
    Align,
    Color,
    /// Link panel; `range` is empty when inserting a new link, `only_link`
    /// asks for the URL of a selection.
    Link { block: BlockId, range: Range<usize>, only_link: bool },
    /// Toolbar 更多.
    TbMore,
    Lang { block: BlockId, selected: usize },
    CalloutColor { block: BlockId, section: usize },
    /// Emoji picker; `selected` is the highlighted search result.
    Emoji { target: super::emoji_picker::EmojiTarget, selected: usize },
    /// Table toolbar: 单元格背景颜色.
    CellBg,
    /// The LaTeX editor of the equation at `offset`.
    Equation { block: BlockId, offset: usize },
    /// 编辑封面 and its 选择图片 panel (官方图库 / 本地上传).
    CoverEdit,
    CoverPick { upload: bool },
    /// The info row's handle: 选择要展示的文档信息.
    /// 按钮设置 of the button at `offset`.
    Button { block: BlockId, offset: usize },
    /// The 日期提醒 at `offset`.
    Reminder { block: BlockId, offset: usize },
}

pub type BoundsMap = Rc<RefCell<HashMap<BlockId, Bounds<Pixels>>>>;

pub struct Editor {
    pub core: Core,
    pub focus: FocusHandle,
    pub cache: LayoutCache,
    /// Window bounds of every rendered block wrapper (last frame).
    pub block_bounds: BoundsMap,
    /// The page is virtualized: one list item per visible top-level block
    /// (plus the title header and the click area below), so only what is on
    /// screen is laid out and painted.
    pub list: ListState,
    pub(super) items: Vec<super::render::Item>,
    /// Per-item change keys of the last frame, diffed to re-measure only the
    /// items that changed.
    pub(super) item_keys: Vec<u64>,
    /// Everything `item_keys` was built from, hashed (`sync_items`).
    pub(super) items_key: Option<u64>,
    /// Scroll request applied once this frame's layout is known.
    pub(super) reveal: Option<(super::render::Reveal, u8)>,
    pub(super) frame: super::render::Frame,
    /// Wrapped rows per hard line of code blocks (last frame).
    pub(super) code_rows: HashMap<BlockId, Vec<usize>>,
    /// Slash / "+" menu list (keyboard selection is kept in view).
    pub slash_scroll: ScrollHandle,
    pub mention_scroll: ScrollHandle,
    pub links: super::links::Links,
    pub mention: Option<super::links::Mention>,
    /// Window origin of the editor (last frame); overlays are placed relative
    /// to it.
    pub content_origin: Rc<Cell<Point<Pixels>>>,
    /// Last-frame layout lookups the overlays were positioned from.
    pub probes: RefCell<Vec<(super::overlays::Probe, Option<Bounds<Pixels>>)>>,
    pub marked: Option<(BlockId, Range<usize>)>,
    pub dragging: bool,
    /// A press in the gap between blocks, for `gap_release`.
    pub gap_press: Option<Point<Pixels>>,
    pub hover: Option<BlockId>,
    /// The hovered block's top has scrolled out of the page viewport (last frame).
    pub hover_clipped: bool,
    /// An edit since the pointer last moved: the hover handle hides until it moves.
    pub typing: bool,
    /// Blocks that can show the hover handle (set during render).
    pub handle_targets: Vec<BlockId>,
    /// Painted bounds of the callout bar and its colour panel (last frame).
    pub callout_zone: Rc<RefCell<Vec<Bounds<Pixels>>>>,
    pub caret_visible: bool,
    blink: Option<Task<()>>,
    pub goal_x: Option<Pixels>,
    pub slash: Option<Slash>,
    pub menu: Option<Menu>,
    pub menu_hovered: bool,
    /// Toolbar suppressed after typing / Esc until the selection changes.
    pub toolbar_hidden: bool,
    /// Typing hides the mouse pointer until it moves, as browsers do, so the
    /// I-beam left where the user clicked doesn't sit on top of the new text.
    pub pointer_hidden: bool,
    pub autoscroll: bool,
    /// Last applied (text color, background) from the color panel.
    pub last_color: (Option<ColorName>, Option<ColorName>),
    pub last_lang: String,
    pub link_field: Entity<TextField>,
    pub link_text_field: Entity<TextField>,
    pub(super) link_ui: super::link::LinkUi,
    pub eq_field: Entity<TextField>,
    pub number_field: Entity<TextField>,
    pub lang_field: Entity<TextField>,
    pub emoji_field: Entity<TextField>,
    pub emoji_scroll: ScrollHandle,
    /// The emoji search box was clicked or typed into (draws its focus ring).
    pub emoji_search_active: bool,
    pub(super) cover_ui: super::cover::CoverUi,
    pub find: Option<super::find::Find>,
    pub find_field: Entity<TextField>,
    pub replace_field: Entity<TextField>,
    /// 页宽设置 of this document.
    pub page_width: PageWidth,
    /// Side margins and catalogue state for this frame's pane width.
    pub(super) layout: super::catalogue::PageLayout,
    /// 文档目录 pinned open (per document).
    pub cat_pinned: bool,
    /// Child pages of the open document (kept current by the workspace).
    pub sub_pages: Vec<SubPage>,
    pub(super) cat_entries: Rc<Vec<super::catalogue::CatEntry>>,
    /// The `core.generation` `cat_entries` was built from.
    pub(super) cat_generation: Option<u64>,
    /// Headings folded in the catalogue only.
    pub(super) cat_collapsed: std::collections::HashSet<BlockId>,
    pub(super) cat_active: Option<BlockId>,
    /// Active row the catalogue list was last scrolled to.
    pub(super) cat_shown_active: Option<BlockId>,
    /// Page scroll position and caret block of the last frame.
    pub(super) cat_seen: ((usize, u32), BlockId),
    /// Heading just clicked in the catalogue.
    pub(super) cat_jump: Option<BlockId>,
    pub(super) cat_scroll: UniformListScrollHandle,
    pub(super) scrollbar: Rc<super::scrollbar::Scrollbar>,
    /// Column-gap hover: (grid, gap index, over the 新增分栏 circle).
    pub(super) gap_hover: Option<(BlockId, usize, bool)>,
    pub(super) col_drag: Option<super::grid::ColDrag>,
    /// Blocks moved by their handle.
    pub(super) block_drag: Option<super::drag::BlockDrag>,
    /// Blocks just dropped, flashing since.
    pub(super) drop_flash: Option<(Vec<BlockId>, std::time::Instant)>,
    /// The fold arrow turning after a click: block → start.
    pub(super) fold_anim: Option<(BlockId, std::time::Instant)>,
    /// Column widths easing to the document's after a column was added:
    /// grid → (widths before, start).
    pub(super) col_anim: HashMap<BlockId, (Vec<f32>, std::time::Instant)>,
    /// Table chrome: insert-point hover, bar / resize drags, bar geometry.
    pub(super) tables: super::table::TableUi,
    /// Name / description fields of code blocks and images (`caption.rs`).
    pub(super) captions: HashMap<BlockId, (Entity<TextField>, Vec<Subscription>)>,
    pub(super) img: super::image::ImageUi,
    pub(super) cd: super::countdown::CdUi,
    pub(super) btn: super::button::BtnUi,
    pub(super) rm: super::reminder::RmUi,
    pub(super) file: super::file::FileUi,
    /// Folded code blocks (view state only).
    pub(super) code_folded: std::collections::HashSet<BlockId>,
    /// Numbers of auto-numbered headings, refreshed each frame.
    pub(super) heading_orders: rustc_hash::FxHashMap<BlockId, String>,
    /// (rows, cols) under the pointer in the "+" panel's table / column size picker.
    pub(super) size_pick: (usize, usize),
    pub modified: u64,
    pub created: u64,
    /// 阅读模式 (`reading.rs`), and the document as it was when it began.
    pub read_only: bool,
    pub(super) ro_doc: Option<crate::model::Document>,
    /// The 文档信息 row under the title.
    pub info: crate::store::InfoRow,
    _subs: Vec<Subscription>,
}

impl EventEmitter<EditorEvent> for Editor {}

impl Focusable for Editor {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}

fn to16(s: &str, o: usize) -> usize {
    s[..o.min(s.len())].encode_utf16().count()
}

fn from16(s: &str, o16: usize) -> usize {
    let mut n = 0;
    for (i, ch) in s.char_indices() {
        if n >= o16 {
            return i;
        }
        n += ch.len_utf16();
    }
    s.len()
}

fn looks_like_url(q: &str) -> bool {
    q.contains("://") || q.starts_with("www.") || (q.contains('@') && q.contains('.'))
}

impl Editor {
    pub fn new(doc: Document, modified: u64, page_width: PageWidth, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let link_field = cx.new(|cx| {
            let mut f = TextField::new("粘贴或输入链接", cx);
            f.tab_events = true;
            f
        });
        let link_text_field = cx.new(|cx| {
            let mut f = TextField::new("输入文本", cx);
            f.tab_events = true;
            f
        });
        let lang_field = cx.new(|cx| TextField::new("搜索", cx));
        let emoji_field = cx.new(|cx| {
            let mut f = TextField::new("搜索", cx);
            f.size = 16.;
            f.line_height = 24.;
            f
        });
        let find_field = cx.new(|cx| TextField::new("在文档内查找", cx));
        let replace_field = cx.new(|cx| TextField::new("请输入文字", cx));
        let number_field = cx.new(|cx| TextField::new("编号", cx));
        let eq_field = cx.new(|cx| {
            let mut f = TextField::new("请输入公式", cx);
            f.multiline = true;
            f.wrap = true;
            f.mono = true;
            f.max_len = Some(10000);
            f.placeholder_color = theme::ta(0xbb, 0xbf, 0xc4, 1.);
            f
        });
        let mut subs = vec![
            cx.subscribe_in(&number_field, window, |this, _, ev: &TextFieldEvent, window, cx| {
                if !matches!(this.menu, Some(Menu::OrderedNumber { edit: true, .. })) { return }
                match ev {
                    TextFieldEvent::Submit => this.submit_ordered_number(window, cx),
                    TextFieldEvent::Cancel => this.close_menu_focus(window, cx),
                    _ => cx.notify(),
                }
            }),
            cx.subscribe_in(&find_field, window, |this, _, ev: &TextFieldEvent, window, cx| this.on_find_event(ev, window, cx)),
            cx.subscribe_in(&replace_field, window, |this, _, ev: &TextFieldEvent, window, cx| match ev {
                TextFieldEvent::Cancel => this.close_find(window, cx),
                _ => cx.notify(),
            }),
            cx.subscribe_in(&link_field, window, |this, _, ev: &TextFieldEvent, window, cx| this.on_link_field(ev, false, window, cx)),
            cx.subscribe_in(&link_text_field, window, |this, _, ev: &TextFieldEvent, window, cx| this.on_link_field(ev, true, window, cx)),
            cx.subscribe_in(&eq_field, window, |this, field, ev: &TextFieldEvent, window, cx| {
                let Some(Menu::Equation { block, offset }) = this.menu.clone() else { return };
                match ev {
                    TextFieldEvent::Changed => {
                        let src = field.read(cx).text.clone();
                        this.core.set_equation_source(Pos::new(block, offset), &src);
                        this.changed(window, cx);
                    }
                    TextFieldEvent::Cancel | TextFieldEvent::RightEdge => this.close_equation_editor(true, window, cx),
                    TextFieldEvent::LeftEdge => this.close_equation_editor(false, window, cx),
                    _ => {}
                }
            }),
            cx.subscribe_in(&emoji_field, window, |this, _, ev: &TextFieldEvent, window, cx| this.on_emoji_field(ev, window, cx)),
            cx.subscribe_in(&lang_field, window, |this, _, ev: &TextFieldEvent, window, cx| {
                let Some(Menu::Lang { block, selected }) = this.menu.clone() else { return };
                let n = menus::filter_languages(&this.lang_field.read(cx).text).len();
                match ev {
                    TextFieldEvent::Changed => this.menu = Some(Menu::Lang { block, selected: 0 }),
                    TextFieldEvent::Up if n > 0 => {
                        this.menu = Some(Menu::Lang { block, selected: (selected + n - 1) % n })
                    }
                    TextFieldEvent::Down if n > 0 => this.menu = Some(Menu::Lang { block, selected: (selected + 1) % n }),
                    TextFieldEvent::Submit => {
                        if let Some(l) = menus::filter_languages(&this.lang_field.read(cx).text).get(selected) {
                            this.set_lang(block, l, window, cx);
                        }
                        return;
                    }
                    TextFieldEvent::Cancel => return this.close_menu_focus(window, cx),
                    _ => {}
                }
                cx.notify();
            }),
        ];
        let (cd, cd_subs) = super::countdown::CdUi::new(window, cx);
        subs.extend(cd_subs);
        let (btn, btn_subs) = super::button::BtnUi::new(window, cx);
        subs.extend(btn_subs);
        let (rm, rm_subs) = super::reminder::RmUi::new(window, cx);
        subs.extend(rm_subs);
        let mut e = Editor {
            core: Core::new(doc),
            focus: cx.focus_handle(),
            cache: Default::default(),
            block_bounds: Default::default(),
            list: ListState::new(0, ListAlignment::Top, px(600.)),
            items: vec![],
            item_keys: vec![],
            items_key: None,
            reveal: None,
            frame: Default::default(),
            code_rows: HashMap::new(),
            slash_scroll: ScrollHandle::new(),
            mention_scroll: ScrollHandle::new(),
            links: Default::default(),
            mention: None,
            content_origin: Rc::new(Cell::new(Point::default())),
            probes: RefCell::new(vec![]),
            marked: None,
            dragging: false,
            gap_press: None,
            hover: None,
            hover_clipped: false,
            typing: false,
            handle_targets: vec![],
            callout_zone: Rc::default(),
            caret_visible: true,
            blink: None,
            goal_x: None,
            slash: None,
            menu: None,
            menu_hovered: false,
            toolbar_hidden: false,
            pointer_hidden: false,
            autoscroll: false,
            last_color: (None, Some(ColorName("light-yellow".into()))),
            last_lang: "Plain Text".into(),
            link_field,
            link_text_field,
            link_ui: Default::default(),
            eq_field,
            number_field,
            lang_field,
            emoji_field,
            emoji_scroll: ScrollHandle::new(),
            emoji_search_active: false,
            cover_ui: super::cover::CoverUi::new(),
            find: None,
            find_field,
            replace_field,
            page_width,
            layout: Default::default(),
            cat_pinned: false,
            sub_pages: vec![],
            cat_entries: Rc::default(),
            cat_generation: None,
            cat_collapsed: Default::default(),
            cat_active: None,
            cat_shown_active: None,
            // An impossible scroll position: the first frame picks the
            // active heading from the viewport, not from the caret.
            cat_seen: ((usize::MAX, 0), TITLE),
            cat_jump: None,
            cat_scroll: UniformListScrollHandle::new(),
            scrollbar: Rc::default(),
            gap_hover: None,
            col_drag: None,
            block_drag: None,
            drop_flash: None,
            fold_anim: None,
            col_anim: HashMap::default(),
            tables: Default::default(),
            captions: HashMap::new(),
            img: super::image::ImageUi::new(cx),
            file: super::file::FileUi::new(),
            code_folded: Default::default(),
            heading_orders: Default::default(),
            size_pick: (0, 0),
            modified,
            created: modified,
            read_only: false,
            ro_doc: None,
            info: Default::default(),
            cd,
            btn,
            rm,
            _subs: subs,
        };
        e.cat_seen.1 = e.core.sel.head.block;
        e.restart_blink(window, cx);
        e
    }

    pub fn focus(&self, window: &mut Window) {
        window.focus(&self.focus);
    }

    // ---- bookkeeping

    pub fn restart_blink(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.caret_visible = true;
        // Screenshot comparisons need a steady caret.
        if std::env::var_os("FDL_NOBLINK").is_some() {
            self.blink = None;
            return;
        }
        self.blink = Some(cx.spawn_in(window, async move |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_millis(500)).await;
                let ok = this.update_in(cx, |e, window, cx| {
                    if e.focus.is_focused(window) {
                        e.caret_visible = !e.caret_visible;
                        cx.notify();
                    }
                });
                if ok.is_err() {
                    break;
                }
            }
        }));
    }

    /// Call after every command: emits Changed, keeps menus consistent,
    /// scrolls the caret into view and restarts the caret blink.
    pub fn receive_document(&mut self, core: &Core, cx: &mut Context<Self>) {
        let sel = self.core.sel;
        let generation = self.core.generation;
        self.core = core.clone();
        self.core.sel = sel;
        self.core.block_sel = None;
        self.core.cell_sel = None;
        self.core.pending_marks = None;
        self.core.generation = generation.wrapping_add(1);
        self.core.fix_selection();
        self.core.break_coalescing();
        self.marked = None;
        self.menu = None;
        self.slash = None;
        self.cat_generation = None;
        self.ro_doc = self.read_only.then(|| self.core.doc.clone());
        self.refresh_find(false, cx);
        cx.notify();
    }

    pub fn changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.core.reveal_caret();
        if self.core.dirty {
            self.core.dirty = false;
            self.typing = true;
            self.hide_link_card(cx);
            cx.emit(EditorEvent::Changed);
            self.refresh_find(false, cx);
        }
        // A folded code block opens once the caret is inside it.
        if !self.core.in_block_mode() {
            self.code_folded.remove(&self.core.sel.head.block);
        }
        self.sync_slash();
        self.sync_mention();
        self.autoscroll = true;
        self.restart_blink(window, cx);
        cx.notify();
    }

    /// 重命名 from the sidebar: swap the title text as one undoable edit,
    /// leaving the caret (clamped) and the scroll position alone.
    /// The open document's child pages, for 子页面列表 blocks.
    pub fn set_sub_pages(&mut self, pages: Vec<SubPage>, cx: &mut Context<Self>) {
        if self.sub_pages != pages {
            self.sub_pages = pages;
            cx.notify();
        }
    }

    pub fn set_title(&mut self, title: &str, cx: &mut Context<Self>) {
        if self.core.doc.title.text == title {
            return;
        }
        let sel = self.core.sel;
        self.core.checkpoint(core::EditKind::Other);
        self.core.doc.title = RichText::plain(title);
        let len = title.len();
        let clamp = |mut p: Pos| {
            if p.block == TITLE {
                p.offset = p.offset.min(len);
            }
            p
        };
        self.core.sel = Selection { anchor: clamp(sel.anchor), head: clamp(sel.head) };
        self.core.dirty = false;
        self.keep_read_only_title();
        cx.emit(EditorEvent::Changed);
        cx.notify();
    }

    fn guard(&self, window: &Window, cx: &mut Context<Self>) -> bool {
        if self.focus.is_focused(window) {
            true
        } else {
            cx.propagate();
            false
        }
    }

    pub(super) fn set_caret(&mut self, p: Pos, extend: bool) {
        self.core.cell_sel = None;
        if extend {
            self.core.sel.head = p;
        } else {
            self.core.sel = Selection::caret(p);
        }
        self.core.pending_marks = None;
        self.core.break_coalescing();
        self.marked = None;
        if extend {
            self.toolbar_hidden = false;
        }
    }

    pub fn close_menu_focus(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.menu = None;
        self.menu_hovered = false;
        window.focus(&self.focus);
        cx.notify();
    }

    // ---- slash menu

    fn sync_slash(&mut self) {
        let Some(s) = self.slash.clone() else { return };
        let sel = self.core.sel;
        let text = self.core.text_of(s.block).map(|t| t.text.clone()).unwrap_or_default();
        let close = !sel.is_collapsed()
            || self.core.in_block_mode()
            || sel.head.block != s.block
            || sel.head.offset < s.start
            || sel.head.offset > text.len()
            || !matches!(text.get(s.trigger..s.start), Some("/") | Some("、"))
            || text.get(s.start..sel.head.offset).is_none_or(|q| q.ends_with("  ") || looks_like_url(q));
        if close {
            self.slash = None;
            return;
        }
        let n = self.slash_results().len();
        if let Some(s) = &mut self.slash {
            s.selected = s.selected.min(n.saturating_sub(1));
        }
    }

    pub fn slash_query(&self) -> String {
        let Some(s) = &self.slash else { return String::new() };
        let text = self.core.text_of(s.block).map(|t| t.text.as_str()).unwrap_or("");
        text.get(s.start..self.core.sel.head.offset).unwrap_or("").to_string()
    }

    /// Matching items in display order: grouped under their group title,
    /// groups ordered by their best hit (design-spec §4).
    pub fn slash_results(&self) -> Vec<usize> {
        let ranked = if self.slash.is_some() {
            menus::filter_slash(&self.slash_query())
        } else {
            (0..menus::SLASH_ITEMS.len()).collect()
        };
        let (grid, with_text) = (self.slash_grid(), self.slash_grid_text());
        let ranked: Vec<usize> =
            ranked.into_iter().filter(|i| menus::SLASH_ITEMS[*i].group != 0 || menus::basic_visible(menus::SLASH_ITEMS[*i].kind, grid, with_text)).collect();
        let mut groups: Vec<usize> = vec![];
        for i in &ranked {
            let g = menus::SLASH_ITEMS[*i].group;
            if !groups.contains(&g) {
                groups.push(g);
            }
        }
        groups
            .iter()
            .flat_map(|g| ranked.iter().copied().filter(move |i| menus::SLASH_ITEMS[*i].group == *g))
            .collect()
    }

    /// The "+" panel, and the slash menu before anything is typed after "/",
    /// pack the basic group into one icon grid; a filtered list stays flat.
    pub fn slash_grid(&self) -> bool {
        match &self.slash {
            Some(_) => self.slash_query().is_empty(),
            None => matches!(self.menu, Some(Menu::Add { .. }) | Some(Menu::Block { sub: Some(BlockSub::Add(_)), .. })),
        }
    }

    /// Whether the grid offers 文本: not on an empty line, where the block
    /// already is one.
    pub fn slash_grid_text(&self) -> bool {
        match &self.slash {
            Some(s) => self.core.text_of(s.block).is_some_and(|t| t.text.len() > s.start - s.trigger),
            None => matches!(self.menu, Some(Menu::Block { .. })),
        }
    }

    fn maybe_open_slash(&mut self, trigger_len: usize) {
        let h = self.core.sel.head;
        if h.block == TITLE || self.core.is_code(h.block) || h.offset < trigger_len {
            return;
        }
        if !self.core.kind_of(h.block).is_some_and(|k| k.has_text()) {
            return;
        }
        let trigger = h.offset - trigger_len;
        let text = &self.core.text_of(h.block).unwrap().text;
        let before = &text[..trigger];
        if before.is_empty() || before.ends_with(char::is_whitespace) {
            self.slash = Some(Slash { block: h.block, trigger, start: h.offset, selected: 0 });
            self.slash_scroll.set_offset(Point::default());
        }
    }

    /// Keys while the slash menu or the "+" panel is open. Returns true when consumed.
    fn list_menu_key(&mut self, key: ListKey, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if self.mention.is_some() {
            return match key {
                ListKey::Up | ListKey::Down => self.mention_key(Some(matches!(key, ListKey::Up)), false, false, window, cx),
                ListKey::Enter => self.mention_key(None, true, false, window, cx),
                ListKey::Close | ListKey::Left | ListKey::Right => false,
                ListKey::CloseAndPass => {
                    self.mention = None;
                    false
                }
            };
        }
        let add = matches!(self.menu, Some(Menu::Add { .. }));
        if self.slash.is_none() && !add {
            return false;
        }
        let grid = self.slash_grid();
        let results = self.slash_results();
        let n = results.len();
        let cur = match (&self.slash, &self.menu) {
            (Some(s), _) => s.selected,
            (None, Some(Menu::Add { selected, .. })) => *selected,
            _ => 0,
        };
        let set = |e: &mut Editor, v: usize| {
            if let Some(s) = &mut e.slash {
                s.selected = v;
            } else if let Some(Menu::Add { selected, .. }) = &mut e.menu {
                *selected = v;
            }
            // Children of the scrolled list are group titles and items interleaved;
            // the icon grid holds the whole basic group.
            let mut child = 0;
            let mut last = usize::MAX;
            for (k, i) in results.iter().enumerate() {
                let g = menus::SLASH_ITEMS[*i].group;
                if g != last {
                    last = g;
                    child += 1;
                }
                if k == v {
                    break;
                }
                if !(grid && g == 0) {
                    child += 1;
                }
            }
            if grid && results.get(v).is_some_and(|i| menus::SLASH_ITEMS[*i].group > 0) {
                child += 1;
            }
            e.slash_scroll.scroll_to_item(child);
        };
        // The icon grid (rows of GRID_COLS, then the list): ← / → step through
        // items in order, ↑ / ↓ move a whole row inside the grid (issue #4).
        let cells = if grid { results.iter().take_while(|i| menus::SLASH_ITEMS[**i].group == 0).count() } else { 0 };
        let (prev, next) = (if cur >= n { n.saturating_sub(1) } else { (cur + n - 1) % n.max(1) }, if cur >= n { 0 } else { (cur + 1) % n.max(1) });
        let key = match key {
            ListKey::Left | ListKey::Right if !grid => ListKey::Close,
            k => k,
        };
        match key {
            _ if n == 0 && matches!(key, ListKey::Up | ListKey::Down | ListKey::Left | ListKey::Right) => {}
            ListKey::Left => set(self, prev),
            ListKey::Right => set(self, next),
            // Up: a row up in the grid; from the list's first item, back to the
            // grid's last row; from the grid's top row, around to the last item.
            ListKey::Up if cur < cells => set(self, if cur >= menus::GRID_COLS { cur - menus::GRID_COLS } else { n - 1 }),
            ListKey::Up if cur == cells && cells > 0 => set(self, (cells - 1) / menus::GRID_COLS * menus::GRID_COLS),
            ListKey::Up => set(self, prev),
            // Down: a row down in the grid; from its last row, into the list.
            ListKey::Down if cur < cells => set(self, if cur + menus::GRID_COLS < cells { cur + menus::GRID_COLS } else if cells < n { cells } else { 0 }),
            ListKey::Down => set(self, next),
            ListKey::Enter => match results.get(cur) {
                Some(i) => {
                    let kind = menus::SLASH_ITEMS[*i].kind;
                    self.run_slash(kind, window, cx);
                    return true;
                }
                None => {
                    self.slash = None;
                    self.menu = None;
                }
            },
            ListKey::Close => {
                self.slash = None;
                if add {
                    self.menu = None;
                }
            }
            ListKey::CloseAndPass => {
                self.slash = None;
                if add {
                    self.menu = None;
                }
                cx.notify();
                return false;
            }
        }
        cx.notify();
        true
    }

    pub fn new_callout_kind(&self) -> BlockKind {
        // The last callout colours, and the last picked emoji (else a
        // random one).
        let s = super::emoji::callout_setting();
        let mut k = menus::callout(s.emoji.as_deref());
        if let BlockKind::Callout { bg: b, border: bo, .. } = &mut k
            && (s.bg.is_some() || s.border.is_some())
        {
            *b = s.bg.map(ColorName);
            *bo = s.border.map(ColorName);
        }
        k
    }

    /// Run a slash / "+" item at the caret.
    pub fn run_slash(&mut self, kind: SlashKind, window: &mut Window, cx: &mut Context<Self>) {
        self.run_slash_sized(kind, None, window, cx)
    }

    /// `size` is the (rows, cols) picked in the "+" panel's size picker.
    pub fn run_slash_sized(&mut self, kind: SlashKind, size: Option<(usize, usize)>, window: &mut Window, cx: &mut Context<Self>) {
        self.run_slash_with(kind, size, crate::model::ButtonAction::OpenLink, window, cx)
    }

    /// 按钮's type submenu.
    pub(super) fn run_slash_button(&mut self, action: crate::model::ButtonAction, window: &mut Window, cx: &mut Context<Self>) {
        self.run_slash_with(SlashKind::Button, None, action, window, cx)
    }

    fn run_slash_with(
        &mut self,
        kind: SlashKind,
        size: Option<(usize, usize)>,
        action: crate::model::ButtonAction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(s) = self.slash.take() {
            let end = self.core.sel.head.offset.max(s.start);
            self.core.sel = Selection { anchor: Pos::new(s.block, s.trigger), head: Pos::new(s.block, end) };
            self.core.delete_selection();
        }
        // 在下方添加 runs the item on a fresh line under the block.
        if let Some(Menu::Block { block, sub: Some(BlockSub::Add(_)) }) = self.menu {
            self.core.clear_block_sel();
            let b = Block::text("");
            let id = b.id;
            self.core.checkpoint(core::EditKind::Other);
            self.core.insert_after(block, b);
            self.menu = Some(Menu::Add { block: id, selected: 0 });
        }
        if let Some(Menu::Add { block, .. }) = self.menu {
            self.core.clear_block_sel();
            self.core.sel = Selection::caret(Pos::new(block, 0));
        }
        self.menu = None;
        self.hover = None;
        let id = self.core.sel.head.block;
        let empty = self
            .core
            .block(id)
            .is_some_and(|b| matches!(b.kind, BlockKind::Text) && b.text.is_empty() && b.children.is_empty());
        let turn = match kind {
            SlashKind::Text => Some(BlockKind::Text),
            SlashKind::Heading(l) => Some(menus::heading(l)),
            SlashKind::Ordered => Some(BlockKind::Ordered { seq: None }),
            SlashKind::Bullet => Some(BlockKind::Bullet),
            SlashKind::Todo => Some(BlockKind::Todo { done: false }),
            SlashKind::Code => Some(menus::code_block(&self.last_lang)),
            SlashKind::Quote => Some(BlockKind::Quote),
            SlashKind::Callout => Some(self.new_callout_kind()),
            _ => None,
        };
        if let Some(k) = turn {
            if empty || id == TITLE && false {
                self.core.turn_into(&[id], k);
            } else {
                let container = matches!(k, BlockKind::Quote | BlockKind::Callout { .. });
                let b = if container {
                    Block::new(k, RichText::default()).with_children(vec![Block::text("")])
                } else {
                    Block::new(k, RichText::default())
                };
                let child = b.children.first().map(|c| c.id);
                let nid = self.core.insert_block(b);
                self.core.sel = Selection::caret(Pos::new(child.unwrap_or(nid), 0));
            }
            return self.changed(window, cx);
        }
        match kind {
            SlashKind::Formula => {
                if let Some(p) = self.core.insert_equation("") {
                    self.changed(window, cx);
                    return self.open_equation_editor(p, window, cx);
                }
            }
            SlashKind::Button => return self.insert_new_button(action, window, cx),
            SlashKind::Reminder => return self.insert_new_reminder(window, cx),
            SlashKind::Divider | SlashKind::SubPageList => {
                let k = if kind == SlashKind::Divider { BlockKind::Divider } else { BlockKind::SubPageList };
                let nid = self.core.insert_block(Block::new(k, RichText::default()));
                let p = self.core.ensure_paragraph_after(nid);
                self.core.sel = Selection::caret(Pos::new(p, 0));
            }
            SlashKind::Countdown => {
                let nid = self.core.insert_block(Block::new(BlockKind::Countdown(Default::default()), RichText::default()));
                let p = self.core.ensure_paragraph_after(nid);
                self.core.sel = Selection::caret(Pos::new(p, 0));
                self.changed(window, cx);
                return self.open_countdown(nid, window, cx);
            }
            SlashKind::Table => {
                let (rows, cols) = size.unwrap_or((3, 3));
                self.insert_table(rows, cols)
            }
            SlashKind::Grid => {
                let n = size.map_or(2, |s| s.1);
                let cols: Vec<Block> = (0..n)
                    .map(|_| Block::new(BlockKind::Column { ratio: 1. / n as f32 }, RichText::default()).with_children(vec![Block::text("")]))
                    .collect();
                let first = cols[0].children[0].id;
                let g = Block::new(BlockKind::Grid, RichText::default()).with_children(cols);
                let nid = self.core.insert_block(g);
                self.core.ensure_paragraph_after(nid);
                self.core.sel = Selection::caret(Pos::new(first, 0));
            }
            SlashKind::Image => self.pick_image(window, cx),
            SlashKind::File => self.pick_files(window, cx),
            SlashKind::Link => {
                let h = self.core.sel.head;
                self.open_link_panel(h.block, h.offset..h.offset, false, window, cx);
            }
            _ => {}
        }
        self.changed(window, cx);
    }

    fn insert_table(&mut self, rows: usize, cols: usize) {
        let t = Table {
            col_widths: vec![100.; cols],
            header_rows: 0,
            header_col: false,
            rows: (0..rows).map(|_| (0..cols).map(|_| TableCell::empty()).collect()).collect(),
        };
        let first = t.rows[0][0].blocks[0].id;
        let nid = self.core.insert_block(Block::new(BlockKind::Table(t), RichText::default()));
        self.core.ensure_paragraph_after(nid);
        self.core.sel = Selection::caret(Pos::new(first, 0));
    }

    fn pick_image(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let rx = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some(crate::settings::text("插入图片").into()),
            extensions: None,
        });
        cx.spawn_in(window, async move |this, cx| {
            if let Ok(Ok(Some(paths))) = rx.await
                && let Some(p) = paths.into_iter().next()
            {
                let _ = this.update_in(cx, |e, window, cx| {
                    let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("png").to_string();
                    if let Ok(bytes) = std::fs::read(&p) {
                        e.insert_image_bytes(&bytes, &ext, window, cx);
                    }
                });
            }
        })
        .detach();
    }

    fn insert_image_bytes(&mut self, bytes: &[u8], ext: &str, window: &mut Window, cx: &mut Context<Self>) {
        let b = match super::image::image_block_from(bytes, ext) {
            Ok(b) => b,
            Err(msg) => return self.toast(msg, cx),
        };
        let nid = self.core.insert_block(b);
        let p = self.core.ensure_paragraph_after(nid);
        self.core.sel = Selection::caret(Pos::new(p, 0));
        self.changed(window, cx);
    }

    // ---- links, languages, colors

    /// Open the LaTeX editor of the equation at `p` (its source selected).
    pub fn open_equation_editor(&mut self, p: Pos, window: &mut Window, cx: &mut Context<Self>) {
        let Some(src) = self.core.equation_source(p) else { return };
        self.eq_field.update(cx, |f, cx| {
            f.set_text(&src, cx);
            f.select_all_text(cx);
        });
        self.core.clear_block_sel();
        self.core.break_coalescing();
        self.core.sel = Selection { anchor: p, head: Pos::new(p.block, p.offset + core::EQUATION.len()) };
        self.dragging = false;
        self.menu = Some(Menu::Equation { block: p.block, offset: p.offset });
        let fh = self.eq_field.read(cx).focus_handle(cx);
        window.focus(&fh);
        cx.notify();
    }

    /// Leave the equation editor with the caret after (or before) it.
    fn close_equation_editor(&mut self, after: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(Menu::Equation { block, offset }) = self.menu.take() else { return };
        self.core.break_coalescing();
        let o = if after { offset + core::EQUATION.len() } else { offset };
        self.core.sel = Selection::caret(Pos::new(block, o));
        window.focus(&self.focus);
        self.restart_blink(window, cx);
        cx.notify();
    }

    fn on_link_field(&mut self, ev: &TextFieldEvent, text: bool, window: &mut Window, cx: &mut Context<Self>) {
        match ev {
            TextFieldEvent::Submit => self.submit_link(window, cx),
            TextFieldEvent::Cancel => self.close_link_panel(window, cx),
            // Basic mode: Tab / Shift+Tab move between 文本 and 链接.
            TextFieldEvent::Tab | TextFieldEvent::ShiftTab => {
                if let Some(Menu::Link { only_link: false, .. }) = self.menu {
                    let f = if text { &self.link_field } else { &self.link_text_field };
                    let fh = f.read(cx).focus_handle(cx);
                    f.update(cx, |f, cx| f.select_all_text(cx));
                    window.focus(&fh);
                }
                cx.notify();
            }
            _ => cx.notify(),
        }
    }

    pub fn set_lang(&mut self, block: BlockId, lang: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.core.checkpoint(core::EditKind::Other);
        if let Some(b) = self.core.block_mut(block)
            && let BlockKind::Code { lang: l, .. } = &mut b.kind
        {
            *l = lang.to_string();
        }
        self.last_lang = lang.to_string();
        self.close_menu_focus(window, cx);
        self.changed(window, cx);
    }

    pub fn open_lang_menu(&mut self, block: BlockId, window: &mut Window, cx: &mut Context<Self>) {
        self.lang_field.update(cx, |f, cx| f.set_text("", cx));
        self.menu = Some(Menu::Lang { block, selected: 0 });
        let fh = self.lang_field.read(cx).focus_handle(cx);
        window.focus(&fh);
        cx.notify();
    }

    /// Text color / background from the color panel (None = default).
    pub fn apply_color(&mut self, text: Option<Option<ColorName>>, bg: Option<Option<ColorName>>, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(t) = &text {
            self.last_color.0 = t.clone();
        }
        if let Some(b) = &bg {
            self.last_color.1 = b.clone();
        }
        let (t, b) = (&text, &bg);
        let f = move |m: &mut Marks| {
            if let Some(t) = t {
                m.text_color = t.clone();
            }
            if let Some(b) = b {
                m.bg_color = b.clone();
            }
        };
        match self.whole_blocks() {
            Some(ids) => self.core.apply_marks_to_blocks(&ids, f),
            None => self.core.apply_marks(f),
        }
        self.changed(window, cx);
    }

    /// Handle menu 颜色 → 背景颜色: fills the selected blocks (or `block`).
    pub fn set_block_bg(&mut self, block: BlockId, bg: Option<ColorName>, window: &mut Window, cx: &mut Context<Self>) {
        let mut ids = self.core.block_sel_ids();
        if !ids.contains(&block) {
            ids = vec![block];
        }
        self.core.set_block_bg(&ids, bg);
        self.changed(window, cx);
    }

    /// Blocks formatted as a whole: the selected blocks in block mode, the
    /// text in the selected table cells. None for a text selection.
    pub fn whole_blocks(&self) -> Option<Vec<BlockId>> {
        if self.core.cell_sel.is_some() {
            Some(self.core.cell_text_blocks())
        } else if self.core.in_block_mode() {
            Some(self.core.block_sel_ids())
        } else {
            None
        }
    }

    /// Current (text color, bg) at the selection start, for the panel's selected state.
    pub fn current_colors(&self) -> (Option<ColorName>, Option<ColorName>) {
        let p = if let Some(ids) = self.whole_blocks() {
            ids.first().map(|id| Pos::new(*id, 0))
        } else {
            Some(self.core.ordered().0)
        };
        let Some(p) = p else { return (None, None) };
        let m = self
            .core
            .text_of(p.block)
            .and_then(|t| t.spans().find(|(r, _)| r.start <= p.offset && p.offset < r.end).map(|(_, m)| m.clone()))
            .unwrap_or_default();
        (m.text_color, m.bg_color)
    }

    pub fn set_callout(
        &mut self,
        block: BlockId,
        f: impl FnOnce(&mut Option<ColorName>, &mut Option<ColorName>, &mut Option<ColorName>, &mut String),
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.core.checkpoint(core::EditKind::Other);
        if let Some(b) = self.core.block_mut(block)
            && let BlockKind::Callout { emoji, bg, border, text_color } = &mut b.kind
        {
            let old = emoji.clone();
            f(bg, border, text_color, emoji);
            let name = |c: &Option<ColorName>| c.as_ref().map(|c| c.0.clone());
            let picked = (*emoji != old).then(|| emoji.clone());
            super::emoji::save_callout(name(bg).as_deref(), name(border).as_deref(), picked.as_deref());
        }
        self.changed(window, cx);
    }

    pub fn toggle_mark(&mut self, which: Mark, window: &mut Window, cx: &mut Context<Self>) {
        let (get, set): (fn(&Marks) -> bool, fn(&mut Marks, bool)) = match which {
            Mark::Bold => (|m| m.bold, |m, v| m.bold = v),
            Mark::Italic => (|m| m.italic, |m, v| m.italic = v),
            Mark::Underline => (|m| m.underline, |m, v| m.underline = v),
            Mark::Strike => (|m| m.strike, |m, v| m.strike = v),
            Mark::Code => (|m| m.code, |m, v| m.code = v),
        };
        if let Some(ids) = self.whole_blocks() {
            let all = ids.iter().all(|id| self.core.text_of(*id).is_none_or(|t| t.all_marks(0..t.len(), get)));
            if all {
                self.core.apply_marks_to_blocks(&ids, move |m| set(m, false));
            } else {
                self.core.apply_marks_to_blocks(&ids, move |m| set(m, true));
            }
        } else {
            self.core.toggle_mark(get, set);
        }
        self.changed(window, cx);
    }

    /// True when every selected character has the mark (toolbar active state).
    pub fn mark_active(&self, get: fn(&Marks) -> bool) -> bool {
        let us = self.core.units();
        let whole: Option<HashSet<BlockId>> = self.whole_blocks().map(|v| v.into_iter().collect());
        let mut any = false;
        for u in us.iter().filter(|u| u.text && u.id != TITLE) {
            let Some(t) = self.core.text_of(u.id) else { continue };
            let r = if let Some(whole) = &whole {
                if !whole.contains(&u.id) {
                    continue;
                }
                0..t.len()
            } else {
                match self.core.range_in(u.id, &us) {
                    Some(r) if !r.is_empty() => r,
                    _ => continue,
                }
            };
            any = true;
            if !t.all_marks(r, get) {
                return false;
            }
        }
        any
    }

    pub fn turn(&mut self, kind: BlockKind, toggle: bool, window: &mut Window, cx: &mut Context<Self>) {
        let ids = self.core.selected_blocks();
        let kind = match kind {
            BlockKind::Code { .. } => menus::code_block(&self.last_lang),
            BlockKind::Callout { .. } => self.new_callout_kind(),
            k => k,
        };
        if toggle {
            self.core.toggle_turn_into(&ids, kind);
        } else {
            self.core.turn_into(&ids, kind);
        }
        self.changed(window, cx);
    }

    pub fn align(&mut self, a: Align, window: &mut Window, cx: &mut Context<Self>) {
        let ids = self.core.selected_blocks();
        self.core.set_align(&ids, a);
        self.changed(window, cx);
    }

    pub fn current_align(&self) -> Align {
        self.core.selected_blocks().first().and_then(|id| self.core.block(*id)).map(|b| b.align).unwrap_or_default()
    }

    // ---- caret movement

    fn move_h(&mut self, right: bool, word: bool, extend: bool) {
        let s = self.core.sel;
        if !extend && !s.is_collapsed() {
            let (a, b) = self.core.ordered();
            self.set_caret(if right { b } else { a }, false);
            return;
        }
        let h = s.head;
        let us = self.core.units();
        let Some(i) = us.iter().position(|u| u.id == h.block) else { return };
        if us[i].text {
            let text = self.core.text_of(h.block).map(|t| t.text.clone()).unwrap_or_default();
            if right && h.offset < text.len() {
                let o = if word { core::next_word(&text, h.offset) } else { core::next_grapheme(&text, h.offset) };
                return self.set_caret(Pos::new(h.block, o), extend);
            }
            if !right && h.offset > 0 {
                let o = if word { core::prev_word(&text, h.offset) } else { core::prev_grapheme(&text, h.offset) };
                return self.set_caret(Pos::new(h.block, o), extend);
            }
        }
        if right {
            if let Some(n) = us.get(i + 1) {
                self.set_caret(Pos::new(n.id, 0), extend);
            }
        } else if i > 0 {
            let p = us[i - 1].id;
            let len = self.core.len_of(p);
            self.set_caret(Pos::new(p, len), extend);
        }
    }

    fn move_v(&mut self, down: bool, extend: bool) {
        let h = self.core.sel.head;
        let us = self.core.units();
        let Some(i) = us.iter().position(|u| u.id == h.block) else { return };
        let cache = self.cache.clone();
        let cache = cache.borrow();
        let goal = self
            .goal_x
            .or_else(|| cache.get(&h.block).map(|l| l.position_for(h.offset).x))
            .unwrap_or_default();
        if us[i].text
            && let Some(l) = cache.get(&h.block)
            && !l.rows.is_empty()
        {
            let ri = l.row_index_for(h.offset);
            let target = if down { (ri + 1 < l.rows.len()).then_some(ri + 1) } else { ri.checked_sub(1) };
            if let Some(t) = target {
                let o = l.index_in_row(t, goal);
                drop(cache);
                self.set_caret(Pos::new(h.block, o), extend);
                self.goal_x = Some(goal);
                return;
            }
        }
        if extend && h.block != TITLE {
            drop(cache);
            let len = self.core.len_of(h.block);
            // At the edge of a block Shift+↑/↓ goes to the block edge first, then into block selection.
            if us[i].text && !down && h.offset > 0 {
                self.set_caret(Pos::new(h.block, 0), true);
            } else if us[i].text && down && h.offset < len {
                self.set_caret(Pos::new(h.block, len), true);
            } else {
                self.core.extend_into_blocks(down);
            }
            return;
        }
        let next = if down { us.get(i + 1) } else { i.checked_sub(1).map(|k| &us[k]) };
        let p = match next {
            Some(u) if u.text => {
                let o = cache
                    .get(&u.id)
                    .filter(|l| !l.rows.is_empty())
                    .map(|l| l.index_in_row(if down { 0 } else { l.rows.len() - 1 }, goal))
                    .unwrap_or(0);
                Pos::new(u.id, o.min(self.core.len_of(u.id)))
            }
            Some(u) => Pos::new(u.id, 0),
            None if down && h.block != TITLE => {
                drop(cache);
                let id = self.core.trailing_line();
                self.set_caret(Pos::new(id, 0), false);
                self.goal_x = None;
                return;
            }
            None if down => Pos::new(h.block, self.core.len_of(h.block)),
            None => Pos::new(h.block, 0),
        };
        drop(cache);
        self.set_caret(p, extend);
        self.goal_x = Some(goal);
    }

    fn line_edge(&mut self, end: bool, extend: bool) {
        let h = self.core.sel.head;
        let o = {
            let cache = self.cache.borrow();
            match cache.get(&h.block) {
                Some(l) if !l.rows.is_empty() => {
                    let ri = l.row_index_for(h.offset);
                    if end {
                        let r = &l.rows[ri];
                        let last_of_hard = l.rows.get(ri + 1).is_none_or(|n| n.hard != r.hard);
                        if last_of_hard { r.range.end } else { l.index_in_row(ri, px(1.0e6)) }
                    } else {
                        l.rows[ri].range.start
                    }
                }
                _ => {
                    if end {
                        self.core.len_of(h.block)
                    } else {
                        0
                    }
                }
            }
        };
        self.set_caret(Pos::new(h.block, o), extend);
    }

    // ---- action handlers

    fn on_move_left(&mut self, _: &k::MoveLeft, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) || self.list_menu_key(ListKey::Left, window, cx) {
            return;
        }
        if self.core.in_block_mode() {
            self.core.block_sel_horizontal(false);
        } else {
            self.goal_x = None;
            self.move_h(false, false, false);
        }
        self.changed(window, cx);
    }

    fn on_move_right(&mut self, _: &k::MoveRight, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) || self.list_menu_key(ListKey::Right, window, cx) {
            return;
        }
        if self.core.in_block_mode() {
            self.core.block_sel_horizontal(true);
        } else {
            self.goal_x = None;
            self.move_h(true, false, false);
        }
        self.changed(window, cx);
    }

    fn on_move_up(&mut self, _: &k::MoveUp, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) || self.list_menu_key(ListKey::Up, window, cx) {
            return;
        }
        if self.core.in_block_mode() {
            self.core.block_sel_move(false, false);
        } else {
            self.move_v(false, false);
        }
        self.changed(window, cx);
    }

    fn on_move_down(&mut self, _: &k::MoveDown, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) || self.list_menu_key(ListKey::Down, window, cx) {
            return;
        }
        if self.core.in_block_mode() {
            self.core.block_sel_move(true, false);
        } else {
            self.move_v(true, false);
        }
        self.changed(window, cx);
    }

    fn on_select_left(&mut self, _: &k::SelectLeft, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        self.list_menu_key(ListKey::CloseAndPass, window, cx);
        if !self.core.in_block_mode() {
            self.goal_x = None;
            self.move_h(false, false, true);
        }
        self.changed(window, cx);
    }

    fn on_select_right(&mut self, _: &k::SelectRight, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        self.list_menu_key(ListKey::CloseAndPass, window, cx);
        if !self.core.in_block_mode() {
            self.goal_x = None;
            self.move_h(true, false, true);
        }
        self.changed(window, cx);
    }

    fn on_select_up(&mut self, _: &k::SelectUp, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        self.list_menu_key(ListKey::CloseAndPass, window, cx);
        if self.core.in_block_mode() {
            self.core.block_sel_move(false, true);
        } else {
            self.move_v(false, true);
        }
        self.changed(window, cx);
    }

    fn on_select_down(&mut self, _: &k::SelectDown, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        self.list_menu_key(ListKey::CloseAndPass, window, cx);
        if self.core.in_block_mode() {
            self.core.block_sel_move(true, true);
        } else {
            self.move_v(true, true);
        }
        self.changed(window, cx);
    }

    fn on_word_left(&mut self, _: &k::WordLeft, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) || self.core.in_block_mode() {
            return;
        }
        self.list_menu_key(ListKey::CloseAndPass, window, cx);
        self.goal_x = None;
        self.move_h(false, true, false);
        self.changed(window, cx);
    }

    fn on_word_right(&mut self, _: &k::WordRight, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) || self.core.in_block_mode() {
            return;
        }
        self.list_menu_key(ListKey::CloseAndPass, window, cx);
        self.goal_x = None;
        self.move_h(true, true, false);
        self.changed(window, cx);
    }

    fn on_select_word_left(&mut self, _: &k::SelectWordLeft, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) || self.core.in_block_mode() {
            return;
        }
        self.list_menu_key(ListKey::CloseAndPass, window, cx);
        self.goal_x = None;
        self.move_h(false, true, true);
        self.changed(window, cx);
    }

    fn on_select_word_right(&mut self, _: &k::SelectWordRight, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) || self.core.in_block_mode() {
            return;
        }
        self.list_menu_key(ListKey::CloseAndPass, window, cx);
        self.goal_x = None;
        self.move_h(true, true, true);
        self.changed(window, cx);
    }

    fn on_line_start(&mut self, _: &k::LineStart, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) || self.core.in_block_mode() {
            return;
        }
        self.list_menu_key(ListKey::CloseAndPass, window, cx);
        self.goal_x = None;
        self.line_edge(false, false);
        self.changed(window, cx);
    }

    fn on_line_end(&mut self, _: &k::LineEnd, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) || self.core.in_block_mode() {
            return;
        }
        self.list_menu_key(ListKey::CloseAndPass, window, cx);
        self.goal_x = None;
        self.line_edge(true, false);
        self.changed(window, cx);
    }

    fn on_select_line_start(&mut self, _: &k::SelectLineStart, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) || self.core.in_block_mode() {
            return;
        }
        self.list_menu_key(ListKey::CloseAndPass, window, cx);
        self.goal_x = None;
        self.line_edge(false, true);
        self.changed(window, cx);
    }

    fn on_select_line_end(&mut self, _: &k::SelectLineEnd, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) || self.core.in_block_mode() {
            return;
        }
        self.list_menu_key(ListKey::CloseAndPass, window, cx);
        self.goal_x = None;
        self.line_edge(true, true);
        self.changed(window, cx);
    }

    fn on_doc_start(&mut self, _: &k::DocStart, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        self.list_menu_key(ListKey::CloseAndPass, window, cx);
        self.core.clear_block_sel();
        self.goal_x = None;
        self.set_caret(Pos::new(TITLE, 0), false);
        self.changed(window, cx);
    }

    fn on_doc_end(&mut self, _: &k::DocEnd, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        self.list_menu_key(ListKey::CloseAndPass, window, cx);
        self.core.clear_block_sel();
        self.goal_x = None;
        if let Some(u) = self.core.units().last() {
            let len = self.core.len_of(u.id);
            self.set_caret(Pos::new(u.id, len), false);
        }
        self.changed(window, cx);
    }

    fn on_select_to_doc_start(&mut self, _: &k::SelectToDocStart, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        self.list_menu_key(ListKey::CloseAndPass, window, cx);
        self.core.block_sel_to_edge(false);
        self.changed(window, cx);
    }

    fn on_select_to_doc_end(&mut self, _: &k::SelectToDocEnd, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        self.list_menu_key(ListKey::CloseAndPass, window, cx);
        self.core.block_sel_to_edge(true);
        self.changed(window, cx);
    }

    fn on_backspace(&mut self, _: &k::Backspace, window: &mut Window, cx: &mut Context<Self>) {
        self.pointer_hidden = true;
        if !self.guard(window, cx) {
            return;
        }
        if self.cropping() {
            return self.cancel_crop(cx);
        }
        if self.core.cell_sel.is_some() {
            self.core.table_clear_cells();
        } else if self.core.in_block_mode() {
            self.core.delete_selected_blocks();
        } else {
            self.core.backspace();
        }
        self.toolbar_hidden = true;
        self.changed(window, cx);
    }

    fn on_delete(&mut self, _: &k::Delete, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        if self.cropping() {
            return self.cancel_crop(cx);
        }
        if self.core.cell_sel.is_some() {
            self.core.table_clear_cells();
        } else if self.core.in_block_mode() {
            self.core.delete_selected_blocks();
        } else {
            self.core.delete_forward();
        }
        self.toolbar_hidden = true;
        self.changed(window, cx);
    }

    fn on_delete_word_back(&mut self, _: &k::DeleteWordBack, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        if self.cropping() {
            return self.cancel_crop(cx);
        }
        if self.core.in_block_mode() {
            self.core.delete_selected_blocks();
        } else {
            self.core.delete_word_back();
        }
        self.changed(window, cx);
    }

    fn on_delete_word_forward(&mut self, _: &k::DeleteWordForward, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        if self.cropping() {
            return self.cancel_crop(cx);
        }
        if self.core.in_block_mode() {
            self.core.delete_selected_blocks();
        } else if self.core.sel.is_collapsed() {
            let h = self.core.sel.head;
            let text = self.core.text_of(h.block).map(|t| t.text.clone()).unwrap_or_default();
            if h.offset < text.len() {
                let e = core::next_word(&text, h.offset);
                self.core.sel = Selection { anchor: h, head: Pos::new(h.block, e) };
                self.core.delete_selection();
            } else {
                self.core.delete_forward();
            }
        } else {
            self.core.delete_selection();
        }
        self.changed(window, cx);
    }

    fn on_delete_to_line_start(&mut self, _: &k::DeleteToLineStart, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        if self.cropping() {
            return self.cancel_crop(cx);
        }
        if self.core.in_block_mode() {
            self.core.delete_selected_blocks();
        } else {
            self.core.delete_to_start();
        }
        self.changed(window, cx);
    }

    fn on_enter(&mut self, _: &k::Enter, window: &mut Window, cx: &mut Context<Self>) {
        self.pointer_hidden = true;
        if !self.guard(window, cx) || self.list_menu_key(ListKey::Enter, window, cx) {
            return;
        }
        if self.cropping() {
            return self.save_crop(true, window, cx);
        }
        if self.core.in_block_mode() {
            self.core.enter_from_block_sel();
        } else if self.core.is_code(self.core.sel.head.block) && self.core.sel.is_collapsed() {
            // Newline keeps the current line's indentation.
            let h = self.core.sel.head;
            let text = self.core.text_of(h.block).map(|t| t.text.clone()).unwrap_or_default();
            let line_start = text[..h.offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let indent: String = text[line_start..h.offset].chars().take_while(|c| *c == ' ' || *c == '\t').collect();
            self.core.break_coalescing();
            self.core.insert_text(&format!("\n{indent}"));
            self.core.break_coalescing();
        } else if !self.core.markdown_enter_shortcut() {
            self.core.enter();
        }
        self.changed(window, cx);
    }

    fn on_soft_break(&mut self, _: &k::SoftBreak, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) || self.core.in_block_mode() || self.core.sel.head.block == TITLE {
            return;
        }
        self.core.break_coalescing();
        self.core.insert_text("\n");
        self.core.break_coalescing();
        self.changed(window, cx);
    }

    fn on_toggle_todo(&mut self, _: &k::ToggleTodo, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        for id in self.core.selected_blocks() {
            if matches!(self.core.kind_of(id), Some(BlockKind::Todo { .. })) {
                self.core.toggle_todo(id);
            }
        }
        self.changed(window, cx);
    }

    fn on_tab(&mut self, _: &k::Tab, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        if !self.core.in_block_mode() && self.core.is_code(self.core.sel.head.block) {
            self.core.insert_text("    ");
        } else if self.core.in_block_mode() || !self.core.cell_step(true) {
            self.core.indent();
        }
        self.changed(window, cx);
    }

    fn on_shift_tab(&mut self, _: &k::ShiftTab, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        if !self.core.in_block_mode() && self.core.is_code(self.core.sel.head.block) {
            // Remove up to 4 leading spaces of the current line.
            let h = self.core.sel.head;
            let text = self.core.text_of(h.block).map(|t| t.text.clone()).unwrap_or_default();
            let ls = text[..h.offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let n = text[ls..].chars().take(4).take_while(|c| *c == ' ').count();
            if n > 0 {
                self.core.checkpoint(core::EditKind::Other);
                self.core.text_mut(h.block).unwrap().replace(ls..ls + n, &RichText::default());
                self.core.sel = Selection::caret(Pos::new(h.block, h.offset.saturating_sub(n).max(ls)));
            }
        } else if self.core.in_block_mode() || !self.core.cell_step(false) {
            self.core.outdent();
        }
        self.changed(window, cx);
    }

    fn on_escape(&mut self, _: &k::Escape, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        if self.cropping() {
            return self.cancel_crop(cx);
        }
        if self.mention.is_some() {
            self.mention = None;
        } else if self.slash.is_some() || self.menu.is_some() {
            self.slash = None;
            self.menu = None;
        } else if self.find.is_some() {
            return self.close_find(window, cx);
        } else if self.read_only {
            // Nothing to select blocks for.
            self.core.sel = Selection::caret(self.core.sel.head);
        } else if self.core.cell_sel.is_some() {
            self.core.cell_sel = None;
        } else if self.core.in_block_mode() {
            self.core.clear_block_sel();
        } else {
            let h = self.core.sel.head.block;
            self.core.select_block(h);
        }
        self.toolbar_hidden = true;
        self.changed(window, cx);
    }

    fn on_select_all(&mut self, _: &k::SelectAll, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        if self.cropping() {
            self.save_crop(false, window, cx);
        }
        self.slash = None;
        self.core.select_all();
        self.toolbar_hidden = false;
        self.changed(window, cx);
    }

    fn copy_selection(&mut self, cx: &mut Context<Self>) -> bool {
        let cells = self.core.cell_sel.is_some();
        let blocks = if cells { self.core.selected_cells_content() } else { self.core.selected_blocks_content() };
        let text = if self.core.in_block_mode() || cells {
            Document { blocks: blocks.clone(), ..Default::default() }.plain_text()
        } else {
            if self.core.sel.is_collapsed() {
                return false;
            }
            self.core.selected_text()
        };
        let xml = docxml::serialize(&Document { blocks, ..Default::default() });
        cx.write_to_clipboard(ClipboardItem::new_string_with_json_metadata(text, xml));
        true
    }

    fn on_copy(&mut self, _: &k::Copy, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        self.copy_selection(cx);
    }

    fn on_cut(&mut self, _: &k::Cut, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) || !self.copy_selection(cx) {
            return;
        }
        if self.core.cell_sel.is_some() {
            self.core.table_clear_cells();
        } else if self.core.in_block_mode() {
            self.core.delete_selected_blocks();
        } else {
            self.core.delete_selection();
        }
        self.changed(window, cx);
    }

    /// ⌘V. Our own copies paste as DocxXML; other apps' HTML is converted;
    /// plain text is read as Markdown; otherwise an image.
    fn on_paste(&mut self, _: &k::Paste, window: &mut Window, cx: &mut Context<Self>) {
        self.paste(false, window, cx);
    }

    /// ⌘⇧V 粘贴为纯文本.
    fn on_paste_plain(&mut self, _: &k::PastePlain, window: &mut Window, cx: &mut Context<Self>) {
        self.paste(true, window, cx);
    }

    fn paste(&mut self, plain: bool, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        let fake = crate::clipboard::FAKE_CLIPBOARD.lock().unwrap().clone();
        let item = if fake.is_some() { None } else { cx.read_from_clipboard() };
        let (text, meta, image) = match (&fake, &item) {
            (Some((_, t)), _) => (Some(t.clone()), None, None),
            (None, Some(item)) => {
                let s = item.entries().iter().find_map(|e| match e {
                    ClipboardEntry::String(s) => Some(s),
                    _ => None,
                });
                let images: Vec<_> = item.entries().iter().filter_map(|e| match e {
                    ClipboardEntry::Image(img) => Some(img),
                    _ => None,
                }).collect();
                let image = (!images.is_empty()).then_some(images);
                (s.map(|s| s.text().clone()), s.and_then(|s| s.metadata_json::<String>()), image)
            }
            (None, None) => return,
        };
        if self.core.in_block_mode() {
            // Paste after the selected blocks.
            if let Some(last) = self.core.block_sel_ids().last().copied() {
                self.core.clear_block_sel();
                self.core.caret_to_end_of(last);
            }
        }
        let head = self.core.sel.head.block;
        let rich_target = head != TITLE && !self.core.is_code(head);
        let internal = meta.and_then(|xml| docxml::parse(&xml).ok()).filter(|d| !d.blocks.is_empty());
        let html = match &fake {
            Some((h, _)) => h.clone(),
            None if !plain && internal.is_none() && rich_target => crate::clipboard::clipboard_html(),
            None => None,
        };
        let html = html.filter(|_| !plain && rich_target).map(|h| {
            let mut b = crate::paste::html_to_blocks(&h);
            super::image::localize_pasted_images(&mut b);
            b
        });
        // HTML that only restates the text (a lone link to it, or the text
        // itself) pastes as text, so Markdown in it still converts.
        let restates = |b: &Vec<Block>| match (b.as_slice(), &text) {
            ([one], Some(t)) => {
                matches!(one.kind, BlockKind::Text)
                    && one.children.is_empty()
                    && one.text.text == t.trim()
                    && one.text.spans().all(|(_, m)| m.link.as_deref().is_none_or(|l| l == t.trim()) && Marks { link: None, ..m.clone() } == Marks::default())
            }
            _ => false,
        };
        if plain {
            // ⌘⇧V: the text only (Markdown still converts, as in the browser).
            if let Some(s) = text.filter(|s| !s.is_empty()) {
                self.core.paste_markdown(&s);
            }
        } else if let Some(d) = internal {
            self.core.paste_blocks(d.blocks);
        } else if let Some(b) = html.filter(|b| !b.is_empty() && !restates(b) && !(image.is_some() && b.iter().all(|b| matches!(b.kind, BlockKind::Image { .. })))) {
            self.core.paste_blocks(b);
        } else if let Some(s) = text.filter(|s| !s.is_empty()) {
            self.core.paste_markdown(&s);
        } else if let Some(images) = image {
            // Several copied image files paste one after another.
            for img in images {
                let ext = match img.format {
                    ImageFormat::Jpeg => "jpg",
                    ImageFormat::Gif => "gif",
                    ImageFormat::Webp => "webp",
                    ImageFormat::Svg => "svg",
                    ImageFormat::Bmp => "bmp",
                    ImageFormat::Tiff => "tiff",
                    _ => "png",
                };
                self.insert_image_bytes(&img.bytes, ext, window, cx);
            }
        }
        if let Some(msg) = self.core.notice.take() {
            self.toast(msg, cx);
        }
        self.slash = None;
        self.changed(window, cx);
    }

    fn on_undo(&mut self, _: &k::Undo, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        self.cancel_crop(cx);
        self.slash = None;
        self.marked = None;
        self.core.undo();
        self.changed(window, cx);
    }

    fn on_redo(&mut self, _: &k::Redo, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        self.cancel_crop(cx);
        self.slash = None;
        self.marked = None;
        self.core.redo();
        self.changed(window, cx);
    }

    fn on_save(&mut self, _: &k::Save, window: &mut Window, cx: &mut Context<Self>) {
        if !self.guard(window, cx) {
            return;
        }
        self.core.break_coalescing();
        cx.emit(EditorEvent::Save);
    }

    fn on_bold(&mut self, _: &k::Bold, window: &mut Window, cx: &mut Context<Self>) {
        if self.guard(window, cx) {
            self.toggle_mark(Mark::Bold, window, cx);
        }
    }

    fn on_italic(&mut self, _: &k::Italic, window: &mut Window, cx: &mut Context<Self>) {
        if self.guard(window, cx) {
            self.toggle_mark(Mark::Italic, window, cx);
        }
    }

    fn on_underline(&mut self, _: &k::Underline, window: &mut Window, cx: &mut Context<Self>) {
        if self.guard(window, cx) {
            self.toggle_mark(Mark::Underline, window, cx);
        }
    }

    fn on_strike(&mut self, _: &k::Strike, window: &mut Window, cx: &mut Context<Self>) {
        if self.guard(window, cx) {
            self.toggle_mark(Mark::Strike, window, cx);
        }
    }

    fn on_inline_code(&mut self, _: &k::InlineCode, window: &mut Window, cx: &mut Context<Self>) {
        if self.guard(window, cx) {
            self.toggle_mark(Mark::Code, window, cx);
        }
    }

    fn on_doc_link(&mut self, _: &k::DocLink, window: &mut Window, cx: &mut Context<Self>) {
        if self.guard(window, cx) { self.link_selection(cx); }
    }

    fn on_link(&mut self, _: &k::Link, window: &mut Window, cx: &mut Context<Self>) {
        if self.guard(window, cx) {
            self.link_from_selection(window, cx);
        }
    }

    fn on_insert_reminder(&mut self, _: &k::InsertReminder, window: &mut Window, cx: &mut Context<Self>) {
        if self.guard(window, cx) {
            self.insert_new_reminder(window, cx);
        }
    }

    fn on_apply_last_color(&mut self, _: &k::ApplyLastColor, window: &mut Window, cx: &mut Context<Self>) {
        if self.guard(window, cx) {
            let (t, b) = self.last_color.clone();
            self.apply_color(Some(t), Some(b), window, cx);
        }
    }

    fn turn_key(&mut self, kind: BlockKind, window: &mut Window, cx: &mut Context<Self>) {
        if self.guard(window, cx) {
            self.turn(kind, true, window, cx);
        }
    }

    fn on_turn_text(&mut self, _: &k::TurnText, w: &mut Window, cx: &mut Context<Self>) {
        self.turn_key(BlockKind::Text, w, cx)
    }
    fn on_turn_h1(&mut self, _: &k::TurnH1, w: &mut Window, cx: &mut Context<Self>) {
        self.turn_key(menus::heading(1), w, cx)
    }
    fn on_turn_h2(&mut self, _: &k::TurnH2, w: &mut Window, cx: &mut Context<Self>) {
        self.turn_key(menus::heading(2), w, cx)
    }
    fn on_turn_h3(&mut self, _: &k::TurnH3, w: &mut Window, cx: &mut Context<Self>) {
        self.turn_key(menus::heading(3), w, cx)
    }
    fn on_turn_h4(&mut self, _: &k::TurnH4, w: &mut Window, cx: &mut Context<Self>) {
        self.turn_key(menus::heading(4), w, cx)
    }
    fn on_turn_h5(&mut self, _: &k::TurnH5, w: &mut Window, cx: &mut Context<Self>) {
        self.turn_key(menus::heading(5), w, cx)
    }
    fn on_turn_h6(&mut self, _: &k::TurnH6, w: &mut Window, cx: &mut Context<Self>) {
        self.turn_key(menus::heading(6), w, cx)
    }
    fn on_turn_h7(&mut self, _: &k::TurnH7, w: &mut Window, cx: &mut Context<Self>) {
        self.turn_key(menus::heading(7), w, cx)
    }
    fn on_turn_h8(&mut self, _: &k::TurnH8, w: &mut Window, cx: &mut Context<Self>) {
        self.turn_key(menus::heading(8), w, cx)
    }
    fn on_turn_h9(&mut self, _: &k::TurnH9, w: &mut Window, cx: &mut Context<Self>) {
        self.turn_key(menus::heading(9), w, cx)
    }
    fn on_turn_ordered(&mut self, _: &k::TurnOrdered, w: &mut Window, cx: &mut Context<Self>) {
        self.turn_key(BlockKind::Ordered { seq: None }, w, cx)
    }
    fn on_turn_bullet(&mut self, _: &k::TurnBullet, w: &mut Window, cx: &mut Context<Self>) {
        self.turn_key(BlockKind::Bullet, w, cx)
    }
    fn on_turn_todo(&mut self, _: &k::TurnTodo, w: &mut Window, cx: &mut Context<Self>) {
        self.turn_key(BlockKind::Todo { done: false }, w, cx)
    }
    fn on_turn_code(&mut self, _: &k::TurnCode, w: &mut Window, cx: &mut Context<Self>) {
        self.turn_key(menus::code_block("Plain Text"), w, cx)
    }
    fn on_turn_quote(&mut self, _: &k::TurnQuote, w: &mut Window, cx: &mut Context<Self>) {
        self.turn_key(BlockKind::Quote, w, cx)
    }

    fn on_insert_divider(&mut self, _: &k::InsertDivider, window: &mut Window, cx: &mut Context<Self>) {
        if self.guard(window, cx) {
            self.core.clear_block_sel();
            self.run_slash(SlashKind::Divider, window, cx);
        }
    }

    fn on_insert_table(&mut self, _: &k::InsertTable, window: &mut Window, cx: &mut Context<Self>) {
        if self.guard(window, cx) {
            self.core.clear_block_sel();
            self.run_slash(SlashKind::Table, window, cx);
        }
    }

    fn on_align_left(&mut self, _: &k::AlignLeft, window: &mut Window, cx: &mut Context<Self>) {
        if self.guard(window, cx) {
            self.align(Align::Left, window, cx);
        }
    }

    fn on_align_center(&mut self, _: &k::AlignCenter, window: &mut Window, cx: &mut Context<Self>) {
        if self.guard(window, cx) {
            self.align(Align::Center, window, cx);
        }
    }

    fn on_align_right(&mut self, _: &k::AlignRight, window: &mut Window, cx: &mut Context<Self>) {
        if self.guard(window, cx) {
            self.align(Align::Right, window, cx);
        }
    }

    fn move_block_by(&mut self, down: bool, window: &mut Window, cx: &mut Context<Self>) {
        let ids = self.core.selected_blocks();
        let (Some(first), Some(last)) = (ids.first().copied(), ids.last().copied()) else { return };
        // Swap with the sibling before / after the selected range.
        let Some(path) = core::find_path(&self.core.doc.blocks, if down { last } else { first }) else { return };
        let siblings: Vec<BlockId> = match core::parent_path(&path) {
            None => self.core.doc.blocks.iter().map(|b| b.id).collect(),
            Some(pp) => match core::block_at(&self.core.doc.blocks, pp) {
                Some(p) => match (&p.kind, path.last()) {
                    (BlockKind::Table(t), Some(core::Seg::Cell(r, c, _))) => t.rows[*r][*c].blocks.iter().map(|b| b.id).collect(),
                    _ => p.children.iter().map(|b| b.id).collect(),
                },
                None => return,
            },
        };
        let Some(i) = siblings.iter().position(|x| *x == if down { last } else { first }) else { return };
        let other = if down { siblings.get(i + 1).copied() } else { i.checked_sub(1).map(|k| siblings[k]) };
        if let Some(o) = other {
            self.core.move_block(o, if down { first } else { last }, !down);
        }
        self.changed(window, cx);
    }

    fn on_move_block_up(&mut self, _: &k::MoveBlockUp, window: &mut Window, cx: &mut Context<Self>) {
        if self.guard(window, cx) {
            self.move_block_by(false, window, cx);
        }
    }

    fn on_move_block_down(&mut self, _: &k::MoveBlockDown, window: &mut Window, cx: &mut Context<Self>) {
        if self.guard(window, cx) {
            self.move_block_by(true, window, cx);
        }
    }

    /// Attach every editor action to the root element.
    pub fn register_actions(&self, d: Div, cx: &mut Context<Self>) -> Div {
        d.on_action(guard(cx, Self::on_move_left))
            .on_action(guard(cx, Self::on_move_right))
            .on_action(guard(cx, Self::on_move_up))
            .on_action(guard(cx, Self::on_move_down))
            .on_action(guard(cx, Self::on_select_left))
            .on_action(guard(cx, Self::on_select_right))
            .on_action(guard(cx, Self::on_select_up))
            .on_action(guard(cx, Self::on_select_down))
            .on_action(guard(cx, Self::on_word_left))
            .on_action(guard(cx, Self::on_word_right))
            .on_action(guard(cx, Self::on_select_word_left))
            .on_action(guard(cx, Self::on_select_word_right))
            .on_action(guard(cx, Self::on_line_start))
            .on_action(guard(cx, Self::on_line_end))
            .on_action(guard(cx, Self::on_select_line_start))
            .on_action(guard(cx, Self::on_select_line_end))
            .on_action(guard(cx, Self::on_doc_start))
            .on_action(guard(cx, Self::on_doc_end))
            .on_action(guard(cx, Self::on_select_to_doc_start))
            .on_action(guard(cx, Self::on_select_to_doc_end))
            .on_action(guard(cx, Self::on_backspace))
            .on_action(guard(cx, Self::on_delete))
            .on_action(guard(cx, Self::on_delete_word_back))
            .on_action(guard(cx, Self::on_delete_word_forward))
            .on_action(guard(cx, Self::on_delete_to_line_start))
            .on_action(guard(cx, Self::on_enter))
            .on_action(guard(cx, Self::on_soft_break))
            .on_action(guard(cx, Self::on_toggle_todo))
            .on_action(guard(cx, Self::on_tab))
            .on_action(guard(cx, Self::on_shift_tab))
            .on_action(guard(cx, Self::on_escape))
            .on_action(guard(cx, Self::on_select_all))
            .on_action(guard(cx, Self::on_copy))
            .on_action(guard(cx, Self::on_cut))
            .on_action(guard(cx, Self::on_paste))
            .on_action(guard(cx, Self::on_paste_plain))
            .on_action(guard(cx, Self::on_undo))
            .on_action(guard(cx, Self::on_redo))
            .on_action(guard(cx, Self::on_save))
            .on_action(guard(cx, Self::on_bold))
            .on_action(guard(cx, Self::on_italic))
            .on_action(guard(cx, Self::on_underline))
            .on_action(guard(cx, Self::on_strike))
            .on_action(guard(cx, Self::on_inline_code))
            .on_action(guard(cx, Self::on_link))
            .on_action(guard(cx, Self::on_doc_link))
            .on_action(guard(cx, Self::on_apply_last_color))
            .on_action(guard(cx, Self::on_insert_reminder))
            .on_action(guard(cx, Self::on_turn_text))
            .on_action(guard(cx, Self::on_turn_h1))
            .on_action(guard(cx, Self::on_turn_h2))
            .on_action(guard(cx, Self::on_turn_h3))
            .on_action(guard(cx, Self::on_turn_h4))
            .on_action(guard(cx, Self::on_turn_h5))
            .on_action(guard(cx, Self::on_turn_h6))
            .on_action(guard(cx, Self::on_turn_h7))
            .on_action(guard(cx, Self::on_turn_h8))
            .on_action(guard(cx, Self::on_turn_h9))
            .on_action(guard(cx, Self::on_turn_ordered))
            .on_action(guard(cx, Self::on_turn_bullet))
            .on_action(guard(cx, Self::on_turn_todo))
            .on_action(guard(cx, Self::on_turn_code))
            .on_action(guard(cx, Self::on_turn_quote))
            .on_action(guard(cx, Self::on_insert_divider))
            .on_action(guard(cx, Self::on_insert_table))
            .on_action(guard(cx, Self::on_align_left))
            .on_action(guard(cx, Self::on_align_center))
            .on_action(guard(cx, Self::on_align_right))
            .on_action(guard(cx, Self::on_move_block_up))
            .on_action(guard(cx, Self::on_move_block_down))
    }

    // ---- mouse

    fn pos_at(&self, p: Point<Pixels>) -> Option<Pos> {
        let us = self.core.units();
        let atomic = {
            let bb = self.block_bounds.borrow();
            us.iter().filter(|u| !u.text).find(|u| bb.get(&u.id).is_some_and(|b| b.contains(&p))).map(|u| u.id)
        };
        if let Some(id) = atomic {
            return Some(Pos::new(id, 0));
        }
        // A pinned header row covers the rows scrolled under it.
        let pinned = self.tables.stuck.borrow().iter().find(|(_, b)| b.contains(&p)).map(|(t, _)| *t);
        let ids: Vec<BlockId> = match pinned.and_then(|t| self.core.table(t)) {
            Some(t) => super::table::first_row_text(t),
            None => us.iter().filter(|u| u.text).map(|u| u.id).collect(),
        };
        hit_test(&self.cache, p, &ids).map(|p| Pos::new(p.block, p.offset.min(self.core.len_of(p.block))))
    }

    pub fn on_mouse_down(&mut self, ev: &MouseDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        window.focus(&self.focus);
        self.reminder_blur(window, cx);
        self.menu = None;
        self.menu_hovered = false;
        self.slash = None;
        self.toolbar_hidden = false;
        self.marked = None;
        self.tables.danger = false;
        self.hide_link_card(cx);
        self.link_ui.target = None;
        self.link_ui.click = None;
        self.btn_press_reset();
        self.gap_press = None;
        if !ev.modifiers.shift && self.button_press(ev.position) {
            // The caret stays; the action runs on release.
            window.prevent_default();
            return cx.notify();
        }
        if ev.click_count == 1 && !ev.modifiers.shift && self.gap_at(ev.position).is_some() {
            // The paragraph comes on release (a drag inserts nothing).
            self.gap_press = Some(ev.position);
            self.core.clear_block_sel();
            return cx.notify();
        }
        let Some(pos) = self.pos_at(ev.position) else { return };
        let eq = self.cache.borrow().get(&pos.block).and_then(|l| l.equation_at(ev.position));
        if let Some(ix) = eq
            && !ev.modifiers.shift
            && !self.read_only
        {
            // Keep the root's focus tracking from taking focus back.
            window.prevent_default();
            return self.open_equation_editor(Pos::new(pos.block, ix), window, cx);
        }
        // A plain click opens a link (the caret still lands).
        if ev.click_count == 1 && !ev.modifiers.shift && self.link_under(ev.position).is_some() {
            self.link_press(pos);
        }
        self.core.clear_block_sel();
        self.goal_x = None;
        let text = self.core.text_of(pos.block).map(|t| t.text.clone()).unwrap_or_default();
        if ev.modifiers.shift {
            self.set_caret(pos, true);
            self.cell_drag_to(ev.position, cx);
        } else if ev.click_count == 2 && !text.is_empty() {
            let r = core::word_at(&text, pos.offset);
            self.set_caret(Pos::new(pos.block, r.start), false);
            self.core.sel.head = Pos::new(pos.block, r.end);
        } else if ev.click_count >= 3 {
            self.set_caret(Pos::new(pos.block, 0), false);
            self.core.sel.head = Pos::new(pos.block, text.len());
        } else {
            self.set_caret(pos, false);
        }
        self.dragging = ev.click_count == 1;
        self.restart_blink(window, cx);
        cx.notify();
    }

    pub fn drag_to(&mut self, p: Point<Pixels>, cx: &mut Context<Self>) {
        if self.cell_drag_to(p, cx) {
            return;
        }
        if let Some(pos) = self.pos_at(p)
            && pos != self.core.sel.head
        {
            self.core.sel.head = pos;
            cx.notify();
        }
    }

    /// Mouse-up (or a move without the button) ends a text drag. Events can
    /// arrive before the frame that registers the window-level drag listeners,
    /// so the element-level handlers call this too.
    pub fn end_drag(&mut self, p: Option<Point<Pixels>>, cx: &mut Context<Self>) {
        if self.dragging {
            if let Some(p) = p {
                self.drag_to(p, cx);
                self.link_release(cx);
            }
            self.link_ui.click = None;
            self.dragging = false;
            cx.notify();
        }
    }

    pub fn on_hover_move(&mut self, ev: &MouseMoveEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self.resizing_image() {
            return match ev.pressed_button {
                Some(MouseButton::Left) => self.resize_to(ev.position.x, cx),
                _ => self.end_resize(cx),
            };
        }
        if self.crop_dragging() {
            return match ev.pressed_button {
                Some(MouseButton::Left) => self.crop_drag_to(ev.position, ev.modifiers.shift, cx),
                _ => self.end_crop_drag(cx),
            };
        }
        if self.block_drag.is_some() {
            return self.block_drag_move(ev.position, ev.pressed_button == Some(MouseButton::Left), window, cx);
        }
        if self.tables.drag.is_some() {
            return self.table_drag_move(ev.position, ev.pressed_button == Some(MouseButton::Left), window, cx);
        }
        if self.col_drag.is_some() {
            match ev.pressed_button {
                Some(MouseButton::Left) => self.col_drag_to(ev.position.x, cx),
                _ => self.end_col_drag(window, cx),
            }
            return;
        }
        if self.dragging {
            match ev.pressed_button {
                Some(MouseButton::Left) => self.drag_to(ev.position, cx),
                _ => self.end_drag(None, cx),
            }
        }
        let p = ev.position;
        if self.typing {
            self.typing = false;
            cx.notify();
        }
        // A clicked callout dropdown stays open until a selection or outside click.
        if matches!(self.menu, Some(Menu::CalloutColor { .. })) {
            return;
        }
        if self.read_only {
            return;
        }
        if ev.pressed_button.is_none() {
            self.image_hover(p, cx);
            self.link_hover(Some(p), cx);
            self.button_hover(Some(p), cx);
        }
        if ev.pressed_button.is_some() || matches!(self.menu, Some(Menu::Block { .. }) | Some(Menu::Add { .. })) {
            return;
        }
        let best = {
            let bb = self.block_bounds.borrow();
            let mut best: Option<(f32, BlockId)> = None;
            for id in &self.handle_targets {
                let Some(b) = bb.get(id) else { continue };
                let ext = Bounds::from_corners(point(b.left() - px(60.), b.top()), b.bottom_right());
                if ext.contains(&p) {
                    let area = f32::from(b.size.width) * f32::from(b.size.height);
                    if best.is_none_or(|(a, _)| area < a) {
                        best = Some((area, *id));
                    }
                }
            }
            best.map(|(_, id)| id)
        };
        if best.is_some() && best != self.hover {
            self.hover = best;
            cx.notify();
        }
    }

    /// Click in the empty area below the content: focus a trailing empty paragraph.
    pub fn on_click_below(&mut self, _: &MouseDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        window.focus(&self.focus);
        if self.read_only {
            return;
        }
        self.menu = None;
        self.slash = None;
        self.core.clear_block_sel();
        let id = self.core.trailing_line();
        self.set_caret(Pos::new(id, 0), false);
        cx.stop_propagation();
        self.changed(window, cx);
    }

    /// Handle (⋮⋮) click: select the block and open its menu; "+" opens the add panel.
    pub fn open_block_menu(&mut self, block: BlockId, add: bool, window: &mut Window, cx: &mut Context<Self>) {
        window.focus(&self.focus);
        self.slash = None;
        self.menu_hovered = false;
        if add {
            self.core.clear_block_sel();
            self.core.sel = Selection::caret(Pos::new(block, 0));
            // No autoSelect: nothing is highlighted until hover or ↑↓.
            self.menu = Some(Menu::Add { block, selected: usize::MAX });
            self.size_pick = (0, 0);
            self.slash_scroll.set_offset(Point::default());
        } else {
            self.core.select_block(block);
            self.menu = Some(Menu::Block { block, sub: None });
        }
        cx.notify();
    }

    pub fn block_menu_action(&mut self, block: BlockId, act: BlockAct, window: &mut Window, cx: &mut Context<Self>) {
        // The 标题行 / 标题列 switches flip in place; the menu stays open.
        if !matches!(act, BlockAct::HeaderRow | BlockAct::HeaderCol) {
            self.menu = None;
        }
        match act {
            BlockAct::Cut => {
                self.core.select_block(block);
                self.copy_selection(cx);
                self.core.delete_selected_blocks();
            }
            BlockAct::Copy => {
                self.core.select_block(block);
                self.copy_selection(cx);
            }
            BlockAct::Delete => {
                self.core.select_block(block);
                self.core.delete_selected_blocks();
            }
            BlockAct::Indent => {
                self.core.select_block(block);
                self.core.indent();
            }
            BlockAct::Outdent => {
                self.core.select_block(block);
                self.core.outdent();
            }
            BlockAct::Align(a) => self.core.set_align(&[block], a),
            BlockAct::Turn(kind) => {
                let kind = match kind {
                    BlockKind::Code { .. } => menus::code_block(&self.last_lang),
                    BlockKind::Callout { .. } => self.new_callout_kind(),
                    k => k,
                };
                // A callout's grid acts on its content; 高亮块 itself unwraps it.
                let children: Option<Vec<BlockId>> = self
                    .core
                    .block(block)
                    .filter(|b| matches!(b.kind, BlockKind::Callout { .. }))
                    .map(|b| b.children.iter().map(|c| c.id).collect());
                match children {
                    Some(ids) if matches!(kind, BlockKind::Callout { .. }) => self.core.unwrap_container(&ids),
                    Some(ids) => self.core.turn_into(&ids, kind),
                    None => self.core.turn_into(&[block], kind),
                }
                self.core.clear_block_sel();
                // The caret lands in the block that was turned, not wherever it was before.
                let sel = self.core.sel;
                if sel.anchor.block != block || sel.head.block != block {
                    let end = self.core.text_of(block).map_or(0, |t| t.len());
                    self.core.sel = Selection::caret(Pos::new(block, end));
                }
            }
            BlockAct::HeaderRow => self.core.table_toggle_header_row(block),
            BlockAct::HeaderCol => self.core.table_toggle_header_col(block),
            BlockAct::DistributeCols => self.core.table_distribute(block),
        }
        window.focus(&self.focus);
        self.changed(window, cx);
    }

    pub fn scroll_to_block(&mut self, id: BlockId, cx: &mut Context<Self>) {
        self.reveal = Some((super::render::Reveal::Block(id), 0));
        if let Some(u) = self.core.units().iter().find(|u| u.id == id) {
            self.core.sel = Selection::caret(Pos::new(u.id, 0));
        }
        cx.notify();
    }

    // ---- typing

    fn type_text(&mut self, s: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.core.cell_sel = None;
        if s.is_empty() {
            self.core.delete_selection();
            return self.changed(window, cx);
        }
        let s = s.replace('\r', "\n");
        if s == "\n" {
            return;
        }
        self.core.insert_text(&s);
        if s == " " {
            let _ = self.core.markdown_block_shortcut() || self.core.markdown_divider() || self.core.autolink();
        } else if s == "-" {
            self.core.markdown_divider();
        }
        self.core.markdown_inline_shortcut();
        if s == "/" || s == "、" {
            self.maybe_open_slash(s.len());
        } else if let Some(c) = s.chars().last().filter(|c| matches!(c, '@' | '＠')) {
            self.maybe_open_mention(c.len_utf8());
        }
        self.sync_wikilink_input(&s, window, cx);
        self.toolbar_hidden = true;
        self.goal_x = None;
        self.changed(window, cx);
    }

    fn head_text(&self) -> Option<String> {
        if self.core.in_block_mode() {
            return None;
        }
        self.core.text_of(self.core.sel.head.block).map(|t| t.text.clone())
    }

    /// Selection reported to the platform: within the head block only.
    fn reported_range(&self, text: &str) -> (Range<usize>, bool) {
        let s = self.core.sel;
        let h = s.head.offset.min(text.len());
        if s.anchor.block == s.head.block {
            let a = s.anchor.offset.min(text.len());
            (a.min(h)..a.max(h), h < a)
        } else {
            (h..h, false)
        }
    }
}

#[derive(Clone, Copy)]
enum ListKey {
    Up,
    Down,
    /// ← / →: step through the icon grid; elsewhere they close the menu.
    Left,
    Right,
    Enter,
    Close,
    CloseAndPass,
}

#[derive(Clone, Copy)]
pub enum Mark {
    Bold,
    Italic,
    Underline,
    Strike,
    Code,
}

#[derive(Clone)]
pub enum BlockAct {
    Cut,
    Copy,
    Delete,
    Indent,
    Outdent,
    Align(Align),
    Turn(BlockKind),
    /// Tables: 设置为标题行 / 标题列, 均分列宽.
    HeaderRow,
    HeaderCol,
    DistributeCols,
}

impl EntityInputHandler for Editor {
    fn text_for_range(
        &mut self,
        range: Range<usize>,
        actual: &mut Option<Range<usize>>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<String> {
        let t = self.head_text()?;
        let r = from16(&t, range.start)..from16(&t, range.end);
        actual.replace(to16(&t, r.start)..to16(&t, r.end));
        Some(t[r].to_string())
    }

    fn selected_text_range(&mut self, _: bool, _: &mut Window, _: &mut Context<Self>) -> Option<UTF16Selection> {
        let t = self.head_text()?;
        let (r, reversed) = self.reported_range(&t);
        Some(UTF16Selection { range: to16(&t, r.start)..to16(&t, r.end), reversed })
    }

    fn marked_text_range(&self, _: &mut Window, _: &mut Context<Self>) -> Option<Range<usize>> {
        let (b, r) = self.marked.as_ref()?;
        if *b != self.core.sel.head.block {
            return None;
        }
        let t = self.core.text_of(*b)?;
        Some(to16(&t.text, r.start)..to16(&t.text, r.end))
    }

    fn unmark_text(&mut self, _: &mut Window, _: &mut Context<Self>) {
        self.marked = None;
    }

    fn replace_text_in_range(&mut self, range: Option<Range<usize>>, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.pointer_hidden = true;
        if self.core.in_block_mode() {
            return;
        }
        let head = self.core.sel.head.block;
        if let Some((b, r)) = self.marked.take() {
            self.core.sel = Selection { anchor: Pos::new(b, r.start), head: Pos::new(b, r.end) };
        } else if let Some(r16) = range
            && let Some(t) = self.head_text()
        {
            let (rep, _) = self.reported_range(&t);
            let r = from16(&t, r16.start)..from16(&t, r16.end);
            if r != rep {
                self.core.sel = Selection { anchor: Pos::new(head, r.start), head: Pos::new(head, r.end) };
            }
        }
        self.type_text(text, window, cx);
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        text: &str,
        new_sel: Option<Range<usize>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.pointer_hidden = true;
        if self.core.in_block_mode() {
            return;
        }
        if self.core.text_of(self.core.sel.head.block).is_none() || self.core.sel.head.block != self.core.sel.anchor.block {
            // Atomic caret or a cross-block selection: clear it first.
            if !self.core.sel.is_collapsed() {
                self.core.delete_selection();
            }
            if self.core.text_of(self.core.sel.head.block).is_none() {
                self.core.insert_text("");
            }
        }
        let head = self.core.sel.head.block;
        let t = self.core.text_of(head).map(|t| t.text.clone()).unwrap_or_default();
        let r = match (&self.marked, range) {
            (Some((b, r)), _) if *b == head => r.clone(),
            (_, Some(r16)) => from16(&t, r16.start)..from16(&t, r16.end),
            _ => self.reported_range(&t).0,
        };
        self.core.replace_in_block(head, r.clone(), text);
        self.marked = (!text.is_empty()).then(|| (head, r.start..r.start + text.len()));
        if let Some(ns) = new_sel {
            let t2 = self.core.text_of(head).map(|t| t.text.clone()).unwrap_or_default();
            let base = to16(&t2, r.start);
            let a = from16(&t2, base + ns.start);
            let b = from16(&t2, base + ns.end);
            self.core.sel = Selection { anchor: Pos::new(head, a), head: Pos::new(head, b) };
        }
        self.slash = None;
        self.toolbar_hidden = true;
        self.changed(window, cx);
    }

    fn bounds_for_range(&mut self, range: Range<usize>, _: Bounds<Pixels>, _: &mut Window, _: &mut Context<Self>) -> Option<Bounds<Pixels>> {
        let head = self.core.sel.head.block;
        let t = self.core.text_of(head)?.text.clone();
        let cache = self.cache.borrow();
        let l = cache.get(&head)?;
        let a = l.position_for(from16(&t, range.start));
        let b = l.position_for(from16(&t, range.end));
        let r = Bounds::from_corners(a, point(b.x.max(a.x), b.y + l.line_height));
        Some(r)
    }

    fn character_index_for_point(&mut self, p: Point<Pixels>, _: &mut Window, _: &mut Context<Self>) -> Option<usize> {
        let head = self.core.sel.head.block;
        let t = self.core.text_of(head)?.text.clone();
        let cache = self.cache.borrow();
        let l = cache.get(&head)?;
        Some(to16(&t, l.index_for(p).min(t.len())))
    }
}

/// An editor action handler that stands aside while 倒计时设置 holds the
/// keyboard (the dialog sits inside the editor's key context).
fn guard<A: Action, F: Fn(&mut Editor, &A, &mut Window, &mut Context<Editor>) + 'static>(
    cx: &mut Context<Editor>,
    f: F,
) -> impl Fn(&A, &mut Window, &mut App) + 'static {
    cx.listener(move |e, a: &A, w, cx| {
        if e.cd.dialog.is_none() {
            f(e, a, w, cx)
        } else {
            cx.propagate()
        }
    })
}

impl Editor { pub(crate) fn refresh_theme(&mut self) { self.items_key = None; self.menu = None; } }
