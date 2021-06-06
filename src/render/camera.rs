#[derive(Default)]
pub struct Camera {
    pos: ultraviolet::Vec3,
    yaw: f32,
    pitch: f32,
}

impl Camera {
    const Z_NEAR: f32 = 0.1;
    const UP: ultraviolet::Vec3 = ultraviolet::Vec3::new(0., 0., 1.);

    pub fn new(pos: ultraviolet::Vec3, yaw: f32, pitch: f32) -> Self {
        Self { pos, yaw, pitch }
    }
    
    pub fn pos_mut(&mut self) -> &mut ultraviolet::Vec3 {
        &mut self.pos
    }

    pub fn matrix(&self, vertical_fov: f32, aspect_ratio: f32) -> ultraviolet::Mat4 {
        let look_vec = ultraviolet::Vec3::new(
            self.pitch.cos() * self.yaw.cos(),
            self.pitch.cos() * self.yaw.sin(),
            self.pitch.sin(),
        );
        let projection = ultraviolet::projection::perspective_infinite_z_vk(
            vertical_fov,
            aspect_ratio,
            Self::Z_NEAR,
        );
        let view = ultraviolet::Mat4::look_at(self.pos, self.pos + look_vec, Self::UP);
        projection * view
    }
}
