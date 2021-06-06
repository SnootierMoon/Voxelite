// Based off of https://github.com/rukai/winit_input_helper

pub struct State {
    main: bool,
    quit: bool,

    start_time: std::time::Instant,
    time: std::time::Instant,
    frame_elapsed: std::time::Duration,

    mouse_pos: ultraviolet::Vec2,
    mouse_rel: ultraviolet::Vec2,

    key_held: [bool; 255],
}

impl State {
    pub fn new() -> Self {
        let now = std::time::Instant::now();
        Self {
            main: false,
            quit: false,
            start_time: now,
            time: now,
            frame_elapsed: Default::default(),
            mouse_pos: ultraviolet::Vec2::zero(),
            mouse_rel: ultraviolet::Vec2::zero(),
            key_held: [false; 255]
        }
    }

    pub fn handle_event(&mut self, event: &winit::event::Event<()>) {
        match event {
            winit::event::Event::MainEventsCleared => {
                let new_time = std::time::Instant::now();
                self.frame_elapsed  = new_time - self.time;
                self.time = new_time;
                self.main = true
            },
            winit::event::Event::LoopDestroyed => self.quit = true,
            winit::event::Event::WindowEvent { event, ..} => match event {
                winit::event::WindowEvent::Destroyed |
                winit::event::WindowEvent::CloseRequested => { self.quit = true }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    let new_pos = ultraviolet::Vec2::new(position.x as f32, position.y as f32);
                    self.mouse_rel = new_pos - self.mouse_pos;
                    self.mouse_pos = new_pos;
                }
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        if keycode == winit::event::VirtualKeyCode::Escape {
                            self.quit = true
                        }
                        self.key_held[keycode as usize] = input.state == winit::event::ElementState::Pressed;
                    }
                }
                _ => ()
            }
            _ => ()
        }
    }

    pub fn reset(&mut self) {
        self.main = false;
        self.mouse_rel = ultraviolet::Vec2::zero()
    }

    pub fn main(&self) -> bool { self.main || self.quit }
    pub fn quit(&self) -> bool { self.quit }

    pub fn elapsed(&self) -> std::time::Duration { self.start_time.elapsed() }
    pub fn frame_elapsed(&self) -> std::time::Duration { self.frame_elapsed }

    pub fn mouse_pos(&self) -> ultraviolet::Vec2 { self.mouse_pos }
    pub fn mouse_rel(&self) -> ultraviolet::Vec2 { self.mouse_rel }

    pub fn key_held(&self, key: winit::event::VirtualKeyCode) -> bool { self.key_held[key as usize] }
}