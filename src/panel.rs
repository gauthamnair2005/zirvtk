use alloc::boxed::Box;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::font;
use crate::rect::Rect;
use crate::widget::{Event, EventResult, Widget};

/// A container widget that draws a rounded panel around a child widget.
pub struct Panel {
    rect: Rect,
    child: Option<Box<dyn Widget>>,
    bg: Color,
    border: Color,
    padding: u32,
}

impl Panel {
    pub fn new(child: Option<Box<dyn Widget>>) -> Self {
        Self {
            rect: Rect::new(0, 0, 240, 120),
            child,
            bg: Color::from_u32(0xFF1D1D35),
            border: Color::from_u32(0xFF3F3F6A),
            padding: 10,
        }
    }

    pub fn set_colors(&mut self, bg: Color, border: Color) {
        self.bg = bg;
        self.border = border;
    }

    pub fn set_padding(&mut self, padding: u32) {
        self.padding = padding;
        self.update_child_layout();
    }

    fn scale(&self) -> u32 {
        let min_side = self.rect.w.min(self.rect.h).max(1);
        (min_side / 160).max(1).min(4)
    }

    fn padding_px(&self) -> i32 {
        let scale = self.scale() as i32;
        self.padding as i32 + (font::FONT_H as i32 / 2) + (scale * 2)
    }

    fn radius_px(&self) -> u32 {
        let scale = self.scale();
        let min_side = self.rect.w.min(self.rect.h);
        let base = (font::FONT_H / 2) + (scale * 3);
        base.min(min_side / 4).max(4)
    }

    fn content_rect(&self) -> Rect {
        let pad = self.padding_px();
        let w = self.rect.w.saturating_sub((pad as u32) * 2);
        let h = self.rect.h.saturating_sub((pad as u32) * 2);
        Rect::new(self.rect.x + pad, self.rect.y + pad, w, h)
    }

    fn update_child_layout(&mut self) {
        if let Some(ref mut child) = self.child {
            let cr = self.content_rect();
            child.set_pos(cr.x, cr.y);
            child.set_size(cr.w, cr.h);
        }
    }
}

impl Widget for Panel {
    fn rect(&self) -> Rect {
        self.rect
    }

    fn min_size(&self) -> (u32, u32) {
        let (cw, ch) = if let Some(ref child) = self.child {
            child.min_size()
        } else {
            (font::text_width("Panel"), font::line_height())
        };
        let pad = self.padding + (font::FONT_H / 2) + 6;
        (cw + pad * 2, ch + pad * 2)
    }

    fn set_pos(&mut self, x: i32, y: i32) {
        self.rect.x = x;
        self.rect.y = y;
        self.update_child_layout();
    }

    fn set_size(&mut self, w: u32, h: u32) {
        self.rect.w = w;
        self.rect.h = h;
        self.update_child_layout();
    }

    fn draw(&self, canvas: &mut Canvas) {
        let r = self.rect;
        let radius = self.radius_px();
        canvas.fill_round_rect(r.x, r.y, r.w, r.h, radius, self.bg);
        canvas.stroke_round_rect(r.x, r.y, r.w, r.h, radius, self.border);

        if let Some(ref child) = self.child {
            child.draw(canvas);
        }
    }

    fn handle_event(&mut self, ev: &Event, _parent: Rect) -> EventResult {
        if let Some(ref mut child) = self.child {
            let cr = self.content_rect();
            let (x, y) = match *ev {
                Event::MouseMove { x, y } => (x, y),
                Event::MouseDown { x, y, .. } => (x, y),
                Event::MouseUp { x, y, .. } => (x, y),
            };
            if cr.contains(x, y) {
                return child.handle_event(ev, cr);
            }
        }
        EventResult::Ignored
    }
}
