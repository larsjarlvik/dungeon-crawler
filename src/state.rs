use std::time::Instant;

use crate::{engine, world};
use cgmath::*;
use specs::{Builder, WorldExt};
use winit::window::Window;

pub struct State {
    engine: engine::Engine,
    world: world::World,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let engine = engine::Engine::new(window).await;
        let world = world::World::new();

        let mut state = Self { engine, world };
        state.init();
        state
    }

    pub fn init(&mut self) {
        let start = Instant::now();
        let room = self.engine.load_model("models/room.glb");
        let character = self.engine.load_model("models/character.glb");

        self.engine.init();
        self.world = world::World::new();

        self.world
            .components
            .insert(world::resources::Camera::new(self.engine.ctx.viewport.get_aspect()));
        self.world.components.insert(world::resources::Time::default());

        self.world
            .components
            .create_entity()
            .with(world::components::Fps::new())
            .with(world::components::Text::new("", vec2(20.0, 20.0)))
            .build();

        self.world
            .components
            .create_entity()
            .with(world::components::Light {
                color: vec3(1.0, 1.0, 0.75),
                intensity: 0.4,
                radius: Some(7.1),
            })
            .with(world::components::Position(vec3(-2.4, 2.0, -2.4)))
            .build();

        self.world
            .components
            .create_entity()
            .with(world::components::Light {
                color: vec3(1.0, 1.0, 0.7),
                intensity: 0.4,
                radius: Some(7.0),
            })
            .with(world::components::Position(vec3(2.4, 2.0, -2.4)))
            .build();

        self.world
            .components
            .create_entity()
            .with(world::components::Light {
                color: vec3(1.0, 1.0, 0.63),
                intensity: 0.4,
                radius: Some(6.8),
            })
            .with(world::components::Position(vec3(-2.4, 2.0, 2.4)))
            .build();

        self.world
            .components
            .create_entity()
            .with(world::components::Light {
                color: vec3(1.0, 1.0, 0.72),
                intensity: 0.4,
                radius: Some(7.3),
            })
            .with(world::components::Position(vec3(2.4, 2.0, 2.4)))
            .build();

        for z in -3..3 {
            for x in -3..3 {
                self.world
                    .components
                    .create_entity()
                    .with(world::components::Model::new(self.engine.model_pipeline.gltf(
                        &self.engine.ctx,
                        &room,
                        "room",
                    )))
                    .with(world::components::Position(vec3(x as f32 * 10.0, 0.0, z as f32 * 10.0)))
                    .with(world::components::Render::default())
                    .build();
            }
        }

        self.world
            .components
            .create_entity()
            .with(world::components::Model::new(self.engine.model_pipeline.gltf(
                &self.engine.ctx,
                &character,
                "Head_LowRes",
            )))
            .with(world::components::Position(vec3(0.0, 1.0, 0.0)))
            .with(world::components::Render::default())
            .build();

        println!("Initialized world in: {} ms", start.elapsed().as_millis());
    }

    pub fn resize(&mut self, width: u32, height: u32, scale_factor: f64) {
        if width > 0 && height > 0 {
            self.engine.set_viewport(width, height, scale_factor);
            self.engine.deferred_pipeline.resize(&self.engine.ctx);

            let mut camera = self.world.components.write_resource::<world::resources::Camera>();
            *camera = world::resources::Camera::new(self.engine.ctx.viewport.get_aspect());
        }
    }

    pub fn update(&mut self, _elapsed: u64) {
        self.world.update();
        self.engine.deferred_pipeline.update(&self.engine.ctx, &self.world.components);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.engine.get_output_frame();

        self.engine
            .model_pipeline
            .render(&self.engine.ctx, &self.world.components, &self.engine.deferred_pipeline);

        self.engine.deferred_pipeline.render(&self.engine.ctx, &frame.view);

        self.engine
            .glyph_pipeline
            .render(&self.engine.ctx, &self.world.components, &frame.view);

        Ok(())
    }
}
