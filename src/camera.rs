use crate::math::Vec3;

#[derive(Clone, Copy)]
pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    pub radius: f32,
    pub target: Vec3,
}

pub struct CameraFrame {
    pub origin: Vec3,
    pub first_direction: Vec3,
    pub step_x: Vec3,
    pub step_y: Vec3,
}

impl Camera {
    fn position(&self) -> Vec3 {
        Vec3::new(
            self.target.x + self.radius * self.pitch.cos() * self.yaw.sin(),
            self.target.y + self.radius * self.pitch.sin(),
            self.target.z + self.radius * self.pitch.cos() * self.yaw.cos(),
        )
    }

    pub fn frame(&self, width: usize, height: usize) -> CameraFrame {
        let origin = self.position();
        let forward = (self.target - origin).normalize();
        let right = forward.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
        let up = right.cross(forward);
        let fov_scale = (45.0_f32.to_radians() * 0.5).tan();
        let aspect = width as f32 / height as f32;
        let step_x = right * (2.0 * aspect * fov_scale / width as f32);
        let step_y = -up * (2.0 * fov_scale / height as f32);
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
