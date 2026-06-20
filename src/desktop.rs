use alloc::format;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::ffi;
use crate::metro_start::MetroStart;
use crate::platform::Platform;
use crate::rect::Rect;
use crate::taskbar::TaskBar;
use crate::widget::{Event, EventResult, Widget};

pub struct Desktop {
    platform: Platform,
    canvas: Canvas,
    display_buf: crate::platform::DisplayBuffer,
    taskbar: TaskBar,
    start: MetroStart,
    running: bool,
    cursor_x: i32,
    cursor_y: i32,
    prev_buttons: u8,
    time_h: u32,
    time_m: u32,
    time_s: u32,
    date_str: [u8; 12],
    date_len: usize,
}

impl Desktop {
    fn read_rtc(&mut self) {
        let mut dt = ffi::DateTime { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 0 };
        let ret = unsafe { ffi::getdatetime(&mut dt) };
        if ret == 0 && dt.year >= 2024 {
            self.time_h = dt.hour as u32;
            self.time_m = dt.minute as u32;
            self.time_s = dt.second as u32;
            let ds = format!("{:04}-{:02}-{:02}", dt.year as u32, dt.month as u32, dt.day as u32);
            let b = ds.as_bytes();
            let n = b.len().min(11);
            self.date_str[..n].copy_from_slice(&b[..n]);
            self.date_len = n;
        }
    }

    pub fn new() -> Option<Self> {
        let platform = Platform::connect()?;
        let info = platform.info();
        let width = info.width;
        let height = info.height;
        let canvas = Canvas::new(width, height);
        let display_buf = platform.create_buffer(width, height)?;

        let mut taskbar = TaskBar::new();
        taskbar.set_size(width, 48);
        taskbar.set_pos(0, height as i32 - 48);

        let mut start = MetroStart::new(width, height);
        start.on_launch = Some(|_idx| {});

        let mut dt = ffi::DateTime { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 0 };
        let mut date_str = [0u8; 12];
        let mut date_len = 0usize;
        let time_h;
        let time_m;
        let time_s;
        if unsafe { ffi::getdatetime(&mut dt) } == 0 && dt.year >= 2024 {
            time_h = dt.hour as u32;
            time_m = dt.minute as u32;
            time_s = dt.second as u32;
            let ds = format!("{:04}-{:02}-{:02}", dt.year as u32, dt.month as u32, dt.day as u32);
            let b = ds.as_bytes();
            let n = b.len().min(11);
            date_str[..n].copy_from_slice(&b[..n]);
            date_len = n;
        } else {
            time_h = 0; time_m = 0; time_s = 0;
        }

        Some(Self {
            platform,
            canvas,
            display_buf,
            taskbar,
            start,
            running: true,
            cursor_x: width as i32 / 2,
            cursor_y: height as i32 / 2,
            prev_buttons: 0,
            time_h, time_m, time_s,
            date_str, date_len,
        })
    }

    pub fn run(&mut self) {
        self.platform.set_cursor(self.cursor_x, self.cursor_y);
        self.taskbar.set_time(self.time_h, self.time_m);
        let mut tick: u64 = 0;
        let mut last_s = self.time_s;

        loop {
            self.poll_input();
            tick += 1;

            if tick % 10 == 0 {
                self.read_rtc();
                if self.time_s != last_s {
                    last_s = self.time_s;
                    self.taskbar.set_time(self.time_h, self.time_m);
                }
            }

            self.render_frame();
            self.platform.write_buffer(&self.display_buf, self.canvas.as_bytes());
            self.platform.present(&self.display_buf);

            if !self.running { break; }
        }
    }

    fn render_frame(&mut self) {
        let w = self.canvas.width;
        let h = self.canvas.height;

        let accent = Color::from_rgb(0, 100, 180);
        let accent_dark = Color::from_rgb(0, 80, 150);
        self.canvas.clear_gradient(accent, accent_dark);

        let cx = w as i32 / 2;
        let cy = h as i32 / 2 - 50;

        let mut buf = [0u8; 10];
        let mut bp = 0usize;
        let hm = self.time_h;
        let mm = self.time_m;
        let sm = self.time_s;
        if hm >= 10 { buf[bp] = b'0' + (hm / 10) as u8; bp += 1; }
        buf[bp] = b'0' + (hm % 10) as u8; bp += 1;
        buf[bp] = b':'; bp += 1;
        buf[bp] = b'0' + (mm / 10) as u8; bp += 1;
        buf[bp] = b'0' + (mm % 10) as u8; bp += 1;
        buf[bp] = b':'; bp += 1;
        buf[bp] = b'0' + (sm / 10) as u8; bp += 1;
        buf[bp] = b'0' + (sm % 10) as u8; bp += 1;
        let len = bp;

        let scale = 4u32;
        let step = (crate::font::FONT_W + 1) as i32 * scale as i32;
        let tw = len as i32 * step;
        let tx = cx - tw / 2;
        let ty = cy - (crate::font::FONT_H * scale) as i32 / 2 + 10;
        for i in 0..len {
            let ch = if buf[i] == b':' { ':' } else { (buf[i] - b'0' + b'0') as char };
            self.canvas.draw_char_scaled(tx + i as i32 * step, ty, ch, Color::from_rgb(255, 255, 255), scale);
        }

        if self.date_len > 0 {
            let date_str = core::str::from_utf8(&self.date_str[..self.date_len]).unwrap_or("");
            let dw = (self.date_len as u32) * (crate::font::FONT_W + 1);
            let dx = cx - (dw as i32) / 2;
            let dy = ty + (crate::font::FONT_H * scale) as i32 + 12;
            self.canvas.draw_text(dx, dy, date_str, Color::from_rgb(200, 220, 255));
        }

        self.taskbar.draw(&mut self.canvas);
        if self.start.open {
            self.start.draw(&mut self.canvas);
        }
    }

    fn poll_input(&mut self) {
        loop {
            let mut raw = ffi::zf_mouse_event { dx: 0, dy: 0, buttons: 0 };
            if unsafe { ffi::zf_read_mouse(&mut raw) } != 0 { break; }

            let old_x = self.cursor_x;
            let old_y = self.cursor_y;
            self.cursor_x = (self.cursor_x + raw.dx).clamp(0, self.canvas.width as i32 - 1);
            self.cursor_y = (self.cursor_y + raw.dy).clamp(0, self.canvas.height as i32 - 1);

            let moved = old_x != self.cursor_x || old_y != self.cursor_y;
            let screen = Rect::new(0, 0, self.canvas.width, self.canvas.height);

            if moved {
                self.platform.set_cursor(self.cursor_x, self.cursor_y);
                let mev = Event::MouseMove { x: self.cursor_x, y: self.cursor_y };
                if !self.start.open {
                    self.taskbar.handle_event(&mev, screen);
                } else {
                    self.start.handle_event(&mev, screen);
                }
            }

            let pressed = (raw.buttons & 1) != 0;
            let was_pressed = (self.prev_buttons & 1) != 0;

            if !pressed && was_pressed {
                let mev = Event::MouseUp { x: self.cursor_x, y: self.cursor_y, button: 1 };
                if self.start.open {
                    if self.start.handle_event(&mev, screen) == EventResult::Handled {
                        self.taskbar.show_launcher = false;
                    }
                } else {
                    if self.taskbar.handle_event(&mev, screen) == EventResult::Handled {
                        self.start.toggle();
                        self.taskbar.show_launcher = self.start.open;
                    }
                }
            }

            if pressed && !was_pressed && !self.start.open {
                let mev = Event::MouseDown { x: self.cursor_x, y: self.cursor_y, button: 1 };
                self.taskbar.handle_event(&mev, screen);
            }

            self.prev_buttons = raw.buttons;
        }
    }
}
