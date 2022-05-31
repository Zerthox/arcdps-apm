use serde::Deserialize;
use std::{
    collections::HashMap,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

const API_URL: &str = "http://api.guildwars2.com/";

type Cache = Arc<Mutex<HashMap<u32, bool>>>;
type Callback = fn(u32, bool);

#[derive(Debug)]
pub struct Api {
    sender: Sender<Message>,
    cache: Cache,
}

impl Api {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        let cache = Arc::new(Mutex::new(HashMap::new()));
        let worker_cache = cache.clone();

        thread::spawn(move || Self::worker(worker_cache, receiver));

        Self { sender, cache }
    }

    /// Worker thread.
    fn worker(cache: Cache, receiver: Receiver<Message>) {
        loop {
            if let Ok(Message { skill_id, callback }) = receiver.recv() {
                if let Some(skill) = Self::fetch(skill_id) {
                    let is_auto = matches!(skill.slot.as_str(), "Weapon_1" | "Downed_1");
                    cache.lock().unwrap().insert(skill_id, is_auto);
                    callback(skill_id, is_auto);
                }
            }
        }
    }

    /// Fetches a skill from the GW2 API.
    fn fetch(skill_id: u32) -> Option<ApiSkill> {
        let response = ureq::get(&format!("{}/v2/skills?lang=en&ids={}", API_URL, skill_id))
            .call()
            .ok()?;
        response.into_json().ok()
    }

    /// Retrieves skill information from the cache or fetches it.
    pub fn get(&mut self, skill_id: u32, callback: Callback) {
        if let Some(is_auto) = self.cache.lock().unwrap().get(&skill_id) {
            callback(skill_id, *is_auto);
        } else {
            let _ = self.sender.send(Message { skill_id, callback });
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ApiSkill {
    pub id: u32,
    pub name: String,

    #[serde(default)]
    pub slot: String,
}

#[derive(Debug)]
struct Message {
    pub skill_id: u32,
    pub callback: Callback,
}
