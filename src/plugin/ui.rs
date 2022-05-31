use super::Plugin;
use arc_util::ui::{Component, Hideable};
use arcdps::imgui::Ui;

impl Plugin {
    /// Callback for standalone UI creation.
    pub fn render_windows(&mut self, ui: &Ui, not_loading: bool) {
        if not_loading {
            self.counter.render(ui, &());
        }
    }

    /// Callback for settings UI creation.
    pub fn render_settings(&mut self, ui: &Ui) {
        ui.text("Nothing here yet...");
    }

    /// Callback for ArcDPS option checkboxes.
    pub fn render_window_options(&mut self, ui: &Ui, option_name: Option<&str>) -> bool {
        if option_name.is_none() {
            ui.checkbox("APM Counter", self.counter.visible_mut());
        }
        false
    }

    /// Handles a key event.
    pub fn key_event(&mut self, _key: usize, _down: bool, _prev_down: bool) -> bool {
        true
    }
}
