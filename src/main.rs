mod render;

const DEBUG_MODE: bool = cfg!(debug_assertions);

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::max())
        .init();

    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(winit::dpi::LogicalSize::new(1280, 800))
        .with_title("sex lol")
        .build(&event_loop)
        .unwrap();

    let instance = render::Instance::new(&window);
    let surface = render::Surface::new(instance.clone(), &window);
    let renderer = render::Renderer::new(&surface);

    std::mem::drop(renderer);
    std::mem::drop(surface);
    std::mem::drop(instance);

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit
            }
            _ => ()
        }
        _ => ()
    })
}
