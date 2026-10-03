use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpawnSettings {
    pub force_spawn_on_first_join: bool,
    pub force_spawn_on_every_join: bool,
    pub force_spawn_on_respawn: bool,
}

impl Default for SpawnSettings {
    fn default() -> Self {
        Self {
            force_spawn_on_first_join: true,
            force_spawn_on_every_join: false,
            force_spawn_on_respawn: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SpawnPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub spawn: SpawnSettings,
    #[serde(default)]
    pub spawn_points: HashMap<String, SpawnPoint>,
    #[serde(default)]
    pub warps: HashMap<String, SpawnPoint>,
}

pub static CONFIG: LazyLock<Mutex<Config>> = LazyLock::new(|| Mutex::new(Config::default()));

fn config_path(folder: &Path) -> PathBuf {
    folder.join("config.toml")
}

pub fn load(folder: &Path) {
    let path = config_path(folder);

    let cfg = match fs::read_to_string(&path) {
        Ok(content) => match toml::from_str::<Config>(&content) {
            Ok(c) => {
                tracing::info!("Loaded config from {:?}", path);
                c
            }
            Err(e) => {
                tracing::error!("Bad config.toml ({e}), using defaults");
                Config::default()
            }
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            tracing::info!("No config.toml, writing defaults");
            Config::default()
        }
        Err(e) => {
            tracing::error!("Could not read config: {e}");
            Config::default()
        }
    };

    *CONFIG.lock().unwrap() = cfg;
    save(folder);
}

pub fn save(folder: &Path) {
    if let Err(e) = fs::create_dir_all(folder) {
        tracing::error!("mkdir {:?} failed: {e}", folder);
        return;
    }
    let cfg = CONFIG.lock().unwrap().clone();
    match toml::to_string_pretty(&cfg) {
        Ok(s) => {
            if let Err(e) = fs::write(config_path(folder), s) {
                tracing::error!("write config.toml failed: {e}");
            }
        }
        Err(e) => tracing::error!("serialize config failed: {e}"),
    }
}