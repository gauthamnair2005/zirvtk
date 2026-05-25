//! Raw framebuffer drawing — operates directly on a `*mut u32` pixel buffer.
//! These are the system-level drawing primitives callable from C.

use crate::color::Color;
use crate::font;

unsafe fn fb_pixel(fb: *mut u32, w: u32, _h: u32, x: i32, y: i32) -> *mut u32 {
    if x < 0 || y < 0 || x as u32 >= w || y as u32 >= _h {
        return core::ptr::null_mut();
    }
    let idx = (y as usize) * (w as usize) + (x as usize);
    fb.add(idx)
}

fn in_round_rect(px: i32, py: i32, x: i32, y: i32, w: u32, h: u32, radius: u32) -> bool {
    if w == 0 || h == 0 { return false; }
    let w_i = w as i32; let h_i = h as i32;
    if px < x || py < y || px >= x + w_i || py >= y + h_i { return false; }
    let r = core::cmp::min(radius, core::cmp::min(w, h) / 2) as i32;
    if r <= 0 { return true; }
    let left  = x + r;
    let right = x + w_i - r - 1;
    let top   = y + r;
    let bot   = y + h_i - r - 1;
    if (px >= left && px <= right) || (py >= top && py <= bot) { return true; }
    let cx = if px < left { left } else { right };
    let cy = if py < top { top } else { bot };
    let dx = px - cx;
    let dy = py - cy;
    dx * dx + dy * dy <= r * r
}

#[no_mangle]
pub unsafe extern "C" fn ztk_fb_blend(fg: u32, bg: u32, alpha: u8) -> u32 {
    Color::from_u32(fg).blend(Color::from_u32(bg), alpha).to_u32()
}

#[no_mangle]
pub unsafe extern "C" fn ztk_fb_set_pixel(
    fb: *mut u32, w: u32, h: u32, x: i32, y: i32, color: u32,
) {
    let p = fb_pixel(fb, w, h, x, y);
    if !p.is_null() {
        *p = color;
    }
}

#[no_mangle]
pub unsafe extern "C" fn ztk_fb_fill_rect(
    fb: *mut u32, w: u32, h: u32,
    x: i32, y: i32, rw: u32, rh: u32, color: u32,
) {
    for row in 0..rh as i32 {
        let py = y + row;
        if py < 0 || py as u32 >= h { continue; }
        let base = (py as usize) * (w as usize);
        for col in 0..rw as i32 {
            let px = x + col;
            if px < 0 || px as u32 >= w { continue; }
            *fb.add(base + (px as usize)) = color;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ztk_fb_fill_gradient_v(
    fb: *mut u32, w: u32, h: u32,
    x: i32, y: i32, gw: u32, gh: u32, top: u32, bot: u32,
) {
    let top_c = Color::from_u32(top);
    let bot_c = Color::from_u32(bot);
    for row in 0..gh as i32 {
        let py = y + row;
        if py < 0 || py as u32 >= h { continue; }
        let t = if gh > 0 { (row as u32 * 255) / gh } else { 0u32 };
        let inv = 255 - t;
        let r = (top_c.r() as u32 * inv + bot_c.r() as u32 * t) / 255;
        let g = (top_c.g() as u32 * inv + bot_c.g() as u32 * t) / 255;
        let b = (top_c.b() as u32 * inv + bot_c.b() as u32 * t) / 255;
        let c = Color::from_argb(0xFF, r as u8, g as u8, b as u8).to_u32();
        let base = (py as usize) * (w as usize);
        for col in 0..gw as i32 {
            let px = x + col;
            if px < 0 || px as u32 >= w { continue; }
            *fb.add(base + (px as usize)) = c;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ztk_fb_draw_char(
    fb: *mut u32, fb_w: u32, fb_h: u32,
    x: i32, y: i32, c: u8, color: u32,
) {
    let bm = font::font_get(c as char);
    for row in 0..font::FONT_H as i32 {
        let py = y + row;
        if py < 0 || py as u32 >= fb_h { continue; }
        let bits = bm[row as usize];
        if bits == 0 { continue; }
        for col in 0..font::FONT_W as i32 {
            let px = x + col;
            if px < 0 || px as u32 >= fb_w { continue; }
            if (bits & (1 << (7 - col))) == 0 { continue; }
            let idx = (py as usize) * (fb_w as usize) + (px as usize);
            *fb.add(idx) = color;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ztk_fb_draw_text(
    fb: *mut u32, fb_w: u32, fb_h: u32,
    x: i32, y: i32, text: *const u8, color: u32,
) {
    let step = (font::FONT_W + 1) as i32;
    let mut cx = x;
    let mut i = 0;
    loop {
        let c = *text.add(i);
        if c == 0 { break; }
        ztk_fb_draw_char(fb, fb_w, fb_h, cx, y, c, color);
        cx += step;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn ztk_fb_draw_char_scaled(
    fb: *mut u32, fb_w: u32, fb_h: u32,
    x: i32, y: i32, c: u8, color: u32, scale: u32,
) {
    let bm = font::font_get(c as char);
    for row in 0..font::FONT_H as i32 {
        let bits = bm[row as usize];
        if bits == 0 { continue; }
        for col in 0..font::FONT_W as i32 {
            if (bits & (1 << (7 - col))) == 0 { continue; }
            for sy in 0..scale as i32 {
                for sx in 0..scale as i32 {
                    let px = x + col * scale as i32 + sx;
                    let py = y + row * scale as i32 + sy;
                    if px < 0 || py < 0 || px as u32 >= fb_w || py as u32 >= fb_h { continue; }
                    let idx = (py as usize) * (fb_w as usize) + (px as usize);
                    *fb.add(idx) = color;
                }
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ztk_fb_draw_text_large(
    fb: *mut u32, fb_w: u32, fb_h: u32,
    x: i32, y: i32, text: *const u8, color: u32, scale: u32,
) {
    let step = (font::FONT_W + 1) as i32 * scale as i32;
    let mut cx = x;
    let mut i = 0;
    loop {
        let c = *text.add(i);
        if c == 0 { break; }
        ztk_fb_draw_char_scaled(fb, fb_w, fb_h, cx, y, c, color, scale);
        cx += step;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn ztk_fb_hline(
    fb: *mut u32, w: u32, h: u32,
    x: i32, y: i32, line_w: u32, color: u32,
) {
    if y < 0 || y as u32 >= h { return; }
    let base = (y as usize) * (w as usize);
    for col in 0..line_w as i32 {
        let px = x + col;
        if px < 0 || px as u32 >= w { continue; }
        *fb.add(base + (px as usize)) = color;
    }
}

#[no_mangle]
pub unsafe extern "C" fn ztk_fb_fill_circle(
    fb: *mut u32, w: u32, h: u32,
    cx: i32, cy: i32, r: u32, color: u32,
) {
    let rr = (r * r) as i32;
    for dy in -(r as i32)..=r as i32 {
        let py = cy + dy;
        if py < 0 || py as u32 >= h { continue; }
        let base = (py as usize) * (w as usize);
        for dx in -(r as i32)..=r as i32 {
            let px = cx + dx;
            if px < 0 || px as u32 >= w { continue; }
            if dx * dx + dy * dy <= rr {
                *fb.add(base + (px as usize)) = color;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ztk_fb_fill_round_rect(
    fb: *mut u32, w: u32, h: u32,
    x: i32, y: i32, rw: u32, rh: u32, radius: u32, color: u32,
) {
    for row in 0..rh as i32 {
        let py = y + row;
        if py < 0 || py as u32 >= h { continue; }
        let base = (py as usize) * (w as usize);
        for col in 0..rw as i32 {
            let px = x + col;
            if px < 0 || px as u32 >= w { continue; }
            if !in_round_rect(px, py, x, y, rw, rh, radius) { continue; }
            *fb.add(base + (px as usize)) = color;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ztk_fb_draw_line(
    fb: *mut u32, fb_w: u32, fb_h: u32,
    x0: i32, y0: i32, x1: i32, y1: i32, color: u32,
) {
    let mut x = x0;
    let mut y = y0;
    let dx = if x1 > x0 { x1 - x0 } else { x0 - x1 };
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = if y1 > y0 { y1 - y0 } else { y0 - y1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = (if dx > dy { dx } else { -dy }) / 2;
    loop {
        ztk_fb_set_pixel(fb, fb_w, fb_h, x, y, color);
        if x == x1 && y == y1 { break; }
        let e2 = err;
        if e2 > -dx { err -= dy; x += sx; }
        if e2 < dy { err += dx; y += sy; }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ztk_draw_icon(
    fb: *mut u32, fb_w: u32, fb_h: u32,
    cx: i32, cy: i32, size: u32, icon_type: u32, color: u32,
) {
    let s = size as i32;
    let half = s / 2;
    match icon_type {
        0 => { // Terminal: dark square + `>_`
            ztk_fb_fill_rect(fb, fb_w, fb_h, cx - half, cy - half, s as u32, s as u32, color);
            let inset = 2 * (s / 10).max(1);
            ztk_fb_fill_rect(fb, fb_w, fb_h, cx - half + inset, cy - half + inset,
                (s as u32).saturating_sub(inset as u32 * 2),
                (s as u32).saturating_sub(inset as u32 * 2), 0xFF000000);
            ztk_fb_draw_char(fb, fb_w, fb_h, cx - half + inset + 2, cy - half + inset + 1, b'>', color);
            ztk_fb_draw_char(fb, fb_w, fb_h, cx - half + inset + 2, cy - half + inset + 1 + 13, b'_', color);
        }
        1 => { // Calculator: 3x3 grid
            let cell = (s / 4).max(2);
            let gap = (cell / 3).max(1);
            let off = cx - half + gap;
            for row in 0..3 {
                for col in 0..3 {
                    ztk_fb_fill_rect(fb, fb_w, fb_h,
                        off + col * (cell + gap), cy - half + gap + row * (cell + gap),
                        cell as u32, cell as u32, color);
                }
            }
        }
        2 => { // Files: folder shape (two rects)
            let bw = (s * 3 / 4).max(4) as u32;
            let bh = (s * 2 / 3).max(4) as u32;
            let fx = cx - (bw / 2) as i32;
            let fy = cy - (bh / 2) as i32;
            ztk_fb_fill_rect(fb, fb_w, fb_h, fx, fy + (bh / 3) as i32, bw, bh - bh / 3, color);
            ztk_fb_fill_rect(fb, fb_w, fb_h, fx - s / 8, fy, bw + s as u32 / 4, bh / 3, color);
        }
        3 => { // Clock: ring + hands
            let r = s / 3;
            ztk_fb_fill_circle(fb, fb_w, fb_h, cx, cy, r as u32, color);
            ztk_fb_fill_circle(fb, fb_w, fb_h, cx, cy, (r - 2).max(1) as u32, 0xFF000000);
            ztk_fb_hline(fb, fb_w, fb_h, cx, cy, (r / 2) as u32, color);
            let hx = cx + (r / 3);
            let hy = cy - (r / 3);
            ztk_fb_hline(fb, fb_w, fb_h, hx, hy, (r / 3) as u32, color);
        }
        4 => { // Settings: cross + partial circle (gear)
            ztk_fb_fill_circle(fb, fb_w, fb_h, cx, cy, (s / 3) as u32, color);
            ztk_fb_fill_circle(fb, fb_w, fb_h, cx, cy, (s / 5).max(2) as u32, 0xFF000000);
            let arm = (s / 6).max(2);
            for &(dx, dy) in &[(1,0),(-1,0),(0,1),(0,-1)] {
                ztk_fb_fill_rect(fb, fb_w, fb_h,
                    cx + dx * arm - arm / 2, cy + dy * arm - arm / 2,
                    arm as u32, arm as u32, color);
            }
        }
        5 => { // Weather: circle + rays
            ztk_fb_fill_circle(fb, fb_w, fb_h, cx, cy, (s / 4) as u32, color);
            let r = s / 3;
            // Hexagon points around the circle (no math needed)
            let pts: [(i32, i32); 6] = [
                (0, -r), (r, -r/2), (r, r/2),
                (0, r), (-r, r/2), (-r, -r/2),
            ];
            for &(dx, dy) in &pts {
                ztk_fb_fill_circle(fb, fb_w, fb_h, cx + dx, cy + dy, (s / 12).max(1) as u32, color);
            }
        }
        6 => { // Photos: color-noise rectangle
            ztk_fb_fill_rect(fb, fb_w, fb_h, cx - half, cy - half, s as u32, s as u32, color);
            let block = (s / 4).max(2);
            ztk_fb_fill_rect(fb, fb_w, fb_h, cx, cy, block as u32, block as u32, 0xFFFF8844);
            ztk_fb_fill_rect(fb, fb_w, fb_h, cx - block, cy + block, block as u32, block as u32, 0xFF44FF88);
            ztk_fb_fill_rect(fb, fb_w, fb_h, cx + block / 2, cy - block, block as u32, block as u32, 0xFF4488FF);
        }
        7 => { // Editor: horizontal lines between verticals
            let margin = (s / 10).max(2);
            let lx = cx - half + margin;
            let rx = cx + half - margin;
            let mid = (lx + rx) / 2;
            ztk_fb_fill_rect(fb, fb_w, fb_h, lx, cy - half + margin, (rx - lx) as u32, (s / 12).max(1) as u32, color);
            ztk_fb_fill_rect(fb, fb_w, fb_h, lx, cy - 2, (rx - lx) as u32, (s / 12).max(1) as u32, color);
            ztk_fb_fill_rect(fb, fb_w, fb_h, lx, cy + half - margin - (s / 12).max(1), (rx - lx) as u32, (s / 12).max(1) as u32, color);
            ztk_fb_fill_rect(fb, fb_w, fb_h, lx, cy - half + margin, 2, s as u32 - margin as u32 * 2, color);
            ztk_fb_fill_rect(fb, fb_w, fb_h, mid, cy - half + margin, 2, s as u32 - margin as u32 * 2, 0xFF888888);
        }
        8 => { // Snake: stacked rects
            let seg = (s / 5).max(3);
            for i in 0..4 {
                ztk_fb_fill_rect(fb, fb_w, fb_h, cx - half + i * (seg + 1), cy - seg / 2, seg as u32, seg as u32, color);
            }
        }
        9 => { // Pong: two paddles + ball
            let pw = (s / 6).max(2);
            let ph = (s / 3).max(4);
            ztk_fb_fill_rect(fb, fb_w, fb_h, cx - half, cy - ph / 2, pw as u32, ph as u32, color);
            ztk_fb_fill_rect(fb, fb_w, fb_h, cx + half - pw, cy - ph / 2, pw as u32, ph as u32, 0xFFFF4444);
            let bsize = (s / 8).max(2);
            ztk_fb_fill_rect(fb, fb_w, fb_h, cx - bsize / 2, cy - bsize / 2, bsize as u32, bsize as u32, 0xFFFFFFFF);
        }
        10 => { // Tetris: T-tetromino
            let cell = (s / 5).max(3);
            for i in 0..3 {
                ztk_fb_fill_rect(fb, fb_w, fb_h, cx - cell / 2 + i * (cell + 1), cy - cell / 2, cell as u32, cell as u32, color);
            }
            ztk_fb_fill_rect(fb, fb_w, fb_h, cx - cell / 2 + (cell + 1), cy - cell / 2 - (cell + 1), cell as u32, cell as u32, color);
        }
        11 => { // Music: four ~ notes
            ztk_fb_fill_rect(fb, fb_w, fb_h, cx - half, cy - half, 4, s as u32, color);
            ztk_fb_fill_rect(fb, fb_w, fb_h, cx - half + s / 3, cy - half, 4, s as u32, color);
            ztk_fb_fill_rect(fb, fb_w, fb_h, cx - half + s * 2 / 3, cy - half, 4, s as u32, color);
        }
        12 => { // Mail: envelope shape
            let w_env = (s * 4 / 5).max(6);
            let h_env = (s * 3 / 5).max(4);
            let ex = cx - w_env / 2;
            let ey = cy - h_env / 2;
            ztk_fb_fill_rect(fb, fb_w, fb_h, ex, ey, w_env as u32, h_env as u32, color);
            for i in 0..3 {
                let lyric = ex + (i * w_env / 4);
                ztk_fb_hline(fb, fb_w, fb_h, lyric, ey + h_env * (i + 1) / 4, (w_env - i * w_env / 4) as u32, 0xFF000000);
            }
        }
        13 => { // Store: characters S, t, o, r
            let chars = [b'S', b't', b'o', b'r'];
            let cw = (font::FONT_W + 1) as i32;
            let tw = (chars.len() as i32) * cw;
            let start_x = cx - tw / 2;
            for (i, &ch) in chars.iter().enumerate() {
                ztk_fb_draw_char(fb, fb_w, fb_h, start_x + (i as i32) * cw, cy - font::FONT_H as i32 / 2, ch, color);
            }
        }
        14 => { // Maps: two diagonal lines meeting
            let half_s = s / 2;
            let step = (half_s / 8).max(1);
            for i in 0..=8 {
                let px1 = cx - half_s + i * step;
                let py1 = cy + half_s - i * step;
                ztk_fb_set_pixel(fb, fb_w, fb_h, px1, py1, color);
                let px2 = cx + i * step;
                let py2 = cy + half_s - i * step;
                ztk_fb_set_pixel(fb, fb_w, fb_h, px2, py2, color);
            }
        }
        15 => { // About: circle with "i"
            ztk_fb_fill_circle(fb, fb_w, fb_h, cx, cy, (s / 3) as u32, color);
            let dot_r = (s / 16).max(1) as u32;
            ztk_fb_fill_circle(fb, fb_w, fb_h, cx, cy - s / 6, dot_r, 0xFF000000);
            ztk_fb_fill_rect(fb, fb_w, fb_h, cx - dot_r as i32 / 2, cy + s / 12, dot_r, (s / 6) as u32, 0xFF000000);
        }
        _ => {}
    }
}
