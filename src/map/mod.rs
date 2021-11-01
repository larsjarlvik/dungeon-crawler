use std::env;

use crate::{engine, world::resources};
use cgmath::*;
use rand::{prelude::StdRng, SeedableRng};
use specs::WorldExt;
mod generator;
mod tile;

pub struct Map {
    seed: u64,
    tile_size: f32,
    tile: engine::model::GltfModel,
    decor: engine::model::GltfModel,
    placed_tiles: Vec<tile::Tile>,
    grid_size: usize,
    number_of_tiles: usize,
}

impl Map {
    pub fn new(engine: &engine::Engine, seed: u64, grid_size: usize) -> Self {
        let tile = engine.load_model("models/catacombs.glb");
        let decor = engine.load_model("models/decor.glb");
        let number_of_tiles = 25;

        Self {
            tile,
            decor,
            tile_size: 14.0,
            placed_tiles: vec![],
            seed,
            grid_size,
            number_of_tiles,
        }
    }

    pub fn update(&mut self, engine: &engine::Engine, world: &mut specs::World) {
        let mut rng = StdRng::seed_from_u64(self.seed);

        let frustum = {
            let camera = world.read_resource::<resources::Camera>();
            camera.frustum
        };

        let tile = &self.tile;
        let decor = &self.decor;
        self.placed_tiles.iter_mut().for_each(|t| {
            if frustum.test_bounding_box(&t.bounding_box) {
                t.build(engine, world, &mut rng, &tile, &decor)
            } else {
                t.destroy(world);
            }
        });
    }

    pub fn generate(&mut self) {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let mut tiles = generator::generate(&mut rng, self.grid_size, self.number_of_tiles);

        for x in 0..(self.grid_size * 2) {
            for z in 0..(self.grid_size * 2) {
                let tx = x as i32 - self.grid_size as i32;
                let tz = z as i32 - self.grid_size as i32;

                self.placed_tiles.push(if let Some(t) = &mut tiles[x][z] {
                    tile::Tile::new(tx, tz, self.tile_size, &t.entrances, &mut rng)
                } else {
                    tile::Tile::new_known(tx, tz, self.tile_size, "tile-empty", tile::TileDecor { decor: vec![] }, 0.0)
                })
            }
        }
    }

    pub fn single_tile(&mut self, engine: &engine::Engine, world: &mut specs::World, tile_name: &str) {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let decor = match tile::get_decor("edit") {
            Ok(variants) => variants[0].clone(),
            Err(err) => {
                println!("{}", err);
                tile::TileDecor { decor: vec![] }
            }
        };

        let tile = tile::Tile::new_known(0, 0, self.tile_size, &tile_name, decor, 0.0);
        tile.add_room(engine, world, &self.tile, Vector3::zero(), 0.0);
        tile.add_decor(engine, world, &mut rng, Vector3::zero(), 0.0, &self.decor);
        tile.add_grid(world, Vector3::zero());
    }

    pub fn reset(&mut self) {
        self.placed_tiles.clear();
    }
}

pub fn edit_mode() -> Option<String> {
    let args: Vec<String> = env::args().collect();

    if let Some(pos) = args.iter().position(|a| a == "--edit") {
        if let Some(tile) = args.get(pos + 1) {
            Some(tile.clone())
        } else {
            None
        }
    } else {
        None
    }
}

