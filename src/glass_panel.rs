use alloc::boxed::Box;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::fx;
use crate::rect::Rect;
use crate::widget::{Event, EventResult, Widget};
use crate::font;

pub struct GlassPanel {
    rect: Rect,
    child: Option<Box<dyn Widget>>,
    tint: Color,
    opacity: u8,
    radius: u32,
    glow_color: Color,
    glow_intensity: u8,
    title: Option<&'static str>,
    title_color: Color,
    border_color: Color,
}

impl GlassPanel {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(0, 0, 400, 300),
            child: None,
            tint: Color::from_argb(0, 18, 18, 31),
            opacity: 200,
            radius: 12,
            glow_color: Color::from_rgb(0, 212, 255),
            glow_intensity: 30,
            title: None,
            title_color: Color::from_rgb(180, 180, 220),
            border_color: Color::from_argb(40, 0, 212, 255),
        }
    }

    pub fn set_child(&mut self, child: Box<dyn Widget>) {
        self.child = Some(child);
        self.update_child_layout();
    }

    pub fn set_title(&mut self, title: &'static str) {
        self.title = Some(title);
    }

    pub fn set_colors(&mut self, tint: Color, opacity: u8, glow: Color, glow_intensity: u8) {
        self.tint = tint;
        self.opacity = opacity;
        self.glow_color = glow;
        self.glow_intensity = glow_intensity;
    }

    pub fn set_radius(&mut self, radius: u32) {
        self.radius = radius;
    }

    fn content_rect(&self) -> Rect {
        let title_h = if self.title.is_some() { (font::FONT_H + 8) as i32 } else { 6 };
        Rect::new(
            self.rect.x + 8,
            self.rect.y + title_h,
            self.rect.w.saturating_sub(16),
            self.rect.h.saturating_sub(title_h as u32 + 8),
        )
    }

    fn update_child_layout(&mut self) {
        let cr = self.content_rect();
        if let Some(ref mut child) = self.child {
            child.set_pos(cr.x, cr.y);
            child.set_size(cr.w, cr.h);
        }
    }
}

impl Widget for GlassPanel {
    fn draw(&self, canvas: &mut Canvas) {
        fx::glass_panel(canvas,
            self.rect.x, self.rect.y, self.rect.w, self.rect.h,
            self.radius, self.tint, self.opacity,
            self.glow_color, self.glow_intensity);

        if let Some(title) = self.title {
            let tx = self.rect.x + 12;
            let ty = self.rect.y + 6;
            canvas.draw_text(tx, ty, title, self.title_color);
        }

        if let Some(ref child) = self.child {
            child.draw(canvas);
        }
    }

    fn rect(&self) -> Rect { self.rect }

    fn min_size(&self) -> (u32, u32) { (200, 120) }

    fn set_pos(&mut self, x: i32, y: i32) {
        let dx = x - self.rect.x;
        let dy = y - self.rect.y;
        self.rect.x = x;
        self.rect.y = y;
        if let Some(ref mut child) = self.child {
            let cr = child.rect();
            child.set_pos(cr.x + dx, cr.y + dy);
        }
    }

    fn set_size(&mut self, w: u32, h: u32) {
        self.rect.w = w;
        self.rect.h = h;
        self.update_child_layout();
    }

    fn handle_event(&mut self, ev: &Event, parent: Rect) -> EventResult {
        let abs = Rect::new(
            parent.x + self.rect.x,
            parent.y + self.rect.y,
            self.rect.w,
            self.rect.h,
        );
        if let Some(ref mut child) = self.child {
            child.handle_event(ev, abs)
        } else {
            EventResult::Ignored
        }
    }
}
