use crate::{
    config,
    engine::{
        self,
        pipelines::{self, builders},
        texture,
    },
};

pub struct PipelineDisplay {
    pub render_pipeline: builders::Pipeline,
    pub uniform_bind_group_layout: builders::MappedBindGroupLayout,
    pub primitive_uniform_bind_group_layout: builders::MappedBindGroupLayout,
    pub texture_bind_group_layout: builders::MappedBindGroupLayout,
    pub sampler: wgpu::Sampler,
}

impl PipelineDisplay {
    pub fn new(ctx: &engine::Context) -> Self {
        let builder = builders::PipelineBuilder::new(&ctx, "model");
        let sampler = texture::Texture::create_sampler(ctx, wgpu::AddressMode::Repeat, wgpu::FilterMode::Linear, None);

        let uniform_bind_group_layout = builder.create_bindgroup_layout(
            0,
            "model_uniform_bind_group_layout",
            &[builder.create_uniform_entry(0, wgpu::ShaderStages::VERTEX)],
        );

        let primitive_uniform_bind_group_layout = builder.create_bindgroup_layout(
            1,
            "model_uniform_bind_group_layout",
            &[builder.create_uniform_entry(0, wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX)],
        );

        let texture_bind_group_layout = builder.create_bindgroup_layout(
            2,
            "texture_bind_group_layout",
            &[
                builder.create_texture_entry(0, wgpu::ShaderStages::FRAGMENT),
                builder.create_texture_entry(1, wgpu::ShaderStages::FRAGMENT),
                builder.create_texture_entry(2, wgpu::ShaderStages::FRAGMENT),
                builder.create_sampler_entry(3, wgpu::ShaderStages::FRAGMENT, false),
            ],
        );

        let render_pipeline = builder
            .with_shader("shaders/model.wgsl")
            .with_color_targets(vec![
                config::COLOR_TEXTURE_FORMAT,
                config::COLOR_TEXTURE_FORMAT,
                config::COLOR_TEXTURE_FORMAT,
            ])
            .with_depth_target(config::DEPTH_FORMAT)
            .with_buffer_layouts(vec![engine::model::Vertex::desc()])
            .with_bind_group_layout(&uniform_bind_group_layout)
            .with_bind_group_layout(&primitive_uniform_bind_group_layout)
            .with_bind_group_layout(&texture_bind_group_layout)
            .build();

        Self {
            render_pipeline,
            uniform_bind_group_layout,
            primitive_uniform_bind_group_layout,
            texture_bind_group_layout,
            sampler,
        }
    }

    pub fn execute_bundles(&self, ctx: &engine::Context, bundles: Vec<&wgpu::RenderBundle>, target: &pipelines::DeferredPipeline) {
        builders::RenderTargetBuilder::new(ctx, "model")
            .with_color_attachment(&target.normal_texture.view, wgpu::LoadOp::Clear(config::CLEAR_COLOR))
            .with_color_attachment(&target.color_texture.view, wgpu::LoadOp::Clear(config::CLEAR_COLOR))
            .with_color_attachment(&target.orm_texture.view, wgpu::LoadOp::Clear(config::CLEAR_COLOR))
            .with_depth_attachment(&target.depth_texture.view, wgpu::LoadOp::Clear(1.0))
            .execute_bundles(bundles);
    }
}
