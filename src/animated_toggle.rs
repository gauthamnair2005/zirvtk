use crate::canvas::Canvas;
use crate::color::Color;
use crate::fx;
use crate::rect::Rect;
use crate::widget::{Event, EventResult, Widget};

pub struct AnimatedToggle {
    rect: Rect,
    pub active: bool,
    anim_progress: f32,
    hovered: bool,
    active_color: Color,
    inactive_color: Color,
    knob_color: Color,
}

impl AnimatedToggle {
    pub fn new(active: bool) -> Self {
        Self {
            rect: Rect::new(0, 0, 48, 24),
            active,
            anim_progress: if active { 1.0 } else { 0.0 },
            hovered: false,
            active_color: Color::from_rgb(0, 212, 255),
            inactive_color: Color::from_argb(60, 100, 100, 140),
            knob_color: Color::from_rgb(240, 240, 255),
        }
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn set_active_color(&mut self, color: Color) {
        self.active_color = color;
    }
}

impl Widget for AnimatedToggle {
    fn draw(&self, canvas: &mut Canvas) {
        let w = self.rect.w as i32;
        let h = self.rect.h as i32;
        let radius = h / 2;
        let track_w = w - radius;
        let knob_pos = (self.anim_progress * track_w as f32) as i32;

        let track_color = Color::from_argb(
            (self.inactive_color.a() as u32 * (255 - (self.anim_progress * 255.0) as u32) / 255 +
             self.active_color.a() as u32 * (self.anim_progress * 255.0) as u32 / 255) as u8,
            (self.inactive_color.r() as u32 * (255 - (self.anim_progress * 255.0) as u32) / 255 +
             self.active_color.r() as u32 * (self.anim_progress * 255.0) as u32 / 255) as u8,
            (self.inactive_color.g() as u32 * (255 - (self.anim_progress * 255.0) as u32) / 255 +
             self.active_color.g() as u32 * (self.anim_progress * 255.0) as u32 / 255) as u8,
            (self.inactive_color.b() as u32 * (255 - (self.anim_progress * 255.0) as u32) / 255 +
             self.active_color.b() as u32 * (self.anim_progress * 255.0) as u32 / 255) as u8,
        );

        canvas.fill_round_rect(self.rect.x, self.rect.y, self.rect.w as u32, self.rect.h as u32,
            radius as u32, track_color);

        if self.active && self.anim_progress > 0.5 {
            let glow_a = ((self.anim_progress - 0.5) * 2.0 * 60.0) as u8;
            fx::glow(canvas, self.rect.x + knob_pos + radius, self.rect.y + radius, 8, self.active_color, glow_a);
        }

        canvas.fill_circle(self.rect.x + knob_pos + radius / 2, self.rect.y + radius / 2,
            (radius - 2) as u32, self.knob_color);

        if self.hovered {
            canvas.stroke_round_rect(self.rect.x, self.rect.y, self.rect.w as u32, self.rect.h as u32,
                radius as u32, Color::from_argb(60, 255, 255, 255));
        }
    }

    fn rect(&self) -> Rect { self.rect }
    fn min_size(&self) -> (u32, u32) { (48, 24) }

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
            Event::MouseUp { x, y, .. } => {
                if abs.contains(x, y) {
                    self.toggle();
                    EventResult::Handled
                } else { EventResult::Ignored }
            }
            _ => EventResult::Ignored,
        }
    }

}

impl AnimatedToggle {
    pub fn update_animation(&mut self, dt_ms: u32) {
        let target = if self.active { 1.0 } else { 0.0 };
        let speed = 0.005 * dt_ms as f32;
        if (self.anim_progress - target).abs() > 0.001 {
            self.anim_progress += (target - self.anim_progress).clamp(-speed, speed);
        } else {
            self.anim_progress = target;
        }
    }
}
