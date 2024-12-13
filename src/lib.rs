use std::{
    cmp::Ordering,
    ffi::c_void,
    hash::{Hash, Hasher},
    process::ExitCode,
    ptr::NonNull,
    rc::Rc,
};
use windows::{
    core::{w, HSTRING, PCWSTR},
    Win32::{
        Foundation::{HMODULE, HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Gdi::{GetStockObject, BLACK_BRUSH, HBRUSH},
        System::{
            Diagnostics::Debug::OutputDebugStringW,
            LibraryLoader::{
                GetModuleHandleExW, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
                GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            },
        },
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetClassInfoExW,
            GetMessageW, GetWindowLongPtrW, LoadCursorW, LoadImageW, MessageBoxW, PostQuitMessage,
            RegisterClassExW, SetWindowLongPtrW, CREATESTRUCTW, CW_USEDEFAULT, HICON, HMENU,
            IDC_ARROW, IDI_APPLICATION, IMAGE_ICON, LR_DEFAULTCOLOR, LR_DEFAULTSIZE, LR_SHARED,
            MB_OK, MESSAGEBOX_RESULT, MSG, WINDOW_EX_STYLE, WINDOW_LONG_PTR_INDEX, WINDOW_STYLE,
            WM_CREATE, WM_DESTROY, WNDCLASSEXW, WNDCLASS_STYLES, WS_CLIPCHILDREN, WS_CLIPSIBLINGS,
            WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
    UI::ViewManagement::{UIColorType, UISettings},
};

pub struct WindowEvent {
    pub hwnd: HWND,
    pub msg: u32,
    pub wparam: WPARAM,
    pub lparam: LPARAM,
}

pub type WindowEvents = Rc<dyn Fn(NonNull<Window>, WindowEvent) -> LRESULT>;

pub struct Window {
    pub class: WNDCLASSEXW,
    pub hwnd: HWND,
    pub events: Option<WindowEvents>,
    pub id: usize,
}

impl Window {
    pub fn new(
        title: &str,
        visible: bool,
        id: Option<usize>,
        events: Option<&WindowEvents>,
    ) -> Box<Self> {
        let class = WNDCLASSEXW {
            cbSize: u32::try_from(std::mem::size_of::<WNDCLASSEXW>()).unwrap(),
            style: WNDCLASS_STYLES::default(),
            lpfnWndProc: Some(Self::wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: i32::try_from(std::mem::size_of::<Self>()).unwrap(),
            hInstance: module_handle().into(),
            hCursor: unsafe { LoadCursorW(None, IDC_ARROW).unwrap() },
            hbrBackground: unsafe { HBRUSH(GetStockObject(BLACK_BRUSH).0) },
            lpszMenuName: PCWSTR::null(),
            lpszClassName: w!("Window"),
            hIcon: Self::load_icon(),
            hIconSm: Self::load_icon(),
        };

        let mut window = Box::new(Self {
            class: class,
            hwnd: HWND::default(),
            events: match events {
                Some(events) => Some(events.clone()),
                None => None,
            },
            id: match id {
                None => 0,
                Some(id) => id,
            },
        });

        window.register();

        window.create_window(title, visible, window.id, WS_OVERLAPPEDWINDOW, None);

        window
    }

    fn register(&mut self) {
        if unsafe {
            GetClassInfoExW(
                self.class.hInstance,
                self.class.lpszClassName,
                &mut self.class,
            )
            .is_err()
        } {
            let atom = unsafe { RegisterClassExW(&self.class) };
            debug_assert!(atom != 0);
        }
    }

    fn create_window(
        &mut self,
        title: &str,
        visible: bool,
        id: usize,
        style: WINDOW_STYLE,
        parent: Option<HWND>,
    ) -> HWND {
        unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE(0),
                self.class.lpszClassName,
                PCWSTR(HSTRING::from(title).as_ptr()),
                style
                    | match parent {
                        None => WS_CLIPSIBLINGS,
                        Some(_) => WS_CLIPCHILDREN,
                    }
                    | match visible {
                        true => WS_VISIBLE,
                        false => WINDOW_STYLE(0),
                    },
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                parent.unwrap_or_default(),
                match parent {
                    None => HMENU::default(),
                    Some(_) => HMENU((id) as *mut c_void),
                },
                self.class.hInstance,
                Some(self as *mut Self as _),
            )
            .unwrap()
        }
    }

    fn load_icon() -> HICON {
        match unsafe {
            LoadImageW(
                module_handle(),
                PCWSTR(1 as _),
                IMAGE_ICON,
                0,
                0,
                LR_DEFAULTCOLOR | LR_DEFAULTSIZE | LR_SHARED,
            )
        } {
            Ok(handle) => HICON(handle.0),
            Err(_) => unsafe {
                HICON(
                    LoadImageW(
                        None,
                        IDI_APPLICATION,
                        IMAGE_ICON,
                        0,
                        0,
                        LR_DEFAULTCOLOR | LR_DEFAULTSIZE | LR_SHARED,
                    )
                    .unwrap()
                    .0,
                )
            },
        }
    }

    // pub fn create(window: &mut Self, title: &str) -> HWND {
    //     window.create_window(0, title, true, WS_OVERLAPPEDWINDOW, None)
    // }

    // pub fn create_hidden(window: &mut Self, title: &str) -> HWND {
    //     window.create_window(0, title, false, WS_OVERLAPPEDWINDOW, None)
    // }

    fn set_instance(hwnd: HWND, lparam: LPARAM) -> Option<NonNull<Self>> {
        let create_struct = lparam.0 as *const CREATESTRUCTW;

        if create_struct.is_null() {
            None
        } else {
            let window = unsafe { (*create_struct).lpCreateParams as *mut Self };

            unsafe {
                SetWindowLongPtrW(hwnd, WINDOW_LONG_PTR_INDEX(0), window as _);
                (*window).hwnd = hwnd;
            }

            NonNull::new(window)
        }
    }

    fn get_instance(hwnd: HWND) -> Option<NonNull<Self>> {
        let window = unsafe { GetWindowLongPtrW(hwnd, WINDOW_LONG_PTR_INDEX(0)) as *mut Self };

        NonNull::new(window)
    }

    fn reset_instance(hwnd: HWND) {
        unsafe { SetWindowLongPtrW(hwnd, WINDOW_LONG_PTR_INDEX(0), 0) };
    }

    pub fn default_procedure(event: WindowEvent) -> LRESULT {
        unsafe { DefWindowProcW(event.hwnd, event.msg, event.wparam, event.lparam) }
    }

    extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        let event = WindowEvent {
            hwnd,
            msg,
            wparam,
            lparam,
        };

        if msg == WM_CREATE {
            if let Some(window) = Self::set_instance(hwnd, lparam) {
                if let Some(events) = unsafe { &window.as_ref().events } {
                    events(window, event);
                }
            }

            return LRESULT(1);
        } else if msg == WM_DESTROY {
            if let Some(window) = Self::get_instance(hwnd) {
                if let Some(events) = unsafe { &window.as_ref().events } {
                    events(window, event);
                }
            }

            Self::reset_instance(hwnd);

            return LRESULT(0);
        } else {
            if let Some(window) = Window::get_instance(hwnd) {
                if let Some(events) = unsafe { &window.as_ref().events } {
                    return events(window, event);
                }
            }
        }

        Self::default_procedure(event)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            let _ = DestroyWindow(self.hwnd);
        }
    }
}

impl Hash for Window {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hwnd.0.hash(state);
    }
}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Window {}

impl PartialOrd for Window {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Window {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.id).cmp(&other.id)
    }
}

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
