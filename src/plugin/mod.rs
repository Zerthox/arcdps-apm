use crate::{data::Data, stats::Stats};
use arc_util::ui::{Window, WindowOptions};

pub mod events;
pub mod ui;

#[derive(Debug)]
pub struct Plugin {
    data: Data,
    stats: Window<Stats>,
}

impl Plugin {
    pub fn new() -> Self {
        Self {
            data: Data::with_defaults(),
            stats: Window::new(
                WindowOptions {
                    auto_resize: true,
                    ..WindowOptions::new("APM Stats")
                },
                Stats::new(),
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
