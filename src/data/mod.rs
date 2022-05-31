use std::collections::HashMap;

/// Default skill data.
pub const SKILLS: &[(u32, bool)] = &include!(concat!(env!("OUT_DIR"), "/skill_data.rs"));

/// Plugin data.
#[derive(Debug)]
pub struct Data {
    skills: HashMap<u32, bool>,
}

impl Data {
    /// Creates a new data set with the given parameters.
    pub fn new(skills: impl IntoIterator<Item = (u32, bool)>) -> Self {
        Self {
            skills: skills.into_iter().collect(),
        }
    }

    /// Creates a new data set with the defaults.
    pub fn with_defaults() -> Self {
        Self::new(SKILLS.iter().copied())
    }

    /// Checks whether the skill is an action.
    pub fn is_action(&self, id: u32) -> bool {
        self.skills.get(&id).copied().unwrap_or(false)
    }
}
