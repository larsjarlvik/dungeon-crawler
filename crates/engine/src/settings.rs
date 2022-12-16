use crate::file;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Settings {
    pub contrast: f32,
    pub bloom: f32,
    pub render_scale: f32,
    pub shadow_map_scale: f32,
    pub show_fps: bool,
    pub window_size: [u32; 2],
    pub window_pos: [i32; 2],
    pub fullscreen: bool,
    pub smaa: bool,
    pub sharpen: bool,
    pub ui_scale: f32,
    pub audio_effects: f32,
    pub audio_ambient: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            contrast: 1.0,
            bloom: 2.5,
            render_scale: 1.0,
            shadow_map_scale: 2.0,
            show_fps: true,
            window_size: [1280, 720],
            window_pos: [100, 100],
            fullscreen: false,
            smaa: true,
            sharpen: false,
            ui_scale: 1.0,
            audio_effects: 1.0,
            audio_ambient: 0.5,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        match file::read_file("settings.json") {
            Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn store(&self) {
        let preferences = serde_json::to_string(self).unwrap();
        file::write_file("settings.json", &preferences);
    }
}
