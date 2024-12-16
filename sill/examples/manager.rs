// #![windows_subsystem = "windows"]

use sill::{message_loop, Window};
use std::process::ExitCode;

fn main() -> ExitCode {
    let manager = Window::manager();

    let _window = manager.builder.build();

    message_loop()
}
