use alloc::vec::Vec;
use crate::color::Color;
use crate::font;
use core::cmp::min;

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub(crate) pixels: Vec<u32>,
    pub(crate) depth: Vec<u16>,
}

fn in_round_rect(px: i32, py: i32, x: i32, y: i32, w: u32, h: u32, radius: u32) -> bool {
    if w == 0 || h == 0 { return false; }
    let w_i = w as i32; let h_i = h as i32;
    if px < x || py < y || px >= x + w_i || py >= y + h_i { return false; }
    let r = min(radius, min(w, h) / 2) as i32;
    if r <= 0 { return true; }
    let left = x + r; let right = x + w_i - r - 1;
    let top = y + r; let bottom = y + h_i - r - 1;
    if (px >= left && px <= right) || (py >= top && py <= bottom) { return true; }
    let cx = if px < left { left } else { right };
    let cy = if py < top { top } else { bottom };
    let dx = px - cx; let dy = py - cy;
    dx * dx + dy * dy <= r * r
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let cap = (width as usize) * (height as usize);
        let mut pixels = Vec::with_capacity(cap);
        pixels.resize(cap, 0);
        let mut depth = Vec::with_capacity(cap);
        depth.resize(cap, 0xFFFF);
        Self { width, height, stride: width, pixels, depth }
    }

    pub fn from_pixels(width: u32, height: u32, pixels: Vec<u32>) -> Self {
        let cap = (width as usize) * (height as usize);
        let mut depth = Vec::with_capacity(cap);
        depth.resize(cap, 0xFFFF);
        Self { width, height, stride: width, pixels, depth }
    }

    pub fn as_bytes(&self) -> &[u8] {
        let byte_len = self.pixels.len() * 4;
        unsafe { core::slice::from_raw_parts(self.pixels.as_ptr() as *const u8, byte_len) }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        let byte_len = self.pixels.len() * 4;
        unsafe { core::slice::from_raw_parts_mut(self.pixels.as_mut_ptr() as *mut u8, byte_len) }
    }

    pub fn pixel(&self, x: i32, y: i32) -> Color {
        if x < 0 || y < 0 || x as u32 >= self.width || y as u32 >= self.height { return Color::BLACK; }
        Color::from_u32(self.pixels[(y as usize) * (self.stride as usize) + (x as usize)])
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || y < 0 || x as u32 >= self.width || y as u32 >= self.height { return; }
        self.pixels[(y as usize) * (self.stride as usize) + (x as usize)] = color.to_u32();
    }

    pub fn blend_pixel(&mut self, x: i32, y: i32, fg: Color, alpha: u8) {
        if x < 0 || y < 0 || x as u32 >= self.width || y as u32 >= self.height { return; }
        let idx = (y as usize) * (self.stride as usize) + (x as usize);
        self.pixels[idx] = fg.blend(Color::from_u32(self.pixels[idx]), alpha).to_u32();
    }

    pub fn clear(&mut self, color: Color) {
        let c = color.to_u32();
        for p in self.pixels.iter_mut() { *p = c; }
        for d in self.depth.iter_mut() { *d = 0xFFFF; }
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color) {
        let c = color.to_u32();
        for row in 0..h as i32 {
            let py = y + row;
            if py < 0 || py as u32 >= self.height { continue; }
            let base = (py as usize) * (self.stride as usize);
            for col in 0..w as i32 {
                let px = x + col;
                if px < 0 || px as u32 >= self.width { continue; }
                self.pixels[base + (px as usize)] = c;
            }
        }
    }

    pub(crate) fn rrect_span(row: i32, h: i32, r: i32, r2: i32, x: i32, w: i32) -> (i32, i32) {
        if r <= 0 {
            return (x, x + w - 1);
        }
        if row < r {
            let d = r - row;
            let ins = libm::sqrtf((r2 - d * d) as f32) as i32;
            (x + r - ins, x + w - r + ins - 1)
        } else if row > h - r - 1 {
            let d = row - (h - r - 1);
            let ins = libm::sqrtf((r2 - d * d) as f32) as i32;
            (x + r - ins, x + w - r + ins - 1)
        } else {
            (x, x + w - 1)
        }
    }

    pub fn fill_round_rect(&mut self, x: i32, y: i32, w: u32, h: u32, radius: u32, color: Color) {
        let c = color.to_u32();
        let r = min(radius, min(w, h) / 2) as i32;
        if r <= 0 { self.fill_rect(x, y, w, h, color); return; }
        let r2 = r * r;
        for row in 0..h as i32 {
            let py = y + row;
            if py < 0 || py as u32 >= self.height { continue; }
            let (x0, x1) = Self::rrect_span(row, h as i32, r, r2, x, w as i32);
            if x0 <= x1 {
                let sx = x0.max(0) as usize;
                let ex = x1.min(self.width as i32 - 1) as usize;
                if ex >= sx {
                    let base = (py as usize) * (self.stride as usize);
                    self.pixels[(base + sx)..=(base + ex)].fill(c);
                }
            }
        }
    }

    pub fn fill_gradient_v(&mut self, x: i32, y: i32, w: u32, h: u32, top: Color, bot: Color) {
        let base_off = |py: usize| py * (self.stride as usize);
        for row in 0..h as i32 {
            let py = y + row;
            if py < 0 || py as u32 >= self.height { continue; }
            let t = if h > 0 { (row as u32 * 255) / h } else { 0u32 };
            let inv = 255 - t;
            let r = (top.r() as u32 * inv + bot.r() as u32 * t) / 255;
            let g = (top.g() as u32 * inv + bot.g() as u32 * t) / 255;
            let b = (top.b() as u32 * inv + bot.b() as u32 * t) / 255;
            let c = Color::from_argb(0xFF, r as u8, g as u8, b as u8).to_u32();
            let base = base_off(py as usize);
            let start = (base + x as usize).max(0);
            let end = (base + (x + w as i32) as usize).min(self.pixels.len());
            if end > start {
                self.pixels[start..end].fill(c);
            }
        }
    }

    pub fn clear_gradient(&mut self, top: Color, bot: Color) {
        let h = self.height;
        let w = self.width;
        let step = self.stride as usize;
        for row in 0..h {
            let t = (row * 255) / h;
            let inv = 255 - t;
            let r = (top.r() as u32 * inv + bot.r() as u32 * t) / 255;
            let g = (top.g() as u32 * inv + bot.g() as u32 * t) / 255;
            let b = (top.b() as u32 * inv + bot.b() as u32 * t) / 255;
            let c = Color::from_argb(0xFF, r as u8, g as u8, b as u8).to_u32();
            let start = row as usize * step;
            self.pixels[start..start + w as usize].fill(c);
        }
        self.depth.fill(0xFFFF);
    }

    pub fn fill_glass_rect(&mut self, x: i32, y: i32, w: u32, h: u32, base: Color) {
        let top = Color::from_argb(0x66, 0xFF, 0xFF, 0xFF);
        let bot = Color::from_argb(0x22, 0xFF, 0xFF, 0xFF);
        for row in 0..h as i32 {
            let py = y + row;
            if py < 0 || py as u32 >= self.height { continue; }
            let t = if h > 0 { (row as u32 * 255) / h } else { 0u32 };
            let inv = 255 - t;
            let glass_top = (top.a() as u32 * inv + bot.a() as u32 * t) / 255;
            let base_idx = (py as usize) * (self.stride as usize);
            for col in 0..w as i32 {
                let px = x + col;
                if px < 0 || px as u32 >= self.width { continue; }
                let bg = Color::from_u32(self.pixels[base_idx + (px as usize)]);
                let blended = base.blend(bg, 180);
                let final_c = Color::from_argb(0xFF,
                    (blended.r() as u32 * (255 - glass_top) + 0xFF * glass_top / 255) as u8,
                    (blended.g() as u32 * (255 - glass_top) + 0xFF * glass_top / 255) as u8,
                    (blended.b() as u32 * (255 - glass_top) + 0xFF * glass_top / 255) as u8);
                self.pixels[base_idx + (px as usize)] = final_c.to_u32();
            }
        }
    }

    pub fn draw_shadow(&mut self, x: i32, y: i32, w: u32, h: u32, radius: i32) {
        for r in 1..=radius {
            let alpha = 32 - (r * 32) / (radius + 1);
            if alpha <= 0 { continue; }
            for row in -r..(h as i32 + r) {
                let py = y + row;
                if py < 0 || py as u32 >= self.height { continue; }
                for col in -r..(w as i32 + r) {
                    let px = x + col;
                    if px < 0 || px as u32 >= self.width { continue; }
                    if row >= 0 && row < h as i32 && col >= 0 && col < w as i32 { continue; }
                    self.blend_pixel(px, py, Color::BLACK, alpha as u8);
                }
            }
        }
    }

    pub fn draw_char(&mut self, x: i32, y: i32, c: char, color: Color) {
        let bm = font::font_get(c);
        for row in 0..font::FONT_H as i32 {
            let py = y + row;
            if py < 0 || py as u32 >= self.height { continue; }
            let bits = bm[row as usize];
            if bits == 0 { continue; }
            for col in 0..font::FONT_W as i32 {
                let px = x + col;
                if px < 0 || px as u32 >= self.width { continue; }
                if (bits & (1 << (7 - col))) == 0 { continue; }
                self.set_pixel(px, py, color);
            }
        }
    }

    pub fn draw_text(&mut self, x: i32, y: i32, text: &str, color: Color) {
        let step = (font::FONT_W + 1) as i32;
        let mut cx = x;
        for ch in text.chars() { self.draw_char(cx, y, ch, color); cx += step; }
    }

    pub fn draw_text_large(&mut self, x: i32, y: i32, text: &str, color: Color, scale: u32) {
        let step = (font::FONT_W + 1) as i32 * scale as i32;
        let mut cx = x;
        for ch in text.chars() {
            self.draw_char_scaled(cx, y, ch, color, scale);
            cx += step;
        }
    }

    pub fn draw_char_scaled(&mut self, x: i32, y: i32, c: char, color: Color, scale: u32) {
        let bm = font::font_get(c);
        let sw = font::FONT_W as i32;
        let sh = font::FONT_H as i32;

        if scale == 0 { return; }

        if scale == 1 {
            for row in 0..sh {
                let py = y + row;
                if py < 0 || py as u32 >= self.height { continue; }
                let bits = bm[row as usize];
                if bits == 0 { continue; }
                let base = (py as usize) * (self.stride as usize);
                for col in 0..sw {
                    let px = x + col;
                    if px < 0 || px as u32 >= self.width { continue; }
                    if (bits & (1 << (7 - col))) == 0 { continue; }
                    self.pixels[base + (px as usize)] = color.to_u32();
                }
            }
            return;
        }

        let s = scale as i32;
        let scaled_w = sw * s;
        let scaled_h = sh * s;
        let sf = scale as f32;
        let max_sx = (font::FONT_W - 1) as f32 + 0.999;
        let max_sy = (font::FONT_H - 1) as f32 + 0.999;

        for row in 0..scaled_h {
            let py = y + row;
            if py < 0 || py as u32 >= self.height { continue; }
            let base = (py as usize) * (self.stride as usize);

            for col in 0..scaled_w {
                let px = x + col;
                if px < 0 || px as u32 >= self.width { continue; }

                let sx = ((col as f32 + 0.5) / sf - 0.5).clamp(0.0, max_sx);
                let sy = ((row as f32 + 0.5) / sf - 0.5).clamp(0.0, max_sy);

                let ix = sx as usize;
                let iy = sy as usize;
                let fx = sx - ix as f32;
                let fy = sy - iy as f32;

                let p00 = ((bm[iy] >> (7 - ix)) & 1) as u32;
                let p10 = if ix + 1 < font::FONT_W as usize { ((bm[iy] >> (7 - ix - 1)) & 1) as u32 } else { p00 };
                let p01 = if iy + 1 < font::FONT_H as usize { ((bm[iy + 1] >> (7 - ix)) & 1) as u32 } else { p00 };
                let p11 = if ix + 1 < font::FONT_W as usize && iy + 1 < font::FONT_H as usize
                    { ((bm[iy + 1] >> (7 - ix - 1)) & 1) as u32 } else { p01 };

                let fx8 = (fx * 256.0) as u32;
                let fy8 = (fy * 256.0) as u32;
                let fx8_inv = 256 - fx8;
                let fy8_inv = 256 - fy8;

                let v = (fx8_inv * fy8_inv * p00
                       + fx8 * fy8_inv * p10
                       + fx8_inv * fy8 * p01
                       + fx8 * fy8 * p11) / 65536;

                if v > 0 {
                    if v >= 255 {
                        self.pixels[base + (px as usize)] = color.to_u32();
                    } else {
                        let idx = base + (px as usize);
                        self.pixels[idx] = color.blend(Color::from_u32(self.pixels[idx]), v as u8).to_u32();
                    }
                }
            }
        }
    }

    pub fn hline(&mut self, x: i32, y: i32, w: u32, color: Color) {
        let c = color.to_u32();
        if y < 0 || y as u32 >= self.height { return; }
        let base = (y as usize) * (self.stride as usize);
        for col in 0..w as i32 {
            let px = x + col;
            if px < 0 || px as u32 >= self.width { continue; }
            self.pixels[base + (px as usize)] = c;
        }
    }

    pub fn fill_circle(&mut self, cx: i32, cy: i32, r: u32, color: Color) {
        let cc = color.to_u32();
        let rr = (r * r) as i32;
        for dy in -(r as i32)..=r as i32 {
            let py = cy + dy;
            if py < 0 || py as u32 >= self.height { continue; }
            let mut dx = 0;
            while dx * dx + dy * dy <= rr { dx += 1; }
            dx -= 1;
            if dx < 0 { continue; }
            let x0 = (cx - dx).max(0) as usize;
            let x1 = (cx + dx).min(self.width as i32 - 1) as usize;
            if x1 >= x0 {
                let base = (py as usize) * (self.stride as usize);
                self.pixels[(base + x0)..=(base + x1)].fill(cc);
            }
        }
    }

    pub fn stroke_round_rect(&mut self, x: i32, y: i32, w: u32, h: u32, radius: u32, color: Color) {
        if w < 3 || h < 3 { return; }
        let c = color.to_u32();
        let r = min(radius, min(w, h) / 2) as i32;
        if r <= 0 { return; }
        let r2 = r * r;
        let ri = (r - 1).max(0);
        let r2i = ri * ri;
        for row in 0..h as i32 {
            let py = y + row;
            if py < 0 || py as u32 >= self.height { continue; }
            let base = (py as usize) * (self.stride as usize);
            let (ol, or_) = Self::rrect_span(row, h as i32, r, r2, x, w as i32);
            let inner_row = row - 1;
            let (il, ir) = if inner_row >= 0 && inner_row < h as i32 - 2 {
                Self::rrect_span(inner_row, h as i32 - 2, ri, r2i, x + 1, w as i32 - 2)
            } else {
                (or_ + 1, ol - 1)
            };
            if il > ol {
                let px = ol;
                if px >= 0 && px < self.width as i32 {
                    self.pixels[base + px as usize] = c;
                }
            }
            if ir < or_ {
                let px = or_;
                if px >= 0 && px < self.width as i32 {
                    self.pixels[base + px as usize] = c;
                }
            }
        }
    }

    pub fn present_region(&mut self, _x: u32, _y: u32, _w: u32, _h: u32) {
    }

    pub fn depth_ptr(&mut self) -> *mut u16 {
        self.depth.as_mut_ptr()
    }

    pub fn pixels_mut(&mut self) -> &mut [u32] {
        &mut self.pixels
    }
}
