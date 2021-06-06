mod render;

const DEBUG_MODE: bool = cfg!(debug_assertions);

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::max())
        .init();

    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(winit::dpi::LogicalSize::new(1280, 800))
        .with_title("voxel render demo")
        .build(&event_loop)
        .unwrap();

    let instance = render::Instance::new(&window);
    let mut surface = render::Surface::new(instance.clone(), &window);
    let mut renderer = render::Renderer::new(&surface);
    let mut voxel_renderer = render::VoxelRenderer::new(&surface);

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit
            }
            _ => (),
        },
        winit::event::Event::MainEventsCleared => {
            if !renderer.render(&mut surface, |command_buffer| {
                voxel_renderer.render(command_buffer)
            }) {
                instance.wait_idle();
                surface.rebuild(&window);
                voxel_renderer.rebuild(&surface)
            }
        }
        winit::event::Event::LoopDestroyed => instance.wait_idle(),
        _ => (),
    })
}
