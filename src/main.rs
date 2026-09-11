mod camera;
mod geometry;
mod lighting;
mod math;
mod ray;
mod renderer;
mod scene;
mod texture;

use camera::Camera;
use math::Vec3;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use renderer::{HEIGHT, WIDTH};
use scene::Scene;

fn main() {
    let mut window = Window::new(
        "Raytracer: mena de diamante con relieve | Flechas: orbitar, W/S: zoom, R: reiniciar, Esc: salir",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("No se pudo crear la ventana");
    window.set_target_fps(60);

    let initial_camera = Camera {
        yaw: 0.65,
        pitch: 0.35,
        radius: 5.5,
        target: Vec3::new(0.0, 0.0, 0.0),
    };
    let mut camera = initial_camera;
    let scene = Scene::diamond_ore();
    let mut frame = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        update_camera(&window, &mut camera, initial_camera);
        renderer::render(&mut frame, &camera, &scene);
        window
            .update_with_buffer(&frame, WIDTH, HEIGHT)
            .expect("No se pudo actualizar la ventana");
    }
}

fn update_camera(window: &Window, camera: &mut Camera, initial: Camera) {
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
        *camera = initial;
    }
}
