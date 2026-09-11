use crate::{
    geometry::{Cube, Hit},
    math::Vec3,
};

const TEXTURE_SIZE: i32 = 16;
const RELIEF_STRENGTH: f32 = 0.62;

// Patrón original de 16x16 inspirado en las vetas dispersas de la mena de diamante.
const ORE_PATTERN: [u16; 16] = [
    0b0000_0000_0000_0000,
    0b0000_0110_0001_0000,
    0b0000_1110_0011_0000,
    0b0100_1000_1110_0010,
    0b0110_0000_1100_0110,
    0b0011_0001_0000_1100,
    0b0001_0011_1000_1000,
    0b1000_0111_1001_1000,
    0b1100_0011_0011_0001,
    0b0110_0100_0110_0011,
    0b0011_1000_1100_0110,
    0b0001_1000_0100_1100,
    0b0100_0110_0011_1000,
    0b1100_1100_0001_0000,
    0b1000_1000_0000_0000,
    0b0000_0000_0000_0000,
];

#[derive(Clone, Copy)]
struct Texel {
    color: Vec3,
    height: f32,
}

pub struct Surface {
    pub albedo: Vec3,
    pub normal: Vec3,
}

pub fn diamond_ore_surface(cube: &Cube, hit: Hit) -> Surface {
    let (u, v, tangent_u, tangent_v, face) = face_coordinates(cube, hit);
    let x = (u.clamp(0.0, 0.999_999) * TEXTURE_SIZE as f32) as i32;
    let y = ((1.0 - v).clamp(0.0, 0.999_999) * TEXTURE_SIZE as f32) as i32;
    let center = sample(face, x, y);

    // El gradiente del mapa de altura altera la normal solo en las vetas.
    let height_left = sample(face, x - 1, y).height;
    let height_right = sample(face, x + 1, y).height;
    let height_up = sample(face, x, y - 1).height;
    let height_down = sample(face, x, y + 1).height;
    let normal = (hit.normal
        + tangent_u * ((height_left - height_right) * RELIEF_STRENGTH)
        + tangent_v * ((height_down - height_up) * RELIEF_STRENGTH))
        .normalize();

    Surface {
        albedo: center.color,
        normal,
    }
}

pub fn floor_color(point: Vec3) -> Vec3 {
    let cell = (point.x.floor() as i32 + point.z.floor() as i32) & 1;
    if cell == 0 {
        Vec3::new(0.32, 0.34, 0.38)
    } else {
        Vec3::new(0.22, 0.24, 0.28)
    }
}

fn face_coordinates(cube: &Cube, hit: Hit) -> (f32, f32, Vec3, Vec3, u32) {
    let size = cube.max.x - cube.min.x;
    let point = hit.point;

    if hit.normal.x > 0.5 {
        (
            (cube.max.z - point.z) / size,
            (point.y - cube.min.y) / size,
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            0,
        )
    } else if hit.normal.x < -0.5 {
        (
            (point.z - cube.min.z) / size,
            (point.y - cube.min.y) / size,
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 1.0, 0.0),
            1,
        )
    } else if hit.normal.y > 0.5 {
        (
            (point.x - cube.min.x) / size,
            (cube.max.z - point.z) / size,
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            2,
        )
    } else if hit.normal.y < -0.5 {
        (
            (point.x - cube.min.x) / size,
            (point.z - cube.min.z) / size,
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            3,
        )
    } else if hit.normal.z > 0.5 {
        (
            (point.x - cube.min.x) / size,
            (point.y - cube.min.y) / size,
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            4,
        )
    } else {
        (
            (cube.max.x - point.x) / size,
            (point.y - cube.min.y) / size,
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            5,
        )
    }
}

fn sample(face: u32, x: i32, y: i32) -> Texel {
    if x < 0 || y < 0 || x >= TEXTURE_SIZE || y >= TEXTURE_SIZE {
        return stone_texel(face, x.clamp(0, 15), y.clamp(0, 15));
    }

    let (pattern_x, pattern_y) = pattern_coordinates(face, x, y);
    let is_ore = (ORE_PATTERN[pattern_y as usize] >> pattern_x) & 1 == 1;
    if is_ore {
        ore_texel(face, x, y)
    } else {
        stone_texel(face, x, y)
    }
}

fn pattern_coordinates(face: u32, x: i32, y: i32) -> (i32, i32) {
    match face {
        0 => (x, y),
        1 => (15 - x, y),
        2 => (y, 15 - x),
        3 => (15 - y, x),
        4 => (15 - x, 15 - y),
        _ => (y, x),
    }
}

fn ore_texel(face: u32, x: i32, y: i32) -> Texel {
    let variation = hash(face, x, y) & 3;
    let color = match variation {
        0 => Vec3::new(0.03, 0.72, 0.78),
        1 => Vec3::new(0.06, 0.92, 0.96),
        2 => Vec3::new(0.18, 1.00, 1.00),
        _ => Vec3::new(0.02, 0.52, 0.64),
    };
    Texel {
        color,
        height: 0.72 + variation as f32 * 0.08,
    }
}

fn stone_texel(face: u32, x: i32, y: i32) -> Texel {
    let noise = (hash(face, x, y) & 7) as f32 / 7.0;
    let value = 0.16 + noise * 0.17;
    Texel {
        color: Vec3::new(value * 0.92, value * 0.98, value),
        height: noise * 0.045,
    }
}

fn hash(face: u32, x: i32, y: i32) -> u32 {
    let mut value = (x as u32).wrapping_mul(0x045d_9f3b)
        ^ (y as u32).wrapping_mul(0x119d_e1f3)
        ^ face.wrapping_mul(0x3449_ba17);
    value ^= value >> 16;
    value = value.wrapping_mul(0x045d_9f3b);
    value ^ (value >> 16)
}
