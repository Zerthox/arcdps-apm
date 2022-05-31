use super::Plugin;
use arc_util::api::{Activation, StateChange};
use arcdps::{Agent, CombatEvent};

impl Plugin {
    /// Handles a combat event from area stats.
    pub fn area_event(
        &mut self,
        event: Option<&CombatEvent>,
        src: Option<Agent>,
        _dest: Option<Agent>,
        _skill_name: Option<&str>,
        _event_id: u64,
        _revision: u64,
    ) {
        if let Some(src) = src {
            let is_self = src.self_ != 0;

            if let Some(event) = event {
                match event.is_statechange.into() {
                    StateChange::EnterCombat => {
                        if is_self {
                            self.stats.start(event.time);
                        }
                    }
                    StateChange::ExitCombat => {
                        if is_self {
                            self.stats.stop();
                        }
                    }
                    StateChange::None => match event.is_activation.into() {
                        Activation::Reset | Activation::CancelFire => {
                            let skill_id = event.skill_id;
                            let is_action = self.data.is_action(skill_id);
                            self.stats.register_cast(event.time, is_action);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}
