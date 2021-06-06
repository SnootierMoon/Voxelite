#[derive(Default)]
pub struct PlayerCamera {
    camera: Camera,
}

impl PlayerCamera {
    pub fn new(pos: ultraviolet::Vec3, yaw: f32, pitch: f32) -> Self {
        Self {
            camera: Camera::new(pos, yaw, pitch),
            ..Default::default()
        }
    }

    pub fn update(&mut self, state: &crate::window::State) {
        let mouse_rel = state.mouse_rel() / 60.;
        self.camera.orientation -= mouse_rel;
        self.camera.clamp_orientation();

        let vel = ultraviolet::Vec3::new(
            (state.key_held(winit::event::VirtualKeyCode::W) as i32
                - state.key_held(winit::event::VirtualKeyCode::S) as i32) as f32,
            (state.key_held(winit::event::VirtualKeyCode::A) as i32
                - state.key_held(winit::event::VirtualKeyCode::D) as i32) as f32,
            (state.key_held(winit::event::VirtualKeyCode::Space) as i32
                - state.key_held(winit::event::VirtualKeyCode::LShift) as i32) as f32,
        );

        self.camera.pos +=
            self.camera.move_matrix() * vel * 30. * state.frame_elapsed().as_secs_f32();
    }

    pub fn matrix(&self, vertical_fov: f32, aspect_ratio: f32) -> ultraviolet::Mat4 {
        self.camera.draw_matrix(vertical_fov, aspect_ratio)
    }
}

#[derive(Default)]
pub struct Camera {
    pos: ultraviolet::Vec3,
    orientation: ultraviolet::Vec2 // (yaw, pitch)
}

impl Camera {
    const Z_NEAR: f32 = 0.1;
    const UP: ultraviolet::Vec3 = ultraviolet::Vec3::new(0., 0., 1.);

    pub fn new(pos: ultraviolet::Vec3, yaw: f32, pitch: f32) -> Self {
        Self {
            pos,
            orientation: ultraviolet::Vec2::new(yaw, pitch)
        }
    }

    pub fn forward(&self) -> ultraviolet::Vec3 {
        let (sin, cos) = self.orientation.x.sin_cos();
        ultraviolet::Vec3::new(cos, sin, 0.)
    }

    pub fn left(&self) -> ultraviolet::Vec3 {
        let (sin, cos) = self.orientation.x.sin_cos();
        ultraviolet::Vec3::new(-sin, cos, 0.)
    }

    pub fn move_matrix(&self) -> ultraviolet::Mat3 {
        ultraviolet::Mat3::from_rotation_z(self.orientation.x)
    }

    pub fn draw_matrix(&self, vertical_fov: f32, aspect_ratio: f32) -> ultraviolet::Mat4 {
        let (p_sin, p_cos) = self.orientation.y.sin_cos();
        let look_vec = self.forward() * p_cos + Self::UP * p_sin;
        let projection = ultraviolet::projection::perspective_infinite_z_vk(
            vertical_fov,
            aspect_ratio,
            Self::Z_NEAR,
        );
        let view = ultraviolet::Mat4::look_at(self.pos, self.pos + look_vec, Self::UP);
        projection * view
    }

    const X_DIR_MAX: f32 = std::f32::consts::TAU;
    const Y_DIR_MAX: f32 = std::f32::consts::FRAC_PI_2 - 0.0001;
    pub fn clamp_orientation(&mut self) {
        self.orientation.x = self.orientation.x.rem_euclid(Self::X_DIR_MAX);
        self.orientation.y = self.orientation.y.clamp(-Self::Y_DIR_MAX, Self::Y_DIR_MAX)
    }
}
