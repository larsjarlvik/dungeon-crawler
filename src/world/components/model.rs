use crate::engine;
use specs::{Component, VecStorage};

pub struct Model {
    pub uniform_buffer: wgpu::Buffer,
    pub render_bundle: wgpu::RenderBundle,
}

impl Model {
    pub fn from(model: engine::pipelines::Model) -> Self {
        Self {
            uniform_buffer: model.uniform_buffer,
            render_bundle: model.render_bundle,
        }
    }
}

impl Component for Model {
    type Storage = VecStorage<Self>;
}
