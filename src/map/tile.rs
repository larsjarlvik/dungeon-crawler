use crate::{
    engine::{self},
    world,
};
use cgmath::*;
use rand::{prelude::StdRng, Rng};
use specs::{Builder, WorldExt};

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

        // self.tile.lights.iter().filter(|l| l.name.contains(tile)).for_each(|l| {
        //     world
        //         .components
        //         .create_entity()
        //         .with(world::components::Light::new(
        //             l.color,
        //             l.intensity,
        //             Some(l.radius),
        //             l.translation,
        //         ))
        //         .with(world::components::Transform::from_translation(center))
        //         .maybe_with(if let Some(flicker) = l.flicker {
        //             Some(world::components::Flicker::new(flicker))
        //         } else {
        //             None
        //         })
        //         .build();
        // });

        let s = self.size / 2.0 - 2.0;
        let decor = vec!["barrel", "table", "torch", "crate"];
        for _ in 0..10 {
            let current = decor[rng.gen_range(0..decor.len())];
            let pos = center + vec3(rng.gen_range(-s..s), 0.0, rng.gen_range(-s..s));
            let flicker_speed = rng.gen::<f32>() * 0.05 + 0.02;

            world
                .components
                .create_entity()
                .with(world::components::Model::new(&engine, &self.decor, current))
                .with(world::components::Collision::new(&self.decor, current))
                .with(world::components::Transform::from_translation_angle(
                    pos,
                    rng.gen::<f32>() * 360.0,
                ))
                .with(world::components::Render { cull_frustum: true })
                .with(world::components::Shadow)
                .build();

            self.decor.lights.iter().filter(|l| l.name.contains(current)).for_each(|l| {
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
                    .maybe_with(if let Some(flicker) = l.flicker {
                        Some(world::components::Flicker::new(flicker, flicker_speed))
                    } else {
                        None
                    })
                    .build();
            });

            self.decor.emitters.iter().filter(|e| e.name.contains(current)).for_each(|e| {
                world
                    .components
                    .create_entity()
                    .with(world::components::Particle::new(
                        engine
                            .particle_pipeline
                            .create_emitter(&engine.ctx, e.particle_count, e.life_time, e.spread, e.speed),
                        e.start_color,
                        e.end_color,
                        e.size,
                    ))
                    .maybe_with(if let Some(flicker) = e.flicker {
                        Some(world::components::Flicker::new(flicker, flicker_speed))
                    } else {
                        None
                    })
                    .with(world::components::Transform::from_translation(pos + e.position))
                    .build();
            });
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
}
