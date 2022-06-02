pub mod events;
pub mod ui;

use crate::{data::Data, stats::Stats};
use arc_util::{
    settings::Settings,
    ui::{Window, WindowOptions},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const SETTINGS_FILE: &str = "arcdps_apm.json";

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
        let mut settings = Settings::from_file(SETTINGS_FILE);
        settings.load_component(&mut self.stats);
    }

    pub fn unload(&mut self) {
        let mut settings = Settings::from_file(SETTINGS_FILE);
        settings.store_data("version", VERSION);
        settings.store_component(&self.stats);
        settings.save_file();
    }
}
