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
        let mut engine = engine::Engine::new(window).await;
        engine.set_depth_texture();
        engine.set_glyph_pipeline();
        engine.set_model_pipeline();

        let mut world = world::World::new();
        world
            .components
            .create_entity()
            .with(world::components::Fps::new())
            .with(world::components::Text::new("", vec2(20.0, 20.0)))
            .build();

        let mut rng = rand::thread_rng();
        for _ in 0..500 {
            let model = engine.load_model();

            world
                .components
                .create_entity()
                .with(world::components::Camera::new(
                    engine.ctx.viewport.width as u32,
                    engine.ctx.viewport.height as u32,
                ))
                .with(world::components::Model::from(model))
                .with(world::components::Position(vec3(
                    rng.gen::<f32>() * 4.0 - 2.0,
                    rng.gen::<f32>() * 4.0 - 2.0,
                    rng.gen::<f32>() * 4.0 - 2.0,
                )))
                .with(world::components::Bouce(vec3(
                    rng.gen::<f32>() * 0.06 - 0.03,
                    rng.gen::<f32>() * 0.06 - 0.03,
                    rng.gen::<f32>() * 0.06 - 0.03,
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

        if let Some(model_pipeline) = &self.engine.model_pipeline {
            model_pipeline.render(&self.engine.ctx, &self.world.components, &frame.view);
        }
        if let Some(glyph_pipeline) = &mut self.engine.glyph_pipeline {
            glyph_pipeline.render(&self.engine.ctx, &self.world.components, &frame.view);
        }

        Ok(())
    }
}
