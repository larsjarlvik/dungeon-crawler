use crate::{
    engine::{self, model::Placeholder},
    world::{self},
};
use cgmath::*;
use rand::{prelude::StdRng, Rng};
use serde_derive::Deserialize;
use specs::{Builder, WorldExt};
use std::{
    collections::HashMap,
    fs::{self},
};

#[derive(Clone, Debug, Deserialize)]
struct PlaceholderDecor {
    name: String,
    rotation: f32,
    rotation_rng: f32,
}

#[derive(Clone, Debug, Deserialize)]
struct TileDecor {
    decor: HashMap<String, PlaceholderDecor>,
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

    pub fn build(&self, engine: &engine::Engine, world: &mut world::World, rng: &mut StdRng, x: i32, y: i32, entrances: &[bool; 4]) {
        let center = vec3(x as f32 * self.size, 0.0, y as f32 * self.size);
        let (tile, rotation) = self.determine_tile(entrances);

        world
            .components
            .create_entity()
            .with(world::components::Model::new(&engine, &self.tile, tile))
            .with(world::components::Collision::new(&self.tile, tile))
            .with(world::components::Render { cull_frustum: true })
            .with(world::components::Transform::from_translation_angle(center, -rotation))
            .build();

        self.tile.lights.iter().filter(|l| l.name.contains(tile)).for_each(|l| {
            world
                .components
                .create_entity()
                .with(world::components::Light::new(
                    l.color,
                    l.intensity,
                    Some(l.radius),
                    l.translation,
                ))
                .with(world::components::Transform::from_translation(center))
                .maybe_with(self.get_flicker(l.flicker, rng.gen::<f32>() * 0.05 + 0.02))
                .build();
        });

        let tile_decor = self.get_decor(tile);
        if tile_decor.len() > 0 {
            let tile_decor = tile_decor[rng.gen_range(0..tile_decor.len())].clone();
            let placeholders = self.tile.get_placeholders(tile);

            for placeholder in placeholders {
                let id = placeholder.name.split('.').last().unwrap();
                if let Some(decor) = tile_decor.decor.get(id) {
                    self.add_decor(engine, world, rng, center, &placeholder, &decor, rotation);
                }
            }
        }
    }

    fn add_decor(
        &self,
        engine: &engine::Engine,
        world: &mut world::World,
        rng: &mut StdRng,
        center: Vector3<f32>,
        placeholder: &Placeholder,
        decor: &PlaceholderDecor,
        rotation: f32,
    ) {
        let rotation = Quaternion::from_angle_y(Deg(-rotation));
        let pos = center + rotation.rotate_vector(vec3(placeholder.position.x, 0.0, placeholder.position.z));
        let flicker_speed = rng.gen::<f32>() * 0.05 + 0.02;

        world
            .components
            .create_entity()
            .with(world::components::Model::new(&engine, &self.decor, &decor.name))
            .with(world::components::Collision::new(&self.decor, &decor.name))
            .with(world::components::Transform::from_translation_angle(
                pos,
                decor.rotation + (rng.gen::<f32>() * 2.0 - 1.0) * decor.rotation_rng,
            ))
            .with(world::components::Render { cull_frustum: true })
            .with(world::components::Shadow)
            .build();

        self.decor.lights.iter().filter(|l| l.name.contains(&decor.name)).for_each(|l| {
            world
                .components
                .create_entity()
                .with(world::components::Light::new(
                    l.color,
                    l.intensity,
                    Some(l.radius),
                    l.translation,
                ))
                .with(world::components::Transform::from_translation(pos))
                .maybe_with(self.get_flicker(l.flicker, flicker_speed))
                .build();
        });

        self.decor.emitters.iter().filter(|e| e.name.contains(&decor.name)).for_each(|e| {
            let emitter = engine
                .particle_pipeline
                .create_emitter(&engine.ctx, e.particle_count, e.life_time, e.spread, e.speed);

            world
                .components
                .create_entity()
                .with(world::components::Particle::new(emitter, e.start_color, e.end_color, e.size))
                .maybe_with(self.get_flicker(e.flicker, flicker_speed))
                .with(world::components::Transform::from_translation(pos + e.position))
                .build();
        });
    }

    fn get_flicker(&self, flicker: Option<f32>, speed: f32) -> Option<world::components::Flicker> {
        if let Some(flicker) = flicker {
            Some(world::components::Flicker::new(flicker, speed))
        } else {
            None
        }
    }

    fn determine_tile(&self, entrances: &[bool; 4]) -> (&str, f32) {
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

    fn get_decor(&self, tile: &str) -> Vec<TileDecor> {
        let path = format!("./assets/tiles/catacombs/{}.json", tile);
        match fs::read_to_string(&path) {
            Ok(file) => serde_json::from_str(&file).expect("Failed to parse tile JSON!"),
            Err(_) => vec![],
        }
    }
}
