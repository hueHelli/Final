use crate::fs;
use log::{info, warn};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct EngineConfig {
    pub window: WindowConfig,
}

impl EngineConfig {
    /// Load configuration from assets/config/engine.toml
    /// or fall back to defaults if loading fails.
    pub fn load() -> Self {
        // Where we expect the config: assets/config/engine.toml
        let path = "config/engine.toml";
        match fs::read_asset_string(path) {
            Ok(contents) => match toml::from_str::<EngineConfig>(&contents) {
                Ok(cfg) => {
                    info!("Loaded engine config from {}", path);
                    cfg
                }
                Err(err) => {
                    warn!(
                        "Failed to parse engine config '{}': {}. Using defaults",
                        path, err
                    );
                    Self::default()
                }
            },
            Err(err) => {
                warn!(
                    "Failed to read engine config '{}': {}. Using defaults",
                    path, err
                );
                Self::default()
            }
        }
    }
}

impl Default for EngineConfig {
    fn default() -> Self {
        info!("Using built-in default EngineConfig");
        Self {
            window: WindowConfig {
                width: 800,
                height: 600,
                title: "Final - HelloWorld".to_string(),
            },
        }
    }
}
