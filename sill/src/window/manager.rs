use super::{Window, WindowBuilder};

#[derive(Default)]
pub struct WindowManager {
    builder: WindowBuilder,
}

impl WindowManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn spawn(&self) -> Box<Window> {
        self.builder.build()
    }
}
