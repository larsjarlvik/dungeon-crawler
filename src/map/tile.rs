use crate::{config, engine, utils, world};
use cgmath::*;
use rand::{prelude::StdRng, Rng};
use serde_derive::Deserialize;
use specs::{Builder, Entity, WorldExt};

#[derive(Clone, Debug, Deserialize)]
pub struct Decor {
    name: String,
    pos: [i32; 2],
    rotation: f32,
    rotation_rng: f32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TileDecor {
    pub decor: Vec<Decor>,
}

pub struct Tile {
    pub tile: engine::model::GltfModel,
    pub decor: engine::model::GltfModel,
    size: f32,
}

impl Tile {
    pub fn new(engine: &engine::Engine, size: f32) -> Self {
        let tile = engine.load_model("models/catacombs.glb");
        let decor = engine.load_model("models/decor.glb");
        Self { tile, decor, size }
    }

    pub fn add_tile(&self, engine: &engine::Engine, world: &mut world::World, tile: &str, center: Vector3<f32>, rotation: f32) {
        world
            .components
            .create_entity()
            .with(world::components::Model::new(&engine, &self.tile, tile))
            .with(world::components::Collision::new(&self.tile, tile))
            .with(world::components::Render { cull_frustum: true })
            .with(world::components::Transform::from_translation_angle(center, rotation))
            .build();
    }

    pub fn add_grid(&self, world: &mut world::World, center: Vector3<f32>) {
        for x in -config::GRID_COUNT..=config::GRID_COUNT {
            for z in -config::GRID_COUNT..=config::GRID_COUNT {
                let off = vec3(x as f32 * config::GRID_DIST, 0.0, z as f32 * config::GRID_DIST);
                let text = format!("{},{}", x, z);
                world
                    .components
                    .create_entity()
                    .with(world::components::Text::new(&text))
                    .with(world::components::Transform::from_translation_scale(center + off, 16.0))
                    .build();
            }
        }
    }

    pub fn build(&mut self, engine: &engine::Engine, world: &mut world::World, rng: &mut StdRng, x: i32, y: i32, entrances: &[bool; 4]) {
        let center = vec3(x as f32 * self.size, 0.0, y as f32 * self.size);
        let (tile, rotation) = determine_tile(entrances);
        self.add_tile(engine, world, tile, center, -rotation);

        world
            .components
            .create_entity()
            .with(world::components::Model::new(&engine, &self.tile, tile))
            .with(world::components::Collision::new(&self.tile, tile))
            .with(world::components::Render { cull_frustum: true })
            .with(world::components::Transform::from_translation_angle(center, -rotation))
            .build();

        match self.get_decor(&format!("catacombs/{}", tile.split('-').last().unwrap()).as_str()) {
            Ok(variants) => {
                if variants.len() > 0 {
                    let tile_decor = variants[rng.gen_range(0..variants.len())].clone();
                    self.add_decor(engine, world, rng, center, rotation, &tile_decor);
                }
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    pub fn add_decor(
        &mut self,
        engine: &engine::Engine,
        world: &mut world::World,
        rng: &mut StdRng,
        center: Vector3<f32>,
        rotation: f32,
        tile_decor: &TileDecor,
    ) {
        for decor in tile_decor.decor.iter() {
            self.add_decor_item(engine, world, rng, center, &decor, rotation);
        }
    }

    pub fn add_decor_item(
        &self,
        engine: &engine::Engine,
        world: &mut world::World,
        rng: &mut StdRng,
        center: Vector3<f32>,
        decor: &Decor,
        rotation: f32,
    ) -> Vec<Entity> {
        let q_rotation = Quaternion::from_angle_y(Deg(-rotation));
        let pos = center
            + q_rotation.rotate_vector(vec3(
                decor.pos[0] as f32 * config::GRID_DIST,
                0.0,
                decor.pos[1] as f32 * config::GRID_DIST,
            ));

        let flicker_speed = rng.gen::<f32>() * 0.05 + 0.02;
        let r = decor.rotation - rotation;
        let mut entities = vec![];

        entities.push(
            world
                .components
                .create_entity()
                .with(world::components::Model::new(&engine, &self.decor, &decor.name))
                .maybe_with(if self.decor.collisions.contains_key(&decor.name) {
                    Some(world::components::Collision::new(&self.decor, &decor.name))
                } else {
                    None
                })
                .with(world::components::Transform::from_translation_angle(
                    pos,
                    r + (rng.gen::<f32>() * 2.0 - 1.0) * decor.rotation_rng,
                ))
                .with(world::components::Render { cull_frustum: true })
                .with(world::components::Shadow)
                .build(),
        );

        entities.append(
            &mut self
                .decor
                .lights
                .iter()
                .filter(|l| l.name.contains(&decor.name))
                .map(|l| {
                    world
                        .components
                        .create_entity()
                        .with(world::components::Light::new(
                            l.color,
                            l.intensity,
                            Some(l.radius),
                            l.translation,
                        ))
                        .with(world::components::Transform::from_translation_angle(pos, r))
                        .maybe_with(self.get_flicker(l.flicker, flicker_speed))
                        .build()
                })
                .collect(),
        );

        entities.append(
            &mut self
                .decor
                .get_emitters(&decor.name)
                .iter()
                .map(|e| {
                    let emitter = engine
                        .particle_pipeline
                        .create_emitter(&engine.ctx, e.particle_count, e.life_time, e.spread, e.speed);

                    world
                        .components
                        .create_entity()
                        .with(world::components::Particle::new(
                            emitter,
                            e.start_color,
                            e.end_color,
                            e.size,
                            e.strength,
                        ))
                        .maybe_with(self.get_flicker(e.flicker, flicker_speed))
                        .with(world::components::Transform::from_translation_angle(
                            pos + Quaternion::from_angle_y(Deg(decor.rotation - rotation)).rotate_vector(e.position),
                            decor.rotation - rotation,
                        ))
                        .build()
                })
                .collect(),
        );

        entities
    }

    fn get_flicker(&self, flicker: Option<f32>, speed: f32) -> Option<world::components::Flicker> {
        if let Some(flicker) = flicker {
            Some(world::components::Flicker::new(flicker, speed))
        } else {
            None
        }
    }

    pub fn get_decor(&self, tile: &str) -> Result<Vec<TileDecor>, String> {
        let path = format!("tiles/{}.json", tile);

        match serde_json::from_str(utils::read_string(&path).as_str()) {
            Ok(decor) => Ok(decor),
            Err(_) => Err(format!("Filed to parse tile: {}!", tile)),
        }
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

        _ => ("tile-catacombs-1111", 0.0),
    }
}
