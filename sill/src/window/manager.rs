use super::WindowBuilder;

#[derive(Default)]
pub struct WindowManager {
    pub builder: WindowBuilder,
}

// impl Default for WindowManager {
//     fn default() -> Self {
//         Self {
//             builder: WindowBuilder::default(),
//         }
//     }
// }

impl WindowManager {
    pub fn new() -> Self {
        Default::default()
    }
}
