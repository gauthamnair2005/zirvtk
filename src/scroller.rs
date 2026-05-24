use alloc::vec::Vec;
use alloc::boxed::Box;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::font;
use crate::rect::Rect;
use crate::widget::{Event, EventResult, Widget};
use crate::tile::MediaTile;

pub struct TileScroller {
    rect: Rect,
    tiles: Vec<Box<MediaTile>>,
    pub scroll_offset: i32,
    target_offset: i32,
    tile_w: u32,
    tile_h: u32,
    gap: i32,
    dragging: bool,
    drag_start_x: i32,
    drag_start_offset: i32,
    pub title: &'static str,
}

impl TileScroller {
    pub fn new(title: &'static str) -> Self {
        Self {
            rect: Rect::new(0, 0, 800, 320),
            tiles: Vec::new(),
            scroll_offset: 0,
            target_offset: 0,
            tile_w: 200,
            tile_h: 260,
            gap: 24,
            dragging: false,
            drag_start_x: 0,
            drag_start_offset: 0,
            title,
        }
    }

    pub fn add_tile(&mut self, tile: MediaTile) {
        self.tiles.push(Box::new(tile));
    }

    fn layout_tiles(&mut self) {
        let start_x = self.rect.x + 40 + self.scroll_offset;
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            let tx = start_x + i as i32 * (self.tile_w as i32 + self.gap);
            let ty = self.rect.y + 50;
            tile.set_pos(tx, ty);
            tile.set_size(self.tile_w, self.tile_h);
        }
    }

    fn total_width(&self) -> i32 {
        self.tiles.len() as i32 * (self.tile_w as i32 + self.gap) - self.gap + 80
    }

    pub fn set_tile_size(&mut self, w: u32, h: u32) { self.tile_w = w; self.tile_h = h; }
}

impl Widget for TileScroller {
    fn rect(&self) -> Rect { self.rect }
    fn min_size(&self) -> (u32, u32) { (400, 300) }
    fn set_pos(&mut self, x: i32, y: i32) { self.rect.x = x; self.rect.y = y; }
    fn set_size(&mut self, w: u32, h: u32) { self.rect.w = w; self.rect.h = h; }

    fn draw(&self, canvas: &mut Canvas) {
        let r = self.rect;

        canvas.draw_text_large(r.x + 20, r.y + 8, self.title, Color::from_u32(0xFFE8E8F0), 2);

        let line_y = r.y + 36;
        canvas.fill_rect(r.x + 20, line_y, 60, 3, Color::from_u32(0xFF6699CC));

        for tile in &self.tiles {
            tile.draw(canvas);
        }

        let vis_w = r.w as i32;
        let total = self.total_width();
        if total > vis_w {
            let bar_w = (vis_w * vis_w / total).max(40);
            let bar_x = r.x + (self.scroll_offset * (vis_w - bar_w) / (total - vis_w)).max(0);
            let bar_y = r.y + r.h as i32 - 12;
            canvas.fill_rect(bar_x, bar_y, bar_w as u32, 4, Color::from_u32(0x88FFFFFF));
        }
    }

    fn handle_event(&mut self, ev: &Event, _parent: Rect) -> EventResult {
        let r = self.rect;
        match *ev {
            Event::MouseMove { x, y } => {
                if self.dragging {
                    let dx = x - self.drag_start_x;
                    let new_off = self.drag_start_offset + dx;
                    let max_off = 0;
                    let min_off = -(self.total_width() - r.w as i32).max(0);
                    self.scroll_offset = new_off.max(min_off).min(max_off);
                    self.layout_tiles();
                    return EventResult::Redraw;
                }
                let mut result = EventResult::Ignored;
                for tile in self.tiles.iter_mut() {
                    if tile.handle_event(ev, r) != EventResult::Ignored {
                        result = EventResult::Redraw;
                    }
                }
                result
            }
            Event::MouseDown { x, y, .. } => {
                if r.contains(x, y) {
                    for tile in self.tiles.iter_mut() {
                        if tile.handle_event(ev, r) != EventResult::Ignored {
                            return EventResult::Redraw;
                        }
                    }
                    self.dragging = true;
                    self.drag_start_x = x;
                    self.drag_start_offset = self.scroll_offset;
                    return EventResult::Redraw;
                }
                EventResult::Ignored
            }
            Event::MouseUp { x, y, .. } => {
                if self.dragging {
                    self.dragging = false;
                    return EventResult::Redraw;
                }
                let mut result = EventResult::Ignored;
                for tile in self.tiles.iter_mut() {
                    if tile.handle_event(ev, r) != EventResult::Ignored {
                        result = EventResult::Redraw;
                    }
                }
                result
            }
        }
    }
}
