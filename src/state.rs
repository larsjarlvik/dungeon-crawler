use crate::{engine, world};
use cgmath::*;
use rand::Rng;
use specs::{Builder, WorldExt};
use winit::window::Window;

pub struct State {
    engine: engine::Engine,
    world: world::World,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let engine = engine::Engine::new(window).await;
        let mut rng = rand::thread_rng();
        let mut world = world::World::new();
        let model = engine.load_model(include_bytes!("../models/bottle.glb"));

        world
            .components
            .insert(world::resources::Camera::new(engine.ctx.viewport.get_aspect()));
        world.components.insert(world::resources::Time::default());

        world
            .components
            .create_entity()
            .with(world::components::Fps::new())
            .with(world::components::Text::new("", vec2(20.0, 20.0)))
            .build();

        world
            .components
            .create_entity()
            .with(world::components::Light {
                color: vec3(1.0, 0.0, 0.0),
                intensity: 4.0,
                radius: Some(5.0),
            })
            .with(world::components::Position(vec3(0.0, 2.0, 0.0)))
            .with(world::components::Bouce(vec3(
                rng.gen::<f32>() * 2.0 - 1.0,
                0.0,
                rng.gen::<f32>() * 2.0 - 1.0,
            )))
            .build();

        world
            .components
            .create_entity()
            .with(world::components::Light {
                color: vec3(0.0, 1.0, 0.0),
                intensity: 4.0,
                radius: Some(5.0),
            })
            .with(world::components::Position(vec3(0.0, 2.0, 0.0)))
            .with(world::components::Bouce(vec3(
                rng.gen::<f32>() * 2.0 - 1.0,
                0.0,
                rng.gen::<f32>() * 2.0 - 1.0,
            )))
            .build();

        world
            .components
            .create_entity()
            .with(world::components::Light {
                color: vec3(0.0, 0.0, 1.0),
                intensity: 4.0,
                radius: Some(5.0),
            })
            .with(world::components::Position(vec3(0.0, 2.0, 0.0)))
            .with(world::components::Bouce(vec3(
                rng.gen::<f32>() * 2.0 - 1.0,
                0.0,
                rng.gen::<f32>() * 2.0 - 1.0,
            )))
            .build();

        for z in -15..15 {
            for x in -15..15 {
                world
                    .components
                    .create_entity()
                    .with(world::components::Model::new(engine.model_pipeline.gltf(
                        &engine.ctx,
                        &model,
                        "WaterBottle",
                    )))
                    .with(world::components::Position(vec3(x as f32 * 0.4, 0.0, z as f32 * 0.4)))
                    .with(world::components::Render::default())
                    .build();
            }
        }

        Self { engine, world }
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
