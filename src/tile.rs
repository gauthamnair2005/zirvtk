use crate::canvas::Canvas;
use crate::color::Color;
use crate::font;
use crate::rect::Rect;
use crate::widget::{Event, EventResult, Widget};

pub struct MediaTile {
    rect: Rect,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub icon_label: &'static str,
    bg: Color,
    hovered: bool,
    pressed: bool,
}

impl MediaTile {
    pub fn new(title: &'static str, subtitle: &'static str,
               icon_label: &'static str, bg: Color) -> Self {
        Self {
            rect: Rect::new(0, 0, 220, 260),
            title, subtitle, icon_label, bg,
            hovered: false, pressed: false,
        }
    }
}

impl Widget for MediaTile {
    fn rect(&self) -> Rect { self.rect }
    fn min_size(&self) -> (u32, u32) { (200, 240) }
    fn set_pos(&mut self, x: i32, y: i32) { self.rect.x = x; self.rect.y = y; }
    fn set_size(&mut self, w: u32, h: u32) { self.rect.w = w; self.rect.h = h; }

    fn draw(&self, canvas: &mut Canvas) {
        let r = self.rect;
        let tile_w = r.w;
        let tile_h = r.h;
        let radius = 14;

        canvas.draw_shadow(r.x - 2, r.y - 2, tile_w + 4, tile_h + 4, 6);

        let bg = if self.hovered {
            Color::from_rgb(
                (self.bg.r() as u32 + 50).min(255) as u8,
                (self.bg.g() as u32 + 50).min(255) as u8,
                (self.bg.b() as u32 + 50).min(255) as u8)
        } else { self.bg };

        canvas.fill_round_rect(r.x, r.y, tile_w, tile_h, radius, bg);
        canvas.fill_glass_rect(r.x, r.y, tile_w, tile_h / 3, Color::WHITE);

        let icon_size = (tile_h / 5).max(32).min(64);
        let cx = r.x + tile_w as i32 / 2;
        let cy = r.y + tile_h as i32 / 4;

        canvas.fill_circle(cx, cy, icon_size / 2, Color::from_u32(0x33FFFFFF));
        let ilw = font::text_width(self.icon_label) as i32 * 3;
        canvas.draw_text_large(cx - ilw / 2, cy - 10, self.icon_label, Color::from_u32(0xAAFFFFFF), 3);

        let title_y = r.y + tile_h as i32 * 3 / 5;
        let tw = font::text_width(self.title) as i32 * 2;
        canvas.draw_text_large(r.x + (tile_w as i32 - tw) / 2, title_y, self.title, Color::WHITE, 2);

        let st_y = title_y + font::FONT_H as i32 * 2 + 6;
        let stw = font::text_width(self.subtitle) as i32;
        canvas.draw_text(r.x + (tile_w as i32 - stw) / 2, st_y, self.subtitle, Color::from_u32(0xFF99AACC));

        canvas.stroke_round_rect(r.x, r.y, tile_w, tile_h, radius, Color::from_u32(0x44FFFFFF));
    }

    fn handle_event(&mut self, ev: &Event, _parent: Rect) -> EventResult {
        let r = self.rect;
        match *ev {
            Event::MouseMove { x, y } => {
                let was = self.hovered;
                self.hovered = r.contains(x, y);
                if was != self.hovered { EventResult::Redraw } else { EventResult::Ignored }
            }
            Event::MouseDown { x, y, .. } => {
                if r.contains(x, y) { self.pressed = true; EventResult::Redraw }
                else { EventResult::Ignored }
            }
            Event::MouseUp { x, y, .. } => {
                let was = self.pressed;
                self.pressed = false;
                if was && r.contains(x, y) { EventResult::Handled }
                else if was { EventResult::Redraw }
                else { EventResult::Ignored }
            }
        }
    }
}
