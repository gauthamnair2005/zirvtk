use crate::canvas::Canvas;
use crate::color::Color;
use crate::rect::Rect;
use crate::widget::{Event, EventResult, Widget};
use crate::font;

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
    pub time_h: u32, pub time_m: u32,
    launcher_hover: bool,
}

impl TaskBar {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(0, 0, 800, 48),
            show_launcher: false,
            time_h: 0, time_m: 0,
            launcher_hover: false,
        }
    }

    pub fn set_time(&mut self, h: u32, m: u32) {
        self.time_h = h;
        self.time_m = m;
    }

    fn start_rect(&self) -> Rect {
        Rect::new(self.rect.x + 4, self.rect.y + 4, 48, self.rect.h - 8)
    }

    fn clock_rect(&self) -> Rect {
        let cw = 100u32;
        Rect::new(self.rect.x + self.rect.w as i32 - cw as i32 - 8, self.rect.y, cw, self.rect.h)
    }
}

impl Widget for TaskBar {
    fn draw(&self, canvas: &mut Canvas) {
        canvas.fill_rect(self.rect.x, self.rect.y, self.rect.w, self.rect.h, Color::from_rgb(16, 16, 16));

        let sr = self.start_rect();
        let is_hover = self.launcher_hover;

        let start_bg = if is_hover { Color::from_rgb(60, 60, 60) } else { Color::from_rgb(40, 40, 40) };
        canvas.fill_rect(sr.x, sr.y, sr.w, sr.h, start_bg);

        let lx = sr.x + sr.w as i32 / 2 - 6;
        let ly = sr.y + sr.h as i32 / 2 - 6;
        let logo_color = Color::from_rgb(0, 120, 215);
        canvas.fill_rect(lx, ly, 4, 4, logo_color);
        canvas.fill_rect(lx + 8, ly, 4, 4, Color::from_rgb(240, 140, 40));
        canvas.fill_rect(lx, ly + 8, 4, 4, Color::from_rgb(80, 180, 80));
        canvas.fill_rect(lx + 8, ly + 8, 4, 4, Color::from_rgb(220, 60, 140));

        if self.show_launcher {
            canvas.fill_rect(sr.x, sr.y + sr.h as i32 - 2, sr.w, 2, Color::from_rgb(0, 120, 215));
        }

        let mut buf = [0u8; 8];
        let mut w = BufW(&mut buf, 0);
        let _ = core::fmt::write(&mut w, format_args!("{:02}:{:02}", self.time_h, self.time_m));
        let len = w.1;
        let step = (font::FONT_W + 1) as i32;
        let tw = len as i32 * step;
        let cr = self.clock_rect();
        let cx = cr.x + (cr.w as i32 - tw) / 2;
        let cy = cr.y + (cr.h as i32 - font::FONT_H as i32) / 2;
        for i in 0..len {
            canvas.draw_char(cx + i as i32 * step, cy, buf[i] as char, Color::from_rgb(220, 220, 220));
        }
    }

    fn rect(&self) -> Rect { self.rect }
    fn min_size(&self) -> (u32, u32) { (100, 48) }

    fn set_pos(&mut self, x: i32, y: i32) { self.rect.x = x; self.rect.y = y; }
    fn set_size(&mut self, w: u32, h: u32) { self.rect.w = w; self.rect.h = h; }

    fn handle_event(&mut self, ev: &Event, parent: Rect) -> EventResult {
        let abs = Rect::new(parent.x + self.rect.x, parent.y + self.rect.y, self.rect.w, self.rect.h);

        match *ev {
            Event::MouseMove { x, y } => {
                let sr = Rect::new(abs.x + 4, abs.y + 4, 48, self.rect.h - 8);
                let new_hover = sr.contains(x, y);
                let changed = new_hover != self.launcher_hover;
                self.launcher_hover = new_hover;
                if changed { EventResult::Redraw } else { EventResult::Ignored }
            }
            Event::MouseUp { x, y, .. } => {
                let sr = Rect::new(abs.x + 4, abs.y + 4, 48, self.rect.h - 8);
                if sr.contains(x, y) {
                    self.show_launcher = !self.show_launcher;
                    EventResult::Handled
                } else {
                    EventResult::Ignored
                }
            }
            _ => EventResult::Ignored,
        }
    }
}
