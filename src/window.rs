use alloc::boxed::Box;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::rect::Rect;
use crate::widget::{Event, EventResult, Widget};

const TITLE_H: i32 = 24;
const CLOSE_W: i32 = 18;

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

    fn title_bar_rect(&self) -> Rect {
        Rect::new(self.rect.x, self.rect.y - TITLE_H, self.rect.w, TITLE_H as u32)
    }

    fn close_rect(&self) -> Rect {
        let x = self.rect.x + self.rect.w as i32 - 24;
        let y = self.rect.y - TITLE_H + 2;
        Rect::new(x, y, CLOSE_W as u32, CLOSE_W as u32)
    }

    fn content_rect(&self) -> Rect {
        Rect::new(
            self.rect.x + 4,
            self.rect.y + 4,
            self.rect.w.saturating_sub(8),
            self.rect.h.saturating_sub(8),
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
        // Shadow
        canvas.draw_shadow(
            self.rect.x - 4,
            self.rect.y - TITLE_H - 4,
            self.rect.w + 8,
            self.rect.h + TITLE_H as u32 + 8,
            self.shadow_radius,
        );

        // Title bar
        let tb = self.title_bar_rect();
        canvas.fill_rect(tb.x, tb.y, tb.w, tb.h, self.title_bg);
        canvas.hline(tb.x, tb.y + tb.h as i32 - 1, tb.w, self.border);

        // Title text
        canvas.draw_text(tb.x + 10, tb.y + 6, self.title, self.title_fg);

        // Close button
        let cl = self.close_rect();
        let clr = if self.close_hovered {
            self.close_hover
        } else {
            self.close_bg
        };
        canvas.fill_rect(cl.x, cl.y, cl.w, cl.h, clr);
        canvas.draw_char(cl.x + 5, cl.y + 3, 'x', Color::WHITE);

        // Content area
        canvas.fill_rect(self.rect.x, self.rect.y, self.rect.w, self.rect.h, self.bg);

        // Border
        canvas.hline(self.rect.x, self.rect.y, self.rect.w, self.border);
        canvas.hline(
            self.rect.x,
            self.rect.y + self.rect.h as i32 - 1,
            self.rect.w,
            self.border,
        );
        canvas.vline(self.rect.x, self.rect.y, self.rect.h, self.border);
        canvas.vline(
            self.rect.x + self.rect.w as i32 - 1,
            self.rect.y,
            self.rect.h,
            self.border,
        );

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
