use std::ops::Rem;

pub struct PlayerCamera {
    camera: Camera,
}

impl PlayerCamera {
    pub fn new(pos: ultraviolet::Vec3, yaw: f32, pitch: f32) -> Self {
        Self {
            camera: Camera::new(pos, yaw, pitch),
        }
    }

    const MOVE_KEYS: [winit::event::VirtualKeyCode; 6] = [
        winit::event::VirtualKeyCode::W,
        winit::event::VirtualKeyCode::S,
        winit::event::VirtualKeyCode::A,
        winit::event::VirtualKeyCode::D,
        winit::event::VirtualKeyCode::Space,
        winit::event::VirtualKeyCode::LShift,
    ];

    pub fn update(&mut self, state: &crate::window::State) {
        self.camera.update_orientation(state.mouse_rel() / -60.);

        self.camera.pos += self.camera.move_matrix()
            * state.move_vector(&Self::MOVE_KEYS)
            * 30.
            * state.frame_elapsed().as_secs_f32();
    }

    pub fn matrix(&self, vertical_fov: f32, aspect_ratio: f32) -> ultraviolet::Mat4 {
        self.camera.view_matrix(vertical_fov, aspect_ratio)
    }
}

pub struct Camera {
    pos: ultraviolet::Vec3,
    orientation: ultraviolet::Vec2, // (yaw, pitch)
}

impl Camera {
    const Z_NEAR: f32 = 0.1;
    const UP: ultraviolet::Vec3 = ultraviolet::Vec3::new(0., 0., 1.);

    pub fn new(pos: ultraviolet::Vec3, yaw: f32, pitch: f32) -> Self {
        Self {
            pos,
            orientation: ultraviolet::Vec2::new(yaw, pitch),
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

    pub fn view_matrix(&self, vertical_fov: f32, aspect_ratio: f32) -> ultraviolet::Mat4 {
        let (sin, cos) = self.orientation.y.sin_cos();
        let look_vec = self.forward() * cos + Self::UP * sin;
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

    pub fn update_orientation(&mut self, delta: ultraviolet::Vec2) {
        self.orientation += delta;
        self.orientation.x = self.orientation.x.rem(Self::X_DIR_MAX);
        self.orientation.y = self.orientation.y.clamp(-Self::Y_DIR_MAX, Self::Y_DIR_MAX);
    }
}
