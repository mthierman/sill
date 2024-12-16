// #![windows_subsystem = "windows"]

use sill::{default_window_procedure, message_loop, quit, Window, WindowEventHandler};
use std::{process::ExitCode, rc::Rc};
use windows::Win32::{
    Foundation::LRESULT,
    UI::WindowsAndMessaging::{WM_CREATE, WM_DESTROY, WS_CLIPCHILDREN, WS_OVERLAPPEDWINDOW},
};

fn main() -> ExitCode {
    let events: WindowEventHandler = Rc::new(move |_window, event| match event.msg {
        WM_CREATE => {
            println!("WM_CREATE");

            LRESULT(0)
        }
        WM_DESTROY => {
            println!("WM_DESTROY");

            quit(0);

            LRESULT(0)
        }
        _ => default_window_procedure(event),
    });

    let _window = Window::builder()
        .title("sill")
        .style(WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN)
        .events(events)
        .create();

    message_loop()
}
