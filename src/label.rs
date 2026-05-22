use crate::canvas::Canvas;
use crate::color::Color;
use crate::rect::Rect;
use crate::widget::{Event, EventResult, Widget};

/// A static text label widget.
pub struct Label {
    rect: Rect,
    text: &'static str,
    color: Color,
    bg: Option<Color>,
}

impl Label {
    pub fn new(text: &'static str) -> Self {
        Self {
            rect: Rect::new(0, 0, 64, 16),
            text,
            color: Color::from_u32(0xFFCCCCCC),
            bg: None,
        }
    }

    pub fn with_bg(mut self, bg: Color) -> Self {
        self.bg = Some(bg);
        self
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl Widget for Label {
    fn rect(&self) -> Rect {
        self.rect
    }

    fn min_size(&self) -> (u32, u32) {
        (self.text.len() as u32 * 9, 16)
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
        if let Some(bg) = self.bg {
            canvas.fill_rect(self.rect.x, self.rect.y, self.rect.w, self.rect.h, bg);
        }
        canvas.draw_text(self.rect.x + 4, self.rect.y + 2, self.text, self.color);
    }

    fn handle_event(&mut self, _ev: &Event, _parent: Rect) -> EventResult {
        EventResult::Ignored
    }
}
