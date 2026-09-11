use crate::{
    geometry::{Cube, Floor},
    lighting::PointLight,
    math::Vec3,
};

pub struct Scene {
    pub cube: Cube,
    pub floor: Floor,
    pub light: PointLight,
}

impl Scene {
    pub fn diamond_ore() -> Self {
        Self {
            cube: Cube::new(Vec3::new(0.0, 0.0, 0.0), 2.0),
            floor: Floor { y: -1.0 },
            // Luz frontal-superior para que las tres caras visibles conserven detalle.
            light: PointLight {
                position: Vec3::new(4.0, 6.0, 5.0),
                intensity: 155.0,
            },
        }
    }
}
