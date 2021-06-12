mod render;
mod voxel;
mod window;

const DEBUG_MODE: bool = cfg!(debug_assertions);

fn main() {
    let chunk = voxel::Chunk::test1();

    env_logger::builder()
        .filter_level(log::LevelFilter::max())
        .init();

    let window = window::Window::new();

    let instance = render::Instance::new(&window);
    let mut surface = render::Surface::new(instance.clone(), &window);
    let mut renderer = render::Renderer::new(&surface);
    let mut voxel_renderer = render::VoxelRenderer::new(&surface, &chunk.faces());

    let mut camera = render::PlayerCamera::new(ultraviolet::Vec3::new(-5., 0., 0.), 0., 0.);

    window.run(move |state, window| {
        if state.quit() {
            instance.wait_idle();
            return;
        }

        camera.update(state);
        let matrix = camera.matrix(45., surface.aspect_ratio());

        if !renderer.render(&mut surface, |command_buffer| {
            voxel_renderer.draw(command_buffer, &matrix)
        }) {
            instance.wait_idle();
            surface.rebuild(&window);
            voxel_renderer.rebuild(&surface, &chunk.faces())
        }
    });
}
