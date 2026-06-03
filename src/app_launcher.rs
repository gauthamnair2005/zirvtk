use alloc::vec::Vec;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::fx;
use crate::rect::Rect;
use crate::widget::{Event, EventResult, Widget};
use crate::font;

pub struct AppInfo {
    pub name: &'static str,
    pub icon: u32,
    pub color: Color,
}

pub struct AppLauncher {
    rect: Rect,
    pub apps: Vec<AppInfo>,
    pub on_launch: Option<fn(usize)>,
    hover_idx: i32,
    open: bool,
    time_ms: u32,
}

const COLS: usize = 4;
const CELL_W: u32 = 120;
const CELL_H: u32 = 130;
const GAP: u32 = 16;

impl AppLauncher {
    fn panel_size(app_count: usize) -> (u32, u32) {
        let rows = (app_count + COLS - 1) / COLS;
        let inner_w = COLS as u32 * CELL_W + (COLS as u32 - 1) * GAP;
        let inner_h = rows as u32 * CELL_H + (rows as u32 - 1) * GAP;
        let pw = inner_w + 40;
        let ph = inner_h + 80;
        (pw, ph)
    }

    pub fn new() -> Self {
        let mut apps = Vec::new();
        apps.push(AppInfo { name: "Terminal", icon: 0, color: Color::from_rgb(0, 200, 255) });
        apps.push(AppInfo { name: "Calculator", icon: 1, color: Color::from_rgb(180, 80, 255) });
        apps.push(AppInfo { name: "Settings", icon: 2, color: Color::from_rgb(0, 255, 136) });
        apps.push(AppInfo { name: "Clock", icon: 3, color: Color::from_rgb(255, 200, 50) });
        apps.push(AppInfo { name: "Editor", icon: 4, color: Color::from_rgb(255, 80, 100) });
        apps.push(AppInfo { name: "Files", icon: 8, color: Color::from_rgb(50, 200, 255) });
        apps.push(AppInfo { name: "Snake", icon: 5, color: Color::from_rgb(50, 255, 100) });
        apps.push(AppInfo { name: "Pong", icon: 6, color: Color::from_rgb(255, 160, 50) });
        apps.push(AppInfo { name: "Tetris", icon: 7, color: Color::from_rgb(255, 60, 120) });
        apps.push(AppInfo { name: "Demo", icon: 9, color: Color::from_rgb(180, 130, 255) });
        let (pw, ph) = Self::panel_size(apps.len());
        Self {
            rect: Rect::new(0, 0, pw, ph),
            apps,
            on_launch: None,
            hover_idx: -1,
            open: false,
            time_ms: 0,
        }
    }

    pub fn update(&mut self, time_ms: u32) {
        self.time_ms = time_ms;
    }

    pub fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    pub fn is_open(&self) -> bool { self.open }

    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    fn app_rect(&self, idx: usize) -> Rect {
        let row = (idx / COLS) as i32;
        let col = (idx % COLS) as i32;
        let total_w = COLS as u32 * CELL_W + (COLS as u32 - 1) * GAP;
        let inner_x = self.rect.x + 20 + (self.rect.w - 40 - total_w) as i32 / 2;
        Rect::new(
            inner_x + col * (CELL_W + GAP) as i32,
            self.rect.y + 60 + row * (CELL_H + GAP) as i32,
            CELL_W, CELL_H,
        )
    }
}

impl Widget for AppLauncher {
    fn draw(&self, canvas: &mut Canvas) {
        if !self.open { return; }

        fx::glass_panel(canvas,
            self.rect.x, self.rect.y, self.rect.w, self.rect.h,
            16, Color::from_argb(200, 10, 10, 20), 220,
            Color::from_rgb(0, 212, 255), 0);

        let cx = self.rect.x + self.rect.w as i32 / 2;
        canvas.draw_text_large(cx - 48, self.rect.y + 12, "Apps", Color::from_rgb(240, 240, 255), 2);

        for (i, app) in self.apps.iter().enumerate() {
            let ar = self.app_rect(i);
            let is_hover = i as i32 == self.hover_idx;

            if is_hover {
                let pulse = 0.7 + 0.3 * libm::sinf(self.time_ms as f32 * 0.004);
                let fill_alpha = (50.0 * pulse) as u8;
                let border_alpha = (180.0 * pulse) as u8;
                canvas.fill_round_rect(ar.x, ar.y, ar.w, ar.h, 8,
                    Color::from_argb(fill_alpha.max(30), app.color.r(), app.color.g(), app.color.b()));
                canvas.stroke_round_rect(ar.x, ar.y, ar.w, ar.h, 8,
                    Color::from_argb(border_alpha, app.color.r(), app.color.g(), app.color.b()));
            } else {
                canvas.fill_round_rect(ar.x, ar.y, ar.w, ar.h, 8,
                    Color::from_argb(25, 60, 60, 100));
                canvas.stroke_round_rect(ar.x, ar.y, ar.w, ar.h, 8,
                    Color::from_argb(40, app.color.r(), app.color.g(), app.color.b()));
            }

            let icon_cy = ar.y + ar.h as i32 / 2 - 20;
            crate::rawfb::draw_icon_canvas(canvas, ar.x + ar.w as i32 / 2, icon_cy, 36, app.icon, app.color);

            let tw = (app.name.len() as u32) * (font::FONT_W + 1);
            let tx = ar.x + (ar.w as i32 - tw as i32) / 2;
            canvas.draw_text(tx, icon_cy + 28, app.name, Color::from_rgb(200, 200, 230));
        }
    }

    fn rect(&self) -> Rect { self.rect }
    fn min_size(&self) -> (u32, u32) { Self::panel_size(self.apps.len()) }

    fn set_pos(&mut self, x: i32, y: i32) { self.rect.x = x; self.rect.y = y; }
    fn set_size(&mut self, w: u32, h: u32) { self.rect.w = w; self.rect.h = h; }

    fn handle_event(&mut self, ev: &Event, parent: Rect) -> EventResult {
        if !self.open { return EventResult::Ignored; }
        let abs = Rect::new(parent.x + self.rect.x, parent.y + self.rect.y, self.rect.w, self.rect.h);

        match *ev {
            Event::MouseMove { x, y } => {
                let old_hover = self.hover_idx;
                self.hover_idx = -1;
                for i in 0..self.apps.len() {
                    let ar = Rect::new(
                        abs.x + self.app_rect(i).x - self.rect.x,
                        abs.y + self.app_rect(i).y - self.rect.y,
                        self.app_rect(i).w, self.app_rect(i).h,
                    );
                    if ar.contains(x, y) {
                        self.hover_idx = i as i32;
                        break;
                    }
                }
                if old_hover != self.hover_idx { EventResult::Redraw } else { EventResult::Ignored }
            }
            Event::MouseUp { x, y, .. } => {
                for i in 0..self.apps.len() {
                    let ar = Rect::new(
                        abs.x + self.app_rect(i).x - self.rect.x,
                        abs.y + self.app_rect(i).y - self.rect.y,
                        self.app_rect(i).w, self.app_rect(i).h,
                    );
                    if ar.contains(x, y) {
                        self.open = false;
                        if let Some(cb) = self.on_launch {
                            cb(i);
                        }
                        return EventResult::Handled;
                    }
                }
                if !abs.contains(x, y) {
                    self.open = false;
                    return EventResult::Redraw;
                }
                EventResult::Ignored
            }
            _ => EventResult::Ignored,
        }
    }
}
