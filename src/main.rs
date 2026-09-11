use minifb::{Key, KeyRepeat, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const EPSILON: f32 = 0.0001;

#[derive(Clone, Copy, Debug)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    const fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    fn dot(self, other: Self) -> f32 { self.x * other.x + self.y * other.y + self.z * other.z }
    fn length(self) -> f32 { self.dot(self).sqrt() }
    fn normalize(self) -> Self { self / self.length() }
    fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

use std::ops::{Add, Div, Mul, Neg, Sub};
impl Add for Vec3 { type Output = Self; fn add(self, b: Self) -> Self { Self::new(self.x+b.x, self.y+b.y, self.z+b.z) } }
impl Sub for Vec3 { type Output = Self; fn sub(self, b: Self) -> Self { Self::new(self.x-b.x, self.y-b.y, self.z-b.z) } }
impl Mul<f32> for Vec3 { type Output = Self; fn mul(self, s: f32) -> Self { Self::new(self.x*s, self.y*s, self.z*s) } }
impl Div<f32> for Vec3 { type Output = Self; fn div(self, s: f32) -> Self { Self::new(self.x/s, self.y/s, self.z/s) } }
impl Neg for Vec3 { type Output = Self; fn neg(self) -> Self { Self::new(-self.x, -self.y, -self.z) } }

#[derive(Clone, Copy)]
struct Ray { origin: Vec3, direction: Vec3 }

struct Hit { point: Vec3, normal: Vec3 }

/// Cubo alineado a los ejes, centrado en `center`, con arista `size`.
struct Cube { center: Vec3, size: f32, color: Vec3 }

impl Cube {
    // Intersección ray-box mediante el método de slabs.
    fn intersect(&self, ray: Ray) -> Option<Hit> {
        let half = self.size * 0.5;
        let min = self.center - Vec3::new(half, half, half);
        let max = self.center + Vec3::new(half, half, half);
        let mut t_near = f32::NEG_INFINITY;
        let mut t_far = f32::INFINITY;

        for (origin, dir, lo, hi) in [
            (ray.origin.x, ray.direction.x, min.x, max.x),
            (ray.origin.y, ray.direction.y, min.y, max.y),
            (ray.origin.z, ray.direction.z, min.z, max.z),
        ] {
            if dir.abs() < EPSILON {
                if origin < lo || origin > hi { return None; }
                continue;
            }
            let mut a = (lo - origin) / dir;
            let mut b = (hi - origin) / dir;
            if a > b { std::mem::swap(&mut a, &mut b); }
            t_near = t_near.max(a);
            t_far = t_far.min(b);
            if t_near > t_far { return None; }
        }

        let distance = if t_near > EPSILON { t_near } else { t_far };
        if distance < EPSILON { return None; }
        let point = ray.origin + ray.direction * distance;
        let local = point - self.center;
        let m = half;
        let normal = if (local.x.abs() - m).abs() < EPSILON { Vec3::new(local.x.signum(), 0.0, 0.0) }
            else if (local.y.abs() - m).abs() < EPSILON { Vec3::new(0.0, local.y.signum(), 0.0) }
            else { Vec3::new(0.0, 0.0, local.z.signum()) };
        Some(Hit { point, normal })
    }
}

#[derive(Clone, Copy)]
struct Camera { yaw: f32, pitch: f32, radius: f32, target: Vec3 }

impl Camera {
    fn position(&self) -> Vec3 {
        Vec3::new(
            self.target.x + self.radius * self.pitch.cos() * self.yaw.sin(),
            self.target.y + self.radius * self.pitch.sin(),
            self.target.z + self.radius * self.pitch.cos() * self.yaw.cos(),
        )
    }

    fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let origin = self.position();
        let forward = (self.target - origin).normalize();
        let right = forward.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
        let up = right.cross(forward);
        let aspect = WIDTH as f32 / HEIGHT as f32;
        let fov_scale = (45.0_f32.to_radians() * 0.5).tan();
        let u = ((x as f32 + 0.5) / WIDTH as f32 * 2.0 - 1.0) * aspect * fov_scale;
        let v = (1.0 - (y as f32 + 0.5) / HEIGHT as f32 * 2.0) * fov_scale;
        Ray { origin, direction: (forward + right * u + up * v).normalize() }
    }
}

fn rgb(color: Vec3) -> u32 {
    let c = |v: f32| (v.clamp(0.0, 1.0) * 255.0) as u32;
    (c(color.x) << 16) | (c(color.y) << 8) | c(color.z)
}

fn render(frame: &mut [u32], camera: &Camera, cube: &Cube) {
    let light_position = Vec3::new(-3.0, 5.0, 4.0);
    let background = Vec3::new(0.025, 0.035, 0.06);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ray = camera.ray_for_pixel(x, y);
            let color = match cube.intersect(ray) {
                Some(hit) => {
                    // Lambert: I = color_del_material * max(0, N . L).
                    // No hay componente ambiente ni reflejo especular.
                    let light_dir = (light_position - hit.point).normalize();
                    let diffuse = hit.normal.dot(light_dir).max(0.0);
                    cube.color * diffuse
                }
                None => background,
            };
            frame[y * WIDTH + x] = rgb(color);
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Raytracer: cubo con luz difusa | Flechas: orbitar, W/S: zoom, R: reiniciar, Esc: salir",
        WIDTH, HEIGHT, WindowOptions::default(),
    ).expect("No se pudo crear la ventana");
    window.set_target_fps(60);

    let initial = Camera { yaw: 0.65, pitch: 0.35, radius: 5.0, target: Vec3::new(0.0, 0.0, 0.0) };
    let mut camera = initial;
    let cube = Cube { center: Vec3::new(0.0, 0.0, 0.0), size: 2.0, color: Vec3::new(0.2, 0.65, 1.0) };
    let mut frame = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let rotation = 0.035;
        if window.is_key_down(Key::Left) { camera.yaw -= rotation; }
        if window.is_key_down(Key::Right) { camera.yaw += rotation; }
        if window.is_key_down(Key::Up) { camera.pitch = (camera.pitch + rotation).clamp(-1.45, 1.45); }
        if window.is_key_down(Key::Down) { camera.pitch = (camera.pitch - rotation).clamp(-1.45, 1.45); }
        if window.is_key_down(Key::W) { camera.radius = (camera.radius - 0.08).max(2.4); }
        if window.is_key_down(Key::S) { camera.radius = (camera.radius + 0.08).min(12.0); }
        if window.is_key_pressed(Key::R, KeyRepeat::No) { camera = initial; }

        render(&mut frame, &camera, &cube);
        window.update_with_buffer(&frame, WIDTH, HEIGHT).expect("No se pudo actualizar la ventana");
    }
}
