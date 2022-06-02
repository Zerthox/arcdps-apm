use arc_util::ui::{Component, Windowable};
use arcdps_imgui::Ui;
use std::time::Duration;

pub mod settings;

#[derive(Debug)]
pub struct Stats {
    /// Combat state.
    active: bool,

    /// Start time of combat.
    start: u64,

    /// Time of last registered cast.
    now: u64,

    /// Casts counter.
    casts: u64,

    /// Actions counter.
    actions: u64,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            active: false,
            start: 0,
            now: 0,
            casts: 0,
            actions: 0,
        }
    }

    /// Start tracking & reset counters.
    pub fn start(&mut self, time: u64) {
        self.active = true;
        self.start = time;
        self.now = 0;
        self.casts = 0;
        self.actions = 0;
    }

    /// Stop tracking.
    pub fn stop(&mut self) {
        self.active = false;
    }

    /// Registers a cast event.
    pub fn register_cast(&mut self, time: u64, is_action: bool) {
        if self.active {
            // update time
            if time > self.now {
                self.now = time;
            }

            // update counters
            self.casts += 1;
            if is_action {
                self.actions += 1;
            }
        }
    }

    fn per_minute(count: u64, start: u64, now: u64) -> f64 {
        let duration = Duration::from_millis(now - start);
        (60.0 * count as f64) / duration.as_secs() as f64
    }
}

impl Component<'_> for Stats {
    type Props = ();

    fn render(&mut self, ui: &Ui, _: &Self::Props) {
        if self.start != 0 && self.now > self.start {
            ui.text(format!(
                "Casts:   {:>6.2}/m",
                Self::per_minute(self.casts, self.start, self.now)
            ));
            ui.text(format!(
                "Actions: {:>6.2}/m",
                Self::per_minute(self.actions, self.start, self.now)
            ));
        } else {
            ui.text("Casts:        -/m");
            ui.text("Actions:      -/m");
        }
    }
}

impl Windowable<'_> for Stats {
    const CONTEXT_MENU: bool = true;
}
