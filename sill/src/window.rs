mod builder;
pub use builder::*;
pub mod event;

use crate::app;
use std::{
    borrow::BorrowMut,
    cmp::Ordering,
    ffi::c_void,
    hash::{Hash, Hasher},
    ptr::NonNull,
    rc::Rc,
};
use windows::{
    core::{w, HSTRING, PCWSTR},
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Gdi::{COLOR_WINDOW, HBRUSH},
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DestroyWindow, GetClassInfoExW, GetWindowLongPtrW,
            LoadCursorW, LoadImageW, RegisterClassExW, SetWindowLongPtrW, CREATESTRUCTW,
            CW_USEDEFAULT, HICON, HMENU, HWND_MESSAGE, IDC_ARROW, IDI_APPLICATION, IMAGE_ICON,
            LR_DEFAULTCOLOR, LR_DEFAULTSIZE, LR_SHARED, WINDOW_EX_STYLE, WINDOW_LONG_PTR_INDEX,
            WINDOW_STYLE, WM_CREATE, WM_DESTROY, WNDCLASSEXW, WNDCLASS_STYLES, WS_CLIPCHILDREN,
            WS_CLIPSIBLINGS, WS_VISIBLE,
        },
    },
};

pub fn default_procedure(event: WindowEvent) -> LRESULT {
    unsafe { DefWindowProcW(event.hwnd, event.msg, event.wparam, event.lparam) }
}

pub struct WindowEvent {
    pub hwnd: HWND,
    pub msg: u32,
    pub wparam: WPARAM,
    pub lparam: LPARAM,
}

pub type WindowEventHandler = Rc<dyn Fn(NonNull<Window>, WindowEvent) -> LRESULT>;

#[derive(Clone)]
pub struct WindowAttributes {
    pub title: Option<String>,
    pub id: Option<usize>,
    pub style: Option<WINDOW_STYLE>,
    pub ex_style: Option<WINDOW_EX_STYLE>,
}

impl Default for WindowAttributes {
    fn default() -> Self {
        Self {
            title: None,
            id: None,
            style: None,
            ex_style: None,
        }
    }
}

pub struct Window {
    pub class: WNDCLASSEXW,
    pub attributes: WindowAttributes,
    pub hwnd: HWND,
    pub events: Option<WindowEventHandler>,
}

impl Default for Window {
    fn default() -> Self {
        let class = WNDCLASSEXW {
            cbSize: u32::try_from(std::mem::size_of::<WNDCLASSEXW>()).unwrap(),
            style: WNDCLASS_STYLES::default(),
            lpfnWndProc: Some(Self::wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: i32::try_from(std::mem::size_of::<Self>()).unwrap(),
            hInstance: app::module_handle().into(),
            hCursor: unsafe { LoadCursorW(None, IDC_ARROW).unwrap() },
            hbrBackground: HBRUSH((COLOR_WINDOW.0 + 1) as _),
            lpszMenuName: PCWSTR::null(),
            lpszClassName: w!("DefaultWindow"),
            hIcon: Self::load_icon(),
            hIconSm: Self::load_icon(),
        };

        Self {
            class: class,
            attributes: WindowAttributes::default(),
            hwnd: HWND::default(),
            events: None,
        }
    }
}

impl Window {
    pub fn register(mut self) -> Self {
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

        self
    }

    pub fn create(self, show: bool) -> Box<Self> {
        let mut window = Box::new(self);

        unsafe {
            let _hwnd = CreateWindowExW(
                match window.attributes.ex_style {
                    None => WINDOW_EX_STYLE::default(),
                    Some(ex_style) => ex_style,
                },
                window.class.lpszClassName,
                match &window.attributes.title {
                    None => PCWSTR::null(),
                    Some(title) => PCWSTR(HSTRING::from(title).as_ptr()),
                },
                match window.attributes.style {
                    None => WINDOW_STYLE::default(),
                    Some(style) => style,
                } | match show {
                    true => WS_VISIBLE,
                    false => WINDOW_STYLE::default(),
                },
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                window.class.hInstance,
                Some(window.borrow_mut() as *mut Self as _),
            )
            .unwrap();
        }

        window
    }

    pub fn create_child(self, parent: HWND, show: bool) -> Box<Self> {
        let mut window = Box::new(self);

        unsafe {
            let _hwnd = CreateWindowExW(
                match window.attributes.ex_style {
                    None => WINDOW_EX_STYLE::default(),
                    Some(ex_style) => ex_style,
                },
                window.class.lpszClassName,
                match &window.attributes.title {
                    None => PCWSTR::null(),
                    Some(title) => PCWSTR(HSTRING::from(title).as_ptr()),
                },
                match window.attributes.style {
                    None => WINDOW_STYLE::default(),
                    Some(style) => style,
                } | match show {
                    true => WS_VISIBLE,
                    false => WINDOW_STYLE::default(),
                },
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                parent,
                match window.attributes.id {
                    None => HMENU::default(),
                    Some(id) => HMENU((id) as *mut c_void),
                },
                window.class.hInstance,
                Some(window.borrow_mut() as *mut Self as _),
            )
            .unwrap();
        }

        window
    }

    pub fn create_message_only(self) -> Box<Self> {
        let mut window = Box::new(self);

        unsafe {
            let _hwnd = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                window.class.lpszClassName,
                match &window.attributes.title {
                    None => PCWSTR::null(),
                    Some(title) => PCWSTR(HSTRING::from(title).as_ptr()),
                },
                WINDOW_STYLE::default(),
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                HWND_MESSAGE,
                None,
                window.class.hInstance,
                Some(window.borrow_mut() as *mut Self as _),
            )
            .unwrap();
        }

        window
    }

    fn load_icon() -> HICON {
        match unsafe {
            LoadImageW(
                app::module_handle(),
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

        default_procedure(event)
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
        self.attributes.id == other.attributes.id
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
        (self.attributes.id).cmp(&other.attributes.id)
    }
}
