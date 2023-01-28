use image::DynamicImage;
use serde_json::Value;

#[derive(Debug)]
pub struct Config {
    pub tile_size: u32,
    pub tiles: DynamicImage,
    pub variants: Vec<Variant>,
    pub seed: u64,
}

impl Config {
    pub fn new(seed: u64, tile_set: &str, variants: Vec<Variant>) -> Self {
        let config = {
            let text = engine::file::read_string(format!("maps/{tile_set}/config.json").as_str());
            serde_json::from_str::<Value>(&text).unwrap()
        };

        let tile_size = config["tile_size"].as_u64().expect("tile_size not defined!") as u32;
        let tile_set = engine::file::read_bytes(format!("maps/{tile_set}/tiles.png").as_str());

        let tiles = image::load_from_memory(&tile_set).expect("Failed to decude tile_set!");

        Self {
            tile_size,
            tiles,
            variants,
            seed,
        }
    }
}

#[derive(Debug)]
pub struct Variant {
    pub index: usize,
    pub weight: f32,
    pub entrance: bool,
    pub exit: bool,
}

impl Default for Variant {
    fn default() -> Self {
        Self {
            index: Default::default(),
            weight: 1.0,
            entrance: false,
            exit: false,
        }
    }
}
