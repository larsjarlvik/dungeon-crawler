use specs::*;

use crate::viewport;
pub mod components;
pub mod pipelines;
pub mod systems;

pub struct World {
    pub components: specs::World,
    pub model_pipeline: pipelines::model::Model,
    pub glyph_pipeline: pipelines::glyph::Glyph,
    pub dispatcher: specs::Dispatcher<'static, 'static>,
}

impl<'a> World {
    pub fn new(device: &wgpu::Device) -> Self {
        let model_pipeline = pipelines::model::Model::new(device);
        let glyph_pipeline = pipelines::glyph::Glyph::new(device);

        let mut components = specs::World::new();
        components.register::<components::Camera>();
        components.register::<components::Render>();
        components.register::<components::Model>();
        components.register::<components::Position>();
        components.register::<components::Bouce>();
        components.register::<components::Text>();
        components.register::<components::Fps>();

        let dispatcher = DispatcherBuilder::new()
            .with(systems::Bounce, "bounce", &[])
            .with(systems::Render, "render", &[])
            .with(systems::Fps, "FPS", &[])
            .build();

        Self {
            model_pipeline,
            glyph_pipeline,
            components,
            dispatcher,
        }
    }

    pub fn load_model(&self, device: &wgpu::Device) -> pipelines::model::GltfModel {
        pipelines::model::GltfModel::new(device, &self.model_pipeline)
    }

    pub fn update(&mut self) {
        self.dispatcher.setup(&mut self.components);
        self.dispatcher.dispatch(&mut self.components);
        self.components.maintain();
    }

    pub fn render(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, viewport: &viewport::Viewport, view: &wgpu::TextureView) {
        self.model_pipeline.render(device, queue, &self.components, view);
        self.glyph_pipeline.render(device, queue, &self.components, viewport, view);
    }
}
