use crate::{engine, world};
use cgmath::*;
use rand::prelude::StdRng;
use specs::{Builder, WorldExt};

pub struct Block {
    pub block: engine::model::GltfModel,
    size: f32,
}

impl Block {
    pub fn new(engine: &engine::Engine, size: f32) -> Self {
        let block = engine.load_model("models/room.glb");
        Self { block, size }
    }

    pub fn build(&self, engine: &engine::Engine, world: &mut world::World, rng: &mut StdRng, x: i32, y: i32, entrances: &Vec<bool>) {
        let center = vec3(x as f32 * self.size, 0.0, y as f32 * self.size);
        let (block, rotation) = self.determine_block(entrances);

        dbg!(x);
        dbg!(y);
        dbg!(&entrances);

        world
            .components
            .create_entity()
            .with(world::components::Model::new(&engine, &self.block, block))
            .with(world::components::Collision::new(&self.block, block))
            .with(world::components::Render { cull_frustum: true })
            .with(world::components::Transform::from_translation_angle(center, -rotation))
            .build();

        world
            .components
            .create_entity()
            .with(world::components::Light::new(vec3(1.0, 1.0, 0.7), 0.4, Some(7.0)))
            .with(world::components::Transform::from_translation(vec3(center.x, 2.0, center.z)))
            .build();
    }

    fn determine_block(&self, entrances: &Vec<bool>) -> (&str, f32) {
        match entrances.as_slice() {
            [true, false, false, false] => ("Room1000-1", 0.0),
            [false, true, false, false] => ("Room1000-1", 90.0),
            [false, false, true, false] => ("Room1000-1", 180.0),
            [false, false, false, true] => ("Room1000-1", 270.0),

            [true, true, false, false] => ("Room1100-1", 0.0),
            [false, true, true, false] => ("Room1100-1", 90.0),
            [false, false, true, true] => ("Room1100-1", 180.0),
            [true, false, false, true] => ("Room1100-1", 270.0),

            [true, false, true, false] => ("Room1010-1", 0.0),
            [false, true, false, true] => ("Room1010-1", 90.0),

            [true, true, true, false] => ("Room1110-1", 0.0),
            [false, true, true, true] => ("Room1110-1", 0.0),
            [true, false, true, true] => ("Room1110-1", 0.0),
            [true, true, false, true] => ("Room1110-1", 0.0),

            _ => ("Room1111-1", 0.0),
        }
    }
}
