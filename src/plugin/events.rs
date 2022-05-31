use crate::PLUGIN;

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
                            self.counter.start(event.time);
                        }
                    }
                    StateChange::ExitCombat => {
                        if is_self {
                            self.counter.reset();
                        }
                    }
                    StateChange::None => match event.is_activation.into() {
                        Activation::Reset | Activation::CancelFire => {
                            self.api.get(event.skill_id, Self::skill_cast);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }

    fn skill_cast(skill_id: u32, is_auto: bool) {
        // this is a bit hacky but whatever
        PLUGIN
            .lock()
            .unwrap()
            .counter
            .register_cast(skill_id, is_auto);
    }
}
