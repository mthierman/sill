// #![windows_subsystem = "windows"]

use sill::{message_loop, Window};
use std::process::ExitCode;

fn main() -> ExitCode {
    let manager = Window::manager().title("Test");

    let _window = manager.add();

    message_loop()
}
