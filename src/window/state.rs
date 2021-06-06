// Based off of https://github.com/rukai/winit_input_helper

pub struct State {
    main: bool,
    quit: bool,

    start_time: std::time::Instant,
    time: std::time::Instant,
    frame_elapsed: std::time::Duration,

    mouse_rel: ultraviolet::Vec2,
    input_mode: super::InputMode,

    key_held: [bool; 255],
}

impl State {
    pub fn new(window: &super::Window) -> Self {
        let now = std::time::Instant::now();
        let input_mode = super::InputMode::CAM3D;
        window.set_input_mode(input_mode);
        Self {
            main: false,
            quit: false,
            start_time: now,
            time: now,
            frame_elapsed: Default::default(),
            mouse_rel: ultraviolet::Vec2::zero(),
            input_mode,
            key_held: [false; 255],
        }
    }

    pub fn handle_event(&mut self, window: &super::Window, event: winit::event::Event<()>) {
        match event {
            winit::event::Event::MainEventsCleared => {
                let new_time = std::time::Instant::now();
                self.frame_elapsed = new_time - self.time;
                self.time = new_time;
                self.main = true
            }
            winit::event::Event::LoopDestroyed => self.quit = true,
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::Destroyed
                | winit::event::WindowEvent::CloseRequested => self.quit = true,
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        if keycode == winit::event::VirtualKeyCode::Escape
                            && self.input_mode == super::InputMode::CAM3D
                        {
                            self.set_input_mode(window, super::InputMode::MOUSE)
                        }
                        self.key_held[keycode as usize] =
                            input.state == winit::event::ElementState::Pressed
                    }
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    if button == winit::event::MouseButton::Left
                        && state == winit::event::ElementState::Pressed
                        && self.input_mode == super::InputMode::MOUSE
                    {
                        self.set_input_mode(window, super::InputMode::CAM3D)
                    }
                }
                _ => (),
            },
            winit::event::Event::DeviceEvent { event, .. } => match event {
                winit::event::DeviceEvent::MouseMotion { delta } => {
                    if self.input_mode == super::InputMode::CAM3D {
                        self.mouse_rel.x = delta.0 as f32;
                        self.mouse_rel.y = delta.1 as f32
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    pub fn move_vector(&self, move_keys: &[winit::event::VirtualKeyCode; 6]) -> ultraviolet::Vec3 {
        match self.input_mode {
            crate::window::InputMode::CAM3D => ultraviolet::Vec3::new(
                (self.key_held(move_keys[0]) as i32 - self.key_held(move_keys[1]) as i32) as f32,
                (self.key_held(move_keys[2]) as i32 - self.key_held(move_keys[3]) as i32) as f32,
                (self.key_held(move_keys[4]) as i32 - self.key_held(move_keys[5]) as i32) as f32,
            ),
            crate::window::InputMode::MOUSE => ultraviolet::Vec3::zero(),
        }
    }

    fn set_input_mode(&mut self, window: &super::Window, input_mode: super::InputMode) {
        self.input_mode = input_mode;
        window.set_input_mode(input_mode)
    }

    pub fn reset(&mut self) {
        self.main = false;
        self.mouse_rel = ultraviolet::Vec2::zero()
    }

    pub fn main(&self) -> bool {
        self.main || self.quit
    }
    pub fn quit(&self) -> bool {
        self.quit
    }

    pub fn elapsed(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }
    pub fn frame_elapsed(&self) -> std::time::Duration {
        self.frame_elapsed
    }

    pub fn mouse_rel(&self) -> ultraviolet::Vec2 {
        self.mouse_rel
    }

    pub fn key_held(&self, key: winit::event::VirtualKeyCode) -> bool {
        self.key_held[key as usize]
    }
}
