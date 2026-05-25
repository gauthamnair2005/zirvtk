use alloc::boxed::Box;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::platform::Platform;
use crate::rect::Rect;
use crate::widget::{Event, Widget};

pub struct App {
    platform: Platform,
    canvas: Canvas,
    root: Option<Box<dyn Widget>>,
    cursor_x: i32,
    cursor_y: i32,
    running: bool,
    frame_ms: u32,
}

impl App {
    pub fn new() -> Option<Self> {
        let platform = Platform::connect()?;
        let info = platform.info();
        let canvas = Canvas::new(info.width, info.height);
        let mut app = Self {
            platform,
            canvas,
            root: None,
            cursor_x: 0,
            cursor_y: 0,
            running: true,
            frame_ms: 33,
        };
        app.render_desktop_bg();
        Some(app)
    }

    pub fn set_root(&mut self, mut widget: Box<dyn Widget>) {
        let w = self.canvas.width;
        let h = self.canvas.height;
        widget.set_size(w, h);
        self.root = Some(widget);
    }

    pub fn render_desktop_bg(&mut self) {
        let w = self.canvas.width;
        let h = self.canvas.height;
        let top = Color::from_u32(0xFF1A1A2E);
        let bot = Color::from_u32(0xFF0F0F1E);
        self.canvas.fill_gradient_v(0, 0, w, h, top, bot);
        for i in 0..20 {
            let x = ((i * 137 + 50) % w as i32) as i32;
            let y = ((i * 251 + 100) % h as i32) as i32;
            let r = 1u32 + (i as u32 % 3);
            let alpha = 8 + (i % 20);
            self.canvas.fill_circle(x, y, r, Color::from_argb(alpha as u8, 100, 150, 255));
        }
    }

    pub fn run(&mut self) {
        self.platform.set_cursor(self.cursor_x, self.cursor_y);
        self.render_frame();
        self.flip();

        while self.running {
            self.process_mouse();
            if self.render_frame() {
                self.flip();
            }
            self.sleep_ms(self.frame_ms);
        }
    }

    fn render_frame(&mut self) -> bool {
        let _info = self.platform.info();
        self.canvas.clear(Color::from_u32(0xFF000000));
        self.render_desktop_bg();

        if let Some(ref root) = self.root {
            root.draw(&mut self.canvas);
        }

        self.draw_cursor();
        true
    }

    fn draw_cursor(&mut self) {
        let cx = self.cursor_x;
        let cy = self.cursor_y;
        for dy in 0..12 {
            for dx in 0..12 {
                if dx == 0 || dy == 0 || dx == dy {
                    let px = cx + dx;
                    let py = cy + dy;
                    let col = if dx < 3 && dy < 3 { Color::WHITE } else { Color::from_u32(0xFF222222) };
                    self.canvas.set_pixel(px, py, col);
                }
            }
        }
    }

    fn flip(&mut self) {
        let buf = self.platform.create_buffer(self.canvas.width, self.canvas.height);
        if let Some(buf) = buf {
            self.platform.write_buffer(&buf, self.canvas.as_bytes());
            self.platform.present(&buf);
        }
    }

    fn process_mouse(&mut self) {
        while let Some(ev) = self.platform.read_mouse() {
            self.cursor_x = (self.cursor_x + ev.dx).clamp(0, self.canvas.width as i32 - 1);
            self.cursor_y = (self.cursor_y + ev.dy).clamp(0, self.canvas.height as i32 - 1);
            let root_rect = Rect::new(0, 0, self.canvas.width, self.canvas.height);
            if ev.buttons & 1 != 0 {
                if let Some(ref mut root) = self.root {
                    root.handle_event(&Event::MouseDown { x: self.cursor_x, y: self.cursor_y, button: 1 },
                                      root_rect);
                }
            } else {
                if let Some(ref mut root) = self.root {
                    root.handle_event(&Event::MouseUp { x: self.cursor_x, y: self.cursor_y, button: 1 },
                                      root_rect);
                }
            }
            if let Some(ref mut root) = self.root {
                root.handle_event(&Event::MouseMove { x: self.cursor_x, y: self.cursor_y },
                                  root_rect);
            }
        }
    }

    fn sleep_ms(&self, _ms: u32) {
        for _ in 0..1000000 {}
    }
}
