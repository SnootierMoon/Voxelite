pub struct State {
    main: bool,
    quit: bool,
    start: std::time::Instant,
    last: std::time::Instant,
    frame_elapsed: std::time::Duration
}

impl State {
    pub fn new() -> Self {
        let now = std::time::Instant::now();
        Self {
            main: false,
            quit: false,
            start: now,
            last: now,
            frame_elapsed: std::time::Duration::new(0, 0)
        }
    }

    pub fn handle_event(&mut self, event: &winit::event::Event<()>) {
        self.main = false;
        match event {
            winit::event::Event::MainEventsCleared => {
                let now = std::time::Instant::now();
                let frame_elapsed = now - self.last;
                self.last = now;
                self.frame_elapsed = frame_elapsed;
                self.main = true
            },
            winit::event::Event::LoopDestroyed => self.quit = true,
            _ => ()
        }
    }

    pub fn main(&self) -> bool { self.main || self.quit }
    pub fn quit(&self) -> bool { self.quit }

    pub fn elapsed(&self) -> std::time::Duration { self.start.elapsed() }
    pub fn frame_elapsed(&self) -> std::time::Duration { self.frame_elapsed }
}