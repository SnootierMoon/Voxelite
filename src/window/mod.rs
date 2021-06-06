mod state;
mod window;

pub use state::State;
pub use window::Window;

#[derive(Copy, Clone, Eq, PartialEq)]
enum InputMode {
    CAM3D,
    MOUSE,
}

impl InputMode {
    fn invert(&self) -> Self {
        match self {
            Self::CAM3D => Self::MOUSE,
            Self::MOUSE => Self::CAM3D,
        }
    }
}
