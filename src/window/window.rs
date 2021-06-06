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

    pub fn run<InputHandler: 'static + FnMut(&super::State, &Self)>(
        mut self,
        mut input_handler: InputHandler,
    ) {
        let event_loop = match self.event_loop.take() {
            Some(event_loop) => event_loop,
            None => return,
        };

        let mut state = super::State::new();

        event_loop.run(move |event, _, control_flow| match event {
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::CloseRequested, .. } => {
                *control_flow = winit::event_loop::ControlFlow::Exit
            },
            _ => {
                state.handle_event(&event);
                if state.main() {
                    input_handler(&state, &self);
                    state.reset()
                }
            },
        })
    }
}
