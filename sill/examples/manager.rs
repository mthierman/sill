// #![windows_subsystem = "windows"]

use sill::{
    default_window_procedure, event, message_loop, quit, Window, WindowEventHandler, WindowManager,
};
use std::{cell::RefCell, process::ExitCode, rc::Rc};
use windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    UI::{
        Input::KeyboardAndMouse::VK_CONTROL,
        WindowsAndMessaging::{
            SendMessageW, KF_REPEAT, WM_CREATE, WM_DESTROY, WM_KEYDOWN, WM_NOTIFY, WS_CLIPCHILDREN,
            WS_OVERLAPPEDWINDOW,
        },
    },
};

#[derive(Default)]
struct App {
    window_manager: WindowManager,
    event_handler: WindowEventHandler,
}

fn main() -> ExitCode {
    // let mut manager = Window::manager().title("Test");
    let app = RefCell::new(App::default());

    // let _window = manager.build();

    let mut events: WindowEventHandler = Rc::new(|_window, event| match event.msg {
        WM_CREATE => {
            println!("WM_CREATE");

            app.borrow_mut().window_manager.build();

            LRESULT(0)
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
        _ => default_window_procedure(event),
    });

    // app.window_manager.build();

    // let builder = Window::builder()
    //     .title("App")
    //     .style(WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN)
    //     .events(events);

    // let _window1 = builder.build();
    // let _window2 = builder.build();

    message_loop()
}
