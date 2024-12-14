use std::process::ExitCode;
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        Foundation::HMODULE,
        System::{
            Diagnostics::Debug::OutputDebugStringW,
            LibraryLoader::{
                GetModuleHandleExW, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
                GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            },
        },
        UI::WindowsAndMessaging::{
            DispatchMessageW, GetMessageW, MessageBoxW, PostQuitMessage, MB_OK, MESSAGEBOX_RESULT,
            MSG,
        },
    },
    UI::ViewManagement::{UIColorType, UISettings},
};

pub fn message_box(caption: &str, text: &str) -> MESSAGEBOX_RESULT {
    unsafe { MessageBoxW(None, &HSTRING::from(text), &HSTRING::from(caption), MB_OK) }
}

pub fn log(message: &str) {
    unsafe {
        OutputDebugStringW(&HSTRING::from(format!("{}\n", message)));
    };
}

pub fn is_dark_mode() -> bool {
    let ui_settings = UISettings::new().unwrap();
    let bg = ui_settings.GetColorValue(UIColorType::Background).unwrap();

    ((5i32 * i32::try_from(bg.G).unwrap())
        + (2i32 * i32::try_from(bg.R).unwrap())
        + i32::try_from(bg.B).unwrap())
        < (8i32 * 128i32)
}

pub fn module_handle() -> HMODULE {
    let mut module = HMODULE::default();

    unsafe {
        GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT | GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
            PCWSTR(module_handle as *const u16),
            &mut module,
        )
        .unwrap();
    }

    return module;
}

pub fn message_loop() -> ExitCode {
    let mut msg = MSG::default();

    loop {
        match unsafe { GetMessageW(&mut msg, None, 0, 0).0 } {
            0 => break,
            -1 => return ExitCode::FAILURE,
            _ => unsafe { DispatchMessageW(&msg) },
        };
    }

    ExitCode::from(u8::try_from(msg.wParam.0).unwrap())
}

pub fn quit(exit_code: i32) {
    unsafe { PostQuitMessage(exit_code) };
}
