use crate::{
    camera::Camera, geometry::Material, lighting, math::Vec3, ray::Ray, scene::Scene, texture,
};

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;

pub fn render(frame: &mut [u32], camera: &Camera, scene: &Scene) {
    let view = camera.frame(WIDTH, HEIGHT);

    for y in 0..HEIGHT {
        let mut direction = view.first_direction + view.step_y * y as f32;
        for x in 0..WIDTH {
            // Para una caja y un plano no hace falta normalizar los rayos primarios.
            let ray = Ray {
                origin: view.origin,
                direction,
            };
            let cube_hit = scene.cube.intersect(ray, f32::INFINITY);
            let closest_t = cube_hit.map_or(f32::INFINITY, |hit| hit.t);
            let floor_hit = scene.floor.intersect(ray, closest_t);

            let color = match cube_hit.or(floor_hit) {
                Some(hit) => match hit.material {
                    Material::DiamondOre => {
                        let surface =
                            texture::diamond_ore_surface(&scene.cube, hit, &scene.diamantito);
                        lighting::shade(
                            hit,
                            surface.albedo,
                            surface.normal,
                            &scene.cube,
                            &scene.light,
                        )
                    }
                    Material::Floor => lighting::shade(
                        hit,
                        texture::floor_color(hit.point),
                        hit.normal,
                        &scene.cube,
                        &scene.light,
                    ),
                },
                None => Vec3::new(0.008, 0.014, 0.028),
            };

            frame[y * WIDTH + x] = linear_to_rgb(color);
            direction = direction + view.step_x;
        }
    }
}

fn linear_to_rgb(color: Vec3) -> u32 {
    let to_srgb = |value: f32| {
        let positive = value.max(0.0);
        let tone_mapped = positive / (1.0 + positive);
        (tone_mapped.powf(1.0 / 2.2) * 255.0) as u32
    };
    (to_srgb(color.x) << 16) | (to_srgb(color.y) << 8) | to_srgb(color.z)
}
