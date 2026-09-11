use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::ops::{Add, Div, Mul, Neg, Sub};

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
    const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    fn length(self) -> f32 {
        self.dot(self).sqrt()
    }
    fn normalize(self) -> Self {
        self / self.length()
    }
    fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        Self::new(self.x + b.x, self.y + b.y, self.z + b.z)
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        Self::new(self.x - b.x, self.y - b.y, self.z - b.z)
    }
}
impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }
}
impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, s: f32) -> Self {
        Self::new(self.x / s, self.y / s, self.z / s)
    }
}
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

#[derive(Clone, Copy)]
struct Ray {
    origin: Vec3,
    direction: Vec3,
}

#[derive(Clone, Copy)]
struct Hit {
    t: f32,
    point: Vec3,
    normal: Vec3,
    material: Material,
}

#[derive(Clone, Copy)]
enum Material {
    Cube,
    Floor,
}

/// Caja alineada a ejes. Sus límites se almacenan para no recalcularlos por píxel.
struct Cube {
    min: Vec3,
    max: Vec3,
    color: Vec3,
}

impl Cube {
    fn new(center: Vec3, size: f32, color: Vec3) -> Self {
        let h = size * 0.5;
        let half = Vec3::new(h, h, h);
        Self {
            min: center - half,
            max: center + half,
            color,
        }
    }

    // Método de slabs. `t_max` permite terminar pronto en rayos de sombra.
    fn intersect(&self, ray: Ray, t_max: f32) -> Option<Hit> {
        let mut near = f32::NEG_INFINITY;
        let mut far = t_max;
        let mut normal = Vec3::new(0.0, 0.0, 0.0);

        for (origin, direction, lo, hi, negative_normal, positive_normal) in [
            (
                ray.origin.x,
                ray.direction.x,
                self.min.x,
                self.max.x,
                Vec3::new(-1.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            (
                ray.origin.y,
                ray.direction.y,
                self.min.y,
                self.max.y,
                Vec3::new(0.0, -1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            (
                ray.origin.z,
                ray.direction.z,
                self.min.z,
                self.max.z,
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
        ] {
            if direction.abs() < EPSILON {
                if origin < lo || origin > hi {
                    return None;
                }
                continue;
            }
            let inv_direction = 1.0 / direction;
            let mut a = (lo - origin) * inv_direction;
            let mut b = (hi - origin) * inv_direction;
            let mut entry_normal = negative_normal;
            if a > b {
                std::mem::swap(&mut a, &mut b);
                entry_normal = positive_normal;
            }
            if a > near {
                near = a;
                normal = entry_normal;
            }
            far = far.min(b);
            if near > far {
                return None;
            }
        }

        if near <= EPSILON || near >= t_max {
            return None;
        }
        Some(Hit {
            t: near,
            point: ray.origin + ray.direction * near,
            normal,
            material: Material::Cube,
        })
    }
}

struct Floor {
    y: f32,
    color_a: Vec3,
    color_b: Vec3,
}

impl Floor {
    fn intersect(&self, ray: Ray, t_max: f32) -> Option<Hit> {
        if ray.direction.y.abs() < EPSILON {
            return None;
        }
        let t = (self.y - ray.origin.y) / ray.direction.y;
        if t <= EPSILON || t >= t_max {
            return None;
        }
        Some(Hit {
            t,
            point: ray.origin + ray.direction * t,
            normal: Vec3::new(0.0, 1.0, 0.0),
            material: Material::Floor,
        })
    }

    fn color_at(&self, point: Vec3) -> Vec3 {
        // Cuadrícula muy sutil: ayuda a percibir escala sin depender de especularidad.
        let cell = (point.x.floor() as i32 + point.z.floor() as i32) & 1;
        if cell == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }
}

#[derive(Clone, Copy)]
struct Camera {
    yaw: f32,
    pitch: f32,
    radius: f32,
    target: Vec3,
}

struct CameraFrame {
    origin: Vec3,
    first_direction: Vec3,
    step_x: Vec3,
    step_y: Vec3,
}

impl Camera {
    fn position(&self) -> Vec3 {
        Vec3::new(
            self.target.x + self.radius * self.pitch.cos() * self.yaw.sin(),
            self.target.y + self.radius * self.pitch.sin(),
            self.target.z + self.radius * self.pitch.cos() * self.yaw.cos(),
        )
    }

    // Se calcula una sola vez por fotograma, en vez de una vez por píxel.
    fn frame(&self) -> CameraFrame {
        let origin = self.position();
        let forward = (self.target - origin).normalize();
        let right = forward.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
        let up = right.cross(forward);
        let fov_scale = (45.0_f32.to_radians() * 0.5).tan();
        let aspect = WIDTH as f32 / HEIGHT as f32;
        let step_x = right * (2.0 * aspect * fov_scale / WIDTH as f32);
        let step_y = -up * (2.0 * fov_scale / HEIGHT as f32);
        let first_direction =
            forward - right * (aspect * fov_scale) + up * fov_scale + (step_x + step_y) * 0.5;
        CameraFrame {
            origin,
            first_direction,
            step_x,
            step_y,
        }
    }
}

struct PointLight {
    position: Vec3,
    intensity: f32,
}

fn shadowed(
    point: Vec3,
    normal: Vec3,
    light_direction: Vec3,
    light_distance: f32,
    cube: &Cube,
) -> bool {
    let shadow_ray = Ray {
        origin: point + normal * (EPSILON * 8.0),
        direction: light_direction,
    };
    cube.intersect(shadow_ray, light_distance - EPSILON)
        .is_some()
}

fn shade(hit: Hit, albedo: Vec3, cube: &Cube, light: &PointLight) -> Vec3 {
    let to_light = light.position - hit.point;
    let distance_squared = to_light.dot(to_light);
    let distance = distance_squared.sqrt();
    let light_direction = to_light / distance;
    let n_dot_l = hit.normal.dot(light_direction).max(0.0);

    if n_dot_l == 0.0 || shadowed(hit.point, hit.normal, light_direction, distance, cube) {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    // BRDF Lambertiana con atenuación de una luz puntual: no hay especular ni luz ambiente.
    // El factor 1/pi conserva energía y la intensidad compensa las unidades de la escena.
    albedo * (light.intensity * n_dot_l / (std::f32::consts::PI * distance_squared))
}

fn rgb(linear_color: Vec3) -> u32 {
    // Tone mapping Reinhard y conversión de espacio lineal a sRGB para la pantalla.
    let to_srgb = |v: f32| {
        let mapped = v.max(0.0) / (1.0 + v.max(0.0));
        (mapped.powf(1.0 / 2.2) * 255.0) as u32
    };
    (to_srgb(linear_color.x) << 16) | (to_srgb(linear_color.y) << 8) | to_srgb(linear_color.z)
}

fn render(frame: &mut [u32], camera: &Camera, cube: &Cube, floor: &Floor, light: &PointLight) {
    let view = camera.frame();

    for y in 0..HEIGHT {
        let mut direction = view.first_direction + view.step_y * y as f32;
        for x in 0..WIDTH {
            // No normalizar el rayo primario es válido para las intersecciones y evita 480k raíces por fotograma.
            let ray = Ray {
                origin: view.origin,
                direction,
            };
            let cube_hit = cube.intersect(ray, f32::INFINITY);
            let closest_t = cube_hit.map_or(f32::INFINITY, |hit| hit.t);
            let floor_hit = floor.intersect(ray, closest_t);
            let color = match cube_hit.or(floor_hit) {
                Some(hit) => match hit.material {
                    Material::Cube => shade(hit, cube.color, cube, light),
                    Material::Floor => shade(hit, floor.color_at(hit.point), cube, light),
                },
                // El fondo no emite luz: solo es el color del entorno visible.
                None => Vec3::new(0.008, 0.014, 0.028),
            };
            frame[y * WIDTH + x] = rgb(color);
            direction = direction + view.step_x;
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Raytracer: cubo, suelo y luz difusa | Flechas: orbitar, W/S: zoom, R: reiniciar, Esc: salir",
        WIDTH, HEIGHT, WindowOptions::default(),
    ).expect("No se pudo crear la ventana");
    window.set_target_fps(60);

    let initial = Camera {
        yaw: 0.65,
        pitch: 0.35,
        radius: 5.5,
        target: Vec3::new(0.0, 0.0, 0.0),
    };
    let mut camera = initial;
    let cube = Cube::new(Vec3::new(0.0, 0.0, 0.0), 2.0, Vec3::new(0.16, 0.55, 0.95));
    let floor = Floor {
        y: -1.0,
        color_a: Vec3::new(0.32, 0.34, 0.38),
        color_b: Vec3::new(0.22, 0.24, 0.28),
    };
    let light = PointLight {
        position: Vec3::new(-3.0, 5.5, 4.0),
        intensity: 145.0,
    };
    let mut frame = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let rotation = 0.035;
        if window.is_key_down(Key::Left) {
            camera.yaw -= rotation;
        }
        if window.is_key_down(Key::Right) {
            camera.yaw += rotation;
        }
        if window.is_key_down(Key::Up) {
            camera.pitch = (camera.pitch + rotation).clamp(-1.30, 1.30);
        }
        if window.is_key_down(Key::Down) {
            camera.pitch = (camera.pitch - rotation).clamp(-1.30, 1.30);
        }
        if window.is_key_down(Key::W) {
            camera.radius = (camera.radius - 0.08).max(2.6);
        }
        if window.is_key_down(Key::S) {
            camera.radius = (camera.radius + 0.08).min(12.0);
        }
        if window.is_key_pressed(Key::R, KeyRepeat::No) {
            camera = initial;
        }

        render(&mut frame, &camera, &cube, &floor, &light);
        window
            .update_with_buffer(&frame, WIDTH, HEIGHT)
            .expect("No se pudo actualizar la ventana");
    }
}
