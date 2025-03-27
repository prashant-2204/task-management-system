use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub database_path: String,
    pub worker_threads: usize,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string("config.json")
            .unwrap_or_else(|_| Self::default_config());
        Ok(serde_json::from_str(&config_str)?)
    }

    fn default_config() -> String {
        let default = Config {
            database_path: "tasks.db".to_string(),
            worker_threads: 4,
        };
        serde_json::to_string(&default).unwrap()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database_path: "tasks.db".to_string(),
            worker_threads: 4,
        }
    }
}