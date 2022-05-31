mod apm;
mod data;
mod plugin;

use arcdps::{arcdps_export, imgui::Ui, Agent, CombatEvent};
use once_cell::sync::Lazy;
use plugin::Plugin;
use std::{error::Error, sync::Mutex};

/// Main plugin instance.
static PLUGIN: Lazy<Mutex<Plugin>> = Lazy::new(|| Mutex::new(Plugin::new()));

// create exports for arcdps
arcdps_export! {
    name: "APM",
    sig: 0x04afedc5, // random id
    init,
    release,
    combat,
    imgui,
    options_end,
    options_windows,
    wnd_filter,
}

fn init() -> Result<(), Box<dyn Error>> {
    // TODO: use error
    PLUGIN.lock().unwrap().load();
    Ok(())
}

fn release() {
    PLUGIN.lock().unwrap().unload()
}

fn combat(
    event: Option<&CombatEvent>,
    src: Option<Agent>,
    dest: Option<Agent>,
    skill_name: Option<&str>,
    id: u64,
    revision: u64,
) {
    PLUGIN
        .lock()
        .unwrap()
        .area_event(event, src, dest, skill_name, id, revision)
}

fn imgui(ui: &Ui, not_loading_or_character_selection: bool) {
    PLUGIN
        .lock()
        .unwrap()
        .render_windows(ui, not_loading_or_character_selection)
}

fn options_windows(ui: &Ui, window_name: Option<&str>) -> bool {
    PLUGIN
        .lock()
        .unwrap()
        .render_window_options(ui, window_name)
}

fn options_end(ui: &Ui) {
    PLUGIN.lock().unwrap().render_settings(ui)
}

fn wnd_filter(key: usize, key_down: bool, prev_key_down: bool) -> bool {
    PLUGIN
        .lock()
        .unwrap()
        .key_event(key, key_down, prev_key_down)
}
