use serde_derive::{Deserialize, Serialize};

use crate::utils;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Settings {
    pub brightness: f32,
    pub contrast: f32,
    pub render_scale: f32,
    pub shadow_map_scale: f32,
    pub show_fps: bool,
    pub window_size: [u32; 2],
    pub window_pos: [i32; 2],
    pub fullscreen: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            brightness: 0.0,
            contrast: 2.2,
            render_scale: 1.0,
            shadow_map_scale: 2.0,
            show_fps: false,
            window_size: [1280, 720],
            window_pos: [100, 100],
            fullscreen: false,
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
