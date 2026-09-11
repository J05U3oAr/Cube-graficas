use crate::{math::Vec3, ray::Ray};

pub const EPSILON: f32 = 0.0001;

#[derive(Clone, Copy)]
pub enum Material {
    DiamondOre,
    Floor,
}

#[derive(Clone, Copy)]
pub struct Hit {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
}

impl Cube {
    pub fn new(center: Vec3, size: f32) -> Self {
        let half = Vec3::new(size * 0.5, size * 0.5, size * 0.5);
        Self {
            min: center - half,
            max: center + half,
        }
    }

    /// Intersección ray-box por slabs. `t_max` acelera los rayos de sombra.
    pub fn intersect(&self, ray: Ray, t_max: f32) -> Option<Hit> {
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

            let inverse = 1.0 / direction;
            let mut entry = (lo - origin) * inverse;
            let mut exit = (hi - origin) * inverse;
            let mut entry_normal = negative_normal;
            if entry > exit {
                std::mem::swap(&mut entry, &mut exit);
                entry_normal = positive_normal;
            }
            if entry > near {
                near = entry;
                normal = entry_normal;
            }
            far = far.min(exit);
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
            material: Material::DiamondOre,
        })
    }
}

pub struct Floor {
    pub y: f32,
}

impl Floor {
    pub fn intersect(&self, ray: Ray, t_max: f32) -> Option<Hit> {
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
}
