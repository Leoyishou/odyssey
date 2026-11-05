//! 分栏 (grid / column blocks): column gaps with the resize handle
//! and the 新增分栏 circle, the drag that redistributes two neighbours, and
//! the width animation after a column is added.

use super::core::{MAX_COLUMNS, MIN_COLUMN_RATIO};
use super::render::{Env, Item, R, margins, record_bounds};
use super::view::Editor;
use crate::model::*;
use crate::editor_theme::{self as theme, Theme, craft};
use crate::editor_ui::TipExt;
use gpui::{prelude::FluentBuilder, *};
use rustc_hash::FxHasher;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};

/// Visual gap between two column boxes.
const GAP: f32 = 20.;
/// Narrower containers stack their columns.
const HORIZONTAL_MIN: f32 = 578.;
const ANIMATION: Duration = Duration::from_millis(250);

/// A column resize in progress.
pub struct ColDrag {
    pub grid: BlockId,
    pub gap: usize,
    x0: f32,
    width: f32,
    start: Vec<f32>,
    live: Vec<f32>,
}

fn ratio_of(b: &Block) -> f32 {
    if let BlockKind::Column { ratio } = b.kind { ratio } else { 0. }
}

fn contains(b: &Block, id: BlockId) -> bool {
    b.id == id || b.children.iter().any(|c| contains(c, id))
}

/// The list item showing `id` (the deepest row on its path).
pub(super) fn item_containing(items: &[Item], blocks: &[Block], id: BlockId) -> Option<usize> {
    let p = super::core::find_path(blocks, id)?;
    let chain: Vec<usize> =
        p.iter().map_while(|s| if let super::core::Seg::Child(i) = s { Some(*i) } else { None }).collect();
    items
        .iter()
        .enumerate()
        .filter_map(|(i, it)| match it {
            Item::Block { path, .. } if chain.starts_with(path) => Some((path.len(), i)),
            _ => None,
        })
        .max()
        .map(|(_, i)| i)
}

impl Editor {
    /// The widths a grid is drawn with: live while dragging, easing after a
    /// column was added, otherwise the document's.
    pub(super) fn grid_ratios(&self, g: &Block) -> Vec<f32> {
        if let Some(d) = self.col_drag.as_ref().filter(|d| d.grid == g.id) {
            return d.live.clone();
        }
        let to: Vec<f32> = g.children.iter().map(ratio_of).collect();
        if let Some((from, t0)) = self.col_anim.get(&g.id)
            && from.len() == to.len()
        {
            let t = (t0.elapsed().as_secs_f32() / ANIMATION.as_secs_f32()).min(1.);
            return from.iter().zip(&to).map(|(a, b)| a + (b - a) * t).collect();
        }
        to
    }

    /// Grids drawn with widths other than the document's, keyed so their
    /// rows are re-measured every frame that the widths change.
    pub(super) fn grid_overrides(&self) -> Vec<(BlockId, u64)> {
        let mut out = vec![];
        let key = |r: &[f32]| {
            let mut h = FxHasher::default();
            r.iter().for_each(|x| x.to_bits().hash(&mut h));
            h.finish() | 1
        };
        if let Some(d) = &self.col_drag {
            out.push((d.grid, key(&d.live)));
        }
        for (&id, (_, t0)) in &self.col_anim {
            out.push((id, key(&[t0.elapsed().as_secs_f32()])));
        }
        out.extend(self.table_override());
        out
    }

    /// Drops finished animations; true while one is running.
    pub(super) fn tick_grid_animations(&mut self) -> bool {
        self.col_anim.retain(|_, (_, t0)| t0.elapsed() < ANIMATION);
        !self.col_anim.is_empty()
    }

    fn add_column(&mut self, grid: BlockId, at: usize, window: &mut Window, cx: &mut Context<Self>) {
        let mut from = self.core.column_ratios(grid);
        if self.core.add_column(grid, at).is_none() {
            return;
        }
        from.insert(at.min(from.len()), 0.);
        self.col_anim.insert(grid, (from, Instant::now()));
        self.gap_hover = None;
        window.focus(&self.focus);
        self.changed(window, cx);
    }

    fn start_col_drag(&mut self, grid: BlockId, gap: usize, x: Pixels) {
        let width = self.block_bounds.borrow().get(&grid).map(|b| f32::from(b.size.width)).unwrap_or(0.);
        let start = self.core.column_ratios(grid);
        if width <= 0. || gap + 1 >= start.len() {
            return;
        }
        self.col_anim.remove(&grid);
        self.col_drag = Some(ColDrag { grid, gap, x0: f32::from(x), width, live: start.clone(), start });
    }

    /// The column drag: the two neighbours trade width, snapping to an
    /// even split and to tenths, never narrower than 1/24.
    pub(super) fn col_drag_to(&mut self, x: Pixels, cx: &mut Context<Self>) {
        let Some(d) = &mut self.col_drag else { return };
        let (s, a) = (d.start[d.gap], d.start[d.gap + 1]);
        if s + a < 2. * MIN_COLUMN_RATIO {
            return;
        }
        let mut o = (f32::from(x) - d.x0) / d.width;
        let even = (s + a) / 2. - s;
        let tenth = (10. * (s + o)).round() / 10. - s;
        if (even - o).abs() < 0.01 {
            o = even;
        } else if (tenth - o).abs() < 0.01 {
            o = tenth;
        }
        o = o.max(MIN_COLUMN_RATIO - s).min(a - MIN_COLUMN_RATIO);
        d.live.clone_from(&d.start);
        d.live[d.gap] = s + o;
        d.live[d.gap + 1] = a - o;
        cx.notify();
    }

    /// Saves the dragged widths as one undo step.
    pub(super) fn end_col_drag(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(d) = self.col_drag.take() else { return };
        self.core.set_column_ratios(d.grid, &d.live);
        if self.core.dirty {
            self.changed(window, cx);
        }
        cx.notify();
    }

    /// While a column drag is on, a layer over the page keeps the resize
    /// cursor and stops hover effects underneath.
    pub(super) fn col_drag_overlay(&self) -> Option<AnyElement> {
        self.col_drag.as_ref()?;
        Some(
            deferred(div().id("col-drag-overlay").absolute().inset_0().occlude().cursor(CursorStyle::ResizeLeftRight))
                .with_priority(4)
                .into_any_element(),
        )
    }
}

impl R<'_> {
    pub(super) fn grid(&self, b: &Block, env: Env, cx: &mut Context<Editor>) -> AnyElement {
        let ed = self.ed;
        let n = b.children.len().max(1);
        let ratios = ed.grid_ratios(b);
        let sum = ratios.iter().sum::<f32>().max(1e-6);
        let horizontal = env.width >= HORIZONTAL_MIN;
        let dragging = ed.col_drag.as_ref().is_some_and(|d| d.grid == b.id);
        // Hovering a gap's circle (above the grid) still counts as hovering the grid.
        let focused = !ed.read_only
            && (dragging || ed.gap_hover.is_some_and(|h| h.0 == b.id) || (!ed.core.in_block_mode() && contains(b, ed.core.sel.head.block)));
        let group = SharedString::from(format!("grid-{}", b.id.0));
        // A top-level grid's column boxes overhang the text column by 2px.
        let span = env.width + if env.root { 4. } else { 0. };
        let free = span - GAP * (n - 1) as f32;
        let mut grid = div().relative().group(group.clone()).flex().when(!horizontal, |d| d.flex_col()).when(env.root, |d| d.mx(px(-2.)));
        for (i, c) in b.children.iter().enumerate() {
            let r = ratios.get(i).copied().unwrap_or(0.) / sum;
            let width = if horizontal { free * r - 4. } else { env.width };
            let col_env = Env { root: false, column: true, width, ..env };
            let cid = c.id;
            let first_mt = c.children.first().map(|f| margins(f, col_env).0).unwrap_or(0.);
            // The column's hover frame: crisp, like in-page containers.
            let area = div().absolute().top(px(-4.)).left(px(-4.)).right(px(-4.)).bottom(px(-4.)).rounded(px(5.));
            let mut col = div()
                .relative()
                .flex()
                .flex_col()
                .min_w_0()
                .px(px(2.))
                .child(if focused { area.bg(Theme::block_bg_area()) } else if ed.read_only { area } else { area.group_hover(group.clone(), |s| s.bg(Theme::block_bg_area())) })
                .child(self.blocks(&c.children, col_env, cx))
                .child(record_bounds(cid, self.bounds.clone()))
                .when(dragging, |d| {
                    // Above the column's text (z-index 1): the width, as a dark tip.
                    d.child(deferred(
                        div()
                            .absolute()
                            .top(px(-2.))
                            .right(px(-2.))
                            .px(px(5.))
                            .rounded(px(6.))
                            .bg(theme::ca(craft::N950, 0.75))
                            .font_family(theme::UI_FONT)
                            .text_size(px(11.))
                            .line_height(px(18.))
                            .text_color(Theme::bg_body())
                            .child(format!("{}%", (100. * ratios.get(i).copied().unwrap_or(0.)).round() as i32)),
                    ))
                });
            if horizontal {
                col.style().flex_basis = Some(px(0.).into());
                col.style().flex_grow = Some(r);
                col.style().flex_shrink = Some(1.);
                // Below the shorter columns' content: a click lands in a trailing empty line.
                col = col.child(div().flex_1().on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |e, _, window, cx| {
                        cx.stop_propagation();
                        e.core.focus_column_end(cid);
                        window.focus(&e.focus);
                        e.changed(window, cx);
                    }),
                ));
            } else if i > 0 {
                col = col.mt(px(first_mt));
            }
            grid = grid.child(col);
            if horizontal {
                grid = grid.child(self.gap(b, i, n, cx));
            }
        }
        grid.into_any_element()
    }

    /// The gap after column `i`: a 12px strip with the resize handle and,
    /// above it, the 新增分栏 circle. The gap after the last column only adds.
    fn gap(&self, b: &Block, i: usize, n: usize, cx: &mut Context<Editor>) -> Stateful<Div> {
        let ed = self.ed;
        let gid = b.id;
        let last = i + 1 == n;
        let drag = ed.col_drag.as_ref().filter(|d| d.grid == gid);
        let this_drag = drag.is_some_and(|d| d.gap == i);
        let hover = ed.gap_hover.filter(|h| h.0 == gid && h.1 == i);
        let circle_hover = hover.is_some_and(|h| h.2) && drag.is_none();
        let shown = (hover.is_some() || this_drag) && !ed.dragging && !ed.read_only;
        let key = gid.0.wrapping_mul(16).wrapping_add(i as u64);
        let hs_group = SharedString::from(format!("gap-hs-{key}"));
        let set_hover = move |circle: bool| {
            move |e: &mut Editor, hovered: &bool, _: &mut Window, cx: &mut Context<Editor>| {
                let me = (gid, i, circle);
                if *hovered {
                    e.gap_hover = Some(me);
                } else if e.gap_hover == Some(me) {
                    e.gap_hover = None;
                } else {
                    return;
                }
                cx.notify();
            }
        };

        let dragger = div()
            .w(px(2.))
            .flex_1()
            .min_h(px(26.))
            .rounded(px(2.))
            .when(!last, |d| {
                let blue = this_drag || circle_hover;
                d.bg(if blue { Theme::primary_500() } else { Theme::line_border() })
                    .when(drag.is_none(), |d| d.group_hover(hs_group.clone(), |s| s.bg(Theme::primary_500())))
            });
        // Hovering the circle stretches the handle up to it (top 4px, 100% + 20px).
        let (top, bottom) = if circle_hover { (-6., -14.) } else { (8., 8.) };
        let hot_spot = div()
            .id(("gap-hot", key))
            .flex_1()
            .min_h(px(26.))
            .w_full()
            .px(px(5.))
            .py(px(4.))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .group(hs_group)
            .child(dragger)
            .when(!last, |d| {
                d.cursor(CursorStyle::ResizeLeftRight).on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |e, ev: &MouseDownEvent, _, cx| {
                        cx.stop_propagation();
                        e.start_col_drag(gid, i, ev.position.x);
                        cx.notify();
                    }),
                )
            });
        let circle = (n < MAX_COLUMNS && drag.is_none()).then(|| {
            div()
                .id(("gap-add", key))
                .absolute()
                .top(px(-15.))
                .left(px(-2.))
                .size(px(16.))
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .on_hover(cx.listener(set_hover(true)))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |e, _, window, cx| {
                        cx.stop_propagation();
                        e.add_column(gid, i + 1, window, cx);
                    }),
                )
                .tip_above(crate::settings::text("新增分栏"), None)
                .when(!circle_hover, |d| d.child(div().size(px(3.)).rounded_full().bg(theme::c(craft::N400))))
                .when(circle_hover, |d| {
                    // The pin grows from its centre (scale 0 → 1, 0.15s).
                    d.child(
                        img("editor-icons/column-add.svg")
                            .absolute()
                            .with_animation(("gap-add-pin", key), Animation::new(Duration::from_millis(150)), |im, t| {
                                im.left(px(8. - 11. * t)).top(px(4. - 12. * t)).w(px(22. * t)).h(px(24. * t)).opacity(t)
                            }),
                    )
                })
        });
        let strip = div()
            .id(("gap-strip", key))
            .absolute()
            .top_0()
            .bottom_0()
            .left(px(4.))
            .w(px(12.))
            .cursor_default()
            .opacity(if shown { 1. } else { 0. })
            .on_hover(cx.listener(set_hover(false)))
            .child(
                div()
                    .absolute()
                    .top(px(top))
                    .bottom(px(bottom))
                    .left_0()
                    .right_0()
                    .flex()
                    .flex_col()
                    .justify_center()
                    .child(hot_spot),
            )
            .children(circle);
        let gap = div().id(("gap", key)).w(px(GAP)).flex_none().relative().child(strip);
        if last { gap.absolute().top_0().bottom_0().left(relative(1.)) } else { gap }
    }
}
