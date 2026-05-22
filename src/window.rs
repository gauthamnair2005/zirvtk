use alloc::boxed::Box;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::font;
use crate::rect::Rect;
use crate::widget::{Event, EventResult, Widget};

/// A movable window with a title bar, close button, and a content widget.
pub struct Window {
    rect: Rect,
    title: &'static str,
    content: Option<Box<dyn Widget>>,
    close_hovered: bool,
    dragging: bool,
    drag_off_x: i32,
    drag_off_y: i32,
    shadow_radius: i32,
    bg: Color,
    title_bg: Color,
    title_fg: Color,
    close_bg: Color,
    close_hover: Color,
    border: Color,
}

impl Window {
    pub fn new(title: &'static str, content: Option<Box<dyn Widget>>) -> Self {
        Self {
            rect: Rect::new(40, 56, 640, 400),
            title,
            content,
            close_hovered: false,
            dragging: false,
            drag_off_x: 0,
            drag_off_y: 0,
            shadow_radius: 6,
            bg: Color::from_u32(0xFF161630),
            title_bg: Color::from_u32(0xFF1A1A3A),
            title_fg: Color::from_u32(0xFFAAAAEE),
            close_bg: Color::from_u32(0xFF992222),
            close_hover: Color::from_u32(0xFFFF5555),
            border: Color::from_u32(0xFF444477),
        }
    }

    fn scale(&self) -> i32 {
        let min_side = self.rect.w.min(self.rect.h).max(1);
        (min_side / 200).max(1).min(4) as i32
    }

    fn title_h(&self) -> i32 {
        let scale = self.scale();
        font::line_height() as i32 + 6 + scale * 2
    }

    fn close_size(&self) -> i32 {
        let sz = self.title_h() - 6;
        sz.max(14)
    }

    fn padding(&self) -> i32 {
        4 + self.scale() * 2
    }

    fn corner_radius(&self) -> u32 {
        let min_side = self.rect.w.min(self.rect.h);
        let base = (self.title_h() / 2).max(6) as u32;
        base.min(min_side / 5).max(6)
    }

    fn title_bar_rect(&self) -> Rect {
        let h = self.title_h();
        Rect::new(self.rect.x, self.rect.y - h, self.rect.w, h as u32)
    }

    fn close_rect(&self) -> Rect {
        let sz = self.close_size();
        let x = self.rect.x + self.rect.w as i32 - (sz + 6);
        let y = self.rect.y - self.title_h() + (self.title_h() - sz) / 2;
        Rect::new(x, y, sz as u32, sz as u32)
    }

    fn content_rect(&self) -> Rect {
        let pad = self.padding();
        Rect::new(
            self.rect.x + pad,
            self.rect.y + pad,
            self.rect.w.saturating_sub((pad as u32) * 2),
            self.rect.h.saturating_sub((pad as u32) * 2),
        )
    }
}

impl Widget for Window {
    fn rect(&self) -> Rect {
        self.rect
    }

    fn min_size(&self) -> (u32, u32) {
        (320, 200)
    }

    fn set_pos(&mut self, x: i32, y: i32) {
        self.rect.x = x;
        self.rect.y = y;
    }

    fn set_size(&mut self, w: u32, h: u32) {
        self.rect.w = w;
        self.rect.h = h;
    }

    fn draw(&self, canvas: &mut Canvas) {
        let title_h = self.title_h();
        let outer_x = self.rect.x;
        let outer_y = self.rect.y - title_h;
        let outer_w = self.rect.w;
        let outer_h = self.rect.h + title_h as u32;
        let radius = self.corner_radius();

        // Shadow
        let shadow = self.shadow_radius + self.scale();
        canvas.draw_shadow(
            outer_x - 4,
            outer_y - 4,
            outer_w + 8,
            outer_h + 8,
            shadow,
        );

        // Frame background
        canvas.fill_round_rect(outer_x, outer_y, outer_w, outer_h, radius, self.bg);

        // Title bar
        let tb = self.title_bar_rect();
        canvas.fill_round_rect(tb.x, tb.y, tb.w, tb.h, radius, self.title_bg);
        canvas.hline(tb.x, tb.y + tb.h as i32 - 1, tb.w, self.border);

        // Title text
        let title_y = tb.y + (tb.h as i32 - font::line_height() as i32) / 2;
        canvas.draw_text(tb.x + 10, title_y, self.title, self.title_fg);

        // Close button
        let cl = self.close_rect();
        let clr = if self.close_hovered {
            self.close_hover
        } else {
            self.close_bg
        };
        let close_radius = (cl.w / 3).max(3);
        canvas.fill_round_rect(cl.x, cl.y, cl.w, cl.h, close_radius, clr);
        let cx = cl.x + (cl.w as i32 - font::FONT_W as i32) / 2;
        let cy = cl.y + (cl.h as i32 - font::FONT_H as i32) / 2;
        canvas.draw_char(cx, cy, 'x', Color::WHITE);

        // Border
        canvas.stroke_round_rect(outer_x, outer_y, outer_w, outer_h, radius, self.border);

        // Content child
        if let Some(ref child) = self.content {
            child.draw(canvas);
        }
    }

    fn handle_event(&mut self, ev: &Event, _parent: Rect) -> EventResult {
        let tb = self.title_bar_rect();
        let cl = self.close_rect();
        let cr = self.content_rect();
        match *ev {
            Event::MouseMove { x, y } => {
                // Close button hover
                let was_close = self.close_hovered;
                self.close_hovered = cl.contains(x, y);

                // Dragging
                if self.dragging {
                    let new_x = x - self.drag_off_x;
                    let new_y = y - self.drag_off_y;
                    // clamp minimally
                    let new_x = new_x.max(2).min(1000); // TODO: screen-relative clamping
                    let new_y = new_y.max(36).min(800);
                    if new_x != self.rect.x || new_y != self.rect.y {
                        self.rect.x = new_x;
                        self.rect.y = new_y;
                        // Forward to child too so its absolute coords follow
                        if let Some(ref mut child) = self.content {
                            child.set_pos(cr.x, cr.y);
                        }
                        return EventResult::Redraw;
                    }
                }

                // Forward mouse move to content
                if cr.contains(x, y) {
                    if let Some(ref mut child) = self.content {
                        return child.handle_event(ev, cr);
                    }
                }

                if was_close != self.close_hovered {
                    return EventResult::Redraw;
                }
                EventResult::Ignored
            }

            Event::MouseDown { x, y, .. } => {
                // Close button click
                if cl.contains(x, y) {
                    // "close" — we just redraw (caller must handle close logic)
                    return EventResult::Handled;
                }
                // Title bar → start drag
                if tb.contains(x, y) {
                    self.dragging = true;
                    self.drag_off_x = x - self.rect.x;
                    self.drag_off_y = y - self.rect.y;
                    return EventResult::Redraw;
                }
                // Forward to content
                if cr.contains(x, y) {
                    if let Some(ref mut child) = self.content {
                        return child.handle_event(ev, cr);
                    }
                }
                EventResult::Ignored
            }

            Event::MouseUp { x, y, .. } => {
                if self.dragging {
                    self.dragging = false;
                    return EventResult::Redraw;
                }
                if cr.contains(x, y) {
                    if let Some(ref mut child) = self.content {
                        return child.handle_event(ev, cr);
                    }
                }
                EventResult::Ignored
            }
        }
    }
}
