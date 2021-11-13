use crate::{
    config,
    engine::{self, pipelines::builders},
};
mod uniforms;
mod vertex;
use uniforms::Uniforms;
pub use vertex::Vertex;

pub struct UiPipeline {
    render_pipeline: builders::Pipeline,
    uniform_bind_group_layout: builders::MappedBindGroupLayout,
}

impl UiPipeline {
    pub fn new(ctx: &engine::Context) -> Self {
        let builder = builders::PipelineBuilder::new(&ctx, "ui");

        let uniform_bind_group_layout = builder.create_bindgroup_layout(
            0,
            "uniform_bind_group_layout",
            &[builder.create_uniform_entry(0, wgpu::ShaderStages::VERTEX_FRAGMENT)],
        );

        let render_pipeline = builder
            .with_shader("shaders/ui.wgsl")
            .with_primitve_topology(wgpu::PrimitiveTopology::TriangleList)
            .with_buffer_layouts(vec![vertex::Vertex::desc()])
            .with_color_targets(vec![config::COLOR_TEXTURE_FORMAT])
            .with_bind_group_layout(&uniform_bind_group_layout)
            .build();

        Self {
            render_pipeline,
            uniform_bind_group_layout,
        }
    }

    pub fn render(&self, ctx: &engine::Context, vertices: Vec<Vertex>, indices: Vec<u32>, target: &wgpu::TextureView) {
        let render_bundle_builder = builders::RenderBundleBuilder::new(ctx, "ui");
        let uniform_buffer = render_bundle_builder.create_uniform_buffer_init(bytemuck::cast_slice(&[Uniforms {
            viewport_width: ctx.viewport.width as f32,
            viewport_height: ctx.viewport.height as f32,
        }]));

        let render_bundle = render_bundle_builder
            .with_pipeline(&self.render_pipeline)
            .with_uniform_bind_group(&self.uniform_bind_group_layout, &uniform_buffer)
            .with_primitive(
                builders::PrimitiveBuilder::new(ctx, "ui")
                    .with_vertices(bytemuck::cast_slice(vertices.as_slice()))
                    .with_indices(bytemuck::cast_slice(&indices.as_slice()))
                    .with_length(indices.len() as u32),
            )
            .build();

        builders::RenderTargetBuilder::new(ctx, "ui")
            .with_color_attachment(&target, wgpu::LoadOp::Load)
            .execute_bundles(vec![&render_bundle]);
    }
}
