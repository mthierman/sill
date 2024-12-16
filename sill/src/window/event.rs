pub mod keydown {
    use super::super::super::macros;
    use super::super::WindowEvent;
    use std::char::from_u32;
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyState, VIRTUAL_KEY};

    pub fn key(event: &WindowEvent) -> Option<char> {
        match u32::try_from(event.wparam.0) {
            Ok(value) => from_u32(value),
            Err(_) => None,
        }
    }

    pub fn key_flags(event: &WindowEvent) -> u16 {
        macros::HIWORD(event.lparam.0 as _)
    }

    pub fn was_key_down(key: &VIRTUAL_KEY) -> bool {
        unsafe { GetKeyState(i32::from(key.0)) as u32 & (1u32 << 31) != 0 }
    }
}
