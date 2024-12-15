// #![windows_subsystem = "windows"]

use sill::{
    message_loop, quit,
    window::{default_procedure, WindowBuilder, WindowEventHandler},
};
use std::{process::ExitCode, rc::Rc};
use windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        SendMessageW, WM_CREATE, WM_DESTROY, WM_NOTIFY, WS_OVERLAPPEDWINDOW,
    },
};

fn main() -> ExitCode {
    let message_only_events: WindowEventHandler = Rc::new(move |window, event| match event.msg {
        WM_NOTIFY => {
            println!("WM_NOTIFY");

            unsafe {
                println!(
                    "{}",
                    window.as_ref().class.lpszClassName.to_string().unwrap()
                );
            }

            LRESULT::default()
        }
        _ => default_procedure(event),
    });

    let message_only = WindowBuilder::new()
        .events(message_only_events)
        .create_message_only();
    let message_only_hwnd = message_only.hwnd;

    let events: WindowEventHandler = Rc::new(move |window, event| match event.msg {
        WM_CREATE => {
            println!("WM_CREATE");

            unsafe {
                println!(
                    "{}",
                    window.as_ref().class.lpszClassName.to_string().unwrap()
                );
            }

            unsafe {
                SendMessageW(
                    message_only_hwnd,
                    WM_NOTIFY,
                    WPARAM::default(),
                    LPARAM::default(),
                );
            }

            LRESULT(1)
        }
        WM_DESTROY => {
            println!("WM_DESTROY");

            quit(0);

            LRESULT(0)
        }
        _ => default_procedure(event),
    });

    let _window = WindowBuilder::new()
        .title("DefaultWindow")
        .style(WS_OVERLAPPEDWINDOW)
        .events(events)
        .create(true);

    message_loop()
}
