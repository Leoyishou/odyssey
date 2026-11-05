//! Image blocks: sizing, the corner resize handles, the crop tool, the image
//! toolbar, the lightbox viewer, dropped / pasted files, remote sources
//! (downloaded to a local cache) and the error toasts (image-spec,
//! crop-spec).

use super::core::{DropPlace, EditKind, Pos, Selection, TITLE};
use super::overlays::{Probe, at};
use super::render::{Env, R};
use super::view::{Editor, EditorEvent};
use crate::store::image_path;
use crate::model::*;
use crate::store::Store;
use crate::editor_theme::{self as theme, Theme, craft};
use crate::editor_ui::{ROW_RADIUS, TipExt, ToastKind, icon, popover};
use gpui::{prelude::FluentBuilder, *};
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Smallest image width and crop box side (`ZM`).
pub(super) const MIN: f32 = 32.;
/// Size of an image whose pixels can't be measured.
const FALLBACK: (f32, f32) = (480., 270.);
const FORMATS: [&str; 6] = ["jpeg", "jpg", "png", "gif", "webp", "bmp"];
const MAX_BYTES: u64 = 20 * 1024 * 1024;
pub(super) const BAD_FORMAT: &str = "图片格式仅支持 jpeg、jpg、png、gif、webp、bmp";
const BAD_DIMS: &str = "识别图片宽高失败";
const TOO_BIG: &str = "上传图片大小不能超过 20 MB";
const FULL: [f32; 4] = [0., 0., 1., 1.];
const FADE: f32 = 0.2;
/// Media in the page stays crisp: image corners round at 5 (an image
/// shown cropped keeps square corners — GPUI can only round a whole image).
const RADIUS: f32 = 5.;

/// `f32::clamp` without the panic when `lo > hi`.
fn clampf(v: f32, lo: f32, hi: f32) -> f32 {
    v.max(lo).min(hi)
}

/// `cubic-bezier(.34,.69,.1,1)`, roughly.
fn ease_out(k: f32) -> f32 {
    1. - (1. - k).powi(3)
}

actions!(image_viewer, [LbClose, LbPrev, LbNext, LbZoomIn, LbZoomOut, LbToggle, LbFit, LbRotate]);

/// The lightbox keys (viewer ~1195, ~4505).
pub fn bind_keys(cx: &mut App) {
    let c = Some("Lightbox");
    cx.bind_keys([
        KeyBinding::new("escape", LbClose, c),
        KeyBinding::new("left", LbPrev, c),
        KeyBinding::new("right", LbNext, c),
        KeyBinding::new("up", LbZoomIn, c),
        KeyBinding::new("cmd-=", LbZoomIn, c),
        KeyBinding::new("cmd-+", LbZoomIn, c),
        KeyBinding::new("down", LbZoomOut, c),
        KeyBinding::new("cmd--", LbZoomOut, c),
        KeyBinding::new("cmd-0", LbToggle, c),
        KeyBinding::new("cmd-1", LbToggle, c),
        KeyBinding::new("space", LbFit, c),
        KeyBinding::new("r", LbRotate, c),
    ]);
}

#[derive(Clone, Copy, PartialEq)]
enum Remote {
    Queued,
    Pending,
    Failed,
}

/// Where an image's pixels come from this frame.
enum Src {
    Local(PathBuf),
    /// A local path that doesn't exist (`文件不存在`).
    Missing,
    /// A remote image still downloading.
    Pending,
    /// A remote image that couldn't be fetched, or a cloud token.
    Failed,
    /// A remote image not fetched: 设置 turned downloads off.
    Blocked,
}

struct Resize {
    id: BlockId,
    from_x: Pixels,
    start_w: f32,
    /// −1 for the left handles.
    dir: f32,
    /// A centred image grows on both sides.
    mul: f32,
    max: f32,
    aspect: f32,
    changed: bool,
    cursor: CursorStyle,
}

struct CropDrag {
    /// The edge dragged on each axis (−1 left/top, 1 right/bottom, 0 none).
    hx: i8,
    hy: i8,
    moving: bool,
    from: Point<Pixels>,
    /// x, y, w, h in displayed px at the start.
    start: [f32; 4],
    /// Displayed size of the whole image.
    size: (f32, f32),
    cursor: CursorStyle,
}

struct CropEdit {
    id: BlockId,
    /// Live box: left, top, right, bottom fractions of the whole image.
    rect: [f32; 4],
    drag: Option<CropDrag>,
}

struct Lightbox {
    ids: Vec<BlockId>,
    ix: usize,
    scale: f32,
    tx: f32,
    ty: f32,
    vp: (f32, f32),
    need_fit: bool,
    /// Quarter turns counter-clockwise.
    rot: u8,
    /// The last toggle went to 原始尺寸.
    orig: bool,
    opened: Instant,
    closing: Option<Instant>,
    chrome: bool,
    chrome_t: Instant,
    last_move: Instant,
    press: Option<(Point<Pixels>, f32, f32)>,
    moved: bool,
    /// The decoded, rotated (and cropped) pixels for (rot, image).
    rotated: Option<(u8, BlockId, Option<Arc<RenderImage>>)>,
}

pub struct ImageUi {
    dims: RefCell<HashMap<PathBuf, Option<(f32, f32)>>>,
    /// Box of every image drawn this frame (window coordinates).
    pub(super) boxes: Rc<RefCell<HashMap<BlockId, Bounds<Pixels>>>>,
    /// Width of the column each image was last laid out in.
    containers: RefCell<HashMap<BlockId, f32>>,
    pub(super) hover: Option<BlockId>,
    /// The image whose description is being typed.
    pub(super) caption_focus: Option<BlockId>,
    resize: Option<Resize>,
    crop: Option<CropEdit>,
    lightbox: Option<Lightbox>,
    lb_focus: FocusHandle,
    chrome_task: Option<Task<()>>,
    toast: Option<(ToastKind, SharedString, Instant)>,
    toast_task: Option<Task<()>>,
    remote: RefCell<HashMap<String, Remote>>,
    remote_dir: PathBuf,
    /// Bumped when remote images arrive or are retried (every row re-measures).
    pub(super) generation: u64,
    retried: HashMap<String, Instant>,
    /// Where dragged-in files would land, and the drop line.
    pub(super) file_drop: Option<(BlockId, DropPlace, Bounds<Pixels>)>,
}

impl ImageUi {
    pub fn new(cx: &mut App) -> Self {
        ImageUi {
            dims: RefCell::default(),
            boxes: Rc::default(),
            containers: RefCell::default(),
            hover: None,
            caption_focus: None,
            resize: None,
            crop: None,
            lightbox: None,
            lb_focus: cx.focus_handle(),
            chrome_task: None,
            toast: None,
            toast_task: None,
            remote: RefCell::default(),
            remote_dir: Store::assets_dir().join("remote"),
            generation: 0,
            retried: HashMap::new(),
            file_drop: None,
        }
    }

    fn remote_path(&self, url: &str) -> PathBuf {
        self.remote_dir.join(remote_name(url))
    }

    fn resolve(&self, src: &str) -> Src {
        if src.starts_with("http://") || src.starts_with("https://") {
            if self.remote.borrow().get(src) == Some(&Remote::Failed) {
                return Src::Failed;
            }
            let p = self.remote_path(src);
            if self.dims.borrow().contains_key(&p) || p.exists() {
                return Src::Local(p);
            }
            // 设置 › 网络与隐私: with downloads off, only an image the user
            // retried is fetched.
            if !crate::settings::prefs().remote_images && !self.retried.contains_key(src) {
                return Src::Blocked;
            }
            self.remote.borrow_mut().entry(src.to_string()).or_insert(Remote::Queued);
            return Src::Pending;
        }
        match image_path(src) {
            Some(p) if self.dims.borrow().contains_key(&p) || p.exists() => Src::Local(p),
            Some(_) => Src::Missing,
            None => Src::Failed,
        }
    }

    /// Pixel size from the file header (cached).
    pub(super) fn dims(&self, p: &Path) -> Option<(f32, f32)> {
        if let Some(d) = self.dims.borrow().get(p) {
            return *d;
        }
        let d = image::ImageReader::open(p)
            .ok()
            .and_then(|r| r.with_guessed_format().ok())
            .and_then(|r| r.into_dimensions().ok())
            .map(|(w, h)| (w as f32, h as f32));
        self.dims.borrow_mut().insert(p.to_path_buf(), d);
        d
    }

    fn local(&self, src: &str) -> Option<PathBuf> {
        match self.resolve(src) {
            Src::Local(p) => Some(p),
            _ => None,
        }
    }
}

/// Laid-out size of an image.
#[derive(Clone, Copy)]
pub(super) struct Geo {
    /// The visible (cropped) part as shown.
    pub w: f32,
    pub h: f32,
    /// The whole image at the same zoom.
    pub fw: f32,
    pub fh: f32,
    pub crop: [f32; 4],
}

/// A crop that is in range, ordered and not the whole image.
fn valid_crop(c: Option<[f32; 4]>) -> Option<[f32; 4]> {
    c.filter(|c| c.iter().all(|v| (0. ..=1.).contains(v)) && c[0] < c[2] && c[1] < c[3] && *c != FULL)
}

/// `width`/`height` are the shown size of the visible part; without them the
/// image shows at its pixel size. Capped to the column (min 32 wide).
fn geometry(width: Option<f32>, height: Option<f32>, crop: Option<[f32; 4]>, natural: Option<(f32, f32)>, container: f32) -> Geo {
    let c = valid_crop(crop).unwrap_or(FULL);
    let (frw, frh) = (c[2] - c[0], c[3] - c[1]);
    let (w, h) = match (width, height, natural) {
        (Some(w), Some(h), _) if w > 0. && h > 0. => (w, h),
        (Some(w), _, Some((nw, nh))) if w > 0. && nw > 0. => (w, w * (nh * frh) / (nw * frw)),
        (_, _, Some((nw, nh))) if nw > 0. && nh > 0. => (nw * frw, nh * frh),
        _ => FALLBACK,
    };
    let dw = clampf(w, MIN, container.max(MIN));
    let dh = h * dw / w;
    Geo { w: dw, h: dh, fw: dw / frw, fh: dh / frh, crop: c }
}

/// The whole image while cropping: same zoom, capped to the column.
fn crop_display(g: &Geo, container: f32) -> (f32, f32) {
    let s = (container.max(MIN) / g.fw).min(1.);
    (g.fw * s, g.fh * s)
}

pub(super) fn spinning(id: &'static str, size: f32, color: Hsla, period: f32) -> AnyElement {
    svg()
        .path("editor-icons/spinner.svg")
        .size(px(size))
        .flex_none()
        .text_color(color)
        .with_animation(id, Animation::new(Duration::from_secs_f32(period)).repeat(), |s, t| {
            s.with_transformation(Transformation::rotate(percentage(t)))
        })
        .into_any_element()
}

/// `.loading`: the 50px grey spinner in the image's box.
fn loading_el() -> AnyElement {
    div()
        .size_full()
        .flex()
        .items_center()
        .justify_center()
        .child(spinning("img-loading", 50., theme::c(craft::N400), 2.))
        .into_any_element()
}

/// The error box: its content depends on the box size. A quiet fill in a
/// hairline, crisp corners, 13pt secondary text and a link-coloured 点击重试.
fn error_el(w: f32, h: f32, missing: bool, blocked: bool, border: Hsla, retry: Option<Rc<dyn Fn(&mut App)>>) -> AnyElement {
    let text = |s: &'static str| div().text_size(px(13.)).line_height(px(24.)).my(px(4.)).child(s);
    let link = |s: &'static str| {
        let r = retry.clone();
        div()
            .id("img-retry")
            .text_color(Theme::link())
            .cursor_pointer()
            .child(s)
            .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                cx.stop_propagation();
                if let Some(r) = &r {
                    r(cx);
                }
            })
    };
    let body = if missing {
        (w > 58. && h > 58.).then(|| text(crate::settings::text("文件不存在")).into_any_element())
    } else if blocked && w > 220. && h > 58. {
        Some(text("").flex().child(crate::settings::tr("未自动下载网络图片，", "Web image not downloaded. ")).child(link(crate::settings::tr("点击加载", "Load"))).into_any_element())
    } else if w > 162. && h > 58. {
        Some(text("").flex().child(crate::settings::text("加载失败，")).child(link(crate::settings::text("点击重试"))).into_any_element())
    } else if w > 58. && h > 58. {
        Some(text("").child(link(crate::settings::text("点击重试"))).into_any_element())
    } else {
        None
    };
    div()
        .absolute()
        .inset_0()
        .rounded(px(RADIUS))
        .border_1()
        .border_color(border)
        .bg(Theme::bg_overlay())
        .px(px(15.))
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .overflow_hidden()
        .text_color(Theme::text_caption())
        .font_family(theme::UI_FONT)
        .when(w > 120. && h > 162., |d| d.child(icon("image-broken", 56., Theme::text_placeholder()).mb(px(8.))))
        .children(body)
        .into_any_element()
}

impl R<'_> {
    pub(super) fn image(&self, b: &Block, env: Env, cx: &mut Context<Editor>) -> AnyElement {
        let BlockKind::Image { src, width, height, caption, crop, .. } = &b.kind else { return div().into_any_element() };
        let ed = self.ed;
        let ui = &ed.img;
        let id = b.id;
        let container = if env.width > 0. { env.width } else { ed.content_width() };
        if !self.ghost {
            ui.containers.borrow_mut().insert(id, container);
        }
        let source = ui.resolve(src);
        let natural = match &source {
            Src::Local(p) => ui.dims(p),
            _ => None,
        };
        let g = geometry(*width, *height, *crop, natural, container);
        let cropping = !self.ghost && ui.crop.as_ref().is_some_and(|c| c.id == id);
        let (bw, bh) = if cropping { crop_display(&g, container) } else { (g.w, g.h) };
        let active = !self.ghost && ed.image_active(id, self.focused);
        let hovered = !self.ghost && ui.hover == Some(id);
        let show_caption = caption.as_ref().is_some_and(|c| !c.is_empty() || (active && !ed.read_only));
        let ok = matches!(source, Src::Local(_));
        // The accent marks only the selection; the pointer gets a
        // quiet darker hairline.
        let ring = if active {
            Theme::primary()
        } else if hovered {
            theme::ca(craft::N1000, 0.16)
        } else {
            Theme::line_border()
        };
        // Whole images round their corners; a crop (or the crop tool) shows square ones.
        let radius = if cropping || g.crop != FULL { 0. } else { RADIUS };

        let weak = cx.weak_entity();
        let src_s = src.clone();
        let retry: Rc<dyn Fn(&mut App)> = Rc::new(move |cx: &mut App| {
            let _ = weak.update(cx, |e, cx| e.retry_image(&src_s, cx));
        });
        let content = match source {
            Src::Local(p) => {
                let (iw, ih, ix, iy) = if g.crop != FULL && !cropping {
                    (g.fw, g.fh, -g.crop[0] * g.fw, -g.crop[1] * g.fh)
                } else {
                    (bw, bh, 0., 0.)
                };
                let r = retry.clone();
                div()
                    .absolute()
                    .inset_0()
                    .overflow_hidden()
                    .child(
                        img(p)
                            .absolute()
                            .left(px(ix))
                            .top(px(iy))
                            .w(px(iw))
                            .h(px(ih))
                            .rounded(px(radius))
                            .object_fit(ObjectFit::Fill)
                            .with_loading(loading_el)
                            .with_fallback(move || error_el(bw, bh, false, false, ring, Some(r.clone())))
                            .id(("img", id.0)),
                    )
                    .into_any_element()
            }
            Src::Pending => loading_el(),
            Src::Missing => error_el(bw, bh, true, false, ring, None),
            Src::Failed => error_el(bw, bh, false, false, ring, Some(retry)),
            Src::Blocked => error_el(bw, bh, false, true, ring, Some(retry)),
        };
        // `.img:after`: drawn inside the image — a 2pt accent ring when
        // selected, the quiet hairline under the pointer.
        let outline = (ok && !cropping && (active || hovered)).then(|| {
            let d = div().absolute().inset_0().rounded(px(radius));
            if active { d.border_2().border_color(Theme::primary()) } else { d.border_1().border_color(ring) }
        });
        // Resize handles: white dots with a soft drop, the accent rim once selected.
        let handles = (ok && !self.ghost && !cropping && !ed.read_only && (active || hovered)).then(|| {
            [(true, true), (false, true), (true, false), (false, false)].map(|(left, top)| {
                let cursor = if left == top { CursorStyle::ResizeUpLeftDownRight } else { CursorStyle::ResizeUpRightDownLeft };
                div()
                    .id(("img-handle", id.0 * 4 + left as u64 * 2 + top as u64))
                    .absolute()
                    .size(px(20.))
                    .map(|d| if left { d.left(px(-9.)) } else { d.right(px(-9.)) })
                    .map(|d| if top { d.top(px(-9.)) } else { d.bottom(px(-9.)) })
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor(cursor)
                    .child(
                        div()
                            .size(px(10.))
                            .rounded_full()
                            .bg(gpui::white())
                            .border_1()
                            .border_color(if active { Theme::primary() } else { theme::ca(craft::N1000, 0.24) })
                            .shadow(vec![BoxShadow { color: Theme::shadow(), offset: point(px(0.), px(1.)), blur_radius: px(2.), spread_radius: px(0.) }]),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, ev: &MouseDownEvent, window, cx| {
                            cx.stop_propagation();
                            window.prevent_default();
                            e.start_resize(id, left, cursor, ev.position.x, window, cx);
                        }),
                    )
            })
        });
        let overlay = cropping.then(|| ui.crop.as_ref().map(|c| self.crop_overlay(c, bw, bh, cx))).flatten();
        let boxes = ui.boxes.clone();
        let recorder = (!self.ghost).then(|| {
            canvas(move |bounds, _, _| { boxes.borrow_mut().insert(id, bounds); }, |_, _, _, _| {})
                .absolute()
                .inset_0()
        });
        let image_box = div()
            .id(("image-box", id.0))
            .relative()
            .flex_none()
            .w(px(bw))
            .h(px(bh))
            .cursor(CursorStyle::PointingHand)
            .child(content)
            .children(outline)
            .when_some(handles, |d, hs| d.children(hs))
            .children(overlay)
            .children(recorder)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |e, ev: &MouseDownEvent, window, cx| {
                    // A click on the selected image opens the viewer.
                    if ev.click_count == 1 && !ev.modifiers.shift && e.img.crop.is_none() && e.image_active(id, e.focus.is_focused(window)) {
                        cx.stop_propagation();
                        window.prevent_default();
                        e.open_lightbox(id, window, cx);
                    }
                }),
            );
        // `.caption-editor-area`: 6px below, as wide as the image; the block
        // keeps 4px below it (`.with-caption`). The caption: 13pt, light,
        // the caption grey, centred.
        let caption_el = show_caption.then(|| {
            let wrap = div().id(("image-caption", id.0)).mt(px(6.)).w_full().flex().font_family(theme::UI_FONT);
            match ed.caption_field(id).filter(|_| !self.ghost) {
                Some(f) => wrap
                    .child(f)
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|e, _, _, cx| {
                            cx.stop_propagation();
                            e.core.clear_block_sel();
                            e.menu = None;
                            cx.notify();
                        }),
                    )
                    .into_any_element(),
                None => wrap
                    .justify_center()
                    .text_size(px(13.))
                    .line_height(px(20.))
                    .font_weight(FontWeight::LIGHT)
                    .text_color(Theme::caption())
                    .child(caption.clone().unwrap_or_default())
                    .into_any_element(),
            }
        });
        div()
            .w_full()
            .flex()
            .map(|d| match b.align {
                Align::Left => d.justify_start(),
                Align::Center => d.justify_center(),
                Align::Right => d.justify_end(),
            })
            .when(show_caption, |d| d.pb(px(4.)))
            .child(div().flex().flex_col().w(px(bw)).child(image_box).children(caption_el))
            .into_any_element()
    }

    /// `.image-cropper`: the dimmed outside, the dashed box, its brackets and
    /// the 8 grab zones (crop-spec §2).
    fn crop_overlay(&self, c: &CropEdit, fw: f32, fh: f32, cx: &mut Context<Editor>) -> AnyElement {
        let id = c.id;
        let [l, t, r, b] = c.rect;
        let (x, y, w, h) = (l * fw, t * fh, (r - l) * fw, (b - t) * fh);
        let mask = |n: u64, x: f32, y: f32, w: f32, h: f32| {
            div()
                .id(("crop-mask", n))
                .absolute()
                .left(px(x))
                .top(px(y))
                .w(px(w.max(0.)))
                .h(px(h.max(0.)))
                .bg(theme::rgbaf(0, 0, 0, 0.3))
                .cursor(CursorStyle::Arrow)
                .on_mouse_down(MouseButton::Left, |_, window, cx| {
                    cx.stop_propagation();
                    window.prevent_default();
                })
        };
        // Brackets: 6px bars, dark #2b2f36 inside a 1px white edge, 1px
        // outside the box.
        let k = |d: f32| ((d - 4.) / 2.).min(24.);
        let v = |d: f32| (d - 52.).min(24.);
        let (kw, kh) = (k(w), k(h));
        let mut bars: Vec<(f32, f32, f32, f32)> = vec![
            (-1., -1., kw, 6.),
            (-1., -1., 6., kh),
            (w + 1. - kw, -1., kw, 6.),
            (w + 1. - 6., -1., 6., kh),
            (-1., h + 1. - 6., kw, 6.),
            (-1., h + 1. - kh, 6., kh),
            (w + 1. - kw, h + 1. - 6., kw, 6.),
            (w + 1. - 6., h + 1. - kh, 6., kh),
        ];
        if w > 52. {
            let vw = v(w);
            bars.push(((w - vw) / 2., -1., vw, 6.));
            bars.push(((w - vw) / 2., h + 1. - 6., vw, 6.));
        }
        if h > 52. {
            let vh = v(h);
            bars.push((-1., (h - vh) / 2., 6., vh));
            bars.push((w + 1. - 6., (h - vh) / 2., 6., vh));
        }
        let bar = |(x, y, w, h): (f32, f32, f32, f32), inset: f32, color: Hsla| {
            div().absolute().left(px(x + inset)).top(px(y + inset)).w(px((w - 2. * inset).max(0.))).h(px((h - 2. * inset).max(0.))).bg(color)
        };
        let grab = |n: u64, hx: i8, hy: i8, moving: bool, cursor: CursorStyle| {
            div().id(("crop-grab", id.0 * 16 + n)).absolute().cursor(cursor).on_mouse_down(
                MouseButton::Left,
                cx.listener(move |e, ev: &MouseDownEvent, window, cx| {
                    cx.stop_propagation();
                    window.prevent_default();
                    if moving && ev.click_count == 2 {
                        return e.save_crop(true, window, cx);
                    }
                    e.start_crop_drag(hx, hy, moving, cursor, ev.position, cx);
                }),
            )
        };
        let corner = |hx: i8, hy: i8| if hx == hy { CursorStyle::ResizeUpLeftDownRight } else { CursorStyle::ResizeUpRightDownLeft };
        let mut area = div()
            .absolute()
            .left(px(x))
            .top(px(y))
            .w(px(w))
            .h(px(h))
            .child(div().absolute().inset_0().border_1().border_dashed().border_color(theme::rgbaf(255, 255, 255, 0.8)))
            .child(grab(0, 0, 0, true, CursorStyle::OpenHand).inset_0());
        for bb in &bars {
            area = area.child(bar(*bb, 0., gpui::white()));
        }
        for bb in &bars {
            area = area.child(bar(*bb, 1., theme::c(craft::N800)));
        }
        let s = 24.;
        area = area
            .child(grab(1, -1, -1, false, corner(-1, -1)).left_0().top_0().size(px(s)))
            .child(grab(2, 1, -1, false, corner(1, -1)).right_0().top_0().size(px(s)))
            .child(grab(3, -1, 1, false, corner(-1, 1)).left_0().bottom_0().size(px(s)))
            .child(grab(4, 1, 1, false, corner(1, 1)).right_0().bottom_0().size(px(s)))
            .child(grab(5, -1, 0, false, CursorStyle::ResizeLeftRight).left_0().top(px(h / 2. - 12.)).size(px(s)))
            .child(grab(6, 1, 0, false, CursorStyle::ResizeLeftRight).right_0().top(px(h / 2. - 12.)).size(px(s)))
            .child(grab(7, 0, -1, false, CursorStyle::ResizeUpDown).top_0().left(px(w / 2. - 12.)).size(px(s)))
            .child(grab(8, 0, 1, false, CursorStyle::ResizeUpDown).bottom_0().left(px(w / 2. - 12.)).size(px(s)))
            .child(grab(9, 0, 0, true, CursorStyle::OpenHand).left(px(12.)).top(px(12.)).w(px((w - 24.).max(0.))).h(px((h - 24.).max(0.))));
        div()
            .absolute()
            .inset_0()
            .child(mask(id.0 * 4, 0., 0., fw, y))
            .child(mask(id.0 * 4 + 1, 0., y + h, fw, fh - y - h))
            .child(mask(id.0 * 4 + 2, 0., y, x, h))
            .child(mask(id.0 * 4 + 3, x + w, y, fw - x - w, h))
            .child(area)
            .into_any_element()
    }
}

impl Editor {
    /// The image showing as selected (toolbar, handles, accent ring).
    pub(super) fn active_image(&self, focused: bool) -> Option<BlockId> {
        if let Some(id) = self.img.caption_focus {
            return Some(id);
        }
        let h = self.core.sel.head.block;
        (focused
            && !self.core.in_block_mode()
            && self.core.cell_sel.is_none()
            && self.core.sel.is_collapsed()
            && matches!(self.core.kind_of(h), Some(BlockKind::Image { .. })))
        .then_some(h)
    }

    pub(super) fn image_active(&self, id: BlockId, focused: bool) -> bool {
        self.active_image(focused) == Some(id)
    }

    /// Geometry and column width of an image as last laid out.
    fn image_geo(&self, id: BlockId) -> Option<(Geo, f32)> {
        let BlockKind::Image { src, width, height, crop, .. } = self.core.kind_of(id)? else { return None };
        let container = self.img.containers.borrow().get(&id).copied().unwrap_or(self.content_width());
        let natural = self.img.local(src).and_then(|p| self.img.dims(&p));
        Some((geometry(*width, *height, *crop, natural, container), container))
    }

    /// Row keys that change with the image UI: the empty description shown
    /// while its image is active, and the uncropped image while cropping.
    pub(super) fn image_nonces(&self, focused: bool) -> Vec<(BlockId, u64)> {
        let mut out = vec![];
        if let Some(id) = self.active_image(focused)
            && matches!(self.core.kind_of(id), Some(BlockKind::Image { caption: Some(c), .. }) if c.is_empty())
        {
            out.push((id, 0x5bd1_e995_u64.wrapping_mul(id.0 | 1)));
        }
        if let Some(c) = &self.img.crop {
            out.push((c.id, 0x9e37_79b9_u64.wrapping_mul(c.id.0 | 1) ^ 0xc0ff_ee));
        }
        out
    }

    pub(super) fn image_hover(&mut self, p: Point<Pixels>, cx: &mut Context<Self>) {
        let prev = self.img.hover;
        // The handles reach 9px outside the image.
        let hit = self
            .img
            .boxes
            .borrow()
            .iter()
            .find(|(id, b)| b.contains(&p) || (prev == Some(**id) && b.dilate(px(9.)).contains(&p)))
            .map(|(id, _)| *id);
        if hit != prev {
            self.img.hover = hit;
            cx.notify();
        }
    }

    fn retry_image(&mut self, src: &str, cx: &mut Context<Self>) {
        if self.img.retried.get(src).is_some_and(|t| t.elapsed() < Duration::from_secs(5)) {
            return;
        }
        self.img.retried.insert(src.to_string(), Instant::now());
        self.img.remote.borrow_mut().remove(src);
        let p = if src.starts_with("http://") || src.starts_with("https://") { Some(self.img.remote_path(src)) } else { image_path(src) };
        if let Some(p) = p {
            self.img.dims.borrow_mut().remove(&p);
            ImageSource::from(p).remove_asset(cx);
        }
        self.img.generation += 1;
        cx.notify();
    }

    // ---- resize

    fn start_resize(&mut self, id: BlockId, left: bool, cursor: CursorStyle, x: Pixels, window: &mut Window, cx: &mut Context<Self>) {
        let Some((g, container)) = self.image_geo(id) else { return };
        let center = self.core.block(id).is_some_and(|b| b.align == Align::Center);
        window.focus(&self.focus);
        self.menu = None;
        self.slash = None;
        self.core.clear_block_sel();
        self.core.cell_sel = None;
        self.core.sel = Selection::caret(Pos::new(id, 0));
        self.img.resize = Some(Resize {
            id,
            from_x: x,
            start_w: g.w,
            dir: if left { -1. } else { 1. },
            mul: if center { 2. } else { 1. },
            max: container.max(MIN),
            aspect: g.h / g.w,
            changed: false,
            cursor,
        });
        cx.notify();
    }

    pub(super) fn resizing_image(&self) -> bool {
        self.img.resize.is_some()
    }

    /// `width = start + round(dx)·dir·(2 if centred)`, 32 … column.
    pub(super) fn resize_to(&mut self, x: Pixels, cx: &mut Context<Self>) {
        let Some(r) = &self.img.resize else { return };
        let dx = f32::from(x - r.from_x).round();
        let w = clampf(r.start_w + dx * r.dir * r.mul, MIN, r.max);
        let (id, h) = (r.id, w * r.aspect);
        let same = matches!(self.core.kind_of(id), Some(BlockKind::Image { width: Some(cw), .. }) if (*cw - w).abs() < 0.01);
        if same || (!r.changed && (w - r.start_w).abs() < 0.01) {
            return;
        }
        if !r.changed {
            self.core.checkpoint(EditKind::Other);
        }
        if let Some(r) = &mut self.img.resize {
            r.changed = true;
        }
        if let Some(BlockKind::Image { width, height, .. }) = self.core.block_mut(id).map(|b| &mut b.kind) {
            *width = Some(w);
            *height = Some(h);
        }
        cx.notify();
    }

    pub(super) fn end_resize(&mut self, cx: &mut Context<Self>) {
        if let Some(r) = self.img.resize.take() {
            if r.changed {
                self.core.dirty = false;
                cx.emit(EditorEvent::Changed);
            }
            cx.notify();
        }
    }

    // ---- crop

    pub(super) fn cropping(&self) -> bool {
        self.img.crop.is_some()
    }

    fn start_crop(&mut self, id: BlockId, window: &mut Window, cx: &mut Context<Self>) {
        if self.img.crop.as_ref().is_some_and(|c| c.id != id) {
            self.save_crop(false, window, cx);
        }
        let Some((g, _)) = self.image_geo(id) else { return };
        if g.w < MIN || g.h < MIN {
            return;
        }
        self.img.crop = Some(CropEdit { id, rect: g.crop, drag: None });
        cx.notify();
    }

    /// Writes the live box (4 decimals) as one undo step; the shown size
    /// keeps the zoom the whole image had while cropping.
    pub(super) fn save_crop(&mut self, reselect: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(c) = self.img.crop.take() else { return };
        let round = |v: f32| (v * 10000.).round() / 10000.;
        let rect = c.rect.map(|v| clampf(round(v), 0., 1.));
        let new = valid_crop(Some(rect));
        if let Some((g, container)) = self.image_geo(c.id)
            && new.unwrap_or(FULL) != g.crop
        {
            let (fw, fh) = crop_display(&g, container);
            let r = new.unwrap_or(FULL);
            self.core.checkpoint(EditKind::Other);
            if let Some(BlockKind::Image { width, height, crop, .. }) = self.core.block_mut(c.id).map(|b| &mut b.kind) {
                *crop = new;
                *width = Some(fw * (r[2] - r[0]));
                *height = Some(fh * (r[3] - r[1]));
            }
            self.core.dirty = false;
            cx.emit(EditorEvent::Changed);
        }
        if reselect {
            window.focus(&self.focus);
            self.core.clear_block_sel();
            self.core.cell_sel = None;
            self.core.sel = Selection::caret(Pos::new(c.id, 0));
        }
        cx.notify();
    }

    pub(super) fn cancel_crop(&mut self, cx: &mut Context<Self>) {
        if self.img.crop.take().is_some() {
            cx.notify();
        }
    }

    /// 重置图片: back to the whole image at the same zoom.
    fn reset_crop(&mut self, id: BlockId, cx: &mut Context<Self>) {
        self.img.crop = None;
        let Some(BlockKind::Image { crop, .. }) = self.core.kind_of(id) else { return };
        let Some(c) = valid_crop(*crop) else { return cx.notify() };
        self.core.checkpoint(EditKind::Other);
        if let Some(BlockKind::Image { width, height, crop, .. }) = self.core.block_mut(id).map(|b| &mut b.kind) {
            *crop = None;
            *width = width.map(|w| w / (c[2] - c[0]));
            *height = height.map(|h| h / (c[3] - c[1]));
        }
        self.core.dirty = false;
        cx.emit(EditorEvent::Changed);
        cx.notify();
    }

    /// Saves the crop once its image stops being the selection; drops it
    /// when the image is gone.
    pub(super) fn sync_crop(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(c) = &self.img.crop else { return };
        let id = c.id;
        if !matches!(self.core.kind_of(id), Some(BlockKind::Image { .. })) {
            self.img.crop = None;
            return;
        }
        if c.drag.is_some() {
            return;
        }
        let caret = !self.core.in_block_mode() && self.core.sel.is_collapsed() && self.core.sel.head.block == id;
        let owner = caret || self.core.block_sel_ids() == [id] || self.img.caption_focus == Some(id);
        if !owner {
            self.save_crop(false, window, cx);
        }
    }

    fn start_crop_drag(&mut self, hx: i8, hy: i8, moving: bool, cursor: CursorStyle, from: Point<Pixels>, cx: &mut Context<Self>) {
        let Some(id) = self.img.crop.as_ref().map(|c| c.id) else { return };
        let Some((g, container)) = self.image_geo(id) else { return };
        let (fw, fh) = crop_display(&g, container);
        let Some(c) = &mut self.img.crop else { return };
        let [l, t, r, b] = c.rect;
        c.drag = Some(CropDrag {
            hx,
            hy,
            moving,
            from,
            start: [l * fw, t * fh, (r - l) * fw, (b - t) * fh],
            size: (fw, fh),
            cursor: if moving { CursorStyle::ClosedHand } else { cursor },
        });
        cx.notify();
    }

    pub(super) fn crop_dragging(&self) -> bool {
        self.img.crop.as_ref().is_some_and(|c| c.drag.is_some())
    }

    /// Moves or resizes the box from where the drag started (min 32, inside
    /// the image); Shift keeps its aspect.
    pub(super) fn crop_drag_to(&mut self, p: Point<Pixels>, shift: bool, cx: &mut Context<Self>) {
        let Some(c) = &mut self.img.crop else { return };
        let Some(d) = &c.drag else { return };
        let (dx, dy) = (f32::from(p.x - d.from.x), f32::from(p.y - d.from.y));
        let (fw, fh) = d.size;
        let [x, y, w, h] = d.start;
        let next = if d.moving {
            Some([clampf(x + dx, 0., fw - w), clampf(y + dy, 0., fh - h), w, h])
        } else {
            let axis = |e: i8, x: f32, w: f32, dx: f32, max: f32| match e {
                -1 => {
                    let nx = clampf(x + dx, 0., x + w - MIN);
                    (nx, x + w - nx)
                }
                1 => (x, clampf(w + dx, MIN, max - x)),
                _ => (x, w),
            };
            let (nx, nw) = axis(d.hx, x, w, dx, fw);
            let (ny, nh) = axis(d.hy, y, h, dy, fh);
            if !shift {
                Some([nx, ny, nw, nh])
            } else {
                let a = w / h;
                let r = match (d.hx, d.hy) {
                    (_, 0) => [nx, y + (h - nw / a) / 2., nw, nw / a],
                    (0, _) => [x + (w - nh * a) / 2., ny, nh * a, nh],
                    (_, hy) if w <= h => {
                        let nh = nw / a;
                        [nx, if hy < 0 { y + h - nh } else { y }, nw, nh]
                    }
                    (hx, _) => {
                        let nw = nh * a;
                        [if hx < 0 { x + w - nw } else { x }, ny, nw, nh]
                    }
                };
                let ok = r[2] >= MIN - 0.01 && r[3] >= MIN - 0.01 && r[0] >= -0.01 && r[1] >= -0.01 && r[0] + r[2] <= fw + 0.01 && r[1] + r[3] <= fh + 0.01;
                ok.then_some(r)
            }
        };
        let Some([nx, ny, nw, nh]) = next else { return };
        let rect = [nx / fw, ny / fh, (nx + nw) / fw, (ny + nh) / fh].map(|v| clampf(v, 0., 1.));
        if rect != c.rect {
            c.rect = rect;
            cx.notify();
        }
    }

    pub(super) fn end_crop_drag(&mut self, cx: &mut Context<Self>) {
        if let Some(c) = &mut self.img.crop
            && c.drag.take().is_some()
        {
            cx.notify();
        }
    }

    /// The layer over the page while a handle or the crop box is dragged:
    /// keeps the cursor and the events with the drag.
    pub(super) fn image_drag_layer(&self) -> Option<AnyElement> {
        let cursor = match (&self.img.resize, &self.img.crop) {
            (Some(r), _) => r.cursor,
            (_, Some(CropEdit { drag: Some(d), .. })) => d.cursor,
            _ => return None,
        };
        Some(div().id("image-drag-layer").absolute().inset_0().occlude().cursor(cursor).into_any_element())
    }

    // ---- toolbar

    /// 裁剪 · 重置图片 | 添加描述 | 左 · 中 · 右 above the selected image.
    pub(super) fn image_toolbar(&self, window: &Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        let focused = self.focus.is_focused(window);
        let id = match &self.img.crop {
            Some(c) => c.id,
            None => self.active_image(focused)?,
        };
        if self.menu.is_some()
            || self.slash.is_some()
            || self.img.resize.is_some()
            || self.img.lightbox.is_some()
            || self.dragging
            || self.block_drag.is_some()
            || self.crop_dragging()
        {
            return None;
        }
        let rect = self.probed(Probe::Image(id))?;
        let viewport = self.list.viewport_bounds();
        if !rect.intersects(&viewport) {
            return None;
        }
        let b = self.core.block(id)?;
        let BlockKind::Image { src, caption, crop, .. } = &b.kind else { return None };
        let ok = self.img.local(src).is_some();
        let cropping = self.img.crop.as_ref().is_some_and(|c| c.id == id);
        let reset = match &self.img.crop {
            Some(c) if cropping => c.rect != FULL,
            _ => valid_crop(*crop).is_some(),
        };
        let small = !cropping && self.image_geo(id).is_some_and(|(g, _)| g.w.round() < MIN || g.h.round() < MIN);
        let has_caption = caption.as_ref().is_some_and(|c| !c.is_empty());
        let align = b.align;

        // Toolbar buttons: 24 square, radius 6, a grey hover; a toggle that
        // is on gets the accent glyph on the accent tint.
        let btn = |id: &'static str| {
            div()
                .id(id)
                .size(px(24.))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(ROW_RADIUS))
                .cursor_pointer()
                .hover(|s| s.bg(Theme::fill_hover_strong()))
        };
        let sep = || div().w(px(1.)).h(px(18.)).bg(Theme::line_divider());
        let act = |f: fn(&mut Editor, BlockId, &mut Window, &mut Context<Editor>)| {
            cx.listener(move |e: &mut Editor, _: &MouseDownEvent, window, cx| {
                cx.stop_propagation();
                window.prevent_default();
                f(e, id, window, cx);
            })
        };
        let mut bar = div().h(px(40.)).px(px(8.)).flex().items_center().gap(px(8.));
        let mut w = 16. + 2.;
        let mut n = 0;
        if ok {
            let crop_btn = if small {
                div()
                    .id("itb-crop")
                    .size(px(24.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .tip_above(crate::settings::text("图片过小，请放大后重试"), None)
                    .child(icon("crop", 18., Theme::text_disabled()))
            } else {
                btn("itb-crop")
                    .when(cropping, |d| d.bg(Theme::fill_turninto_active()))
                    .tip_above(if cropping { crate::settings::text("保存裁剪") } else { crate::settings::text("裁剪") }, None)
                    .child(icon("crop", 18., if cropping { Theme::primary() } else { Theme::text_code() }))
                    .on_mouse_down(
                        MouseButton::Left,
                        act(|e, id, window, cx| {
                            if e.img.crop.as_ref().is_some_and(|c| c.id == id) {
                                e.save_crop(true, window, cx);
                            } else {
                                e.start_crop(id, window, cx);
                            }
                        }),
                    )
            };
            bar = bar.child(crop_btn);
            w += 24.;
            n += 1;
            if reset {
                bar = bar.child(
                    btn("itb-reset")
                        .tip_above(crate::settings::text("重置图片"), None)
                        .child(icon("image-reset", 18., Theme::text_code()))
                        .on_mouse_down(MouseButton::Left, act(|e, id, _, cx| e.reset_crop(id, cx))),
                );
                w += 24.;
                n += 1;
            }
            bar = bar.child(sep());
            w += 1.;
            n += 1;
        }
        bar = bar
            .child(
                btn("itb-caption")
                    .tip_above(if has_caption { crate::settings::text("编辑描述") } else { crate::settings::text("添加描述") }, None)
                    .child(icon("caption", 18., Theme::text_code()))
                    .on_mouse_down(MouseButton::Left, act(|e, id, window, cx| e.edit_caption(id, window, cx))),
            )
            .child(sep());
        w += 25.;
        n += 2;
        for (key, ic, tip, a) in [
            ("itb-left", "align-left", crate::settings::text("左对齐"), Align::Left),
            ("itb-center", "align-center", crate::settings::text("居中对齐"), Align::Center),
            ("itb-right", "align-right", crate::settings::text("右对齐"), Align::Right),
        ] {
            let on = align == a;
            bar = bar.child(
                btn(key)
                    .when(on, |d| d.bg(Theme::fill_turninto_active()))
                    .tip_above(tip, None)
                    .child(icon(ic, 18., if on { Theme::primary() } else { Theme::text_code() }))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, _, window, cx| {
                            cx.stop_propagation();
                            window.prevent_default();
                            e.core.set_align(&[id], a);
                            e.changed(window, cx);
                        }),
                    ),
            );
            w += 24.;
            n += 1;
        }
        w += (n - 1) as f32 * 8.;
        let x = clampf(f32::from(rect.center().x) - w / 2., f32::from(viewport.left()) + 8., f32::from(viewport.right()) - w - 8.);
        let h = 42.;
        let above = f32::from(rect.top()) - 8. - h;
        let y = if above >= f32::from(viewport.top()) { above } else { f32::from(rect.bottom()) + 8. };
        let el = popover()
            .id("image-toolbar")
            .occlude()
            .on_mouse_down(MouseButton::Left, |_, window, cx| {
                cx.stop_propagation();
                window.prevent_default();
            })
            .child(bar);
        Some(at(self.rel(point(px(x), px(y))), Corner::TopLeft, el))
    }

    /// 添加描述 / 编辑描述: shows the description and puts the caret in it.
    fn edit_caption(&mut self, id: BlockId, window: &mut Window, cx: &mut Context<Self>) {
        if self.img.crop.is_some() {
            self.save_crop(false, window, cx);
        }
        // An empty description is only shown while it's being typed (it's
        // dropped again on blur), so this isn't an undo step.
        if let Some(BlockKind::Image { caption, .. }) = self.core.block_mut(id).map(|b| &mut b.kind)
            && caption.is_none()
        {
            *caption = Some(String::new());
        }
        self.img.caption_focus = Some(id);
        if let Some(f) = self.caption_field(id) {
            window.focus(&f.read(cx).focus_handle(cx));
        }
        cx.notify();
    }

    /// The description field lost focus: an empty one goes away.
    pub(super) fn caption_blurred(&mut self, id: BlockId, cx: &mut Context<Self>) {
        if self.img.caption_focus == Some(id) {
            self.img.caption_focus = None;
        }
        if let Some(BlockKind::Image { caption, .. }) = self.core.block_mut(id).map(|b| &mut b.kind)
            && caption.as_deref() == Some("")
        {
            *caption = None;
        }
        cx.notify();
    }

    // ---- lightbox

    fn open_lightbox(&mut self, id: BlockId, window: &mut Window, cx: &mut Context<Self>) {
        let mut ids = vec![];
        self.core.doc.for_each_block(&mut |b: &Block| {
            if matches!(b.kind, BlockKind::Image { .. }) {
                ids.push(b.id);
            }
        });
        let ix = ids.iter().position(|i| *i == id).unwrap_or(0);
        let now = Instant::now();
        let vp = window.viewport_size();
        self.img.lightbox = Some(Lightbox {
            ids,
            ix,
            scale: 1.,
            tx: 0.,
            ty: 0.,
            vp: (f32::from(vp.width), f32::from(vp.height)),
            need_fit: true,
            rot: 0,
            orig: false,
            opened: now,
            closing: None,
            chrome: false,
            chrome_t: now - Duration::from_secs(1),
            last_move: now,
            press: None,
            moved: false,
            rotated: None,
        });
        window.focus(&self.img.lb_focus);
        cx.notify();
    }

    fn close_lightbox(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(lb) = &mut self.img.lightbox else { return };
        if lb.closing.is_none() {
            lb.closing = Some(Instant::now());
            window.focus(&self.focus);
            cx.notify();
        }
    }

    /// Path, crop and pixel size of the visible part of image `id`.
    fn lb_source(&self, id: BlockId) -> Option<(PathBuf, [f32; 4], f32, f32)> {
        let BlockKind::Image { src, crop, .. } = self.core.kind_of(id)? else { return None };
        let p = self.img.local(src)?;
        let c = valid_crop(*crop).unwrap_or(FULL);
        let (w, h) = match self.img.dims(&p) {
            Some((w, h)) => (w * (c[2] - c[0]), h * (c[3] - c[1])),
            None => self.image_geo(id).map(|(g, _)| (g.w, g.h))?,
        };
        Some((p, c, w, h))
    }

    /// Rotated pixel size of the current image.
    fn lb_size(&self) -> Option<(f32, f32)> {
        let lb = self.img.lightbox.as_ref()?;
        let (_, _, w, h) = self.lb_source(*lb.ids.get(lb.ix)?)?;
        Some(if lb.rot % 2 == 1 { (h, w) } else { (w, h) })
    }

    /// The initial fit (viewer `_` ~3809).
    fn lb_fit(&mut self) {
        let Some((w, h)) = self.lb_size() else { return };
        let Some(lb) = &mut self.img.lightbox else { return };
        let (vw, vh) = lb.vp;
        if h / w > 2.4 && h >= vh && w > 400. {
            lb.scale = (520f32).min(0.95 * vw) / w;
            lb.tx = 0.;
            lb.ty = (h * lb.scale - vh) / 2.;
        } else {
            let cap: f32 = if w < 400. || h < 400. {
                2.55
            } else if w < 800. || h < 800. {
                2.
            } else {
                1.
            };
            lb.scale = cap.min(0.95 * vw / w).min(0.9 * vh / h);
            lb.tx = 0.;
            lb.ty = 0.;
        }
        lb.need_fit = false;
    }

    /// Keeps the image edges outside the viewport; an axis that fits is centred.
    fn lb_clamp(&mut self) {
        let Some((w, h)) = self.lb_size() else { return };
        let Some(lb) = &mut self.img.lightbox else { return };
        let fix = |t: f32, size: f32, vp: f32| if size <= vp { 0. } else { clampf(t, -(size - vp) / 2., (size - vp) / 2.) };
        lb.tx = fix(lb.tx, w * lb.scale, lb.vp.0);
        lb.ty = fix(lb.ty, h * lb.scale, lb.vp.1);
    }

    fn lb_zoom(&mut self, factor: f32, cx: &mut Context<Self>) {
        let Some(lb) = &mut self.img.lightbox else { return };
        let s = clampf(lb.scale * factor, 0.1, 5.);
        let k = s / lb.scale;
        lb.scale = s;
        lb.tx *= k;
        lb.ty *= k;
        lb.orig = false;
        self.lb_clamp();
        cx.notify();
    }

    fn lb_go(&mut self, d: isize, cx: &mut Context<Self>) {
        let Some(lb) = &mut self.img.lightbox else { return };
        let ix = lb.ix as isize + d;
        if ix < 0 || ix as usize >= lb.ids.len() {
            return;
        }
        lb.ix = ix as usize;
        lb.rot = 0;
        lb.need_fit = true;
        lb.orig = false;
        cx.notify();
    }

    fn lb_toggle(&mut self, cx: &mut Context<Self>) {
        let Some(lb) = &mut self.img.lightbox else { return };
        if lb.orig && (lb.scale - 1.).abs() < 0.01 {
            self.lb_fit();
        } else {
            lb.scale = 1.;
            lb.tx = 0.;
            lb.ty = 0.;
            lb.orig = true;
            self.lb_clamp();
        }
        cx.notify();
    }

    fn lb_rotate(&mut self, cx: &mut Context<Self>) {
        let Some(lb) = &mut self.img.lightbox else { return };
        lb.rot = (lb.rot + 1) % 4;
        lb.tx = 0.;
        lb.ty = 0.;
        self.lb_clamp();
        cx.notify();
    }

    fn lb_show_chrome(&mut self, cx: &mut Context<Self>) {
        let Some(lb) = &mut self.img.lightbox else { return };
        lb.last_move = Instant::now();
        if !lb.chrome {
            lb.chrome = true;
            lb.chrome_t = Instant::now();
            cx.notify();
        }
        self.img.chrome_task = Some(cx.spawn(async |this, cx| {
            cx.background_executor().timer(Duration::from_secs(5)).await;
            let _ = this.update(cx, |e, cx| {
                if let Some(lb) = &mut e.img.lightbox
                    && lb.chrome
                    && lb.last_move.elapsed() >= Duration::from_millis(4990)
                {
                    lb.chrome = false;
                    lb.chrome_t = Instant::now();
                    cx.notify();
                }
            });
        }));
    }

    /// 下载: saves the shown (cropped) image.
    fn lb_download(&mut self, cx: &mut Context<Self>) {
        let Some(lb) = &self.img.lightbox else { return };
        let Some((path, crop, _, _)) = lb.ids.get(lb.ix).and_then(|id| self.lb_source(*id)) else { return };
        let fmt = image::ImageReader::open(&path).ok().and_then(|r| r.with_guessed_format().ok()).and_then(|r| r.format());
        let ext = fmt.and_then(|f| f.extensions_str().first().copied()).unwrap_or("png");
        let dir = dirs::download_dir().or_else(dirs::home_dir).unwrap_or_default();
        let rx = cx.prompt_for_new_path(&dir, Some(&format!("image.{ext}")));
        let bg = cx.background_executor().clone();
        cx.spawn(async move |_, _| {
            let Ok(Ok(Some(dest))) = rx.await else { return };
            bg.spawn(async move {
                if crop == FULL {
                    let _ = std::fs::copy(&path, &dest);
                } else if let Ok(im) = image::ImageReader::open(&path).and_then(|r| r.with_guessed_format()).map_err(anyhow::Error::from).and_then(|r| Ok(r.decode()?)) {
                    let (w, h) = (im.width() as f32, im.height() as f32);
                    let (x, y) = ((crop[0] * w).round() as u32, (crop[1] * h).round() as u32);
                    let (cw, ch) = (((crop[2] - crop[0]) * w).round().max(1.) as u32, ((crop[3] - crop[1]) * h).round().max(1.) as u32);
                    let _ = im.crop_imm(x, y, cw, ch).save(&dest);
                }
            })
            .await;
        })
        .detach();
    }

    /// Decodes the rotated pixels of the current image off the main thread.
    fn lb_ensure_rotated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(lb) = &mut self.img.lightbox else { return };
        let Some(&id) = lb.ids.get(lb.ix) else { return };
        let rot = lb.rot;
        if rot == 0 || lb.rotated.as_ref().is_some_and(|(r, i, _)| *r == rot && *i == id) {
            return;
        }
        if let Some((_, _, Some(old))) = lb.rotated.take() {
            let _ = window.drop_image(old);
        }
        lb.rotated = Some((rot, id, None));
        let Some((path, crop, _, _)) = self.lb_source(id) else { return };
        let task = cx.background_executor().spawn(async move { decode_rotated(&path, crop, rot) });
        cx.spawn_in(window, async move |this, cx| {
            let Some(im) = task.await else { return };
            let _ = this.update_in(cx, |e, window, cx| match &mut e.img.lightbox {
                Some(lb) if matches!(lb.rotated, Some((r, i, None)) if r == rot && i == id) => {
                    lb.rotated = Some((rot, id, Some(im)));
                    cx.notify();
                }
                _ => {
                    let _ = window.drop_image(im);
                }
            });
        })
        .detach();
    }

    /// Animations, the fit and the end of the closing fade.
    pub(super) fn tick_image_ui(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.img.toast.as_ref().is_some_and(|(_, _, t)| t.elapsed() >= Duration::from_secs(3)) {
            self.img.toast = None;
        }
        if !cx.has_active_drag() {
            self.img.file_drop = None;
        }
        // The image stays selected (toolbar, outline) while its description
        // is typed in. gpui's focus listeners don't see fields inside list
        // rows, so this follows the focus here instead.
        let typing = self
            .captions
            .iter()
            .find(|(id, (f, _))| f.read(cx).focus_handle(cx).is_focused(window) && matches!(self.core.kind_of(**id), Some(BlockKind::Image { .. })))
            .map(|(id, _)| *id);
        if let Some(id) = self.img.caption_focus
            && typing != Some(id)
        {
            self.caption_blurred(id, cx);
        }
        self.img.caption_focus = typing;
        let Some(lb) = &mut self.img.lightbox else { return };
        if lb.closing.is_some_and(|t| t.elapsed().as_secs_f32() >= FADE) {
            if let Some((_, _, Some(im))) = lb.rotated.take() {
                let _ = window.drop_image(im);
            }
            self.img.lightbox = None;
            self.img.chrome_task = None;
            return;
        }
        let vp = window.viewport_size();
        let vp = (f32::from(vp.width), f32::from(vp.height));
        if lb.vp != vp {
            lb.vp = vp;
            lb.need_fit = true;
        }
        let animating = lb.opened.elapsed().as_secs_f32() < FADE || lb.closing.is_some() || lb.chrome_t.elapsed().as_secs_f32() < FADE;
        let cur = lb.ids.get(lb.ix).copied();
        if cur.is_none_or(|id| self.lb_source(id).is_none()) {
            // The image went away (undo, another edit).
            if let Some(lb) = &mut self.img.lightbox
                && lb.closing.is_none()
            {
                lb.closing = Some(Instant::now());
                window.focus(&self.focus);
            }
            window.request_animation_frame();
            return;
        }
        if self.img.lightbox.as_ref().is_some_and(|l| l.need_fit) {
            self.lb_fit();
        }
        self.lb_ensure_rotated(window, cx);
        if animating {
            window.request_animation_frame();
        }
    }

    pub(super) fn lightbox_el(&self, window: &mut Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        let lb = self.img.lightbox.as_ref()?;
        let id = *lb.ids.get(lb.ix)?;
        let (path, crop, _, _) = self.lb_source(id)?;
        let (w, h) = self.lb_size()?;
        let (vw, vh) = lb.vp;
        let fade = match lb.closing {
            Some(t) => 1. - (t.elapsed().as_secs_f32() / FADE).min(1.),
            None => ease_out((lb.opened.elapsed().as_secs_f32() / FADE).min(1.)),
        };
        let k = ease_out((lb.chrome_t.elapsed().as_secs_f32() / FADE).min(1.));
        let chrome = if lb.chrome { k } else { 1. - k };
        let (sw, sh) = (w * lb.scale, h * lb.scale);
        let (left, top) = (vw / 2. + lb.tx - sw / 2., vh / 2. + lb.ty - sh / 2.);
        let can_drag = sw > vw + 0.5 || sh > vh + 0.5;
        let pic = if lb.rot == 0 {
            let (frw, frh) = (crop[2] - crop[0], crop[3] - crop[1]);
            let (fw, fh) = (sw / frw, sh / frh);
            div()
                .absolute()
                .left(px(left))
                .top(px(top))
                .w(px(sw))
                .h(px(sh))
                .overflow_hidden()
                .bg(gpui::white())
                .child(
                    img(path)
                        .absolute()
                        .left(px(-crop[0] * fw))
                        .top(px(-crop[1] * fh))
                        .w(px(fw))
                        .h(px(fh))
                        .object_fit(ObjectFit::Fill)
                        .with_loading(|| {
                            div().size_full().flex().items_center().justify_center().child(spinning("lb-loading", 40., theme::rgbaf(255, 255, 255, 0.6), 1.4)).into_any_element()
                        })
                        .id(("lb-img", id.0)),
                )
        } else {
            let base = div().absolute().left(px(left)).top(px(top)).w(px(sw)).h(px(sh));
            match &lb.rotated {
                Some((_, _, Some(im))) => base.bg(gpui::white()).child(img(im.clone()).size_full().object_fit(ObjectFit::Fill)),
                _ => base.flex().items_center().justify_center().child(spinning("lb-loading", 40., theme::rgbaf(255, 255, 255, 0.6), 1.4)),
            }
        };
        let pic = div()
            .id("lb-pic")
            .absolute()
            .inset_0()
            .child(pic.id("lb-pic-box").when(can_drag, |d| d.cursor(if lb.press.is_some() { CursorStyle::ClosedHand } else { CursorStyle::OpenHand })));

        let white = gpui::white();
        // Chrome over the dark backdrop: near-black glass, white glyphs.
        let close = div()
            .id("lb-close")
            .absolute()
            .top(px(20.))
            .right(px(20.))
            .size(px(40.))
            .rounded_full()
            .bg(theme::ca(craft::N950, 0.6))
            .hover(|s| s.bg(theme::ca(craft::N800, 0.75)))
            .border_1()
            .border_color(theme::rgbaf(255, 255, 255, 0.1))
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .opacity(chrome)
            .child(icon("x", 20., white))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|e, _, window, cx| {
                    cx.stop_propagation();
                    e.close_lightbox(window, cx);
                }),
            );
        let tool = |key: &'static str, ic: &'static str, tip: &'static str, enabled: bool, f: fn(&mut Editor, &mut Window, &mut Context<Editor>)| {
            div()
                .id(key)
                .size(px(28.))
                .mx(px(4.))
                .rounded(px(ROW_RADIUS))
                .flex()
                .items_center()
                .justify_center()
                .tip_above(tip, None)
                .child(icon(ic, 18., white))
                .map(|d| {
                    if enabled {
                        d.cursor_pointer().hover(|s| s.bg(theme::rgbaf(255, 255, 255, 0.16))).on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |e, _, window, cx| {
                                cx.stop_propagation();
                                f(e, window, cx);
                            }),
                        )
                    } else {
                        d.opacity(0.4)
                    }
                })
        };
        let sep = || div().w(px(1.)).h(px(18.)).mx(px(8.)).my(px(2.)).bg(theme::rgbaf(255, 255, 255, 0.2));
        let label = |s: String, min: f32| div().min_w(px(min)).flex().justify_center().text_size(px(13.)).line_height(px(20.)).text_color(white).child(s);
        let n = lb.ids.len();
        let fitted = lb.orig && (lb.scale - 1.).abs() < 0.01;
        // The controls: a dark glass pill, rounded like the header pills.
        let pill = div()
            .id("lb-pill")
            .h(px(44.))
            .px(px(10.))
            .rounded(px(12.))
            .bg(theme::ca(craft::N950, 0.78))
            .border_1()
            .border_color(theme::rgbaf(255, 255, 255, 0.1))
            .shadow(theme::shadow_panel())
            .flex()
            .items_center()
            .font_family(theme::UI_FONT)
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_mouse_up(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(tool("lb-prev", "chevron-left", crate::settings::text("上一张"), lb.ix > 0, |e, _, cx| e.lb_go(-1, cx)))
            .child(label(format!("{}/{}", lb.ix + 1, n), 0.))
            .child(tool("lb-next", "chevron-right", crate::settings::text("下一张"), lb.ix + 1 < n, |e, _, cx| e.lb_go(1, cx)))
            .child(sep())
            .child(tool("lb-out", "zoom-out", crate::settings::text("缩小"), lb.scale > 0.1 + 1e-4, |e, _, cx| e.lb_zoom(1. / 1.15, cx)))
            .child(label(format!("{}%", (lb.scale * 100.).round() as i32), 40.))
            .child(tool("lb-in", "zoom-in", crate::settings::text("放大"), lb.scale < 5. - 1e-4, |e, _, cx| e.lb_zoom(1.15, cx)))
            .child(if fitted {
                tool("lb-size", "fit-page", crate::settings::text("适应页面"), true, |e, _, cx| e.lb_toggle(cx))
            } else {
                tool("lb-size", "one-to-one", crate::settings::text("原始尺寸"), true, |e, _, cx| e.lb_toggle(cx))
            })
            .child(sep())
            .child(tool("lb-rotate", "rotate-ccw-square", crate::settings::text("旋转"), true, |e, _, cx| e.lb_rotate(cx)))
            .child(tool("lb-download", "download", crate::settings::text("下载"), true, |e, _, cx| e.lb_download(cx)));
        let pill = div().absolute().bottom(px(20.)).left_0().w(px(vw)).flex().justify_center().opacity(chrome).child(pill);

        let layer = div()
            .id("lightbox")
            .key_context("Lightbox")
            .track_focus(&self.img.lb_focus)
            .occlude()
            .relative()
            .w(px(vw))
            .h(px(vh))
            .bg(theme::rgbaf(0, 0, 0, 0.8))
            .opacity(fade)
            .when(!lb.chrome && k >= 1., |d| d.cursor(CursorStyle::None))
            .on_action(cx.listener(|e, _: &LbClose, window, cx| e.close_lightbox(window, cx)))
            .on_action(cx.listener(|e, _: &LbPrev, _, cx| e.lb_go(-1, cx)))
            .on_action(cx.listener(|e, _: &LbNext, _, cx| e.lb_go(1, cx)))
            .on_action(cx.listener(|e, _: &LbZoomIn, _, cx| e.lb_zoom(1.15, cx)))
            .on_action(cx.listener(|e, _: &LbZoomOut, _, cx| e.lb_zoom(1. / 1.15, cx)))
            .on_action(cx.listener(|e, _: &LbToggle, _, cx| e.lb_toggle(cx)))
            .on_action(cx.listener(|e, _: &LbFit, _, cx| {
                e.lb_fit();
                if let Some(lb) = &mut e.img.lightbox {
                    lb.orig = false;
                }
                cx.notify();
            }))
            .on_action(cx.listener(|e, _: &LbRotate, _, cx| e.lb_rotate(cx)))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|e, ev: &MouseDownEvent, _, cx| {
                    if let Some(lb) = &mut e.img.lightbox {
                        lb.press = Some((ev.position, lb.tx, lb.ty));
                        lb.moved = false;
                        cx.notify();
                    }
                }),
            )
            .on_mouse_move(cx.listener(|e, ev: &MouseMoveEvent, _, cx| {
                e.lb_show_chrome(cx);
                let can_drag = e.lb_size().zip(e.img.lightbox.as_ref()).is_some_and(|((w, h), lb)| w * lb.scale > lb.vp.0 + 0.5 || h * lb.scale > lb.vp.1 + 0.5);
                let Some(lb) = &mut e.img.lightbox else { return };
                let Some((from, tx, ty)) = lb.press else { return };
                if ev.pressed_button != Some(MouseButton::Left) {
                    lb.press = None;
                    return cx.notify();
                }
                let d = ev.position - from;
                if f32::from(d.x).hypot(f32::from(d.y)) > 5. {
                    lb.moved = true;
                }
                if can_drag {
                    lb.tx = tx + f32::from(d.x);
                    lb.ty = ty + f32::from(d.y);
                    e.lb_clamp();
                    cx.notify();
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|e, _: &MouseUpEvent, window, cx| {
                    let Some(lb) = &mut e.img.lightbox else { return };
                    // A click without a drag, on the image or around it, closes.
                    if lb.press.take().is_some() && !lb.moved {
                        e.close_lightbox(window, cx);
                    }
                    cx.notify();
                }),
            )
            .on_scroll_wheel(cx.listener(|e, ev: &ScrollWheelEvent, _, cx| {
                let (dx, dy, lines) = match ev.delta {
                    ScrollDelta::Pixels(p) => (f32::from(p.x), f32::from(p.y), false),
                    ScrollDelta::Lines(l) => (l.x, l.y, true),
                };
                let (dx, dy) = if lines { (dx * 150., dy * 150.) } else { (dx, dy) };
                if ev.modifiers.control || ev.modifiers.platform {
                    let f = 1. + dy.abs() / if ev.modifiers.control { 100. } else { 400. };
                    e.lb_zoom(if dy > 0. { f } else { 1. / f }, cx);
                } else if let Some(lb) = &mut e.img.lightbox {
                    lb.tx += 3. * dx;
                    lb.ty += 3. * dy;
                    e.lb_clamp();
                    cx.notify();
                }
            }))
            .child(pic)
            .child(close)
            .child(pill);
        let _ = window;
        Some(deferred(anchored().position_mode(AnchoredPositionMode::Window).position(point(px(0.), px(0.))).child(layer)).with_priority(10).into_any_element())
    }

    // ---- toast

    /// The error toast (image-spec, paste and link failures).
    pub(super) fn toast(&mut self, msg: &str, cx: &mut Context<Self>) {
        self.toast_kind(ToastKind::Error, msg, cx);
    }

    pub fn clear_toast(&mut self, cx: &mut Context<Self>) {
        if self.img.toast.take().is_some() {
            cx.notify();
        }
    }

    pub(super) fn toast_kind(&mut self, kind: ToastKind, msg: &str, cx: &mut Context<Self>) {
        self.img.toast = Some((kind, SharedString::from(msg.to_string()), Instant::now()));
        // One toast at a time: this one replaces the workspace's.
        cx.emit(super::view::EditorEvent::Toast);
        self.img.toast_task = Some(cx.spawn(async |this, cx| {
            cx.background_executor().timer(Duration::from_millis(2700)).await;
            let _ = this.update(cx, |_, cx| cx.notify());
        }));
        cx.notify();
    }

    pub(super) fn toast_el(&self, window: &mut Window) -> Option<AnyElement> {
        let (kind, msg, t0) = self.img.toast.as_ref()?;
        crate::editor_ui::toast_el(*kind, msg, *t0, window)
    }

    // ---- inserting

    /// Dragged files over the page: the drop line follows the pointer.
    pub(super) fn file_drag_move(&mut self, ev: &DragMoveEvent<ExternalPaths>, _: &mut Window, cx: &mut Context<Self>) {
        let p = ev.event.position;
        let d = if ev.bounds.contains(&p) { self.file_drop_at(p) } else { None };
        if d != self.img.file_drop {
            self.img.file_drop = d;
            cx.notify();
        }
    }

    pub(super) fn file_drop_line(&self) -> Option<AnyElement> {
        let (_, _, l) = self.img.file_drop?;
        let o = self.content_origin.get();
        // The drop indicator: a 2pt accent line, its ends rounded.
        Some(div().absolute().left(l.left() - o.x).top(l.top() - o.y).w(l.size.width).h(px(2.)).rounded(px(1.)).bg(Theme::primary()).into_any_element())
    }

    pub(super) fn drop_files(&mut self, paths: &ExternalPaths, window: &mut Window, cx: &mut Context<Self>) {
        let target = self.img.file_drop.take();
        let fallback = || {
            let h = self.core.sel.head.block;
            match self.core.doc.blocks.first() {
                Some(first) if h == TITLE => Some((first.id, DropPlace::Before)),
                _ if self.core.block(h).is_some() => Some((h, DropPlace::After)),
                _ => self.core.doc.blocks.last().map(|b| (b.id, DropPlace::After)),
            }
        };
        let target = target.map(|(t, p, _)| (t, p)).or_else(fallback);
        // Images become image blocks, anything else an attachment after them.
        let (pics, others): (Vec<&PathBuf>, Vec<&PathBuf>) = paths.paths().iter().partition(|p| {
            let ext = p.extension().and_then(|s| s.to_str()).unwrap_or_default().to_ascii_lowercase();
            FORMATS.contains(&ext.as_str())
        });
        let mut blocks = vec![];
        let mut err = None;
        for p in pics {
            match image_block_from_file(p) {
                Ok(b) => blocks.push(b),
                Err(e) => err = Some(e),
            }
        }
        if let Some(e) = err {
            self.toast(e, cx);
        }
        if !others.is_empty() {
            let after = blocks.last().map(|b| (b.id, DropPlace::After)).or(target);
            self.import_files(others.into_iter().cloned().collect(), after, window, cx);
        }
        let Some(last) = blocks.last().map(|b| b.id) else { return cx.notify() };
        match target {
            Some((t, place)) => self.core.insert_blocks_at(blocks, t, place),
            None => {
                self.core.checkpoint(EditKind::Other);
                self.core.doc.blocks.extend(blocks);
            }
        }
        self.core.clear_block_sel();
        self.core.cell_sel = None;
        self.core.sel = Selection::caret(Pos::new(last, 0));
        window.focus(&self.focus);
        self.changed(window, cx);
    }

    // ---- remote images

    /// Fetches queued remote images into the cache (`curl`, off the main thread).
    pub(super) fn start_downloads(&mut self, cx: &mut Context<Self>) {
        let queued: Vec<String> = self.img.remote.borrow().iter().filter(|(_, s)| **s == Remote::Queued).map(|(u, _)| u.clone()).collect();
        for url in queued {
            self.img.remote.borrow_mut().insert(url.clone(), Remote::Pending);
            let dest = self.img.remote_path(&url);
            let u = url.clone();
            let task = cx.background_executor().spawn(async move { download(&u, &dest) });
            cx.spawn(async move |this, cx| {
                let ok = task.await;
                let _ = this.update(cx, |e, cx| {
                    if ok {
                        e.img.remote.borrow_mut().remove(&url);
                    } else {
                        e.img.remote.borrow_mut().insert(url, Remote::Failed);
                    }
                    e.img.generation += 1;
                    cx.notify();
                });
            })
            .detach();
        }
    }
}

fn download(url: &str, dest: &Path) -> bool {
    if let Some(d) = dest.parent() {
        let _ = std::fs::create_dir_all(d);
    }
    let tmp = dest.with_extension("part");
    let fetched = std::process::Command::new("curl")
        .args(["-fsSL", "--max-time", "60", "-o"])
        .arg(&tmp)
        .arg(url)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|s| s.success());
    let ok = fetched
        && image::ImageReader::open(&tmp).ok().and_then(|r| r.with_guessed_format().ok()).and_then(|r| r.into_dimensions().ok()).is_some()
        && std::fs::rename(&tmp, dest).is_ok();
    if !ok {
        let _ = std::fs::remove_file(&tmp);
    }
    ok
}

/// Crops and turns the image `rot` quarter turns counter-clockwise (BGRA).
fn decode_rotated(path: &Path, crop: [f32; 4], rot: u8) -> Option<Arc<RenderImage>> {
    let im = image::ImageReader::open(path).ok()?.with_guessed_format().ok()?.decode().ok()?;
    let (w, h) = (im.width() as f32, im.height() as f32);
    let im = if crop == FULL {
        im
    } else {
        let (x, y) = ((crop[0] * w).round() as u32, (crop[1] * h).round() as u32);
        im.crop_imm(x, y, (((crop[2] - crop[0]) * w).round() as u32).max(1), (((crop[3] - crop[1]) * h).round() as u32).max(1))
    };
    let im = match rot {
        1 => im.rotate270(),
        2 => im.rotate180(),
        3 => im.rotate90(),
        _ => im,
    };
    let mut buf = im.into_rgba8();
    for px in buf.pixels_mut() {
        px.0.swap(0, 2);
    }
    Some(Arc::new(RenderImage::new(vec![image::Frame::new(buf)])))
}

/// A new image block for pasted or picked bytes, stored in the assets
/// folder (format, size and pixel checks on upload).
pub(super) fn image_block_from(bytes: &[u8], ext: &str) -> Result<Block, &'static str> {
    let path = store_image(bytes, ext)?;
    let mut b = Block::new(
        BlockKind::Image { src_kind: "src".into(), src: path.to_string_lossy().to_string(), width: None, height: None, caption: None, crop: None },
        RichText::default(),
    );
    b.align = Align::Center;
    Ok(b)
}

/// Checks picked or pasted image bytes and copies them into the assets folder.
pub(super) fn store_image(bytes: &[u8], ext: &str) -> Result<PathBuf, &'static str> {
    let ext = ext.to_ascii_lowercase();
    if !FORMATS.contains(&ext.as_str()) {
        return Err(BAD_FORMAT);
    }
    if bytes.len() as u64 > MAX_BYTES {
        return Err(TOO_BIG);
    }
    image::ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .ok()
        .and_then(|r| r.into_dimensions().ok())
        .ok_or(BAD_DIMS)?;
    let path = Store::assets_dir().join(format!("{}.{ext}", uuid::Uuid::new_v4()));
    std::fs::write(&path, bytes).map_err(|_| BAD_DIMS)?;
    Ok(path)
}

fn image_block_from_file(p: &Path) -> Result<Block, &'static str> {
    let ext = p.extension().and_then(|s| s.to_str()).unwrap_or_default().to_ascii_lowercase();
    if !p.is_file() || !FORMATS.contains(&ext.as_str()) {
        return Err(BAD_FORMAT);
    }
    if std::fs::metadata(p).map(|m| m.len()).unwrap_or(0) > MAX_BYTES {
        return Err(TOO_BIG);
    }
    let bytes = std::fs::read(p).map_err(|_| BAD_DIMS)?;
    image_block_from(&bytes, &ext)
}

/// File name of a remote image's download in `assets/remote`.
fn remote_name(url: &str) -> String {
    let mut h = rustc_hash::FxHasher::default();
    url.hash(&mut h);
    format!("{:016x}", h.finish())
}

/// The downloaded copy of a remote image, if it has been fetched.
pub fn cached_remote(url: &str) -> Option<PathBuf> {
    Some(Store::assets_dir().join("remote").join(remote_name(url))).filter(|p| p.is_file())
}

/// Pasted HTML images: `data:` URIs and local files become owned assets;
/// unreadable ones are dropped. Remote images stay remote.
pub(crate) fn localize_pasted_images(blocks: &mut Vec<Block>) {
    use base64::Engine;
    blocks.retain_mut(|b| {
        localize_pasted_images(&mut b.children);
        if let BlockKind::Table(t) = &mut b.kind {
            for cell in t.rows.iter_mut().flatten() {
                localize_pasted_images(&mut cell.blocks);
                if cell.blocks.is_empty() && !cell.is_covered() {
                    cell.blocks.push(Block::text(""));
                }
            }
        }
        let BlockKind::Image { src, .. } = &b.kind else { return true };
        let local = if let Some(data) = src.strip_prefix("data:image/") {
            let Some((head, payload)) = data.split_once(',') else { return false };
            let ext = head.split(';').next().unwrap_or_default().replace("jpeg", "jpg");
            let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(payload.trim()) else { return false };
            image_block_from(&bytes, &ext)
        } else if src.starts_with('/') {
            image_block_from_file(Path::new(src.as_str()))
        } else {
            return true;
        };
        match local {
            Ok(n) => {
                b.kind = n.kind;
                true
            }
            Err(_) => false,
        }
    });
}
