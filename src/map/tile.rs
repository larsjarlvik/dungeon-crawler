use crate::{engine, world};
use cgmath::*;
use rand::{prelude::StdRng, Rng};
use specs::{Builder, WorldExt};

pub struct Tile {
    pub tile: engine::model::GltfModel,
    pub assets: engine::model::GltfModel,
    size: f32,
}

impl Tile {
    pub fn new(engine: &engine::Engine, size: f32) -> Self {
        let tile = engine.load_model("models/catacombs.glb");
        let assets = engine.load_model("models/decor.glb");
        Self { tile, assets, size }
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

            world
                .components
                .create_entity()
                .with(world::components::Model::new(&engine, &self.assets, current))
                .with(world::components::Collision::new(&self.assets, current))
                .with(world::components::Transform::from_translation_angle(
                    pos,
                    rng.gen::<f32>() * 360.0,
                ))
                .with(world::components::Render { cull_frustum: true })
                .with(world::components::Shadow)
                .build();

            if current == "torch" {
                world
                    .components
                    .create_entity()
                    .with(world::components::Particle::new(
                        engine.particle_pipeline.create_emitter(&engine.ctx, 1000, 0.8, 0.35, 0.25),
                        vec3(0.03, 0.21, 0.73),
                        vec3(0.5, 0.3, 0.0),
                        0.04
                    ))
                    .with(world::components::Light::new(
                        vec3(1.0, 1.0, 0.72),
                        1.0,
                        Some(3.5),
                        vec3(0.0, 0.3, 0.0),
                    ))
                    .with(world::components::Flicker::new(0.1, rng.gen::<f32>() * 0.1 + 0.1))
                    .with(world::components::Transform::from_translation(vec3(pos.x, pos.y + 1.3, pos.z)))
                    .build();
            }
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
