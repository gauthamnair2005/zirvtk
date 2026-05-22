//! Software framebuffer with drawing primitives (rect, text, shadow, etc.).

use alloc::vec::Vec;
use crate::color::Color;
use crate::font::{self, FONT_H, FONT_W};

/// Software pixel canvas backed by `width × height × 4` bytes of ARGB8888 data.
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pixels: Vec<u32>, // ARGB8888, row-major
}

impl Canvas {
    /// Allocate a new canvas.
    pub fn new(width: u32, height: u32) -> Self {
        let cap = (width as usize) * (height as usize);
        let mut pixels = Vec::with_capacity(cap);
        pixels.resize(cap, 0);
        Self {
            width,
            height,
            stride: width,
            pixels,
        }
    }

    /// Create a canvas from an existing pixel buffer (takes ownership).
    pub fn from_pixels(width: u32, height: u32, pixels: Vec<u32>) -> Self {
        Self {
            width,
            height,
            stride: width,
            pixels,
        }
    }

    /// Get a raw pointer to the pixel data (for `zf_write_buffer`).
    pub fn as_bytes(&self) -> &[u8] {
        let byte_len = self.pixels.len() * 4;
        unsafe { core::slice::from_raw_parts(self.pixels.as_ptr() as *const u8, byte_len) }
    }

    /// Mutable byte slice (for direct pixel manipulation).
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        let byte_len = self.pixels.len() * 4;
        unsafe { core::slice::from_raw_parts_mut(self.pixels.as_mut_ptr() as *mut u8, byte_len) }
    }

    /// Access a pixel.
    pub fn pixel(&self, x: i32, y: i32) -> Color {
        if x < 0 || y < 0 || x as u32 >= self.width || y as u32 >= self.height {
            return Color::BLACK;
        }
        Color::from_u32(self.pixels[(y as usize) * (self.stride as usize) + (x as usize)])
    }

    /// Set a pixel (bounds-checked).
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || y < 0 || x as u32 >= self.width || y as u32 >= self.height {
            return;
        }
        let idx = (y as usize) * (self.stride as usize) + (x as usize);
        self.pixels[idx] = color.to_u32();
    }

    /// Blend a pixel onto the canvas.
    pub fn blend_pixel(&mut self, x: i32, y: i32, fg: Color, alpha: u8) {
        if x < 0 || y < 0 || x as u32 >= self.width || y as u32 >= self.height {
            return;
        }
        let idx = (y as usize) * (self.stride as usize) + (x as usize);
        let bg = Color::from_u32(self.pixels[idx]);
        self.pixels[idx] = fg.blend(bg, alpha).to_u32();
    }

    /// Clear the entire canvas with a color.
    pub fn clear(&mut self, color: Color) {
        let c = color.to_u32();
        for p in self.pixels.iter_mut() {
            *p = c;
        }
    }

    /// Fill a rectangle.
    pub fn fill_rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color) {
        let c = color.to_u32();
        for row in 0..h as i32 {
            let py = y + row;
            if py < 0 || py as u32 >= self.height {
                continue;
            }
            let base = (py as usize) * (self.stride as usize);
            for col in 0..w as i32 {
                let px = x + col;
                if px < 0 || px as u32 >= self.width {
                    continue;
                }
                self.pixels[base + (px as usize)] = c;
            }
        }
    }

    /// Draw a character glyph at (x, y) with edge-blending anti-aliasing.
    pub fn draw_char(&mut self, x: i32, y: i32, c: char, color: Color) {
        let bm = font::font_get(c);
        for row in 0..FONT_H as i32 {
            let py = y + row;
            if py < 0 || py as u32 >= self.height {
                continue;
            }
            let bits = bm[row as usize];
            if bits == 0 {
                continue;
            }
            for col in 0..FONT_W as i32 {
                let px = x + col;
                if px < 0 || px as u32 >= self.width {
                    continue;
                }
                if (bits & (1 << (7 - col))) == 0 {
                    continue;
                }
                // Anti-aliasing: check 4 neighbours — if all set, solid; otherwise blend.
                let tl = col > 0 && (bits & (1 << (7 - col + 1))) != 0;
                let tr = col < FONT_W as i32 - 1 && (bits & (1 << (7 - col - 1))) != 0;
                let tu = row > 0 && (bm[(row - 1) as usize] & (1 << (7 - col))) != 0;
                let td =
                    row < FONT_H as i32 - 1 && (bm[(row + 1) as usize] & (1 << (7 - col))) != 0;
                if tl && tr && tu && td {
                    self.set_pixel(px, py, color);
                } else {
                    self.blend_pixel(px, py, color, 180);
                }
            }
        }
    }

    /// Draw text at (x, y).
    pub fn draw_text(&mut self, x: i32, y: i32, text: &str, color: Color) {
        let step = (FONT_W + 1) as i32;
        let mut cx = x;
        for ch in text.chars() {
            self.draw_char(cx, y, ch, color);
            cx += step;
        }
    }

    /// Draw a drop shadow around a rectangle.
    pub fn draw_shadow(&mut self, x: i32, y: i32, w: u32, h: u32, radius: i32) {
        for r in 1..=radius {
            let alpha = 32 - (r * 32) / (radius + 1);
            if alpha <= 0 {
                continue;
            }
            for row in -r..(h as i32 + r) {
                let py = y + row;
                if py < 0 || py as u32 >= self.height {
                    continue;
                }
                for col in -r..(w as i32 + r) {
                    let px = x + col;
                    if px < 0 || px as u32 >= self.width {
                        continue;
                    }
                    // Skip the interior rectangle
                    if row >= 0 && row < h as i32 && col >= 0 && col < w as i32 {
                        continue;
                    }
                    self.blend_pixel(px, py, Color::BLACK, alpha as u8);
                }
            }
        }
    }

    /// Draw a horizontal line.
    pub fn hline(&mut self, x: i32, y: i32, w: u32, color: Color) {
        self.fill_rect(x, y, w, 1, color);
    }

    /// Draw a vertical line.
    pub fn vline(&mut self, x: i32, y: i32, h: u32, color: Color) {
        self.fill_rect(x, y, 1, h, color);
    }
}
