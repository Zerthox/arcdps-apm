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

    /// Retrieves a skill from the data set.
    pub fn skill(&self, id: u32) -> Option<bool> {
        self.skills.get(&id).copied()
    }
}
