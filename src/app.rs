use alloc::boxed::Box;
use crate::canvas::Canvas;
use crate::platform::{DisplayBuffer, Platform};
use crate::rect::Rect;
use crate::widget::Event;
use crate::widget::EventResult;
use crate::widget::Widget;

extern "C" {
    fn msleep(ms: u32);
}

/// The ZirvTK application driver.
///
/// Owns the platform connection, the framebuffer, and the root widget.
/// Call [`App::run`] to enter the event loop.
pub struct App {
    platform: Platform,
    buffer: DisplayBuffer,
    canvas: Canvas,
    root: Option<Box<dyn Widget>>,
    cursor_x: i32,
    cursor_y: i32,
    prev_buttons: u8,
    dirty: bool,
    running: bool,
    frame_ms: u32,
}

impl App {
    /// Connect to the ZirvFlux compositor and create the framebuffer.
    pub fn new() -> Option<Self> {
        let platform = Platform::connect()?;
        let w = platform.info().width;
        let h = platform.info().height;
        let canvas = Canvas::new(w, h);
        let buffer = platform.create_buffer(w, h)?;
        platform.suppress_dbg();
        Some(Self {
            platform,
            buffer,
            canvas,
            root: None,
            cursor_x: (w / 2) as i32,
            cursor_y: (h / 2) as i32,
            prev_buttons: 0,
            dirty: true,
            running: false,
            frame_ms: 16,
        })
    }

    /// Set the frame interval (ms). 16 ≈ 60 FPS, 33 ≈ 30 FPS.
    pub fn set_frame_ms(&mut self, ms: u32) {
        self.frame_ms = ms;
    }

    /// Set the root widget.
    pub fn set_root(&mut self, mut widget: Box<dyn Widget>) {
        let r = widget.rect();
        // Position the widget to fill the screen
        let info = self.platform.info();
        let w = info.width.min(r.w + 80);
        let h = info.height.min(r.h + 80);
        widget.set_size(w, h);
        // Center on screen
        let cx = (info.width as i32 - w as i32) / 2;
        let cy = (info.height as i32 - h as i32) / 2;
        widget.set_pos(cx.max(40), cy.max(56));
        self.root = Some(widget);
        self.dirty = true;
    }

    /// Get a reference to the platform for low-level access.
    pub fn platform(&self) -> &Platform {
        &self.platform
    }

    /// Execute one iteration of the event loop.
    /// Returns `false` to signal the caller to stop.
    pub fn tick(&mut self) -> bool {
        // Read all pending mouse events
        while let Some(ev) = self.platform.read_mouse() {
            let nx = (self.cursor_x + ev.dx).max(0).min(self.platform.info().width as i32 - 1);
            let ny = (self.cursor_y + ev.dy).max(0).min(self.platform.info().height as i32 - 1);
            self.cursor_x = nx;
            self.cursor_y = ny;
            self.platform.set_cursor(self.cursor_x, self.cursor_y);

            // Determine event type
            let prev = self.prev_buttons;
            self.prev_buttons = ev.buttons;

            let left_down = (ev.buttons & 1) != 0 && (prev & 1) == 0;
            let left_up = (ev.buttons & 1) == 0 && (prev & 1) != 0;

            if let Some(ref mut root) = self.root {
                let result = if left_down {
                    root.handle_event(
                        &Event::MouseDown {
                            x: nx,
                            y: ny,
                            button: ev.buttons,
                        },
                        Rect::new(0, 0, self.platform.info().width, self.platform.info().height),
                    )
                } else if left_up {
                    root.handle_event(
                        &Event::MouseUp {
                            x: nx,
                            y: ny,
                            button: ev.buttons,
                        },
                        Rect::new(0, 0, self.platform.info().width, self.platform.info().height),
                    )
                } else {
                    root.handle_event(
                        &Event::MouseMove { x: nx, y: ny },
                        Rect::new(0, 0, self.platform.info().width, self.platform.info().height),
                    )
                };
                if result == EventResult::Redraw || result == EventResult::Handled {
                    self.dirty = true;
                }
            }
        }

        // Re-render if dirty
        if self.dirty {
            self.render();
            let fb_size = (self.buffer.stride as usize) * (self.buffer.height as usize);
            self.platform
                .write_buffer(&self.buffer, &self.canvas.as_bytes()[..fb_size]);
            self.dirty = false;
        }

        // Present (always, even if not dirty, for cursor updates)
        self.platform.present(&self.buffer);
        unsafe { msleep(self.frame_ms); }

        self.running
    }

    /// Run the event loop indefinitely.
    pub fn run(&mut self) {
        self.dirty = true;
        self.running = true;
        loop {
            if !self.tick() {
                break;
            }
        }
    }

    /// Render the entire scene.
    fn render(&mut self) {
        let w = self.platform.info().width;
        let h = self.platform.info().height;

        // Gradient background (matching ZirvUI)
        for y in 0..h {
            for x in 0..w {
                let r = ((x * 35) / w + 18) as u8;
                let g = (12 + (y * 28) / h) as u8;
                let b = (28 + ((w - x) * 45) / w) as u8;
                self.canvas
                    .set_pixel(x as i32, y as i32, crate::color::Color::from_rgb(r, g, b));
            }
        }

        if let Some(ref root) = self.root {
            root.draw(&mut self.canvas);
        }
    }
}
