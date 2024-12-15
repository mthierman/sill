use crate::window::{Window, WindowAttributes, WindowEventHandler};
use windows::Win32::UI::WindowsAndMessaging::{WINDOW_EX_STYLE, WINDOW_STYLE};

pub struct WindowBuilder {
    pub attributes: WindowAttributes,
    pub events: Option<WindowEventHandler>,
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            attributes: WindowAttributes::default(),
            events: None,
        }
    }
}

impl WindowBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn title(mut self, title: &str) -> Self {
        self.attributes.title = title.to_owned();

        self
    }

    pub fn id(mut self, id: usize) -> Self {
        self.attributes.id = id;

        self
    }

    pub fn style(mut self, style: WINDOW_STYLE) -> Self {
        self.attributes.style = style;

        self
    }

    pub fn ex_style(mut self, ex_style: WINDOW_EX_STYLE) -> Self {
        self.attributes.ex_style = ex_style;

        self
    }

    pub fn events(mut self, events: WindowEventHandler) -> Self {
        self.events = Some(events.clone());

        self
    }

    pub fn create(&self, show: bool) -> Box<Window> {
        let mut window = Box::new(Window::default());
        window.attributes = self.attributes.clone();
        window.events = self.events.clone();

        window.register().create(show)
    }

    pub fn create_message_only(&self) -> Box<Window> {
        let mut window = Box::new(Window::default());
        window.attributes = self.attributes.clone();
        window.events = self.events.clone();

        window.register().create_message_only()
    }
}
