use crate::{
    geometry::{Cube, Hit, EPSILON},
    math::Vec3,
    ray::Ray,
};

pub struct PointLight {
    pub position: Vec3,
    pub intensity: f32,
}

pub fn shade(
    hit: Hit,
    albedo: Vec3,
    shading_normal: Vec3,
    cube: &Cube,
    light: &PointLight,
) -> Vec3 {
    let to_light = light.position - hit.point;
    let distance_squared = to_light.dot(to_light);
    let distance = distance_squared.sqrt();
    let light_direction = to_light / distance;
    let diffuse = shading_normal.dot(light_direction).max(0.0);

    if diffuse == 0.0 || is_shadowed(hit, light_direction, distance, cube) {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    // Lambert con atenuación cuadrática. Sigue siendo iluminación solo difusa.
    albedo * (light.intensity * diffuse / (std::f32::consts::PI * distance_squared))
}

fn is_shadowed(hit: Hit, light_direction: Vec3, light_distance: f32, cube: &Cube) -> bool {
    let ray = Ray {
        origin: hit.point + hit.normal * (EPSILON * 8.0),
        direction: light_direction,
    };
    cube.intersect(ray, light_distance - EPSILON).is_some()
}
