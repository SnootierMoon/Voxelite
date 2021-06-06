pub struct Window {
    event_loop: Option<winit::event_loop::EventLoop<()>>,
    window: winit::window::Window,
}

impl Window {
    pub fn new() -> Self {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_inner_size(winit::dpi::LogicalSize::new(1280, 800))
            .with_title("voxel render demo")
            .build(&event_loop)
            .unwrap();

        Self {
            event_loop: Some(event_loop),
            window,
        }
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.window.inner_size()
    }

    pub fn center(&self) -> winit::dpi::PhysicalPosition<u32> {
        let size = self.window.inner_size();
        winit::dpi::PhysicalPosition::new(size.width / 2, size.height / 2)
    }

    pub(super) fn set_input_mode(&self, input_mode: super::InputMode) {
        match input_mode {
            super::InputMode::CAM3D => {
                self.window.set_cursor_visible(false);
                self.window.set_cursor_grab(true).unwrap();
            }
            super::InputMode::MOUSE => {
                self.window.set_cursor_position(self.center()).unwrap();
                self.window.set_cursor_visible(true);
                self.window.set_cursor_grab(false).unwrap();
            }
        }
    }

    pub fn run<InputHandler: 'static + FnMut(&super::State, &Self)>(
        mut self,
        mut input_handler: InputHandler,
    ) {
        let event_loop = match self.event_loop.take() {
            Some(event_loop) => event_loop,
            None => return,
        };

        let mut state = super::State::new(&self);

        event_loop.run(move |event, _, control_flow| {
            state.handle_event(&self, event);
            if state.quit() {
                *control_flow = winit::event_loop::ControlFlow::Exit
            }
            if state.main() {
                input_handler(&state, &self);
                state.reset()
            }
        })
    }
}
