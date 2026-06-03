use crate::canvas::Canvas;
use crate::color::Color;
use crate::fx;
use crate::rect::Rect;
use crate::widget::{Event, EventResult, Widget};

pub struct GlowSlider {
    rect: Rect,
    pub value: f32,
    pub min_val: f32,
    pub max_val: f32,
    active_color: Color,
    track_color: Color,
    knob_color: Color,
    hovered: bool,
    dragging: bool,
}

impl GlowSlider {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(0, 0, 200, 20),
            value: 0.5,
            min_val: 0.0,
            max_val: 1.0,
            active_color: Color::from_rgb(0, 212, 255),
            track_color: Color::from_argb(60, 100, 100, 140),
            knob_color: Color::from_rgb(240, 240, 255),
            hovered: false,
            dragging: false,
        }
    }

    pub fn set_value(&mut self, val: f32) {
        self.value = val.clamp(self.min_val, self.max_val);
    }

    pub fn set_active_color(&mut self, color: Color) {
        self.active_color = color;
    }

    fn value_from_x(&self, x: i32) -> f32 {
        let track_w = self.rect.w.saturating_sub(self.rect.h) as i32;
        if track_w <= 0 { return self.min_val; }
        let rel = (x - self.rect.x - self.rect.h as i32 / 2).clamp(0, track_w) as f32 / track_w as f32;
        self.min_val + rel * (self.max_val - self.min_val)
    }
}

impl Widget for GlowSlider {
    fn draw(&self, canvas: &mut Canvas) {
        let h = self.rect.h as i32;
        let radius = h / 2;
        let track_w = self.rect.w as i32 - radius;
        let knob_pos = ((self.value - self.min_val) / (self.max_val - self.min_val).max(0.001) * track_w as f32) as i32;

        canvas.fill_round_rect(self.rect.x, self.rect.y + h / 4, self.rect.w, h as u32 / 2, 4, self.track_color);

        if knob_pos > 0 {
            let active_w = (knob_pos + radius / 2) as u32;
            canvas.fill_round_rect(self.rect.x, self.rect.y + h / 4, active_w, h as u32 / 2, 4, self.active_color);
        }

        if self.hovered || self.dragging {
            fx::glow(canvas, self.rect.x + knob_pos + radius / 2, self.rect.y + h / 2, 10, self.active_color, 50);
        }

        canvas.fill_circle(self.rect.x + knob_pos + radius / 2, self.rect.y + h / 2, (radius - 2) as u32, self.knob_color);

        if self.hovered || self.dragging {
            canvas.stroke_round_rect(self.rect.x, self.rect.y, self.rect.w, self.rect.h as u32, radius as u32,
                Color::from_argb(40, 255, 255, 255));
        }
    }

    fn rect(&self) -> Rect { self.rect }
    fn min_size(&self) -> (u32, u32) { (100, 20) }

    fn set_pos(&mut self, x: i32, y: i32) { self.rect.x = x; self.rect.y = y; }
    fn set_size(&mut self, w: u32, h: u32) { self.rect.w = w; self.rect.h = h; }

    fn handle_event(&mut self, ev: &Event, parent: Rect) -> EventResult {
        let abs = Rect::new(parent.x + self.rect.x, parent.y + self.rect.y, self.rect.w, self.rect.h);
        match *ev {
            Event::MouseMove { x, y } => {
                let new_hover = abs.contains(x, y);
                if self.dragging {
                    self.value = self.value_from_x(x - parent.x).clamp(self.min_val, self.max_val);
                    EventResult::Redraw
                } else {
                    let changed = new_hover != self.hovered;
                    self.hovered = new_hover;
                    if changed { EventResult::Redraw } else { EventResult::Ignored }
                }
            }
            Event::MouseDown { x, y, .. } => {
                if abs.contains(x, y) {
                    self.dragging = true;
                    self.value = self.value_from_x(x - parent.x).clamp(self.min_val, self.max_val);
                    EventResult::Redraw
                } else { EventResult::Ignored }
            }
            Event::MouseUp { .. } => {
                if self.dragging {
                    self.dragging = false;
                    EventResult::Redraw
                } else { EventResult::Ignored }
            }
        }
    }
}
