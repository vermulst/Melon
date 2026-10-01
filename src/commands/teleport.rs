use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

#[derive(Clone, Debug)]
pub struct BackLocation {
    pub position: (f64, f64, f64),
    pub yaw: f32,
    pub pitch: f32,
    pub world_name: String,
}

pub struct TeleportManager {
    back_locations: Mutex<HashMap<String, BackLocation>>,
}

impl TeleportManager {
    pub fn new() -> Self {
        Self {
            back_locations: Mutex::new(HashMap::new()),
        }
    }

    pub fn set_back_location(&self, uuid: String, loc: BackLocation) {
        if let Ok(mut map) = self.back_locations.lock() {
            map.insert(uuid, loc);
        }
    }

    pub fn get_back_location(&self, uuid: &str) -> Option<BackLocation> {
        if let Ok(map) = self.back_locations.lock() {
            map.get(uuid).cloned()
        } else {
            None
        }
    }
}

pub static TELEPORT_MANAGER: LazyLock<TeleportManager> = LazyLock::new(TeleportManager::new);