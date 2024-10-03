use glam::{Mat4, Vec2, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub rotation: Vec2,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Vec2::ZERO,
            fov: std::f32::consts::FRAC_PI_2,
            near: 0.1,
            far: 100.0,
        }
    }
}

impl Camera {
    // TODO: Add camera z rotation (roll)
    pub fn view(&self) -> Mat4 {
        let (sin_yaw, cos_yaw) = self.rotation.x.sin_cos();
        let (sin_pitch, cos_pitch) = self.rotation.y.sin_cos();

        let dir = Vec3 {
            x: cos_yaw * cos_pitch,
            y: sin_pitch,
            z: sin_yaw * cos_pitch,
        };

        Mat4::look_to_lh(self.position, dir, Vec3::Y)
    }

    pub fn projection(&self, width: f32, height: f32) -> Mat4 {
        Mat4::perspective_lh(self.fov, width / height, self.near, self.far)
    }
}
