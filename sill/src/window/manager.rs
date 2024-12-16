use super::WindowBuilder;

#[derive(Default)]
pub struct WindowManager {
    pub builder: WindowBuilder,
}

impl WindowManager {
    pub fn new() -> Self {
        Default::default()
    }
}
