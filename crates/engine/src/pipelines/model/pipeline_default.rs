use crate::{config, model, pipelines::builders, Context};

pub struct PipelineDefault {
    pub render_pipeline: builders::Pipeline,
    pub uniform_bind_group_layout: builders::MappedBindGroupLayout,
}

impl PipelineDefault {
    pub fn new(ctx: &Context) -> Self {
        let builder = builders::PipelineBuilder::new(ctx, "model");

        let uniform_bind_group_layout = builder.create_bindgroup_layout(
            0,
            "model_uniform_bind_group_layout",
            &[builder.create_uniform_entry(0, wgpu::ShaderStages::all())],
        );

        let render_pipeline = builder
            .with_shader("shaders/model.wgsl")
            .with_color_targets(vec![ctx.color_format])
            .with_depth_target(config::DEPTH_FORMAT)
            .with_buffer_layouts(vec![model::Vertex::desc()])
            .with_bind_group_layout(&uniform_bind_group_layout)
            .build();

        Self {
            render_pipeline,
            uniform_bind_group_layout,
        }
    }

    pub fn execute_bundles(
        &self,
        ctx: &Context,
        bundles: Vec<&wgpu::RenderBundle>,
        target: &wgpu::TextureView,
        depth_target: &wgpu::TextureView,
    ) {
        builders::RenderTargetBuilder::new(ctx, "model")
            .with_color_attachment(target, wgpu::LoadOp::Load)
            .with_depth_attachment(depth_target, wgpu::LoadOp::Load)
            .execute_bundles(bundles);
    }
}
