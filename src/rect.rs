use core::default::Default;

/// 2D rectangle (integer coordinates).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Default for Rect {
    fn default() -> Self { Self { x: 0, y: 0, w: 0, h: 0 } }
}

impl Rect {
    pub const fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(self, px: i32, py: i32) -> bool {
        px >= self.x && py >= self.y && px < self.x + self.w as i32 && py < self.y + self.h as i32
    }

    pub fn right(self) -> i32 {
        self.x + self.w as i32
    }
    pub fn bottom(self) -> i32 {
        self.y + self.h as i32
    }
}
