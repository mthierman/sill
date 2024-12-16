use crate::window::{Window, WindowAttributes, WindowEventHandler};
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{WINDOW_EX_STYLE, WINDOW_STYLE},
};

#[derive(Default)]
pub struct WindowBuilder {
    pub attributes: WindowAttributes,
    pub events: Option<WindowEventHandler>,
}

impl WindowBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn title(mut self, title: &str) -> Self {
        self.attributes.title = Some(title.to_owned());

        self
    }

    pub fn id(mut self, id: usize) -> Self {
        self.attributes.id = Some(id);

        self
    }

    pub fn style(mut self, style: WINDOW_STYLE) -> Self {
        self.attributes.style = Some(style);

        self
    }

    pub fn ex_style(mut self, ex_style: WINDOW_EX_STYLE) -> Self {
        self.attributes.ex_style = Some(ex_style);

        self
    }

    pub fn hidden(mut self) -> Self {
        self.attributes.hidden = true;

        self
    }

    pub fn events(mut self, events: WindowEventHandler) -> Self {
        self.events = Some(events.clone());

        self
    }

    pub fn create(&self) -> Box<Window> {
        let mut window = Box::new(Window::new());
        window.attributes = self.attributes.clone();
        window.events = self.events.clone();

        window.register().create()
    }

    pub fn create_child(&self, parent: HWND) -> Box<Window> {
        let mut window = Box::new(Window::new());
        window.attributes = self.attributes.clone();
        window.events = self.events.clone();

        window.register().create_child(parent)
    }

    pub fn create_message_only(&self) -> Box<Window> {
        let mut window = Box::new(Window::new());
        window.attributes = self.attributes.clone();
        window.events = self.events.clone();

        window.register().create_message_only()
    }
}
