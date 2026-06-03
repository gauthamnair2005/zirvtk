use alloc::vec;
use alloc::vec::Vec;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::fx;

#[derive(Clone)]
pub struct Particle {
    pub x: f32, pub y: f32,
    pub vx: f32, pub vy: f32,
    pub life: f32, pub max_life: f32,
    pub color: Color,
    pub size: f32,
    pub glow: bool,
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub emission_rate: u32,
    pub max_particles: usize,
    pub spawn_timer: u32,
    pub bounds_w: u32,
    pub bounds_h: u32,
    pub colors: Vec<Color>,
}

impl ParticleSystem {
    pub fn new(max: usize, w: u32, h: u32) -> Self {
        Self {
            particles: Vec::with_capacity(max),
            emission_rate: 2,
            max_particles: max,
            spawn_timer: 0,
            bounds_w: w,
            bounds_h: h,
            colors: vec![
                Color::from_rgb(0, 212, 255),
                Color::from_rgb(123, 47, 247),
                Color::from_rgb(0, 255, 136),
                Color::from_rgb(255, 170, 0),
            ],
        }
    }

    pub fn update(&mut self, dt_ms: u32) {
        self.spawn_timer += dt_ms;
        while self.spawn_timer >= 50 && self.particles.len() < self.max_particles {
            self.spawn_timer -= 50;
            let i = (self.particles.len() * 7 + 13) % self.colors.len();
            let color = self.colors[i];
            let t = self.particles.len() as f32 * 0.6180339887;
            let x = (self.bounds_w as f32) * (t - (t as i32) as f32);
            let y = self.bounds_h as f32 + 10.0;
            let vx = libm::cosf(self.particles.len() as f32 * 1.1) * 0.3;
            let vy = -(1.0 + libm::sinf(self.particles.len() as f32 * 0.7) * 0.5);
            self.particles.push(Particle {
                x, y, vx, vy,
                life: 1.0, max_life: 1.0,
                color, size: 1.5 + (self.particles.len() % 3) as f32 * 0.5,
                glow: self.particles.len() % 2 == 0,
            });
        }

        let dt = dt_ms as f32 * 0.001;
        self.particles.retain_mut(|p| {
            p.x += p.vx * dt * 60.0;
            p.y += p.vy * dt * 60.0;
            p.vy -= 0.02 * dt * 60.0;
            p.vx += libm::sinf(p.y * 0.01) * 0.01;
            p.life -= dt * 0.15;
            p.max_life > 0.0 && p.life > 0.0 && p.y > -20.0 && p.x > -20.0 && p.x < self.bounds_w as f32 + 20.0
        });
    }

    pub fn draw(&self, canvas: &mut Canvas, _time_ms: u32) {
        for p in &self.particles {
            let alpha = (p.life * 255.0) as u8;
            let color = Color::from_argb(alpha, p.color.r(), p.color.g(), p.color.b());
            let r = p.size as u32;
            if r < 1 { continue; }
            if false && p.glow {
                let glow_a = (alpha as u32 * 80 / 255) as u8;
                fx::glow(canvas, p.x as i32, p.y as i32, 8, p.color, glow_a);
            }
            canvas.fill_circle(p.x as i32, p.y as i32, r, color);
        }
    }

    pub fn burst(&mut self, cx: f32, cy: f32, count: u32) {
        for i in 0..count {
            if self.particles.len() >= self.max_particles { break; }
            let angle = (i as f32 * 6.28 / count as f32) + 0.1;
            let speed = 1.0 + libm::sinf(i as f32 * 0.3) * 0.5;
            let ci = (i as usize) % self.colors.len();
            self.particles.push(Particle {
                x: cx, y: cy,
                vx: libm::cosf(angle) * speed,
                vy: libm::sinf(angle) * speed - 1.0,
                life: 1.0, max_life: 1.0,
                color: self.colors[ci],
                size: 2.0,
                glow: true,
            });
        }
    }
}
