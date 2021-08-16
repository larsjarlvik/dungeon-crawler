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
        let mut world = world::World::new();
        world
            .components
            .create_entity()
            .with(world::components::Fps::new())
            .with(world::components::Text::new("", vec2(20.0, 20.0)))
            .build();

        let mut rng = rand::thread_rng();
        let model = engine.load_model(include_bytes!("../models/ship.glb"));

        for _ in 0..500 {
            world
                .components
                .create_entity()
                .with(world::components::Camera::new(
                    engine.ctx.viewport.width as u32,
                    engine.ctx.viewport.height as u32,
                ))
                .with(world::components::Model::new(engine.model_pipeline.gltf(
                    &engine.ctx,
                    &model,
                    "ship",
                )))
                .with(world::components::Position(vec3(
                    rng.gen::<f32>() * 40.0 - 20.0,
                    rng.gen::<f32>() * 40.0 - 20.0,
                    rng.gen::<f32>() * 40.0 - 20.0,
                )))
                .with(world::components::Bouce(vec3(
                    rng.gen::<f32>() * 0.6 - 0.3,
                    rng.gen::<f32>() * 0.6 - 0.3,
                    rng.gen::<f32>() * 0.6 - 0.3,
                )))
                .with(world::components::Render::default())
                .build();
        }
        Self { engine, world }
    }

    pub fn resize(&mut self, width: u32, height: u32, scale_factor: f64) {
        if width > 0 && height > 0 {
            self.engine.set_viewport(width, height, scale_factor);
            self.engine.set_depth_texture();
        }
    }

    pub fn update(&mut self, _elapsed: u64) {
        self.world.update();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.engine.get_output_frame();

        self.engine
            .model_pipeline
            .render(&self.engine.ctx, &self.world.components, &frame.view);
        self.engine
            .glyph_pipeline
            .render(&self.engine.ctx, &self.world.components, &frame.view);

        Ok(())
    }
}
