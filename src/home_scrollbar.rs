//! The page scrollbar: a thin trackless thumb on the right edge, drawn over
//! the virtualized list after it lays out, so it never lags a frame behind.

use crate::theme::{self, Theme, craft};
use gpui::*;
use std::cell::Cell;

const W: f32 = 4.;
const W_HOT: f32 = 6.;
const INSET: f32 = 2.;
const MIN_H: f32 = 32.;
/// The grab zone is wider than the thumb.
const HIT_W: f32 = 12.;

#[derive(Default)]
pub struct Scrollbar {
    /// The pointer was over the thumb last frame.
    hot: Cell<bool>,
    /// While dragging: pointer offset into the thumb, and the content and
    /// thumb heights at the press (held still so the thumb stays under the
    /// pointer as newly measured blocks refine the estimate).
    grab: Cell<Option<(Pixels, Pixels, Pixels)>>,
}

impl Scrollbar {
    pub fn render(self: &std::rc::Rc<Self>, handle: &ScrollHandle) -> impl IntoElement {
        let list = handle.clone();
        let sb = self.clone();
        let sb2 = sb.clone();
        let list2 = list.clone();
        canvas(
            move |bounds, window, _| {
                let view = list.bounds().size.height;
                let content = view + list.max_offset().height;
                let offset = -list.offset().y;
                let (content, h) = match sb.grab.get() {
                    Some((_, content, h)) => (content, h),
                    None => (content, (view * (view / content)).max(px(MIN_H)).min(bounds.size.height)),
                };
                if content <= view + px(1.) {
                    return None;
                }
                let travel = bounds.size.height - h;
                let top = bounds.top() + travel * (offset / (content - view)).clamp(0., 1.);
                let zone = Bounds::new(point(bounds.right() - px(HIT_W), top), size(px(HIT_W), h));
                Some((top, h, content, view, window.insert_hitbox(zone, HitboxBehavior::BlockMouseExceptScroll)))
            },
            move |bounds, state, window, _| {
                let sb = sb2;
                let Some((top, h, content, _view, hitbox)) = state else {
                    sb.grab.set(None);
                    return;
                };
                window.set_cursor_style(CursorStyle::Arrow, &hitbox);
                let hovered = hitbox.is_hovered(window);
                sb.hot.set(hovered);
                let dragging = sb.grab.get().is_some();
                let w = if hovered || dragging { W_HOT } else { W };
                // Black at 22%, darker under the pointer and while held.
                let color = if dragging {
                    theme::ca(craft::N1000, 0.4)
                } else if hovered {
                    theme::ca(craft::N1000, 0.32)
                } else {
                    Theme::scrollbar_thumb()
                };
                let thumb = Bounds::new(point(bounds.right() - px(INSET + w), top), size(px(w), h));
                window.paint_quad(fill(thumb, color).corner_radii(px(w / 2.)));

                window.on_mouse_event({
                    let (sb, hitbox) = (sb.clone(), hitbox.clone());
                    move |ev: &MouseDownEvent, phase, window, cx| {
                        if phase == DispatchPhase::Bubble && ev.button == MouseButton::Left && hitbox.is_hovered(window) {
                            sb.grab.set(Some((ev.position.y - top, content, h)));
                            cx.stop_propagation();
                            window.refresh();
                        }
                    }
                });
                window.on_mouse_event({
                    let (sb, hitbox, list) = (sb.clone(), hitbox.clone(), list2.clone());
                    move |ev: &MouseMoveEvent, phase, window, _| {
                        if phase != DispatchPhase::Bubble {
                            return;
                        }
                        match sb.grab.get() {
                            Some((dy, _, h)) if ev.pressed_button == Some(MouseButton::Left) => {
                                let travel = (bounds.size.height - h).max(px(1.));
                                let f = ((ev.position.y - dy - bounds.top()) / travel).clamp(0., 1.);
                                // At the end, go to the real bottom: the rows there may not
                                // be measured yet, so the estimate can stop short.
                                list.set_offset(point(list.offset().x, -list.max_offset().height * f));
                                window.refresh();
                            }
                            // The button went up somewhere we didn't see.
                            Some(_) => {
                                sb.grab.set(None);
                                window.refresh();
                            }
                            None if hitbox.is_hovered(window) != sb.hot.get() => window.refresh(),
                            None => {}
                        }
                    }
                });
                window.on_mouse_event({
                    let sb = sb.clone();
                    move |ev: &MouseUpEvent, phase, window, _| {
                        if phase == DispatchPhase::Bubble && ev.button == MouseButton::Left && sb.grab.take().is_some() {
                            window.refresh();
                        }
                    }
                });
            },
        )
        .absolute()
        .top_0()
        .right_0()
        .bottom_0()
        .w(px(HIT_W))
    }
}
