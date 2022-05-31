use arc_util::ui::{Component, Windowable};
use arcdps_imgui::Ui;
use std::time::Duration;
use windows::Win32::Media::timeGetTime;

#[derive(Debug)]
pub struct Counter {
    /// Start time of combat.
    start: Option<u64>,

    /// Cast counter.
    count: u64,

    /// Last completed skill cast.
    last_cast: Option<(u32, bool)>,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            start: None,
            count: 0,
            last_cast: None,
        }
    }

    /// Start tracking.
    pub fn start(&mut self, time: u64) {
        self.start = Some(time);
    }

    /// Resets combat state & counter.
    pub fn reset(&mut self) {
        self.start = None;
        self.count = 0;
    }

    /// Registers a cast event.
    pub fn register_cast(&mut self, skill_id: u32, is_auto: bool) {
        if !is_auto {
            self.count += 1;
        }
        self.last_cast = Some((skill_id, is_auto));
    }
}

impl Component<'_> for Counter {
    type Props = ();

    fn render(&mut self, ui: &Ui, _: &Self::Props) {
        if let Some(start) = self.start {
            let now = unsafe { timeGetTime() } as u64;
            let duration = Duration::from_millis(now - start);
            let apm = (60.0 * self.count as f64) / duration.as_secs() as f64;
            ui.text(format!("APM: {:.2}", apm));
        } else {
            ui.text("APM: -");
        }
        if let Some((skill_id, is_auto)) = self.last_cast {
            ui.text(format!(
                "Last: {}{}",
                skill_id,
                if is_auto { " (auto)" } else { "" }
            ));
        } else {
            ui.text("Last: -");
        }
    }
}

impl Windowable<'_> for Counter {
    const CONTEXT_MENU: bool = false;
}
