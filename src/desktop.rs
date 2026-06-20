use alloc::format;
use crate::app_launcher::AppLauncher;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::ffi;
use crate::futuristic_clock::FuturisticClock;
use crate::particle::ParticleSystem;
use crate::platform::Platform;
use crate::rect::Rect;
use crate::taskbar::TaskBar;
use crate::widget::{Event, Widget};

pub struct Desktop {
    platform: Platform,
    canvas: Canvas,
    display_buf: crate::platform::DisplayBuffer,
    particles: ParticleSystem,
    taskbar: TaskBar,
    launcher: AppLauncher,
    clock: FuturisticClock,
    running: bool,
    cursor_x: i32,
    cursor_y: i32,
    prev_buttons: u8,
}

impl Desktop {
    fn read_rtc(&self) -> (u32, u32, u32, u32, u32, u32) {
        let mut dt = ffi::DateTime { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 0 };
        let ret = unsafe { ffi::getdatetime(&mut dt) };
        if ret == 0 && dt.year >= 2024 {
            (dt.year as u32, dt.month as u32, dt.day as u32, dt.hour as u32, dt.minute as u32, dt.second as u32)
        } else {
            (0, 0, 0, 0, 0, 0)
        }
    }

    pub fn new() -> Option<Self> {
        let platform = Platform::connect()?;
        let info = platform.info();
        let width = info.width;
        let height = info.height;
        let canvas = Canvas::new(width, height);
        let display_buf = platform.create_buffer(width, height)?;

        let mut launcher = AppLauncher::new();
        launcher.set_pos((width as i32 - launcher.rect().w as i32) / 2, (height as i32 - launcher.rect().h as i32) / 2);
        launcher.on_launch = Some(|idx| {
            let path: &[u8] = match idx {
                0 => &b"/bin/shell\0"[..],
                1 => &b"/bin/hello\0"[..],
                2 => &b"/bin/sysinfo\0"[..],
                3 => &b"/bin/sysinfo\0"[..],
                4 => &b"/bin/clear\0"[..],
                5 => &b"/bin/cat\0"[..],
                6 => &b"/bin/nokia\0"[..],
                7 => &b"/bin/hello\0"[..],
                8 => &b"/bin/hello\0"[..],
                _ => &b"/bin/hello\0"[..],
            };
            unsafe { ffi::execve(path.as_ptr(), core::ptr::null(), core::ptr::null()); }
        });

        let mut taskbar = TaskBar::new();
        taskbar.set_size(width, 44);

        let mut clock = FuturisticClock::new();
        clock.set_pos(width as i32 / 2 - 150, height as i32 / 2 - 100);

        let particles = ParticleSystem::new(80, width, height);

        Some(Self {
            platform,
            canvas,
            display_buf,
            particles,
            taskbar,
            launcher,
            clock,
            running: true,
            cursor_x: width as i32 / 2,
            cursor_y: height as i32 / 2,
            prev_buttons: 0,
        })
    }

    pub fn run(&mut self) {
        self.platform.set_cursor(self.cursor_x, self.cursor_y);

        let (y, mo, d, h, mi, s) = self.read_rtc();
        if y >= 2024 {
            self.clock.set_time(h, mi, s);
            let ds = format!("{:04}-{:02}-{:02}", y, mo, d);
            self.clock.set_date(&ds);
        }

        let mut last_second = s;
        let mut tick: u64 = 0;

        loop {
            self.poll_input();
            tick += 1;

            self.particles.update(16);

            if tick % 10 == 0 {
                let (y, mo, d, h, mi, s) = self.read_rtc();
                if y >= 2024 {
                    self.clock.set_time(h, mi, s);
                    if s != last_second {
                        last_second = s;
                        let ds = format!("{:04}-{:02}-{:02}", y, mo, d);
                        self.clock.set_date(&ds);
                        self.taskbar.set_time(h, mi);
                    }
                }
            }
            self.clock.update(16);
            self.launcher.update((tick as u32).wrapping_mul(16));

            self.render_frame();
            self.platform.write_buffer(&self.display_buf, self.canvas.as_bytes());
            self.platform.present(&self.display_buf);

            if !self.running { break; }
        }
    }

    fn render_frame(&mut self) {
        let w = self.canvas.width;
        let h = self.canvas.height;

        self.canvas.clear_gradient(
            Color::from_rgb(10, 10, 30),
            Color::from_rgb(5, 5, 15));

        let dot_color = Color::from_argb(20, 40, 60, 120);
        for i in 0..40 {
            let x = ((i * 197 + 31) as u32 % w) as i32;
            let y = ((i * 251 + 67) as u32 % h) as i32;
            self.canvas.fill_circle(x, y, 1 + (i % 2), dot_color);
        }

        self.particles.draw(&mut self.canvas, 0);
        self.clock.draw(&mut self.canvas);
        self.taskbar.draw(&mut self.canvas);
        if self.launcher.is_open() {
            self.launcher.draw(&mut self.canvas);
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
                self.taskbar.handle_event(&mev, screen);
                self.launcher.handle_event(&mev, screen);
            }

            let pressed = (raw.buttons & 1) != 0;
            let was_pressed = (self.prev_buttons & 1) != 0;

            if pressed && !was_pressed {
                let mev = Event::MouseDown { x: self.cursor_x, y: self.cursor_y, button: 1 };
                self.taskbar.handle_event(&mev, screen);
                self.launcher.handle_event(&mev, screen);
            } else if !pressed && was_pressed {
                let mev = Event::MouseUp { x: self.cursor_x, y: self.cursor_y, button: 1 };
                if self.taskbar.handle_event(&mev, screen) == crate::widget::EventResult::Handled {
                    self.launcher.toggle();
                } else {
                    self.launcher.handle_event(&mev, screen);
                }
            }

            self.prev_buttons = raw.buttons;
        }

        self.taskbar.show_launcher = self.launcher.is_open();
    }
}
