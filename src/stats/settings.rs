use super::Stats;
use arc_util::settings::HasSettings;

impl HasSettings for Stats {
    type Settings = ();

    const SETTINGS_ID: &'static str = "stats";

    fn current_settings(&self) -> Self::Settings {}

    fn load_settings(&mut self, _: Self::Settings) {}
}
