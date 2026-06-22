use alloc::vec::Vec;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::rect::Rect;
use crate::widget::{Event, EventResult, Widget};
use crate::font;

pub struct AppInfo {
    pub name: &'static str,
    pub color: Color,
}

pub struct MetroStart {
    pub open: bool,
    pub apps: Vec<AppInfo>,
    pub on_launch: Option<fn(usize)>,
    hover_idx: i32,
    pub screen_w: u32,
    pub screen_h: u32,
}

const TILE: u32 = 140;
const GAP: u32 = 14;
const COLS: usize = 5;
const TITLE_H: u32 = 80;

impl MetroStart {
    pub fn new(w: u32, h: u32) -> Self {
        let mut apps = Vec::new();
        apps.push(AppInfo { name: "Terminal",  color: Color::from_rgb(0, 120, 215) });
        apps.push(AppInfo { name: "Calc",      color: Color::from_rgb(240, 140, 40) });
        apps.push(AppInfo { name: "Settings",  color: Color::from_rgb(120, 120, 120) });
        apps.push(AppInfo { name: "Clock",     color: Color::from_rgb(0, 170, 170) });
        apps.push(AppInfo { name: "Editor",    color: Color::from_rgb(200, 60, 60) });
        apps.push(AppInfo { name: "Files",     color: Color::from_rgb(80, 180, 80) });
        apps.push(AppInfo { name: "Snake",     color: Color::from_rgb(160, 80, 200) });
        apps.push(AppInfo { name: "Pong",      color: Color::from_rgb(220, 60, 140) });
        apps.push(AppInfo { name: "Tetris",    color: Color::from_rgb(220, 200, 40) });
        apps.push(AppInfo { name: "Demo",      color: Color::from_rgb(0, 190, 200) });
        apps.push(AppInfo { name: "Shutdown",  color: Color::from_rgb(200, 40, 40) });
        apps.push(AppInfo { name: "Reboot",    color: Color::from_rgb(200, 160, 40) });
        Self { open: false, apps, on_launch: None, hover_idx: -1, screen_w: w, screen_h: h }
    }

    pub fn is_open(&self) -> bool { self.open }
    pub fn set_open(&mut self, open: bool) { self.open = open; }
    pub fn toggle(&mut self) { self.open = !self.open; self.hover_idx = -1; }

    fn grid_origin(&self) -> (i32, i32) {
        let rows = (self.apps.len() + COLS - 1) / COLS;
        let total_w = COLS as u32 * TILE + (COLS as u32 - 1) * GAP;
        let total_h = rows as u32 * TILE + (rows as u32 - 1) * GAP;
        let gx = (self.screen_w as i32 - total_w as i32) / 2;
        let gy = TITLE_H as i32 + ((self.screen_h as i32 - TITLE_H as i32 - total_h as i32) / 2);
        (gx, gy)
    }

    fn tile_rect(&self, idx: usize, gx: i32, gy: i32) -> Rect {
        let col = idx % COLS;
        let row = idx / COLS;
        Rect::new(
            gx + col as i32 * (TILE + GAP) as i32,
            gy + row as i32 * (TILE + GAP) as i32,
            TILE, TILE,
        )
    }
}

impl Widget for MetroStart {
    fn draw(&self, canvas: &mut Canvas) {
        if !self.open { return; }

        canvas.fill_rect(0, 0, self.screen_w, self.screen_h, Color::from_rgb(30, 30, 30));

        canvas.draw_text_large(44, 28, "Start", Color::from_rgb(255, 255, 255), 3);

        let (gx, gy) = self.grid_origin();

        for (i, app) in self.apps.iter().enumerate() {
            let tr = self.tile_rect(i, gx, gy);
            let is_hover = i as i32 == self.hover_idx;

            let base = if is_hover {
                Color::from_argb(60, 255, 255, 255)
            } else {
                Color::from_argb(0, 0, 0, 0)
            };
            if is_hover {
                canvas.fill_rect(tr.x - 2, tr.y - 2, tr.w + 4, tr.h + 4, base);
            }

            canvas.fill_rect(tr.x, tr.y, tr.w, tr.h, app.color);

            let name = app.name;
            let tw = (name.len() as u32) * (font::FONT_W + 1);
            let tx = tr.x + (tr.w as i32 - tw as i32) / 2;
            let ty = tr.y + tr.h as i32 - font::FONT_H as i32 - 6;
            canvas.draw_text(tx, ty, name, Color::from_rgb(255, 255, 255));
        }
    }

    fn rect(&self) -> Rect { Rect::new(0, 0, self.screen_w, self.screen_h) }
    fn min_size(&self) -> (u32, u32) { (self.screen_w, self.screen_h) }
    fn set_pos(&mut self, _x: i32, _y: i32) {}
    fn set_size(&mut self, w: u32, h: u32) { self.screen_w = w; self.screen_h = h; }

    fn handle_event(&mut self, ev: &Event, _parent: Rect) -> EventResult {
        if !self.open { return EventResult::Ignored; }
        let (gx, gy) = self.grid_origin();

        match *ev {
            Event::MouseMove { x, y } => {
                let old = self.hover_idx;
                self.hover_idx = -1;
                for i in 0..self.apps.len() {
                    if self.tile_rect(i, gx, gy).contains(x, y) {
                        self.hover_idx = i as i32;
                        break;
                    }
                }
                if old != self.hover_idx { EventResult::Redraw } else { EventResult::Ignored }
            }
            Event::MouseUp { x, y, .. } => {
                for i in 0..self.apps.len() {
                    if self.tile_rect(i, gx, gy).contains(x, y) {
                        self.open = false;
                        if let Some(cb) = self.on_launch { cb(i); }
                        return EventResult::Handled;
                    }
                }
                if y < gy - 20 || x < gx || x > gx + (COLS as i32 * (TILE + GAP) as i32 - GAP as i32) || y > gy + ((self.apps.len() as u32 / COLS as u32 + 1) * (TILE + GAP) - GAP) as i32 {
                    self.open = false;
                    return EventResult::Redraw;
                }
                EventResult::Ignored
            }
            _ => EventResult::Ignored,
        }
    }
}
