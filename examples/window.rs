// #![windows_subsystem = "windows"]

use sill::{message_loop, quit, Window};
use std::process::ExitCode;
use windows::Win32::{
    Foundation::LRESULT,
    UI::WindowsAndMessaging::{WM_CREATE, WM_DESTROY},
};

fn main() -> ExitCode {
    let mut window = Window::new(Some(Box::new(|window, event| match event.msg {
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
    })));

    Window::create(&mut window, "Window");
    Window::create(&mut window, "Window2");

    message_loop()
}
