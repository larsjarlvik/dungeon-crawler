use crate::{engine, world};
use cgmath::*;
use rand::{prelude::StdRng, Rng, SeedableRng};
use specs::{Builder, WorldExt};

const ENTRANCE_MAP: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub struct Map {
    num_tiles: usize,
    seed: u64,
    room: engine::model::GltfModel,
}

impl Map {
    pub fn new(engine: &engine::Engine, seed: u64, num_tiles: usize) -> Self {
        let room = engine.load_model("models/room.glb");

        Self { room, num_tiles, seed }
    }

    pub fn generate(&self, engine: &engine::Engine, world: &mut world::World) {
        let mut placed_tiles = vec![];
        let mut to_gen = vec![(0, 0)];
        self.add_tile(engine, world, &mut placed_tiles, &mut to_gen, 0);
    }

    fn add_tile(
        &self,
        engine: &engine::Engine,
        world: &mut world::World,
        placed_tiles: &mut Vec<(i32, i32)>,
        to_gen: &mut Vec<(i32, i32)>,
        tile_count: usize,
    ) {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let tile_count = tile_count + 1;
        if tile_count > self.num_tiles {
            return;
        }

        let max_entrance_count = (rng.gen::<f32>().powf(3.0) * 4.0).ceil() as usize;

        let mut is_entrance = vec![true; max_entrance_count];
        is_entrance.extend(vec![false; 4 - max_entrance_count]);
        // shuffle_array(arr)

        let mut entrances = vec![false; 4];

        for i in 0..4 {
            let (gen_x, gen_y) = (to_gen[0].0, to_gen[0].1);
            let tx = gen_x * ENTRANCE_MAP[i].0;
            let ty = gen_y * ENTRANCE_MAP[i].1;

            let a = ((tx * tx + ty * ty) as f32).sqrt();
            let b = ((gen_x * gen_x + gen_y * gen_y) as f32).sqrt();

            if a < b - rng.gen::<f32>() * 8.0 {
                continue;
            }

            let exists = placed_tiles.iter().any(|(ptx, pty)| *ptx == tx && *pty == ty);
            let is_entrance = is_entrance[i] && rng.gen::<f32>() < (self.num_tiles - tile_count) as f32 / (to_gen.len() as f32 * 20.0);

            let needs_gen = to_gen.len() < 2 && tile_count < self.num_tiles;
            if (!exists && is_entrance) || needs_gen {
                to_gen.push((tx, ty));
            }

            entrances[i] = is_entrance || needs_gen;
        }

        placed_tiles.push(to_gen[0]);

        dbg!(&to_gen);
        world
            .components
            .create_entity()
            .with(world::components::Model::new(&engine, &self.room, "room"))
            .with(world::components::Collision::new(&self.room, "room"))
            .with(world::components::Render { cull_frustum: true })
            .with(world::components::Transform::from_translation(vec3(
                to_gen[0].0 as f32 * 10.0,
                0.0,
                to_gen[0].1 as f32 * 10.0,
            )))
            .build();

        if to_gen.len() > 1 {
            to_gen.remove(0);
            self.add_tile(engine, world, placed_tiles, to_gen, tile_count);
        }
    }
}
