use rand::{rngs::StdRng, SeedableRng};
mod map;
pub use map::Map;

pub fn load_map(tile_set: &str, seed: u64) -> map::Map {
    let mut rng = StdRng::seed_from_u64(seed);

    let config = map::Config::new(tile_set, vec![]);
    let mut map = map::Map::new(12, 20..40);
    map.build(&mut rng, &config, false);

    map
}
