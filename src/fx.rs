use crate::canvas::Canvas;
use crate::color::Color;
use core::cmp::min;

const GLOW_LUT: [u8; 16] = [255, 180, 100, 55, 30, 16, 8, 4, 2, 1, 0, 0, 0, 0, 0, 0];

pub fn glow(canvas: &mut Canvas, cx: i32, cy: i32, radius: u32, color: Color, intensity: u8) {
    let r = min(radius as i32, 15);
    for dy in -r..=r {
        let py = cy + dy;
        if py < 0 || py as u32 >= canvas.height { continue; }
        for dx in -r..=r {
            let px = cx + dx;
            if px < 0 || px as u32 >= canvas.width { continue; }
            let dist = libm::roundf(libm::sqrtf((dx * dx + dy * dy) as f32)) as usize;
            if dist > r as usize { continue; }
            let a = (GLOW_LUT[dist] as u32 * intensity as u32 / 255) as u8;
            if a > 0 {
                let bg = canvas.pixel(px, py);
                canvas.set_pixel(px, py, color.blend(bg, a));
            }
        }
    }
}

pub fn glow_rect(canvas: &mut Canvas, x: i32, y: i32, w: u32, h: u32, radius: u32, color: Color, intensity: u8) {
    let glow_r = min(radius + 4, 15);
    let r = min(radius, min(w, h) / 2) as i32;
    for dy in -(glow_r as i32)..(h as i32 + glow_r as i32) {
        let py = y + dy;
        if py < 0 || py as u32 >= canvas.height { continue; }
        for dx in -(glow_r as i32)..(w as i32 + glow_r as i32) {
            let px = x + dx;
            if px < 0 || px as u32 >= canvas.width { continue; }
            if dx >= 0 && dx < w as i32 && dy >= 0 && dy < h as i32 { continue; }
            let inside = in_rrect(px, py, x - glow_r as i32, y - glow_r as i32, w + glow_r as u32 * 2, h + glow_r as u32 * 2, r + glow_r as i32);
            if !inside { continue; }
            let inside_inner = in_rrect(px, py, x, y, w, h, r);
            if inside_inner { continue; }
            let cx2 = clamp(px, x + r, x + w as i32 - r - 1);
            let cy2 = clamp(py, y + r, y + h as i32 - r - 1);
            let d2 = (px - cx2) * (px - cx2) + (py - cy2) * (py - cy2);
            let dist = (libm::roundf(libm::sqrtf(d2 as f32)) as usize).min(15);
            let a = (GLOW_LUT[dist] as u32 * intensity as u32 / 255) as u8;
            if a > 0 {
                let bg = canvas.pixel(px, py);
                canvas.set_pixel(px, py, color.blend(bg, a));
            }
        }
    }
}

fn in_rrect(px: i32, py: i32, x: i32, y: i32, w: u32, h: u32, r: i32) -> bool {
    if px < x || py < y || px >= x + w as i32 || py >= y + h as i32 { return false; }
    if r <= 0 { return true; }
    let left = x + r; let right = x + w as i32 - r - 1;
    let top = y + r; let bot = y + h as i32 - r - 1;
    if (px >= left && px <= right) || (py >= top && py <= bot) { return true; }
    let cx = if px < left { left } else { right };
    let cy = if py < top { top } else { bot };
    let dx = px - cx; let dy = py - cy;
    dx * dx + dy * dy <= r * r
}

fn clamp(v: i32, lo: i32, hi: i32) -> i32 {
    if v < lo { lo } else if v > hi { hi } else { v }
}

pub fn box_blur(canvas: &mut Canvas, x: i32, y: i32, w: u32, h: u32, radius: u32) {
    let r = radius as i32;
    if r <= 0 { return; }
    for pass in 0..2 {
        for py in y..(y + h as i32) {
            if py < 0 || py as u32 >= canvas.height { continue; }
            for px in x..(x + w as i32) {
                if px < 0 || px as u32 >= canvas.width { continue; }
                let mut ar: u32 = 0; let mut ag: u32 = 0; let mut ab: u32 = 0;
                let mut count: u32 = 0;
                for dy in -r..=r {
                    let sy = py + dy;
                    if sy < 0 || sy as u32 >= canvas.height { continue; }
                    for dx in -r..=r {
                        let sx = px + dx;
                        if sx < 0 || sx as u32 >= canvas.width { continue; }
                        let c = canvas.pixel(sx, sy);
                        ar += c.r() as u32; ag += c.g() as u32; ab += c.b() as u32;
                        count += 1;
                    }
                }
                if count > 0 {
                    let avg = Color::from_rgb((ar / count) as u8, (ag / count) as u8, (ab / count) as u8);
                    if pass == 1 { canvas.set_pixel(px, py, avg); }
                }
            }
        }
    }
}

pub fn glass_panel(canvas: &mut Canvas, x: i32, y: i32, w: u32, h: u32, radius: u32, tint: Color, opacity: u8, glow_color: Color, glow_intensity: u8) {
    if glow_intensity > 0 {
        glow_rect(canvas, x, y, w, h, radius, glow_color, glow_intensity);
    }
    let r = core::cmp::min(radius, core::cmp::min(w, h) / 2) as i32;
    let r2 = r * r;
    for row in 0..h as i32 {
        let py = y + row;
        if py < 0 || py as u32 >= canvas.height { continue; }
        let (x0, x1) = Canvas::rrect_span(row, h as i32, r, r2, x, w as i32);
        if x0 <= x1 {
            let sx = x0.max(0) as usize;
            let ex = x1.min(canvas.width as i32 - 1) as usize;
            if ex >= sx {
                let base = (py as usize) * (canvas.stride as usize);
                for col in sx..=ex {
                    let bg = Color::from_u32(canvas.pixels[base + col]);
                    canvas.pixels[base + col] = tint.blend(bg, opacity).to_u32();
                }
            }
        }
    }
    let border = Color::from_argb(60, glow_color.r(), glow_color.g(), glow_color.b());
    canvas.stroke_round_rect(x, y, w, h, radius, border);
}

pub fn pulse_glow(canvas: &mut Canvas, cx: i32, cy: i32, base_radius: u32, time_ms: u32, color: Color, intensity: u8) {
    let pulse = (libm::sinf(time_ms as f32 * 0.003) * 0.3 + 0.7) as u8;
    let radius = base_radius + ((time_ms % 1000) / 100) % 4;
    let adj = (intensity as u32 * pulse as u32 / 255) as u8;
    glow(canvas, cx, cy, radius, color, adj);
}

pub fn scanline(canvas: &mut Canvas, time_ms: u32) {
    let h = canvas.height;
    let offset = (time_ms / 16) % h;
    for y in 0..h {
        let dist = ((y as i32 - offset as i32).abs() as u32).min(h - 1);
        if dist > 2 { continue; }
        let alpha = if dist == 0 { 6 } else { 3 };
        for x in 0..canvas.width {
            canvas.blend_pixel(x as i32, y as i32, Color::WHITE, alpha);
        }
    }
}
