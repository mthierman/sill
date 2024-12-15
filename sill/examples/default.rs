// #![windows_subsystem = "windows"]

use sill::{message_loop, window::WindowBuilder};
use std::process::ExitCode;
use windows::Win32::UI::WindowsAndMessaging::WS_OVERLAPPEDWINDOW;

fn main() -> ExitCode {
    let _window = WindowBuilder::new()
        .title("DefaultWindow")
        .style(WS_OVERLAPPEDWINDOW)
        .create(true);

    message_loop()
}
