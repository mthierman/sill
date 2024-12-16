// #![windows_subsystem = "windows"]

use sill::{message_loop, Window};
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut manager = Window::manager().title("Test");

    let _window = manager.build();

    message_loop()
}
