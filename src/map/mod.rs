use crate::{
    engine,
    world::{self, resources},
};
use cgmath::*;
use rand::{prelude::StdRng, Rng, SeedableRng};
use specs::WorldExt;
mod tile;

#[derive(Clone)]
struct Tile {
    entrances: [bool; 4],
    x: i32,
    z: i32,
}

pub struct Map {
    seed: u64,
    tile_size: f32,
    tile: engine::model::GltfModel,
    decor: engine::model::GltfModel,
    placed_tiles: Vec<tile::Tile>,
    grid_size: usize,
    number_of_tiles: i32,
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

    pub fn update(&mut self, engine: &engine::Engine, world: &mut world::World) {
        let mut rng = StdRng::seed_from_u64(self.seed);

        let frustum = {
            let camera = world.components.read_resource::<resources::Camera>();
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
        let mut tiles = self.create_tiles(&mut rng);
        self.add_entrances(&mut tiles);

        for x in 0..(self.grid_size * 2) {
            for z in 0..(self.grid_size * 2) {
                if let Some(tile) = &mut tiles[x][z] {
                    self.placed_tiles
                        .push(tile::Tile::new(tile.x, tile.z, self.tile_size, &tile.entrances));
                }
            }
        }
    }

    pub fn single_tile(&mut self, engine: &engine::Engine, world: &mut world::World, tile_name: &str) {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let tile = tile::Tile::new_known(0, 0, self.tile_size, &tile_name, 0.0);

        tile.add_room(engine, world, &self.tile, Vector3::zero(), 0.0);
        tile.add_grid(world, Vector3::zero());

        match tile.get_decor("edit") {
            Ok(variants) => {
                if let Some(tile_decor) = variants.get(0) {
                    tile.add_decor(engine, world, &mut rng, Vector3::zero(), 0.0, &tile_decor, &self.decor);
                }
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }

    fn create_tiles(&self, rng: &mut StdRng) -> Vec<Vec<Option<Tile>>> {
        let mut tiles = vec![vec![None; self.grid_size * 2]; self.grid_size * 2];
        let mut taken_positions = vec![(0, 0)];

        tiles[self.grid_size][self.grid_size] = Some(Tile {
            x: 0,
            z: 0,
            entrances: [false, false, false, false],
        });

        let random_compare_start = 0.2f32;
        let random_compare_end = 0.01f32;

        for i in 0..(self.number_of_tiles - 1) {
            let random_perc = (i as f32) / (self.number_of_tiles - 1) as f32;
            let random_compare = vec1(random_compare_start).lerp(vec1(random_compare_end), random_perc).x;
            let mut check_pos = self.new_position(rng, &taken_positions);

            if self.number_of_neighbors(&check_pos, &taken_positions) > 1 && rng.gen::<f32>() > random_compare * 0.5 {
                for _ in 0..100 {
                    check_pos = self.selective_new_position(rng, &taken_positions);
                    if self.number_of_neighbors(&check_pos, &taken_positions) == 0 {
                        break;
                    }
                }
            }

            tiles[check_pos.0 as usize + self.grid_size][check_pos.1 as usize + self.grid_size] = Some(Tile {
                x: check_pos.0,
                z: check_pos.1,
                entrances: [false, false, false, false],
            });

            taken_positions.insert(0, check_pos);
        }

        tiles
    }

    fn add_entrances(&self, tiles: &mut Vec<Vec<Option<Tile>>>) {
        for x in 0..(self.grid_size * 2) {
            for z in 0..(self.grid_size * 2) {
                self.add_entrance(tiles, x as i32, z as i32, 0, -1, 0);
                self.add_entrance(tiles, x as i32, z as i32, 1, 0, 1);
                self.add_entrance(tiles, x as i32, z as i32, 0, 1, 2);
                self.add_entrance(tiles, x as i32, z as i32, -1, 0, 3);
            }
        }
    }

    fn add_entrance(&self, tiles: &mut Vec<Vec<Option<Tile>>>, x: i32, z: i32, ox: i32, oz: i32, entrance: usize) {
        let existing = tiles.clone();

        if let Some(tile) = &mut tiles[x as usize][z as usize] {
            if z + oz >= 0 && z + oz < self.grid_size as i32 * 2 && x + ox >= 0 && x + ox < self.grid_size as i32 * 2 {
                if existing[(x + ox) as usize][(z + oz) as usize].is_some() {
                    tile.entrances[entrance] = true;
                }
            }
        }
    }

    fn selective_new_position(&self, rng: &mut StdRng, taken_positions: &Vec<(i32, i32)>) -> (i32, i32) {
        let mut check_pos: (i32, i32);

        loop {
            let mut index = 0;
            for _ in 0..100 {
                index = rng.gen_range(0..taken_positions.len());
                if self.number_of_neighbors(&taken_positions[index], taken_positions) <= 1 {
                    break;
                }
            }

            let (mut x, mut z) = taken_positions[index];
            let up_down = rng.gen::<f32>() < 0.5;
            let positive = rng.gen::<f32>() < 0.5;
            if up_down {
                if positive {
                    z += 1;
                } else {
                    z -= 1;
                }
            } else {
                if positive {
                    x += 1;
                } else {
                    x -= 1;
                }
            }

            check_pos = (x, z);

            let gs = self.grid_size as i32;
            if !taken_positions.iter().any(|t| t == &check_pos) && x >= -gs && x < gs && z >= -gs && z < gs {
                break;
            }
        }

        check_pos
    }

    fn number_of_neighbors(&self, pos: &(i32, i32), taken_positions: &Vec<(i32, i32)>) -> usize {
        let (x, z) = pos;
        taken_positions.iter().filter(|(tx, tz)| tx == &(x + 1) && tz == z).count()
            + taken_positions.iter().filter(|(tx, tz)| tx == &(x - 1) && tz == z).count()
            + taken_positions.iter().filter(|(tx, tz)| tx == x && tz == &(z + 1)).count()
            + taken_positions.iter().filter(|(tx, tz)| tx == x && tz == &(z - 1)).count()
    }

    fn new_position(&self, rng: &mut StdRng, taken_positions: &Vec<(i32, i32)>) -> (i32, i32) {
        let mut checking_pos: (i32, i32);

        loop {
            let index = rng.gen_range(0..taken_positions.len());
            let (mut x, mut z) = taken_positions[index];

            let up_down = rng.gen::<f32>() < 0.5;
            let positive = rng.gen::<f32>() < 0.5;

            if up_down {
                if positive {
                    z += 1;
                } else {
                    z -= 1;
                }
            } else {
                if positive {
                    x += 1;
                } else {
                    x -= 1;
                }
            }

            checking_pos = (x as i32, z as i32);

            let gs = self.grid_size as i32;
            if !taken_positions.iter().any(|t| t == &checking_pos) && x >= -gs && x < gs && z >= -gs && z < gs {
                break;
            }
        }

        checking_pos
    }
}
