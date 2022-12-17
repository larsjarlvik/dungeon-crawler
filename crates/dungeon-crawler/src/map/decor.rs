use engine::file;
use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Decor {
    pub name: String,
    pub pos: [i32; 2],
    pub rotation: f32,
    pub rotation_rng: f32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TileDecor {
    pub decor: Vec<Decor>,
}

pub fn get_decor(tile: &str, variant: usize) -> Vec<Decor> {
    if tile.contains("empty") {
        return vec![];
    }

    let path = format!("tiles/{}.json", tile);

    match serde_json::from_str::<Vec<TileDecor>>(file::read_string(&path).as_str()) {
        Ok(variants) if !variants.is_empty() => variants[variant].decor.clone(),
        Ok(_) => vec![],
        Err(err) => panic!("{}", err),
    }
}

pub fn get_variants_count(tile: &str) -> usize {
    if tile.contains("empty") {
        return 0;
    }

    let path = format!("tiles/{}.json", tile);
    match serde_json::from_str::<Vec<TileDecor>>(file::read_string(&path).as_str()) {
        Ok(variants) => variants.len(),
        Err(err) => panic!("{}", err),
    }
}
