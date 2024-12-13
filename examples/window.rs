// #![windows_subsystem = "windows"]

use sill::{message_loop, quit, Window, WindowEvents};
use std::{process::ExitCode, rc::Rc};
use windows::Win32::{
    Foundation::LRESULT,
    UI::WindowsAndMessaging::{WM_CREATE, WM_DESTROY},
};

fn main() -> ExitCode {
    let window_events: WindowEvents = Rc::new(move |window, event| match event.msg {
        WM_CREATE => {
            println!("WM_CREATE");

            unsafe {
                println!(
                    "{}",
                    window.as_ref().class.lpszClassName.to_string().unwrap()
                );
            }

            LRESULT(1)
        }
        WM_DESTROY => {
            println!("WM_DESTROY");

            unsafe {
                println!(
                    "{}",
                    window.as_ref().class.lpszClassName.to_string().unwrap()
                );
            }

            quit(0);

            LRESULT(0)
        }
        _ => Window::default_procedure(event),
    });

    let _window = Window::new("App", true, None, Some(&window_events));

    message_loop()
}
