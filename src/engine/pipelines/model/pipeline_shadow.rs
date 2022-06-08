use crate::{
    config,
    engine::{
        self,
        pipelines::{self, builders},
    },
};

pub struct PipelineShadow {
    pub render_pipeline: builders::Pipeline,
    pub uniform_bind_group_layout: builders::MappedBindGroupLayout,
}

impl PipelineShadow {
    pub fn new(ctx: &engine::Context) -> Self {
        let builder = builders::PipelineBuilder::new(&ctx, "model_shadows");

        let uniform_bind_group_layout = builder.create_bindgroup_layout(
            0,
            "uniform_bind_group_layout",
            &[builder.create_uniform_entry(0, wgpu::ShaderStages::all())],
        );

        let render_pipeline = builder
            .with_shader("shaders/shadow.wgsl")
            .with_depth_bias()
            .with_depth_target(config::DEPTH_FORMAT)
            .with_buffer_layouts(vec![engine::model::VertexPosition::desc()])
            .with_bind_group_layout(&uniform_bind_group_layout)
            .build();

        Self {
            render_pipeline,
            uniform_bind_group_layout,
        }
    }

    pub fn execute_bundles(&self, ctx: &engine::Context, bundles: Vec<&wgpu::RenderBundle>, target: &pipelines::DeferredPipeline) {
        builders::RenderTargetBuilder::new(ctx, "model_shadows")
            .with_depth_attachment(&target.shadow_texture.view, wgpu::LoadOp::Clear(1.0))
            .execute_bundles(bundles);
    }
}
