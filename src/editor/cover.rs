//! 文档图标与封面: the header's hover row (添加图标 / 添加封面), the icon
//! before the title, and the cover above the header with its toolbar,
//! 编辑封面 menu, 选择图片 panel and drag-to-reposition.
//! The 官方图库 is our own set of images bundled under `assets/covers`.

use super::emoji;
use super::emoji_picker::EmojiTarget;
use super::overlays::at;
use super::view::{Editor, EditorEvent, Menu};
use crate::model::Cover;
use crate::editor_theme::{self as theme, Theme, craft};
use crate::editor_ui::{ROW_RADIUS, TipExt, icon, menu_glyph, menu_row, panel, popover};
use gpui::{prelude::FluentBuilder, *};
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

/// 官方图库: (title, file prefix, count) — `covers/<prefix>-<n>.jpg`, with
/// 236×128 thumbnails under `covers/thumb/`.
const SERIES: [(&str, &str, usize); 4] = [("渐变", "gradient", 8), ("极光", "aurora", 6), ("山峦", "landscape", 6), ("几何", "geometry", 6)];

const PANEL_W: f32 = 540.;

pub struct CoverUi {
    /// Window bounds of the 文档图标 and of 编辑封面 (last frame).
    pub icon_bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
    edit_bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
    /// Image and box sizes of the drawn cover (last frame).
    geo: Cell<Option<(f32, f32, f32, f32)>>,
    pub drag: Option<CoverDrag>,
    gallery_scroll: ScrollHandle,
    task: Option<Task<()>>,
}

impl CoverUi {
    pub fn new() -> Self {
        CoverUi {
            icon_bounds: Rc::default(),
            edit_bounds: Rc::default(),
            geo: Cell::new(None),
            drag: None,
            gallery_scroll: ScrollHandle::new(),
            task: None,
        }
    }
}

/// 调整图片位置: the ratios being dragged, the saved ones, and the grab
/// point with the margin it started from.
#[derive(Clone, Copy, Debug)]
pub struct CoverDrag {
    orig: (f32, f32),
    x: f32,
    y: f32,
    moved: bool,
    grab: Option<(Point<Pixels>, f32)>,
}

/// The cover image placed in a `cw`×`ch` box: scaled to cover it, then
/// shifted along the one overflowing axis by the offset ratio (0 = centred).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Geo {
    pub w: f32,
    pub h: f32,
    pub ml: f32,
    pub mt: f32,
    pub horizontal: bool,
}

fn clamp_margin(m: f32, img: f32, cont: f32) -> f32 {
    m.max(-(img - cont).max(0.)).min(0.)
}

/// `(m + (img - cont) / 2) / img`, to 6 places.
fn ratio(m: f32, img: f32, cont: f32) -> f32 {
    ((m + (img - cont) / 2.) / img * 1e6).round() / 1e6
}

pub fn geometry(iw: f32, ih: f32, cw: f32, ch: f32, rx: f32, ry: f32) -> Geo {
    if iw / ih > cw / ch {
        let w = iw * ch / ih;
        Geo { w, h: ch, ml: clamp_margin(-(w - cw) / 2. + rx * w, w, cw), mt: 0., horizontal: true }
    } else {
        let h = ih * cw / iw;
        Geo { w: cw, h, ml: 0., mt: clamp_margin(-(h - ch) / 2. + ry * h, h, ch), horizontal: false }
    }
}

/// 27vh, at most 400.
pub fn cover_height(window: &Window) -> f32 {
    (f32::from(window.viewport_size().height) * 0.27).min(400.)
}

fn preset_bytes(name: &str) -> Option<&'static [u8]> {
    let path = format!("covers/{name}.jpg");
    crate::assets::FILES.iter().find(|(p, _)| *p == path).map(|(_, b)| *b)
}

/// The header's icon / cover entry: 28 tall, 16px icon with 4px each side, 13pt
/// secondary text, a grey hover.
fn meta_entry(id: &'static str, ic: &'static str, label: &'static str) -> Stateful<Div> {
    div()
        .id(id)
        .flex_none()
        .h(px(28.))
        .pr(px(6.))
        .mr(px(4.))
        .flex()
        .items_center()
        .rounded(px(ROW_RADIUS))
        .cursor_pointer()
        .text_size(px(13.))
        .line_height(px(28.))
        .text_color(Theme::text_caption())
        .opacity(0.)
        .group_hover("doc-header", |s| s.opacity(1.))
        .hover(|s| s.bg(Theme::fill_hover()))
        .active(|s| s.bg(Theme::fill_pressed()))
        .child(div().mx(px(4.)).child(icon(ic, 16., Theme::text_caption())))
        .child(label)
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
}

/// The toolbar's small buttons on the image:
/// white with a hairline and a 1px drop, or (`primary`, 确认) the accent in
/// white semibold; radius 9.
fn toolbar_button(id: &'static str, label: &'static str, primary: bool) -> Stateful<Div> {
    let (bg, hover, active, rim, color) = if primary {
        (Theme::primary(), theme::c(craft::B600), theme::c(craft::B700), Theme::primary(), gpui::white())
    } else {
        (theme::ta(255, 255, 255, 0.92), theme::ta(255, 255, 255, 1.), theme::c(craft::N100), Theme::button_border(), Theme::text())
    };
    div()
        .id(id)
        .ml(px(8.))
        .min_w(px(60.))
        .px(px(10.))
        .py(px(4.))
        .flex()
        .justify_center()
        .items_center()
        .border_1()
        .border_color(rim)
        .rounded(px(9.))
        .shadow(theme::shadow_button())
        .text_size(px(13.))
        .line_height(px(18.))
        .when(primary, |d| d.font_weight(FontWeight::SEMIBOLD))
        .text_color(color)
        .bg(bg)
        .hover(move |s| s.bg(hover))
        .active(move |s| s.bg(active))
        .cursor_pointer()
        .child(label)
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
}

impl Editor {
    // ---- icon

    /// The icon is file-level meta: saved, but outside undo.
    fn set_doc_icon(&mut self, icon: Option<String>, cx: &mut Context<Self>) {
        if self.core.doc.icon == icon {
            return;
        }
        self.core.doc.icon = icon;
        self.core.generation += 1;
        cx.emit(EditorEvent::Changed);
        cx.notify();
    }

    pub(super) fn pick_doc_icon(&mut self, e: Option<&'static emoji::Emoji>, cx: &mut Context<Self>) {
        self.set_doc_icon(e.map(|e| e.char.to_string()), cx);
    }

    /// 添加图标: a random emoji at once, then its picker opens on it.
    fn add_doc_icon(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.set_doc_icon(Some(emoji::random_pick().char.to_string()), cx);
        self.cover_ui.task = Some(cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(Duration::from_millis(200)).await;
            let _ = this.update_in(cx, |e, window, cx| {
                if !matches!(e.menu, Some(Menu::Emoji { target: EmojiTarget::DocIcon, .. })) {
                    e.toggle_emoji_picker(EmojiTarget::DocIcon, window, cx);
                }
            });
        }));
    }

    // ---- cover

    fn set_cover(&mut self, cover: Option<Cover>, cx: &mut Context<Self>) {
        self.cover_ui.drag = None;
        if self.core.doc.cover == cover {
            return;
        }
        self.core.checkpoint(super::core::EditKind::Other);
        self.core.doc.cover = cover;
        self.core.dirty = false;
        cx.emit(EditorEvent::Changed);
        cx.notify();
    }

    fn apply_preset(&mut self, name: &str, cx: &mut Context<Self>) {
        let Some(bytes) = preset_bytes(name) else { return };
        match super::image::store_image(bytes, "jpg") {
            Ok(p) => self.set_cover(Some(Cover { src: p.to_string_lossy().to_string(), x: 0., y: 0. }), cx),
            Err(msg) => self.toast(msg, cx),
        }
    }

    /// 随机: a random series, then a random cover in it.
    fn random_cover(&mut self, cx: &mut Context<Self>) {
        let (_, prefix, n) = SERIES[super::menus::random_index(SERIES.len())];
        self.apply_preset(&format!("{prefix}-{}", super::menus::random_index(n) + 1), cx);
    }

    fn add_cover(&mut self, cx: &mut Context<Self>) {
        self.open_cover_menu(Some(Menu::CoverPick { upload: false }), cx);
    }

    fn upload_cover(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let rx = cx.prompt_for_paths(PathPromptOptions { files: true, directories: false, multiple: false, prompt: Some(crate::settings::text("上传图片").into()), extensions: None });
        cx.spawn_in(window, async move |this, cx| {
            if let Ok(Ok(Some(paths))) = rx.await
                && let Some(p) = paths.into_iter().next()
            {
                let _ = this.update_in(cx, |e, _, cx| {
                    let ext = p.extension().and_then(|s| s.to_str()).unwrap_or_default().to_string();
                    let stored = std::fs::read(&p).map_err(|_| super::image::BAD_FORMAT).and_then(|b| super::image::store_image(&b, &ext));
                    match stored {
                        Ok(p) => e.set_cover(Some(Cover { src: p.to_string_lossy().to_string(), x: 0., y: 0. }), cx),
                        Err(msg) => e.toast(msg, cx),
                    }
                });
            }
        })
        .detach();
    }

    fn open_cover_menu(&mut self, menu: Option<Menu>, cx: &mut Context<Self>) {
        self.menu = menu;
        self.menu_hovered = false;
        cx.notify();
    }

    fn start_cover_drag(&mut self, cx: &mut Context<Self>) {
        let Some(c) = &self.core.doc.cover else { return };
        self.menu = None;
        self.cover_ui.drag = Some(CoverDrag { orig: (c.x, c.y), x: c.x, y: c.y, moved: false, grab: None });
        cx.notify();
    }

    pub(super) fn cover_grabbed(&self) -> bool {
        self.cover_ui.drag.is_some_and(|d| d.grab.is_some())
    }

    pub(super) fn cover_drag_to(&mut self, pos: Point<Pixels>, cx: &mut Context<Self>) {
        let Some((iw, ih, cw, ch)) = self.cover_ui.geo.get() else { return };
        let Some(d) = &mut self.cover_ui.drag else { return };
        let Some((start, m0)) = d.grab else { return };
        let g = geometry(iw, ih, cw, ch, d.x, d.y);
        if g.horizontal {
            d.x = ratio(clamp_margin(m0 + f32::from(pos.x - start.x), g.w, cw), g.w, cw);
        } else {
            d.y = ratio(clamp_margin(m0 + f32::from(pos.y - start.y), g.h, ch), g.h, ch);
        }
        d.moved = true;
        cx.notify();
    }

    pub(super) fn end_cover_grab(&mut self, cx: &mut Context<Self>) {
        if let Some(d) = &mut self.cover_ui.drag {
            d.grab = None;
            cx.notify();
        }
    }

    fn finish_cover_drag(&mut self, keep: bool, cx: &mut Context<Self>) {
        let Some(d) = self.cover_ui.drag.take() else { return };
        if keep
            && (d.x, d.y) != d.orig
            && let Some(c) = self.core.doc.cover.clone()
        {
            self.set_cover(Some(Cover { x: d.x, y: d.y, ..c }), cx);
        }
        cx.notify();
    }

    // ---- header

    /// The 添加图标 / 添加封面 row: shown on header hover.
    pub(super) fn header_entries(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let doc = &self.core.doc;
        if self.read_only || (doc.icon.is_some() && doc.cover.is_some() && !self.info.hidden) {
            return None;
        }
        Some(
            div()
                .absolute()
                .top(px(20.))
                .left_0()
                .h(px(28.))
                .flex()
                .when(doc.icon.is_none(), |d| {
                    d.child(meta_entry("add-doc-icon", "smile-plus", crate::settings::text("添加图标")).on_click(cx.listener(|e, _, window, cx| e.add_doc_icon(window, cx))))
                })
                .when(doc.cover.is_none(), |d| d.child(meta_entry("add-doc-cover", "image", crate::settings::text("添加封面")).relative()
                    .child({ let bounds = self.cover_ui.edit_bounds.clone(); canvas(move |b, _, _| bounds.set(Some(b)), |_, _, _, _| {}).absolute().inset_0() })
                    .on_click(cx.listener(|e, _, _, cx| e.add_cover(cx)))))
                .when(self.info.hidden, |d| d.child(meta_entry("show-doc-info", "doc-info", crate::settings::text("展示文档信息")).on_click(cx.listener(|e, _, _, cx| e.show_info(cx)))))
                .into_any_element(),
        )
    }

    /// The 40×40 icon before the title, on the title's first baseline.
    pub(super) fn doc_icon_el(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let ic = self.core.doc.icon.clone()?;
        let record = self.cover_ui.icon_bounds.clone();
        let open = matches!(self.menu, Some(Menu::Emoji { target: EmojiTarget::DocIcon, .. }));
        let ro = self.read_only;
        Some(
            div()
                .id("doc-icon")
                .relative()
                .flex_none()
                .size(px(40.))
                // (55.25 line − 40) / 2: the glyphs share a baseline.
                .mt(px(7.625))
                .mr(px(6.))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(6.))
                .text_size(px(theme::title().size))
                .line_height(px(40.))
                .child(ic)
                .child(canvas(move |b, _, _| record.set(Some(b)), |_, _, _, _| {}).absolute().inset_0())
                .when(!ro, |d| {
                    d.cursor_pointer()
                        .when(open, |d| d.bg(Theme::fill_hover()))
                        .hover(|s| s.bg(Theme::fill_hover()))
                        .when(!open, |d| d.tip(crate::settings::text("更换图标"), None))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|e, _, window, cx| {
                                cx.stop_propagation();
                                e.toggle_emoji_picker(EmojiTarget::DocIcon, window, cx);
                            }),
                        )
                })
                .into_any_element(),
        )
    }

    /// The cover: the page-wide image above the header.
    pub(super) fn cover_el(&self, window: &Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        let c = self.core.doc.cover.as_ref()?;
        let ch = cover_height(window);
        let cw = self.layout.ml + self.layout.content + self.layout.mr;
        let drag = self.cover_ui.drag;
        let (rx, ry) = drag.map_or((c.x, c.y), |d| (d.x, d.y));
        let path = crate::store::image_path(&c.src);
        let dims = path.as_ref().and_then(|p| self.img.dims(p));
        self.cover_ui.geo.set(dims.map(|(iw, ih)| (iw, ih, cw, ch)));
        let image = path.zip(dims).map(|(p, (iw, ih))| {
            let g = geometry(iw, ih, cw, ch, rx, ry);
            img(p).absolute().left(px(g.ml)).top(px(g.mt)).w(px(g.w)).h(px(g.h)).object_fit(ObjectFit::Fill)
        });
        let menu_open = matches!(self.menu, Some(Menu::CoverEdit | Menu::CoverPick { .. }));
        let record = self.cover_ui.edit_bounds.clone();
        let toolbar = div()
            .absolute()
            .bottom(px(8.))
            .right(px(self.layout.mr))
            .flex()
            .items_center()
            .when(drag.is_none() && !menu_open, |d| d.opacity(0.).group_hover("doc-cover", |s| s.opacity(1.)))
            .map(|d| match drag {
                Some(_) => d
                    .child(toolbar_button("cover-cancel", crate::settings::text("取消"), false).on_click(cx.listener(|e, _, _, cx| e.finish_cover_drag(false, cx))))
                    .child(toolbar_button("cover-confirm", crate::settings::text("确认"), true).on_click(cx.listener(|e, _, _, cx| e.finish_cover_drag(true, cx)))),
                None => d.child(
                    toolbar_button("cover-edit", crate::settings::text("编辑封面"), false)
                        .child(canvas(move |b, _, _| record.set(Some(b)), |_, _, _, _| {}).absolute().inset_0())
                        .on_click(cx.listener(move |e, _, _, cx| e.open_cover_menu((!menu_open).then_some(Menu::CoverEdit), cx))),
                ),
            });
        let tip = drag.filter(|d| !d.moved).map(|_| {
            // A dark tip: near-black glass, white 12pt.
            div().absolute().inset_0().flex().items_center().justify_center().child(
                div()
                    .px(px(12.))
                    .py(px(6.))
                    .rounded(px(8.))
                    .bg(theme::ca(craft::N950, 0.72))
                    .text_size(px(12.))
                    .line_height(px(18.))
                    .text_color(gpui::white())
                    .child(crate::settings::text("拖拽以调整图片的位置")),
            )
        });
        Some(
            div()
                .id("doc-cover")
                .group("doc-cover")
                .relative()
                .w(px(cw))
                .h(px(ch))
                .overflow_hidden()
                // Edge to edge under the header, square corners: the quiet
                // fill shows until the image loads.
                .bg(Theme::bg_overlay())
                .children(image)
                .children(tip)
                .when(self.read_only, |d| d.child(div().absolute().inset_0()))
                .when(drag.is_some(), |d| d.cursor(if self.cover_grabbed() { CursorStyle::ClosedHand } else { CursorStyle::OpenHand }))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|e, ev: &MouseDownEvent, _, cx| {
                        cx.stop_propagation();
                        e.menu = None;
                        if let (Some(d), Some((iw, ih, cw, ch))) = (&mut e.cover_ui.drag, e.cover_ui.geo.get()) {
                            let g = geometry(iw, ih, cw, ch, d.x, d.y);
                            d.grab = Some((ev.position, if g.horizontal { g.ml } else { g.mt }));
                        }
                        cx.notify();
                    }),
                )
                .when(!self.read_only, |d| d.child(toolbar))
                .into_any_element(),
        )
    }

    // ---- popovers

    fn cover_popover_at(&self, el: impl IntoElement) -> Option<AnyElement> {
        let a = self.cover_ui.edit_bounds.get()?;
        let (x, corner) = if self.core.doc.cover.is_none() { (a.left(), Corner::TopLeft) } else { (a.right(), Corner::TopRight) };
        Some(at(self.rel(point(x, a.bottom() + px(4.))), corner, el))
    }

    /// 编辑封面: 选择图片 / 调整图片位置 / 移除封面, a native-style menu.
    pub(super) fn cover_menu(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let loaded = self.cover_ui.geo.get().is_some();
        let item = |id: &'static str, ic: &'static str, label: &'static str| {
            let g = SharedString::from(id);
            menu_row(id, g.clone())
                .pr(px(16.))
                .whitespace_nowrap()
                .child(menu_glyph(ic, 16., g))
                .child(label)
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        };
        let el = popover()
            .id("cover-menu")
            .occlude()
            .py(px(crate::editor_ui::MENU_PAD_Y))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(item("cover-pick", "image", crate::settings::text("选择图片")).on_click(cx.listener(|e, _, _, cx| e.open_cover_menu(Some(Menu::CoverPick { upload: false }), cx))))
            .when(loaded, |d| d.child(item("cover-move", "move", crate::settings::text("调整图片位置")).on_click(cx.listener(|e, _, _, cx| e.start_cover_drag(cx)))))
            .child(item("cover-remove", "trash-2", crate::settings::text("移除封面")).on_click(cx.listener(|e, _, _, cx| {
                e.menu = None;
                e.set_cover(None, cx);
            })));
        self.cover_popover_at(el)
    }

    /// 选择图片: 官方图库 / 本地上传, with 随机 and 移除 on the right.
    pub(super) fn cover_panel(&self, upload: bool, window: &Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        let a = self.cover_ui.edit_bounds.get()?;
        let h = (f32::from(window.viewport_size().height - a.bottom()) - 17.).clamp(120., 520.);
        // Tabs: the chosen one semibold in the accent over a 2pt accent bar.
        let tab = |id: &'static str, label: &'static str, on: bool, to_upload: bool| {
            div()
                .id(id)
                .relative()
                .h_full()
                .mr(px(24.))
                .flex()
                .items_center()
                .cursor_pointer()
                .when(on, |d| {
                    d.font_weight(FontWeight::SEMIBOLD)
                        .text_color(Theme::primary())
                        .child(div().absolute().bottom(px(-0.5)).left_0().right_0().h(px(2.)).rounded(px(1.)).bg(Theme::primary()))
                })
                .when(!on, |d| d.text_color(Theme::text_caption()).hover(|s| s.text_color(Theme::text())))
                .child(label)
                .on_click(cx.listener(move |e, _, _, cx| e.open_cover_menu(Some(Menu::CoverPick { upload: to_upload }), cx)))
        };
        let action = |id: &'static str| {
            div()
                .id(id)
                .h(px(26.))
                .px(px(6.))
                .flex()
                .items_center()
                .rounded(px(ROW_RADIUS))
                .cursor_pointer()
                .hover(|s| s.bg(Theme::fill_hover_strong()).text_color(Theme::text()))
                .active(|s| s.bg(Theme::fill_pressed()))
        };
        let header = div()
            .flex_none()
            .h(px(44.))
            .px(px(16.))
            .flex()
            .justify_between()
            .border_b_1()
            .border_color(Theme::line_divider())
            .text_size(px(13.))
            .text_color(Theme::text())
            .child(div().h_full().flex().child(tab("cover-tab-gallery", crate::settings::text("官方图库"), !upload, false)).child(tab("cover-tab-upload", crate::settings::text("本地上传"), upload, true)))
            .child(
                div()
                    .h_full()
                    .flex()
                    .items_center()
                    .text_color(Theme::text_caption())
                    .when(!upload, |d| {
                        d.child(
                            action("cover-random")
                                .child(div().mr(px(4.)).child(icon("shuffle", 16., Theme::text_caption())))
                                .child(crate::settings::text("随机"))
                                .on_click(cx.listener(|e, _, _, cx| e.random_cover(cx))),
                        )
                        .child(div().mx(px(10.)).w(px(1.)).h(px(16.)).bg(Theme::line_divider()))
                    })
                    .child(action("cover-panel-remove").child(crate::settings::text("移除")).on_click(cx.listener(|e, _, _, cx| {
                        e.menu = None;
                        e.set_cover(None, cx);
                    }))),
            );
        let body = if upload {
            div()
                .flex_1()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .child(
                    // The primary button: the accent, white 13pt semibold, radius 9.
                    div()
                        .id("cover-upload")
                        .min_w(px(270.))
                        .px(px(16.))
                        .rounded(px(9.))
                        .text_size(px(13.))
                        .line_height(px(30.))
                        .font_weight(FontWeight::SEMIBOLD)
                        .flex()
                        .justify_center()
                        .bg(Theme::primary())
                        .shadow(theme::shadow_button())
                        .hover(|s| s.bg(theme::c(craft::B600)))
                        .active(|s| s.bg(theme::c(craft::B700)))
                        .text_color(gpui::white())
                        .cursor_pointer()
                        .child(crate::settings::text("上传图片"))
                        .on_click(cx.listener(|e, _, window, cx| {
                            e.menu = None;
                            e.upload_cover(window, cx);
                        })),
                )
                .child(
                    div()
                        .mt(px(12.))
                        .px(px(32.))
                        .text_size(px(12.))
                        .line_height(px(20.))
                        .text_color(Theme::text_caption())
                        .child(crate::settings::text("推荐使用尺寸为 2,000 × 400 的图片，显示效果最佳")),
                )
                .into_any_element()
        } else {
            let mut list = div().pb(px(2.));
            for (si, (title, prefix, n)) in SERIES.iter().enumerate() {
                let mut thumbs = div().flex().flex_wrap().pl(px(10.));
                for i in 1..=*n {
                    let name = format!("{prefix}-{i}");
                    thumbs = thumbs.child(
                        div()
                            .id(("cover-thumb", si * 100 + i))
                            .relative()
                            .w(px(118.))
                            .h(px(64.))
                            .mx(px(6.))
                            .mb(px(10.))
                            .cursor_pointer()
                            .child(img(SharedString::from(format!("covers/thumb/{name}.jpg"))).size_full().rounded(px(6.)).object_fit(ObjectFit::Cover))
                            .child(
                                // A hairline over the thumbnail; darker under the pointer.
                                div()
                                    .id(("cover-thumb-mask", si * 100 + i))
                                    .absolute()
                                    .inset_0()
                                    .rounded(px(6.))
                                    .border_1()
                                    .border_color(Theme::swatch_border())
                                    .hover(|s| s.bg(theme::ca(craft::N1000, 0.15)))
                                    .active(|s| s.bg(theme::ca(craft::N1000, 0.2))),
                            )
                            .on_click(cx.listener(move |e, _, _, cx| e.apply_preset(&name, cx))),
                    );
                }
                list = list
                    .child(div().mt(px(2.)).mb(px(8.)).pl(px(16.)).h(px(22.)).text_size(px(12.)).line_height(px(22.)).font_weight(FontWeight::SEMIBOLD).text_color(Theme::text_caption()).child(crate::settings::text(title)))
                    .child(thumbs);
            }
            div()
                .id("cover-gallery")
                .flex_1()
                .min_h_0()
                .mt(px(10.))
                .overflow_y_scroll()
                .track_scroll(&self.cover_ui.gallery_scroll)
                .child(list)
                .into_any_element()
        };
        // The second-level panel: white, hairline, radius 12, the panel drop.
        let el = panel()
            .id("cover-panel")
            .occlude()
            .w(px(PANEL_W))
            .h(px(h))
            .flex()
            .flex_col()
            .overflow_hidden()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_scroll_wheel(|_, _, cx| cx.stop_propagation())
            .child(header)
            .child(body);
        self.cover_popover_at(el)
    }
}

#[cfg(test)]
mod tests {
    use super::{SERIES, geometry, preset_bytes, ratio};

    #[test]
    fn centred_and_shifted() {
        // 2000×400 in 1000×216: wider than the box, scaled to its height.
        let g = geometry(2000., 400., 1000., 216., 0., 0.);
        assert!(g.horizontal);
        assert_eq!((g.w, g.h), (1080., 216.));
        assert_eq!(g.ml, -40.);
        // Fully left; ratio round-trips through the margin.
        let g = geometry(2000., 400., 1000., 216., 1., 0.);
        assert_eq!(g.ml, 0.);
        assert_eq!(ratio(0., 1080., 1000.), 0.037037);
        assert_eq!(geometry(2000., 400., 1000., 216., 0.037037, 0.).ml.round(), 0.);
        // Taller: scaled to the width, moves vertically.
        let g = geometry(800., 800., 1000., 216., 0., -1.);
        assert!(!g.horizontal);
        assert_eq!(g.mt, -(1000. - 216.));
    }

    #[test]
    fn gallery_files_exist() {
        for (_, prefix, n) in SERIES {
            for i in 1..=n {
                assert!(preset_bytes(&format!("{prefix}-{i}")).is_some(), "{prefix}-{i}");
            }
        }
    }
}
