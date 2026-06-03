use crate::canvas::Canvas;
use crate::color::Color;
use crate::fx;
use crate::font;
use crate::rect::Rect;
use crate::widget::{Event, EventResult, Widget};
use core::fmt::Write;

struct BufW<'a>(&'a mut [u8], usize);

impl<'a> core::fmt::Write for BufW<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            if self.1 < self.0.len() { self.0[self.1] = b; self.1 += 1; }
        }
        Ok(())
    }
}

pub struct TaskBar {
    rect: Rect,
    pub show_launcher: bool,
    pub time_h: u32, pub time_m: u32, pub time_s: u32,
    launcher_hover: bool,
    app_count: u32,
    pub on_launcher_click: Option<fn()>,
}

impl TaskBar {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(0, 0, 800, 40),
            show_launcher: false,
            time_h: 0, time_m: 0, time_s: 0,
            launcher_hover: false,
            app_count: 0,
            on_launcher_click: None,
        }
    }

    pub fn set_time(&mut self, h: u32, m: u32) {
        self.time_h = h;
        self.time_m = m;
    }

    fn launcher_rect(&self) -> Rect {
        Rect::new(self.rect.x + 6, self.rect.y + 4, 32, self.rect.h - 8)
    }

    fn clock_rect(&self) -> Rect {
        let clock_w = 80u32;
        Rect::new(self.rect.x + self.rect.w as i32 - clock_w as i32 - 8, self.rect.y, clock_w, self.rect.h)
    }
}

impl Widget for TaskBar {
    fn draw(&self, canvas: &mut Canvas) {
        let glow_intensity = if self.launcher_hover { 50u8 } else { 15u8 };
        fx::glass_panel(canvas,
            self.rect.x, self.rect.y, self.rect.w, self.rect.h,
            0, Color::from_argb(200, 10, 10, 20), 200,
            Color::from_rgb(0, 212, 255), glow_intensity);

        canvas.hline(self.rect.x, self.rect.y, self.rect.w, Color::from_argb(50, 0, 212, 255));

        let lr = self.launcher_rect();
        let is_hover = self.launcher_hover;

        canvas.fill_round_rect(lr.x, lr.y, lr.w, lr.h, 6,
            if is_hover { Color::from_argb(60, 0, 212, 255) } else { Color::from_argb(20, 100, 100, 140) });

        let dot_cy = lr.y + lr.h as i32 / 2;
        for i in 0..3 {
            let dot_cx = lr.x + (lr.w as i32 / 4) * (i + 1);
            canvas.fill_circle(dot_cx, dot_cy, 2, Color::from_rgb(0, 212, 255));
        }

        if self.show_launcher {
            canvas.fill_round_rect(lr.x, lr.y + lr.h as i32 - 2, lr.w, 2, 1, Color::from_rgb(0, 212, 255));
        }

        let mut buf = [0u8; 16];
        let mut w = BufW(&mut buf, 0);
        let _ = write!(w, "{:02}:{:02}", self.time_h, self.time_m);
        let len = w.1;
        let step = (font::FONT_W + 1) as i32;
        let tw = len as i32 * step;
        let cr = self.clock_rect();
        let cx = cr.x + (cr.w as i32 - tw) / 2;
        let cy = cr.y + (cr.h as i32 - font::FONT_H as i32) / 2;
        for i in 0..len {
            canvas.draw_char(cx + i as i32 * step, cy, buf[i] as char, Color::from_rgb(200, 200, 230));
        }
    }

    fn rect(&self) -> Rect { self.rect }
    fn min_size(&self) -> (u32, u32) { (100, 40) }

    fn set_pos(&mut self, x: i32, y: i32) { self.rect.x = x; self.rect.y = y; }
    fn set_size(&mut self, w: u32, h: u32) { self.rect.w = w; self.rect.h = h; }

    fn handle_event(&mut self, ev: &Event, parent: Rect) -> EventResult {
        let abs = Rect::new(parent.x + self.rect.x, parent.y + self.rect.y, self.rect.w, self.rect.h);

        match *ev {
            Event::MouseMove { x, y } => {
                let lr = Rect::new(abs.x + 6, abs.y + 4, 32, self.rect.h - 8);
                let new_hover = lr.contains(x, y);
                let changed = new_hover != self.launcher_hover;
                self.launcher_hover = new_hover;
                if changed { EventResult::Redraw } else { EventResult::Ignored }
            }
            Event::MouseUp { x, y, .. } => {
                let lr = Rect::new(abs.x + 6, abs.y + 4, 32, self.rect.h - 8);
                if lr.contains(x, y) {
                    self.show_launcher = !self.show_launcher;
                    if let Some(cb) = self.on_launcher_click { cb(); }
                    EventResult::Handled
                } else {
                    EventResult::Ignored
                }
            }
            _ => EventResult::Ignored,
        }
    }
}
