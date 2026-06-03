use alloc::string::String;
use core::fmt::Write;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::fx;
use crate::rect::Rect;
use crate::widget::{Event, EventResult, Widget};
use crate::font;

struct BufWrite<'a>(&'a mut [u8], usize);

impl<'a> Write for BufWrite<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            if self.1 < self.0.len() {
                self.0[self.1] = b;
                self.1 += 1;
            }
        }
        Ok(())
    }
}

pub struct FuturisticClock {
    rect: Rect,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    text_color: Color,
    glow_color: Color,
    date_str: String,
    prev_second: u32,
    pulse_timer: u32,
}

impl FuturisticClock {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(0, 0, 300, 100),
            hours: 0, minutes: 0, seconds: 0,
            text_color: Color::from_rgb(240, 240, 255),
            glow_color: Color::from_rgb(0, 212, 255),
            date_str: String::new(),
            prev_second: 0,
            pulse_timer: 0,
        }
    }

    pub fn set_time(&mut self, h: u32, m: u32, s: u32) {
        self.hours = h;
        self.minutes = m;
        self.seconds = s;
    }

    pub fn set_date(&mut self, date: &str) {
        self.date_str.clear();
        self.date_str.push_str(date);
    }

    pub fn update(&mut self, dt_ms: u32) {
        self.pulse_timer += dt_ms;
        if self.seconds != self.prev_second {
            self.prev_second = self.seconds;
            self.pulse_timer = 0;
        }
    }

    fn draw_time(canvas: &mut Canvas, x: i32, y: i32, h: u32, m: u32, s: u32, color: Color, scale: u32) {
        let mut buf = [0u8; 16];
        let mut w = BufWrite(&mut buf, 0);
        let _ = write!(w, "{:02}:{:02}:{:02}", h, m, s);
        let len = w.1;
        let step = (font::FONT_W + 1) as i32 * scale as i32;
        let tw = (len as u32) * step as u32;
        let mut cx = x - (tw as i32) / 2;
        for i in 0..len {
            canvas.draw_char_scaled(cx, y, buf[i] as char, color, scale);
            cx += step;
        }
    }
}

impl Widget for FuturisticClock {
    fn draw(&self, canvas: &mut Canvas) {
        let cx = self.rect.x + self.rect.w as i32 / 2;
        let cy = self.rect.y + self.rect.h as i32 / 2 - 8;

        fx::glow(canvas, cx, cy, 12, self.glow_color, 10);

        let ty = cy - font::FONT_H as i32;
        Self::draw_time(canvas, cx, ty, self.hours, self.minutes, self.seconds, self.text_color, 2);

        if !self.date_str.is_empty() {
            let dw = (self.date_str.len() as u32) * (font::FONT_W + 1);
            let dx = cx - (dw as i32) / 2;
            let dy = cy + 8;
            canvas.draw_text(dx, dy, &self.date_str, Color::from_rgb(136, 136, 187));
        }

        let colon_x = cx - 8;
        let colon_y = ty + font::FONT_H as i32 - 4;
        canvas.fill_circle(colon_x, colon_y, 2, Color::from_argb(160, 0, 212, 255));
        canvas.fill_circle(colon_x + 24, colon_y, 2, Color::from_argb(160, 0, 212, 255));
    }

    fn rect(&self) -> Rect { self.rect }
    fn min_size(&self) -> (u32, u32) { (200, 60) }

    fn set_pos(&mut self, x: i32, y: i32) { self.rect.x = x; self.rect.y = y; }
    fn set_size(&mut self, w: u32, h: u32) { self.rect.w = w; self.rect.h = h; }

    fn handle_event(&mut self, _ev: &Event, _parent: Rect) -> EventResult {
        EventResult::Ignored
    }
}
