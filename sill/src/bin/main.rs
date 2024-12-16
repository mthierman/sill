// #![windows_subsystem = "windows"]

use sill::{
    message_loop, quit,
    window::{default_procedure, WindowBuilder, WindowEventHandler},
};
use std::{process::ExitCode, rc::Rc};
use windows::Win32::{
    Foundation::LRESULT,
    UI::WindowsAndMessaging::{WM_CREATE, WM_DESTROY, WS_OVERLAPPEDWINDOW},
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
        _ => default_procedure(event),
    });

    let _window = WindowBuilder::new()
        .title("Sill")
        .style(WS_OVERLAPPEDWINDOW)
        .events(events)
        .create(true);

    message_loop()
}
