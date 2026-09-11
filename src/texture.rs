use crate::{
    geometry::{Cube, Hit},
    math::Vec3,
};
use image::ImageFormat;

const RELIEF_STRENGTH: f32 = 0.52;
const PIXEL_GRID_SIZE: f32 = 16.0;

pub struct ImageTexture {
    width: usize,
    height: usize,
    colors: Vec<Vec3>,
    heights: Vec<f32>,
}

pub struct Surface {
    pub albedo: Vec3,
    pub normal: Vec3,
}

impl ImageTexture {
    pub fn from_png_bytes(bytes: &[u8]) -> Result<Self, image::ImageError> {
        let image = image::load_from_memory_with_format(bytes, ImageFormat::Png)?.to_rgb8();
        let width = image.width() as usize;
        let height = image.height() as usize;
        let mut colors = Vec::with_capacity(width * height);
        let mut heights = Vec::with_capacity(width * height);

        for pixel in image.pixels() {
            let red = pixel[0] as f32 / 255.0;
            let green = pixel[1] as f32 / 255.0;
            let blue = pixel[2] as f32 / 255.0;

            colors.push(Vec3::new(
                srgb_to_linear(red),
                srgb_to_linear(green),
                srgb_to_linear(blue),
            ));

            // El cian sobresale; la piedra gris permanece casi plana.
            let cyan = ((green + blue) * 0.5 - red).max(0.0);
            heights.push((cyan * 2.5).clamp(0.0, 1.0));
        }

        Ok(Self {
            width,
            height,
            colors,
            heights,
        })
    }

    fn color_at(&self, u: f32, v: f32) -> Vec3 {
        self.colors[self.index_at(u, v)]
    }

    fn height_at(&self, u: f32, v: f32) -> f32 {
        self.heights[self.index_at(u, v)]
    }

    fn index_at(&self, u: f32, v: f32) -> usize {
        let x = (u.clamp(0.0, 0.999_999) * self.width as f32) as usize;
        let y = ((1.0 - v).clamp(0.0, 0.999_999) * self.height as f32) as usize;
        y * self.width + x
    }
}

pub fn diamond_ore_surface(cube: &Cube, hit: Hit, texture: &ImageTexture) -> Surface {
    let (u, v, tangent_u, tangent_v) = face_coordinates(cube, hit);
    let texel_step = 1.0 / PIXEL_GRID_SIZE;

    // El color viene directamente de diamantito.png. El gradiente de sus zonas
    // cian inclina la normal y conserva el relieve solicitado.
    let height_left = texture.height_at(u - texel_step, v);
    let height_right = texture.height_at(u + texel_step, v);
    let height_down = texture.height_at(u, v - texel_step);
    let height_up = texture.height_at(u, v + texel_step);
    let normal = (hit.normal
        + tangent_u * ((height_left - height_right) * RELIEF_STRENGTH)
        + tangent_v * ((height_down - height_up) * RELIEF_STRENGTH))
        .normalize();

    Surface {
        albedo: texture.color_at(u, v),
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

fn face_coordinates(cube: &Cube, hit: Hit) -> (f32, f32, Vec3, Vec3) {
    let size = cube.max.x - cube.min.x;
    let point = hit.point;

    if hit.normal.x > 0.5 {
        (
            (cube.max.z - point.z) / size,
            (point.y - cube.min.y) / size,
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
        )
    } else if hit.normal.x < -0.5 {
        (
            (point.z - cube.min.z) / size,
            (point.y - cube.min.y) / size,
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 1.0, 0.0),
        )
    } else if hit.normal.y > 0.5 {
        (
            (point.x - cube.min.x) / size,
            (cube.max.z - point.z) / size,
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
        )
    } else if hit.normal.y < -0.5 {
        (
            (point.x - cube.min.x) / size,
            (point.z - cube.min.z) / size,
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        )
    } else if hit.normal.z > 0.5 {
        (
            (point.x - cube.min.x) / size,
            (point.y - cube.min.y) / size,
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )
    } else {
        (
            (cube.max.x - point.x) / size,
            (point.y - cube.min.y) / size,
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )
    }
}

fn srgb_to_linear(value: f32) -> f32 {
    if value <= 0.04045 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

#[cfg(test)]
mod tests {
    use super::ImageTexture;

    #[test]
    fn loads_diamantito_asset() {
        let texture = ImageTexture::from_png_bytes(include_bytes!("../assets/diamantito.png"))
            .expect("diamantito.png debe ser un PNG válido");
        assert_eq!((texture.width, texture.height), (360, 360));
    }
}
