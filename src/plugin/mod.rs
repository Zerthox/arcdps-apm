use crate::{api::Api, apm::Counter};
use arc_util::ui::{Window, WindowOptions};

pub mod events;
pub mod ui;

#[derive(Debug)]
pub struct Plugin {
    api: Api,
    counter: Window<Counter>,
}

impl Plugin {
    pub fn new() -> Self {
        Self {
            api: Api::new(),
            counter: Window::new(
                WindowOptions {
                    auto_resize: true,
                    ..WindowOptions::new("APM Counter")
                },
                Counter::new(),
            ),
        }
    }

    pub fn load(&mut self) {
        // TODO: load settings
    }

    pub fn unload(&mut self) {
        // TODO: save settings
    }
}
