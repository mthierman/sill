// #![windows_subsystem = "windows"]

use sill::{message_loop, window::WindowBuilder};
use std::process::ExitCode;
use windows::Win32::UI::WindowsAndMessaging::WS_OVERLAPPEDWINDOW;

fn main() -> ExitCode {
    let builder = WindowBuilder::new()
        .title("DefaultWindow")
        .style(WS_OVERLAPPEDWINDOW);

    let _window1 = builder.create(true);
    let _window2 = builder.create(true);

    message_loop()
}
