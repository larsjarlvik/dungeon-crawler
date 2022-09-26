use engine::file;
use rand::{prelude::StdRng, Rng};
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

pub fn get_decor(tile: &str, rng: &mut StdRng) -> Vec<Decor> {
    if tile.contains("empty") {
        return vec![];
    }

    let path = format!("tiles/{}.json", tile);

    match serde_json::from_str::<Vec<TileDecor>>(file::read_string(&path).as_str()) {
        Ok(variants) if !variants.is_empty() => variants[rng.gen_range(0..variants.len())].decor.clone(),
        Ok(_) => {
            vec![]
        }
        Err(err) => {
            panic!("{}", err);
        }
    }
}
