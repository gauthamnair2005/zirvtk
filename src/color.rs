/// RGBA colour packed as `0xAABBGGRR` (matching DisplayJet / ZirvUI convention).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color(u32);

impl Color {
    pub const BLACK: Self = Self(0xFF000000);
    pub const WHITE: Self = Self(0xFFFFFFFF);
    pub const RED: Self = Self(0xFFFF0000);
    pub const GREEN: Self = Self(0xFF00FF00);
    pub const BLUE: Self = Self(0xFF0000FF);

    pub const fn from_argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self((a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32)
    }

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from_argb(0xFF, r, g, b)
    }

    pub const fn from_u32(val: u32) -> Self {
        Self(val)
    }

    pub const fn to_u32(self) -> u32 {
        self.0
    }

    pub fn r(self) -> u8 {
        (self.0 >> 16) as u8
    }
    pub fn g(self) -> u8 {
        (self.0 >> 8) as u8
    }
    pub fn b(self) -> u8 {
        self.0 as u8
    }
    pub fn a(self) -> u8 {
        (self.0 >> 24) as u8
    }

    pub fn blend(self, bg: Color, alpha: u8) -> Color {
        let a = alpha as u32;
        let inv = 255 - a;
        let r = (self.r() as u32 * a + bg.r() as u32 * inv) / 255;
        let g = (self.g() as u32 * a + bg.g() as u32 * inv) / 255;
        let b = (self.b() as u32 * a + bg.b() as u32 * inv) / 255;
        Color::from_argb(0xFF, r as u8, g as u8, b as u8)
    }
}
