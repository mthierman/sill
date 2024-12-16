use windows::Win32::UI::WindowsAndMessaging::{WINDOW_EX_STYLE, WINDOW_STYLE};

use super::{Window, WindowBuilder, WindowEventHandler};

#[derive(Default)]
pub struct WindowManager {
    builder: WindowBuilder,
}

impl WindowManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_builder(mut self, builder: WindowBuilder) -> Self {
        self.builder = builder;

        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.builder.attributes.title = Some(title.to_owned());

        self
    }

    pub fn id(mut self, id: usize) -> Self {
        self.builder.attributes.id = Some(id);

        self
    }

    pub fn style(mut self, style: WINDOW_STYLE) -> Self {
        self.builder.attributes.style = Some(style);

        self
    }

    pub fn ex_style(mut self, ex_style: WINDOW_EX_STYLE) -> Self {
        self.builder.attributes.ex_style = Some(ex_style);

        self
    }

    pub fn hidden(mut self) -> Self {
        self.builder.attributes.hidden = true;

        self
    }

    pub fn events(mut self, events: WindowEventHandler) -> Self {
        self.builder.events = Some(events.clone());

        self
    }

    pub fn spawn(&self) -> Box<Window> {
        self.builder.build()
    }
}
