/// 3D object primitives for wireframe rendering.
///
/// The basic shapes (Cube, Tetrahedron) work without `alloc`.
/// Tessellated shapes (Sphere, Cylinder, Torus) require `feature = "alloc"`.

use crate::color::Color;

/// A 3D vertex in object space.
#[derive(Clone, Copy, Debug)]
pub struct Vertex3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// An edge connecting two vertices by index.
#[derive(Clone, Copy, Debug)]
pub struct Edge {
    pub a: usize,
    pub b: usize,
}

/// A face defined by three vertex indices.
#[derive(Clone, Copy, Debug)]
pub struct TriFace {
    pub v0: usize,
    pub v1: usize,
    pub v2: usize,
}

/// A 3D wireframe mesh.
pub struct Mesh3D {
    pub vertices: &'static [Vertex3],
    pub edges: &'static [Edge],
    pub faces: &'static [TriFace],
}

// ── 4x4 column-major matrix ─────────────────────────────────────────────
#[derive(Clone, Copy, Debug)]
pub struct Mat4(pub [f32; 16]);

impl Mat4 {
    pub fn identity() -> Self {
        let mut m = [0.0f32; 16];
        m[0] = 1.0; m[5] = 1.0; m[10] = 1.0; m[15] = 1.0;
        Mat4(m)
    }

    pub fn perspective(fov_rad: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / libm::tanf(fov_rad * 0.5);
        let range_inv = 1.0 / (near - far);
        let mut m = [0.0f32; 16];
        m[0] = f / aspect;
        m[5] = f;
        m[10] = (near + far) * range_inv;
        m[11] = -1.0;
        m[14] = 2.0 * near * far * range_inv;
        Mat4(m)
    }

    pub fn look_at(eye: [f32; 3], center: [f32; 3], up: [f32; 3]) -> Self {
        let f = normalize(sub(center, eye));
        let s = normalize(cross(f, up));
        let u = cross(s, f);
        let mut m = [0.0f32; 16];
        m[0] = s[0]; m[4] = s[1]; m[8]  = s[2];
        m[1] = u[0]; m[5] = u[1]; m[9]  = u[2];
        m[2] = -f[0]; m[6] = -f[1]; m[10] = -f[2];
        m[12] = -dot(s, eye);
        m[13] = -dot(u, eye);
        m[14] =  dot(f, eye);
        m[15] = 1.0;
        Mat4(m)
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        let mut m = Self::identity();
        m.0[12] = x; m.0[13] = y; m.0[14] = z;
        m
    }

    pub fn rotate_y(angle: f32) -> Self {
        let (s, c) = (libm::sinf(angle), libm::cosf(angle));
        let mut m = Self::identity();
        m.0[5] = c;  m.0[9]  = -s;
        m.0[6] = s;  m.0[10] = c;
        m
    }

    /// Multiply self * other.
    pub fn mul(&self, rhs: &Mat4) -> Mat4 {
        let mut r = [0.0f32; 16];
        for col in 0..4 {
            for row in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.0[k * 4 + row] * rhs.0[col * 4 + k];
                }
                r[col * 4 + row] = sum;
            }
        }
        Mat4(r)
    }

    /// Transform a 3D point (w=1) through the matrix.
    pub fn transform(&self, p: [f32; 3]) -> [f32; 4] {
        let m = self.0;
        [
            m[0] * p[0] + m[4] * p[1] + m[8]  * p[2] + m[12],
            m[1] * p[0] + m[5] * p[1] + m[9]  * p[2] + m[13],
            m[2] * p[0] + m[6] * p[1] + m[10] * p[2] + m[14],
            m[3] * p[0] + m[7] * p[1] + m[11] * p[2] + m[15],
        ]
    }
}

// ── Vector helpers ──────────────────────────────────────────────────────
fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn normalize(v: [f32; 3]) -> [f32; 3] {
    let len = libm::sqrtf(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
    if len < 1e-8 { return [0.0; 3]; }
    [v[0] / len, v[1] / len, v[2] / len]
}

// ── Built-in meshes ─────────────────────────────────────────────────────

/// A unit cube centered at origin.
pub fn mesh_cube() -> Mesh3D {
    const VERTS: &[Vertex3] = &[
        Vertex3 { x: -0.5, y: -0.5, z: -0.5 },
        Vertex3 { x:  0.5, y: -0.5, z: -0.5 },
        Vertex3 { x:  0.5, y: -0.5, z:  0.5 },
        Vertex3 { x: -0.5, y: -0.5, z:  0.5 },
        Vertex3 { x: -0.5, y:  0.5, z: -0.5 },
        Vertex3 { x:  0.5, y:  0.5, z: -0.5 },
        Vertex3 { x:  0.5, y:  0.5, z:  0.5 },
        Vertex3 { x: -0.5, y:  0.5, z:  0.5 },
    ];
    const EDGES: &[Edge] = &[
        Edge { a: 0, b: 1 }, Edge { a: 1, b: 2 }, Edge { a: 2, b: 3 }, Edge { a: 3, b: 0 },
        Edge { a: 4, b: 5 }, Edge { a: 5, b: 6 }, Edge { a: 6, b: 7 }, Edge { a: 7, b: 4 },
        Edge { a: 0, b: 4 }, Edge { a: 1, b: 5 }, Edge { a: 2, b: 6 }, Edge { a: 3, b: 7 },
    ];
    const FACES: &[TriFace] = &[
        TriFace { v0: 0, v1: 1, v2: 2 }, TriFace { v0: 0, v1: 2, v2: 3 },
        TriFace { v0: 4, v1: 5, v2: 6 }, TriFace { v0: 4, v1: 6, v2: 7 },
        TriFace { v0: 0, v1: 1, v2: 5 }, TriFace { v0: 0, v1: 5, v2: 4 },
        TriFace { v0: 2, v1: 3, v2: 7 }, TriFace { v0: 2, v1: 7, v2: 6 },
        TriFace { v0: 0, v1: 3, v2: 7 }, TriFace { v0: 0, v1: 7, v2: 4 },
        TriFace { v0: 1, v1: 2, v2: 6 }, TriFace { v0: 1, v1: 6, v2: 5 },
    ];
    Mesh3D { vertices: VERTS, edges: EDGES, faces: FACES }
}

/// A regular tetrahedron centered at origin.
pub fn mesh_tetrahedron() -> Mesh3D {
    const VERTS: &[Vertex3] = &[
        Vertex3 { x:  0.5, y:  0.5, z:  0.5 },
        Vertex3 { x: -0.5, y: -0.5, z:  0.5 },
        Vertex3 { x: -0.5, y:  0.5, z: -0.5 },
        Vertex3 { x:  0.5, y: -0.5, z: -0.5 },
    ];
    const EDGES: &[Edge] = &[
        Edge { a: 0, b: 1 }, Edge { a: 0, b: 2 }, Edge { a: 0, b: 3 },
        Edge { a: 1, b: 2 }, Edge { a: 1, b: 3 }, Edge { a: 2, b: 3 },
    ];
    const FACES: &[TriFace] = &[];
    Mesh3D { vertices: VERTS, edges: EDGES, faces: FACES }
}

// ── Tessellated shapes (require alloc) ─────────────────────────────────

#[cfg(feature = "alloc")]
mod tessellated {
    use alloc::vec::Vec;
    use super::{Vertex3, Edge, TriFace};

    pub fn sphere(radius: f32, slices: usize, stacks: usize) -> (Vec<Vertex3>, Vec<Edge>, Vec<TriFace>) {
        let mut verts = Vec::new();
        let mut edges = Vec::new();
        let mut faces = Vec::new();

        for j in 0..=stacks {
            let theta = core::f32::consts::PI * j as f32 / stacks as f32;
            let sin_theta = libm::sinf(theta);
            let cos_theta = libm::cosf(theta);
            for i in 0..=slices {
                let phi = 2.0 * core::f32::consts::PI * i as f32 / slices as f32;
                let x = radius * sin_theta * libm::cosf(phi);
                let y = radius * cos_theta;
                let z = radius * sin_theta * libm::sinf(phi);
                verts.push(Vertex3 { x, y, z });
            }
        }

        for j in 0..stacks {
            for i in 0..slices {
                let a = j * (slices + 1) + i;
                let b = a + slices + 1;
                edges.push(Edge { a, b: a + 1 });
                edges.push(Edge { a: a + 1, b: b + 1 });
                edges.push(Edge { a, b });
                edges.push(Edge { a: b, b: b + 1 });
                faces.push(TriFace { v0: a, v1: a + 1, v2: b });
                faces.push(TriFace { v0: a + 1, v1: b + 1, v2: b });
            }
        }

        (verts, edges, faces)
    }

    pub fn cylinder(radius: f32, height: f32, slices: usize) -> (Vec<Vertex3>, Vec<Edge>, Vec<TriFace>) {
        let mut verts = Vec::new();
        let mut edges = Vec::new();
        let mut faces = Vec::new();
        let hh = height * 0.5;

        verts.push(Vertex3 { x: 0.0, y: -hh, z: 0.0 });
        verts.push(Vertex3 { x: 0.0, y:  hh, z: 0.0 });

        for i in 0..slices {
            let a = 2.0 * core::f32::consts::PI * i as f32 / slices as f32;
            let x = radius * libm::cosf(a);
            let z = radius * libm::sinf(a);
            verts.push(Vertex3 { x, y: -hh, z });
        }
        for i in 0..slices {
            let a = 2.0 * core::f32::consts::PI * i as f32 / slices as f32;
            let x = radius * libm::cosf(a);
            let z = radius * libm::sinf(a);
            verts.push(Vertex3 { x, y: hh, z });
        }

        let ring_bot = 2;
        let ring_top = 2 + slices;

        for i in 0..slices {
            let next = (i + 1) % slices;
            edges.push(Edge { a: ring_bot + i, b: ring_bot + next });
            edges.push(Edge { a: ring_top + i, b: ring_top + next });
            edges.push(Edge { a: ring_bot + i, b: ring_top + i });
            faces.push(TriFace { v0: 0, v1: ring_bot + i, v2: ring_bot + next });
            faces.push(TriFace { v0: 1, v1: ring_top + next, v2: ring_top + i });
            faces.push(TriFace { v0: ring_bot + i, v1: ring_top + i, v2: ring_top + next });
            faces.push(TriFace { v0: ring_bot + i, v1: ring_top + next, v2: ring_bot + next });
        }

        (verts, edges, faces)
    }

    pub fn torus(major_r: f32, minor_r: f32, major_seg: usize, minor_seg: usize) -> (Vec<Vertex3>, Vec<Edge>, Vec<TriFace>) {
        let mut verts = Vec::new();
        let mut edges = Vec::new();
        let mut faces = Vec::new();

        for j in 0..=major_seg {
            let theta = 2.0 * core::f32::consts::PI * j as f32 / major_seg as f32;
            let ct = libm::cosf(theta);
            let st = libm::sinf(theta);
            for i in 0..=minor_seg {
                let phi = 2.0 * core::f32::consts::PI * i as f32 / minor_seg as f32;
                let x = (major_r + minor_r * libm::cosf(phi)) * ct;
                let y = minor_r * libm::sinf(phi);
                let z = (major_r + minor_r * libm::cosf(phi)) * st;
                verts.push(Vertex3 { x, y, z });
            }
        }

        for j in 0..major_seg {
            for i in 0..minor_seg {
                let a = j * (minor_seg + 1) + i;
                let b = a + minor_seg + 1;
                edges.push(Edge { a, b: a + 1 });
                edges.push(Edge { a, b });
                edges.push(Edge { a: a + 1, b: b + 1 });
                edges.push(Edge { a: b, b: b + 1 });
                faces.push(TriFace { v0: a, v1: a + 1, v2: b });
                faces.push(TriFace { v0: a + 1, v1: b + 1, v2: b });
            }
        }

        (verts, edges, faces)
    }
}

#[cfg(feature = "alloc")]
pub use tessellated::*;

// ── Rendering ───────────────────────────────────────────────────────────

/// Draw a wireframe mesh into a raw pixel buffer using the given MVP matrix.
///
/// `fb` points to a `width × height` RGBA pixel buffer.
/// Face filling is not performed in wireframe mode.
#[cfg(feature = "alloc")]
pub fn draw_mesh_wireframe(
    fb: &mut [u32],
    width: u32,
    height: u32,
    mesh: &Mesh3D,
    mvp: &Mat4,
    color: Color,
) {
    use alloc::vec::Vec;
    let c = color.to_u32();
    let half_w = width as f32 * 0.5;
    let half_h = height as f32 * 0.5;

    let mut screen: Vec<[f32; 2]> = Vec::with_capacity(mesh.vertices.len());
    for v in mesh.vertices {
        let h = mvp.transform([v.x, v.y, v.z]);
        let sx = (h[0] / h[3]) * half_w + half_w;
        let sy = -(h[1] / h[3]) * half_h + half_h;
        screen.push([sx, sy]);
    }

    for edge in mesh.edges {
        let a = screen[edge.a];
        let b = screen[edge.b];
        rasterize_line(fb, width, height, a[0], a[1], b[0], b[1], c);
    }
}

#[cfg(feature = "alloc")]
fn rasterize_line(
    fb: &mut [u32],
    w: u32,
    h: u32,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    color: u32,
) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let steps = dx.max(dy) as i32;
    if steps == 0 {
        let ix = (x0 + 0.5) as i32;
        let iy = (y0 + 0.5) as i32;
        if ix >= 0 && iy >= 0 && ix < w as i32 && iy < h as i32 {
            fb[(iy as usize) * (w as usize) + (ix as usize)] = color;
        }
        return;
    }
    let sx = if x0 < x1 { 1.0 } else { -1.0 };
    let sy = if y0 < y1 { 1.0 } else { -1.0 };
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let x = x0 + dx * sx * t;
        let y = y0 + dy * sy * t;
        let ix = (x + 0.5) as i32;
        let iy = (y + 0.5) as i32;
        if ix >= 0 && iy >= 0 && ix < w as i32 && iy < h as i32 {
            fb[(iy as usize) * (w as usize) + (ix as usize)] = color;
        }
    }
}

/// Draw a filled mesh (triangles) into a raw pixel buffer with a depth buffer.
///
/// `depth` must be `width × height` floats, initialized to a large value.
/// Uses simple flat-shaded triangle rasterization.
#[cfg(feature = "alloc")]
pub fn draw_mesh_filled(
    fb: &mut [u32],
    depth: &mut [f32],
    width: u32,
    height: u32,
    mesh: &Mesh3D,
    mvp: &Mat4,
    color: Color,
) {
    use alloc::vec::Vec;
    let c = color.to_u32();
    let half_w = width as f32 * 0.5;
    let half_h = height as f32 * 0.5;

    let mut screen: Vec<[f32; 3]> = Vec::with_capacity(mesh.vertices.len());
    for v in mesh.vertices {
        let h = mvp.transform([v.x, v.y, v.z]);
        let sx = (h[0] / h[3]) * half_w + half_w;
        let sy = -(h[1] / h[3]) * half_h + half_h;
        let sz = h[2] / h[3];
        screen.push([sx, sy, sz]);
    }

    for tri in mesh.faces {
        let a = screen[tri.v0];
        let b = screen[tri.v1];
        let c2 = screen[tri.v2];
        rasterize_tri(fb, depth, width, height, a, b, c2, c);
    }
}

#[cfg(feature = "alloc")]
fn rasterize_tri(
    fb: &mut [u32],
    depth: &mut [f32],
    w: u32,
    h: u32,
    v0: [f32; 3],
    v1: [f32; 3],
    v2: [f32; 3],
    color: u32,
) {
    let mut pts = [v0, v1, v2];
    pts.sort_by(|a, b| a[1].partial_cmp(&b[1]).unwrap());
    let [a, b, c] = pts;

    let dy_total = c[1] - a[1];
    if dy_total < 0.5 { return; }

    let y_start = (a[1] + 0.5) as i32;
    let y_end = (c[1] + 0.5) as i32;
    let y0 = y_start.max(0);
    let y1 = y_end.min(h as i32);

    for y in y0..y1 {
        let t = ((y as f32 + 0.5) - a[1]) / dy_total;
        let t = t.max(0.0).min(1.0);

        let (left, right) = if b[1] - a[1] < 0.5 {
            (lerp_vert(a, c, t), lerp_vert(b, c, t))
        } else if c[1] - b[1] < 0.5 {
            (lerp_vert(a, b, t), lerp_vert(a, c, t))
        } else {
            let mid_t = (b[1] - a[1]) / dy_total;
            let _mid = lerp_vert(a, c, mid_t);
            if (y as f32 + 0.5) < b[1] {
                (lerp_vert(a, b, t / mid_t), lerp_vert(a, c, t))
            } else {
                let t2 = ((y as f32 + 0.5) - b[1]) / (c[1] - b[1]);
                (lerp_vert(b, c, t2), lerp_vert(a, c, t))
            }
        };

        let (lx, rx) = if left[0] <= right[0] { (left, right) } else { (right, left) };
        let x_start = (lx[0] + 0.5) as i32;
        let x_end   = (rx[0] + 0.5) as i32;
        let xs = x_start.max(0);
        let xe = x_end.min(w as i32);
        let dx = rx[0] - lx[0];

        for x in xs..xe {
            let z = if dx > 0.5 {
                let seg_t = ((x as f32 + 0.5) - lx[0]) / dx;
                lx[2] + (rx[2] - lx[2]) * seg_t
            } else {
                lx[2]
            };
            let idx = (y as usize) * (w as usize) + (x as usize);
            if z < depth[idx] {
                depth[idx] = z;
                fb[idx] = color;
            }
        }
    }
}

fn lerp_vert(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}
