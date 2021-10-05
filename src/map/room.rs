use crate::{engine, world};
use cgmath::*;
use rand::prelude::StdRng;
use specs::{Builder, WorldExt};

pub struct Room {
    room: engine::model::GltfModel,
}

impl Room {
    pub fn new(engine: &engine::Engine) -> Self {
        let room = engine.load_model("models/room.glb");
        Self { room }
    }

    pub fn build(&self, engine: &engine::Engine, world: &mut world::World, rng: &mut StdRng, x: i32, y: i32) {
        let center = vec3(x as f32 * 10.0, 0.0, y as f32 * 10.0);

        world
            .components
            .create_entity()
            .with(world::components::Model::new(&engine, &self.room, "room"))
            .with(world::components::Collision::new(&self.room, "room"))
            .with(world::components::Render { cull_frustum: true })
            .with(world::components::Transform::from_translation(center))
            .build();

        world
            .components
            .create_entity()
            .with(world::components::Light::new(vec3(1.0, 1.0, 0.7), 0.4, Some(7.0)))
            .with(world::components::Transform::from_translation(vec3(center.x, 2.0, center.z)))
            .build();
    }
}
