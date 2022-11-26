use crate::{
    config,
    world::components::{self},
};
use bevy_ecs::{prelude::World, world::EntityMut};
use cgmath::*;
use engine::{
    collision::{Polygon, PolygonMethods},
    Engine,
};
use rand::{prelude::StdRng, Rng, SeedableRng};
use std::env;
mod decor;
mod generator;

pub struct Map {
    seed: u64,
    tile_size: f32,
    grid_size: usize,
    number_of_tiles: usize,
    tiles: engine::model::GltfModel,
    decor: engine::model::GltfModel,
    hostiles: engine::model::GltfModel,
}

impl Map {
    pub fn new(ctx: &engine::Context, seed: u64, grid_size: usize) -> Self {
        let tiles = engine::load_model(ctx, "models/catacombs.glb");
        let decor = engine::load_model(ctx, "models/decor.glb");
        let hostiles = engine::load_model(ctx, "models/skeleton.glb");
        let number_of_tiles = 25;

        Self {
            tile_size: 14.0,
            seed,
            grid_size,
            number_of_tiles,
            tiles,
            decor,
            hostiles,
        }
    }

    pub fn generate(&mut self, world: &mut World, engine: &mut Engine) {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let mut tiles = generator::generate(&mut rng, self.grid_size, self.number_of_tiles);
        let gs_2 = self.grid_size * 2;

        (0..(gs_2 + 1)).for_each(|x| {
            (0..(gs_2 + 1)).for_each(|z| {
                let tx = x as i32 - self.grid_size as i32;
                let tz = z as i32 - self.grid_size as i32;
                let mut entity = world.spawn_empty();
                let center = vec3(tx as f32 * self.tile_size, 0.0, tz as f32 * self.tile_size);

                if x == gs_2 || z == gs_2 {
                    self.empty_tile(engine, &mut entity, center);
                } else if let Some(t) = &mut tiles[x][z] {
                    self.tile(engine, &mut entity, &mut rng, t, center);
                } else {
                    self.empty_tile(engine, &mut entity, center);
                };
            });
        });
    }

    pub fn single_tile(&mut self, engine: &mut engine::Engine, world: &mut World, tile_name: &str) {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let mut entity = world.spawn_empty();

        let collisions = self.tiles.collisions.get(tile_name).unwrap_or(&vec![]).clone();
        let decor = decor::get_decor(format!("catacombs/{}", tile_name).as_str(), &mut rng)
            .iter()
            .map(|d| self.add_decor(engine, d, Vector3::zero(), 0.0))
            .collect();

        let model = engine.initialize_model(&self.tiles, format!("tile-catacombs-{}", tile_name).as_str(), 1.0);
        entity.insert(components::Tile::new(
            model,
            collisions,
            Vector3::zero(),
            self.tile_size,
            0.0,
            decor,
            vec![],
        ));

        self.add_grid(world, Vector3::zero());
    }

    fn empty_tile(&self, engine: &mut engine::Engine, entity: &mut EntityMut, pos: Vector3<f32>) {
        let model = engine.initialize_model(&self.tiles, "tile-empty", 1.0);
        entity.insert(components::Tile::new(model, vec![], pos, self.tile_size, 0.0, vec![], vec![]));
    }

    fn tile(&self, engine: &mut engine::Engine, entity: &mut EntityMut, rng: &mut StdRng, tile: &mut generator::Tile, pos: Vector3<f32>) {
        let entrances = tile.entrances;
        let (t, rot) = determine_tile(&entrances);
        let name = t.split('-').last().expect("Could not get map name!");

        let decor: Vec<components::Decor> = decor::get_decor(format!("catacombs/{}", name).as_str(), rng)
            .iter()
            .map(|d| self.add_decor(engine, d, pos, rot))
            .collect();

        let decor_collisions: Vec<Polygon> = decor
            .iter()
            .flat_map(|d| {
                d.collisions
                    .iter()
                    .map(|polygon| polygon.transform(d.position, Quaternion::from_angle_y(Deg(d.rotation))))
                    .collect::<Vec<Polygon>>()
            })
            .collect();

        let mut hostiles = vec![];

        // Do not spawn hostiles on starting tile
        if pos.distance(Vector3::zero()) > 1.0 {
            for _ in 0..(rng.gen::<f32>() * 4.0) as usize {
                hostiles.push(self.add_hostile(rng, engine, pos, &decor_collisions));
            }
        }

        let collisions = self
            .tiles
            .collisions
            .get(t)
            .unwrap_or_else(|| panic!("Could not find collision for: {}!", name))
            .clone();

        let model = engine.initialize_model(&self.tiles, t, 1.0);
        entity.insert(components::Tile::new(
            model,
            collisions,
            pos,
            self.tile_size,
            -rot,
            decor,
            hostiles,
        ));
    }

    fn add_decor(&self, engine: &mut engine::Engine, d: &decor::Decor, tile_center: Vector3<f32>, tile_rotation: f32) -> components::Decor {
        let position = tile_center
            + Quaternion::from_angle_y(Deg(-tile_rotation)).rotate_vector(vec3(
                d.pos[0] as f32 * config::GRID_DIST,
                0.0,
                d.pos[1] as f32 * config::GRID_DIST,
            ));
        let rotation = d.rotation - tile_rotation;

        let lights = self
            .decor
            .lights
            .iter()
            .filter(|l| l.name.contains(format!("{}_", &d.name).as_str()))
            .map(|l| components::DecorLight {
                color: l.color,
                intensity: l.intensity,
                radius: Some(l.radius),
                offset: l.translation,
                flicker: l.flicker,
                bloom: 1.0,
                position,
                rotation,
            })
            .collect();

        let emitters = self
            .decor
            .get_emitters(&d.name)
            .iter()
            .map(|e| {
                let emitter_id = uuid::Uuid::new_v4().to_string();
                let emitter = engine
                    .particle_pipeline
                    .create_emitter(&engine.ctx, e.particle_count, e.life_time, e.spread, e.speed);

                engine.initialize_particle(emitter, emitter_id.clone());
                components::DecorEmitter {
                    emitter_id,
                    start_color: e.start_color,
                    end_color: e.end_color,
                    size: e.size,
                    strength: e.strength,
                    flicker: e.flicker,
                    position: position + Quaternion::from_angle_y(Deg(d.rotation - tile_rotation)).rotate_vector(e.position),
                    rotation: d.rotation - tile_rotation,
                }
            })
            .collect();

        let collisions = self.decor.collisions.get(&d.name).unwrap_or(&vec![]).clone();
        let model = engine.initialize_model(&self.decor, d.name.as_str(), 1.0);

        components::Decor {
            model,
            collisions,
            lights,
            emitters,
            position,
            rotation,
        }
    }

    fn add_hostile(
        &self,
        rng: &mut StdRng,
        engine: &mut engine::Engine,
        tile_center: Vector3<f32>,
        collisions: &[Polygon],
    ) -> components::Hostile {
        let model = engine.initialize_model(&self.hostiles, "skeleton", 1.3);
        let mut position;

        let collider = self
            .hostiles
            .collisions
            .get("skeleton")
            .expect("Could not find skeleton collider!")
            .clone();

        loop {
            let mut is_colliding = false;
            position = tile_center
                + vec3(
                    (rng.gen::<f32>() - 0.5) * (self.tile_size - 3.0),
                    0.0,
                    (rng.gen::<f32>() - 0.5) * (self.tile_size - 3.0),
                );

            for polygon in collider.iter() {
                let p = polygon.transform(position, Quaternion::zero());
                if engine::collision::check_collision_array(Vector3::zero(), &p, collisions) {
                    is_colliding = true;
                    break;
                }
            }

            if !is_colliding {
                break;
            }
        }

        components::Hostile {
            model,
            collider,
            position,
            health: 10.0,
        }
    }

    fn add_grid(&self, world: &mut World, center: Vector3<f32>) {
        for x in -config::GRID_COUNT..=config::GRID_COUNT {
            for z in -config::GRID_COUNT..=config::GRID_COUNT {
                let off = vec3(x as f32 * config::GRID_DIST, 0.0, z as f32 * config::GRID_DIST);
                let text = format!("{},{}", x, z);

                world.spawn((
                    engine::ecs::components::Text::new(&text),
                    engine::ecs::components::Transform::from_translation_scale(center + off, 16.0),
                ));
            }
        }
    }
}

pub fn edit_mode() -> Option<String> {
    let args: Vec<String> = env::args().collect();

    if let Some(pos) = args.iter().position(|a| a == "--edit") {
        args.get(pos + 1).cloned()
    } else {
        None
    }
}

fn determine_tile(entrances: &[bool; 4]) -> (&str, f32) {
    match entrances {
        [true, false, false, false] => ("tile-catacombs-1000", 0.0),
        [false, true, false, false] => ("tile-catacombs-1000", 90.0),
        [false, false, true, false] => ("tile-catacombs-1000", 180.0),
        [false, false, false, true] => ("tile-catacombs-1000", 270.0),

        [true, true, false, false] => ("tile-catacombs-1100", 0.0),
        [false, true, true, false] => ("tile-catacombs-1100", 90.0),
        [false, false, true, true] => ("tile-catacombs-1100", 180.0),
        [true, false, false, true] => ("tile-catacombs-1100", 270.0),

        [true, false, true, false] => ("tile-catacombs-1010", 0.0),
        [false, true, false, true] => ("tile-catacombs-1010", 90.0),

        [true, true, true, false] => ("tile-catacombs-1110", 0.0),
        [false, true, true, true] => ("tile-catacombs-1110", 90.0),
        [true, false, true, true] => ("tile-catacombs-1110", 180.0),
        [true, true, false, true] => ("tile-catacombs-1110", 270.0),

        [true, true, true, true] => ("tile-catacombs-1111", 0.0),

        _ => ("tile-empty", 0.0),
    }
}
