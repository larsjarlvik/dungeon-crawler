use serde_derive::{Deserialize, Serialize};

use crate::utils;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Settings {
    pub brightness: f32,
    pub contrast: f32,
    pub render_scale: f32,
    pub shadow_map_scale: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            brightness: 0.0,
            contrast: 2.2,
            render_scale: 1.0,
            shadow_map_scale: 2.0,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        match utils::read_file("settings.json") {
            Ok(json) => serde_json::from_str(&json).unwrap_or(Self::default()),
            Err(_) => Self::default(),
        }
    }

    pub fn store(&self) {
        let preferences = serde_json::to_string(self).unwrap();
        utils::write_file("settings.json", &preferences);
    }
}
