use crate::{
    config,
    engine::{self, bounding_box},
    utils, world,
};
use bevy_ecs::{entity::Entity, prelude::World};
use cgmath::*;
use rand::{prelude::StdRng, Rng};
use serde_derive::Deserialize;

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

pub enum TileState {
    Active(Entity),
    Destroyed,
}

pub struct Tile {
    tile: String,
    state: TileState,
    center: Vector3<f32>,
    rotation: f32,
    decor: TileDecor,
    pub bounding_box: bounding_box::BoundingBox,
}

impl Tile {
    pub fn new(x: i32, z: i32, size: f32, entrances: &[bool; 4], rng: &mut StdRng) -> Self {
        let (tile, rotation) = determine_tile(entrances);
        let name = tile.split('-').last().unwrap();

        let decor = match get_decor(&format!("catacombs/{}", name).as_str()) {
            Ok(variants) => {
                if variants.len() > 0 {
                    variants[rng.gen_range(0..variants.len())].clone()
                } else {
                    TileDecor { decor: vec![] }
                }
            }
            Err(err) => {
                panic!("{}", err);
            }
        };

        Self::new_known(x, z, size, tile, decor, rotation)
    }

    pub fn new_known(x: i32, z: i32, size: f32, tile: &str, decor: TileDecor, rotation: f32) -> Self {
        let center = vec3(x as f32 * size, 0.0, z as f32 * size);
        let h_size = size / 2.0;

        Self {
            tile: tile.to_string(),
            state: TileState::Destroyed,
            center,
            bounding_box: bounding_box::BoundingBox {
                min: point3(center.x - h_size, 0.0, center.z - h_size),
                max: point3(center.x + h_size, 2.5, center.z + h_size),
            },
            rotation,
            decor,
        }
    }

    pub fn build(
        &mut self,
        engine: &engine::Engine,
        world: &mut World,
        rng: &mut StdRng,
        tile_model: &engine::model::GltfModel,
        decor_model: &engine::model::GltfModel,
    ) {
        match self.state {
            TileState::Destroyed => {
                self.add_room(engine, world, &tile_model, self.center, -self.rotation);
                self.add_decor(engine, world, rng, self.center, self.rotation, &decor_model);
            }
            _ => (),
        };
    }

    pub fn destroy(&mut self, world: &mut World) {
        if let TileState::Active(entity) = &mut self.state {
            // Remove entity
            world.despawn(*entity);
            self.state = TileState::Destroyed;
        };
    }

    pub fn add_room(
        &mut self,
        engine: &engine::Engine,
        world: &mut World,
        tile_model: &engine::model::GltfModel,
        center: Vector3<f32>,
        rotation: f32,
    ) {
        let mut entity = world.spawn();
        entity
            .insert(world::components::Model::new(&engine, &tile_model, &self.tile))
            .insert(world::components::Render { cull_frustum: true })
            .insert(world::components::Transform::from_translation_angle(center, rotation));

        if let Some(collision) = world::components::Collision::new(&tile_model, &self.tile) {
            entity.insert(collision);
        }

        self.state = TileState::Active(entity.id());
    }

    pub fn add_grid(&self, world: &mut World, center: Vector3<f32>) {
        for x in -config::GRID_COUNT..=config::GRID_COUNT {
            for z in -config::GRID_COUNT..=config::GRID_COUNT {
                let off = vec3(x as f32 * config::GRID_DIST, 0.0, z as f32 * config::GRID_DIST);
                let text = format!("{},{}", x, z);

                world
                    .spawn()
                    .insert(world::components::Text::new(&text))
                    .insert(world::components::Transform::from_translation_scale(center + off, 16.0));
            }
        }
    }

    pub fn add_decor(
        &self,
        engine: &engine::Engine,
        world: &mut World,
        rng: &mut StdRng,
        center: Vector3<f32>,
        rotation: f32,
        decor_model: &engine::model::GltfModel,
    ) {
        match self.state {
            TileState::Active(entity) => {
                for decor in self.decor.decor.iter() {
                    self.add_decor_item(entity, engine, world, rng, center, &decor, &decor_model, rotation);
                }
            }
            _ => {}
        }
    }

    pub fn add_decor_item(
        &self,
        parent: Entity,
        engine: &engine::Engine,
        world: &mut World,
        rng: &mut StdRng,
        center: Vector3<f32>,
        decor: &Decor,
        decor_model: &engine::model::GltfModel,
        rotation: f32,
    ) {
        let q_rotation = Quaternion::from_angle_y(Deg(-rotation));
        let pos = center
            + q_rotation.rotate_vector(vec3(
                decor.pos[0] as f32 * config::GRID_DIST,
                0.0,
                decor.pos[1] as f32 * config::GRID_DIST,
            ));

        let flicker_speed = rng.gen::<f32>() * 0.05 + 0.02;
        let r = decor.rotation - rotation;

        let entity = {
            let mut entity = world.spawn();
            entity
                .insert(world::components::Child::new(parent))
                .insert(world::components::Model::new(&engine, &decor_model, &decor.name))
                .insert(world::components::Transform::from_translation_angle(
                    pos,
                    r + (rng.gen::<f32>() * 2.0 - 1.0) * decor.rotation_rng,
                ))
                .insert(world::components::Render { cull_frustum: true })
                .insert(world::components::Shadow)
                .insert(world::components::Health::new(10.0));

            if let Some(collision) = world::components::Collision::new(&decor_model, &decor.name) {
                entity.insert(collision);
            }

            entity.id()
        };

        decor_model
            .lights
            .iter()
            .filter(|l| l.name.contains(format!("{}_", &decor.name).as_str()))
            .for_each(|l| {
                {
                    let mut light_entity = world.spawn();
                    light_entity
                        .insert(world::components::Child::new(entity))
                        .insert(world::components::Light::new(
                            l.color,
                            l.intensity,
                            Some(l.radius),
                            l.translation,
                            1.0,
                        ))
                        .insert(world::components::Transform::from_translation_angle(pos, r));

                    if let Some(flicker) = self.get_flicker(l.flicker, flicker_speed) {
                        light_entity.insert(flicker);
                    }

                    light_entity.id();
                };
            });

        decor_model.get_emitters(&decor.name).iter().for_each(|e| {
            let emitter = engine
                .particle_pipeline
                .create_emitter(&engine.ctx, e.particle_count, e.life_time, e.spread, e.speed);

            let mut emitter_entity = world.spawn();
            emitter_entity
                .insert(world::components::Child::new(entity))
                .insert(world::components::Particle::new(
                    emitter,
                    e.start_color,
                    e.end_color,
                    e.size,
                    e.strength,
                ))
                .insert(world::components::Render { cull_frustum: true })
                .insert(world::components::Transform::from_translation_angle(
                    pos + Quaternion::from_angle_y(Deg(decor.rotation - rotation)).rotate_vector(e.position),
                    decor.rotation - rotation,
                ));

            if let Some(flicker) = self.get_flicker(e.flicker, flicker_speed) {
                emitter_entity.insert(flicker);
            }
        });
    }

    fn get_flicker(&self, flicker: Option<f32>, speed: f32) -> Option<world::components::Flicker> {
        if let Some(flicker) = flicker {
            Some(world::components::Flicker::new(flicker, speed))
        } else {
            None
        }
    }
}

pub fn get_decor(tile: &str) -> Result<Vec<TileDecor>, String> {
    if tile.contains("empty") {
        return Ok(vec![]);
    }

    let path = format!("tiles/{}.json", tile);

    match serde_json::from_str(utils::read_string(&path).as_str()) {
        Ok(decor) => Ok(decor),
        Err(_) => Err(format!("Filed to parse tile: {}!", tile)),
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
