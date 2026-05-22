use crate::canvas::Canvas;
use crate::color::Color;
use crate::font;
use crate::rect::Rect;
use crate::widget::{Event, EventResult, Widget};

/// A clickable button widget.
pub struct Button {
    rect: Rect,
    text: &'static str,
    hovered: bool,
    pressed: bool,
    // Colour scheme
    bg: Color,
    bg_hover: Color,
    bg_press: Color,
    fg: Color,
    border: Color,
}

impl Button {
    pub fn new(text: &'static str) -> Self {
        Self {
            rect: Rect::new(0, 0, 96, 32),
            text,
            hovered: false,
            pressed: false,
            bg: Color::from_u32(0xFF2A2A4E),
            bg_hover: Color::from_u32(0xFF3A3A6E),
            bg_press: Color::from_u32(0xFF1A1A3E),
            fg: Color::from_u32(0xFFCCCCDD),
            border: Color::from_u32(0xFF444477),
        }
    }

    pub fn set_colors(&mut self, bg: Color, hover: Color, press: Color, fg: Color, border: Color) {
        self.bg = bg;
        self.bg_hover = hover;
        self.bg_press = press;
        self.fg = fg;
        self.border = border;
    }
}

impl Widget for Button {
    fn rect(&self) -> Rect {
        self.rect
    }

    fn min_size(&self) -> (u32, u32) {
        let text_w = font::text_width(self.text);
        let pad_x = font::FONT_W + 6;
        let pad_y = font::FONT_H / 2 + 4;
        let w = text_w.max(64) + pad_x * 2;
        let h = font::line_height() + pad_y * 2;
        (w, h.max(28))
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
        let bg = if self.pressed {
            self.bg_press
        } else if self.hovered {
            self.bg_hover
        } else {
            self.bg
        };
        let r = self.rect;
        let min_side = r.w.min(r.h).max(1);
        let scale = (min_side / 120).max(1).min(3) as i32;
        let radius = (font::FONT_H as i32 / 2 + 2 * scale) as u32;
        canvas.fill_round_rect(r.x, r.y, r.w, r.h, radius, bg);
        canvas.stroke_round_rect(r.x, r.y, r.w, r.h, radius, self.border);

        // Centered text
        let text_w = font::text_width(self.text) as i32;
        let text_h = font::line_height() as i32;
        let tx = r.x + (r.w as i32 - text_w) / 2;
        let ty = r.y + (r.h as i32 - text_h) / 2;
        canvas.draw_text(tx, ty, self.text, self.fg);
    }

    fn handle_event(&mut self, ev: &Event, _parent: Rect) -> EventResult {
        let r = self.rect;
        match *ev {
            Event::MouseMove { x, y } => {
                let was = self.hovered;
                self.hovered = r.contains(x, y);
                if was != self.hovered {
                    EventResult::Redraw
                } else {
                    EventResult::Ignored
                }
            }
            Event::MouseDown { x, y, .. } => {
                if r.contains(x, y) {
                    self.pressed = true;
                    EventResult::Redraw
                } else {
                    EventResult::Ignored
                }
            }
            Event::MouseUp { x, y, .. } => {
                let was = self.pressed;
                self.pressed = false;
                if was && r.contains(x, y) {
                    // "Clicked"
                    EventResult::Handled
                } else if was {
                    EventResult::Redraw
                } else {
                    EventResult::Ignored
                }
            }
        }
    }
}
