use crate::canvas::Canvas;
use crate::color::Color;
use crate::fx;
use crate::rect::Rect;
use crate::widget::{Event, EventResult, Widget};
use crate::font;

pub struct NeonButton {
    rect: Rect,
    text: &'static str,
    neon_color: Color,
    hovered: bool,
    pressed: bool,
    bg_color: Color,
    text_color: Color,
    corner_radius: u32,
    glow_intensity: u8,
}

impl NeonButton {
    pub fn new(text: &'static str) -> Self {
        Self {
            rect: Rect::new(0, 0, 140, 40),
            text,
            neon_color: Color::from_rgb(0, 212, 255),
            hovered: false,
            pressed: false,
            bg_color: Color::from_argb(30, 0, 212, 255),
            text_color: Color::from_rgb(240, 240, 255),
            corner_radius: 6,
            glow_intensity: 40,
        }
    }

    pub fn set_neon_color(&mut self, color: Color) {
        self.neon_color = color;
        self.bg_color = Color::from_argb(30, color.r(), color.g(), color.b());
    }

    pub fn set_text(&mut self, text: &'static str) {
        self.text = text;
    }

    pub fn is_hovered(&self) -> bool { self.hovered }
}

impl Widget for NeonButton {
    fn draw(&self, canvas: &mut Canvas) {
        let glow = if self.hovered { self.glow_intensity.saturating_mul(2) } else { self.glow_intensity };
        let bg = if self.pressed {
            self.neon_color.blend(self.bg_color, 100)
        } else if self.hovered {
            self.neon_color.blend(self.bg_color, 60)
        } else {
            self.bg_color
        };

        if self.hovered {
            fx::glow_rect(canvas, self.rect.x, self.rect.y, self.rect.w, self.rect.h,
                self.corner_radius, self.neon_color, glow);
        }

        canvas.fill_round_rect(self.rect.x, self.rect.y, self.rect.w, self.rect.h,
            self.corner_radius, bg);

        canvas.stroke_round_rect(self.rect.x, self.rect.y, self.rect.w, self.rect.h,
            self.corner_radius, Color::from_argb(120, self.neon_color.r(), self.neon_color.g(), self.neon_color.b()));

        let tw = (self.text.len() as u32) * (font::FONT_W + 1);
        let tx = self.rect.x + (self.rect.w as i32 - tw as i32) / 2;
        let ty = self.rect.y + (self.rect.h as i32 - font::FONT_H as i32) / 2;
        canvas.draw_text(tx, ty, self.text, self.text_color);
    }

    fn rect(&self) -> Rect { self.rect }
    fn min_size(&self) -> (u32, u32) {
        let tw = (self.text.len() as u32) * (font::FONT_W + 1) + 20;
        (tw.max(80), font::FONT_H + 20)
    }

    fn set_pos(&mut self, x: i32, y: i32) { self.rect.x = x; self.rect.y = y; }
    fn set_size(&mut self, w: u32, h: u32) { self.rect.w = w; self.rect.h = h; }

    fn handle_event(&mut self, ev: &Event, parent: Rect) -> EventResult {
        let abs = Rect::new(parent.x + self.rect.x, parent.y + self.rect.y, self.rect.w, self.rect.h);
        match *ev {
            Event::MouseMove { x, y } => {
                let new_hover = abs.contains(x, y);
                let changed = new_hover != self.hovered;
                self.hovered = new_hover;
                if changed { EventResult::Redraw } else { EventResult::Ignored }
            }
            Event::MouseDown { x, y, .. } => {
                if abs.contains(x, y) {
                    self.pressed = true;
                    EventResult::Redraw
                } else { EventResult::Ignored }
            }
            Event::MouseUp { x, y, .. } => {
                let was_pressed = self.pressed;
                self.pressed = false;
                if was_pressed && abs.contains(x, y) {
                    EventResult::Handled
                } else {
                    EventResult::Redraw
                }
            }
        }
    }
}
