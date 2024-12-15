// #![windows_subsystem = "windows"]

use sill::{
    app::{message_loop, quit},
    window::{default_procedure, event, WindowBuilder, WindowEventHandler},
};
use std::{process::ExitCode, rc::Rc};
use windows::Win32::{
    Foundation::LRESULT,
    UI::{
        Input::KeyboardAndMouse::VK_CONTROL,
        WindowsAndMessaging::{KF_REPEAT, WM_CREATE, WM_DESTROY, WM_KEYDOWN, WS_OVERLAPPEDWINDOW},
    },
};

fn main() -> ExitCode {
    let events: WindowEventHandler = Rc::new(move |window, event| match event.msg {
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

            quit(0);

            LRESULT(0)
        }
        WM_KEYDOWN => {
            if let Some(key) = event::keydown::key(&event) {
                let flags = event::keydown::key_flags(&event);
                let is_repeat = (flags & KF_REPEAT as u16) == KF_REPEAT as u16;
                println!("{}", is_repeat);

                if event::keydown::was_key_down(&VK_CONTROL) && key == 'N' {
                    println!("New window!");
                }
            }

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
