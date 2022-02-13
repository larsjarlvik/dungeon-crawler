use serde_derive::{Deserialize, Serialize};

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
        let cfg: Settings = confy::load("dungeon_crawler").unwrap_or(Self::default());
        cfg
    }

    pub fn store(&self) {
        confy::store("dungeon_crawler", self).unwrap();
    }
}
