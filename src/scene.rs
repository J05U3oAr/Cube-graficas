use crate::{
    geometry::{Cube, Floor},
    lighting::PointLight,
    math::Vec3,
    texture::ImageTexture,
};

pub struct Scene {
    pub cube: Cube,
    pub floor: Floor,
    pub light: PointLight,
    pub diamantito: ImageTexture,
}

impl Scene {
    pub fn diamantito() -> Self {
        Self {
            cube: Cube::new(Vec3::new(0.0, 0.0, 0.0), 2.0),
            floor: Floor { y: -1.0 },
            // Luz frontal-superior para que las tres caras visibles conserven detalle.
            light: PointLight {
                position: Vec3::new(4.0, 6.0, 5.0),
                intensity: 155.0,
            },
            diamantito: ImageTexture::from_png_bytes(include_bytes!("../assets/diamantito.png"))
                .expect("No se pudo cargar assets/diamantito.png"),
        }
    }
}
