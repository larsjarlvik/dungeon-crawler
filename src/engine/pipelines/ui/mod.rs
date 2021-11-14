use crate::{
    config,
    engine::{self, pipelines::builders, texture},
};
mod uniforms;
mod vertex;
use uniforms::Uniforms;
pub use vertex::Vertex;

pub struct UiPipeline {
    render_pipeline: builders::Pipeline,
    uniform_bind_group_layout: builders::MappedBindGroupLayout,
    texture_bind_group_layout: builders::MappedBindGroupLayout,
    sampler: wgpu::Sampler,
}

impl UiPipeline {
    pub fn new(ctx: &engine::Context) -> Self {
        let builder = builders::PipelineBuilder::new(&ctx, "ui");

        let uniform_bind_group_layout = builder.create_bindgroup_layout(
            0,
            "uniform_bind_group_layout",
            &[builder.create_uniform_entry(0, wgpu::ShaderStages::VERTEX_FRAGMENT)],
        );

        let texture_bind_group_layout = builder.create_bindgroup_layout(
            1,
            "texture_bind_group_layout",
            &[
                builder.create_texture_entry(0, wgpu::ShaderStages::FRAGMENT),
                builder.create_sampler_entry(1, wgpu::ShaderStages::FRAGMENT, false),
            ],
        );

        let render_pipeline = builder
            .with_shader("shaders/ui.wgsl")
            .with_primitve_topology(wgpu::PrimitiveTopology::TriangleList)
            .with_blend(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    operation: wgpu::BlendOperation::Add,
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                },
                alpha: wgpu::BlendComponent::REPLACE,
            })
            .with_buffer_layouts(vec![vertex::Vertex::desc()])
            .with_color_targets(vec![config::COLOR_TEXTURE_FORMAT])
            .with_bind_group_layout(&uniform_bind_group_layout)
            .with_bind_group_layout(&texture_bind_group_layout)
            .build();

        let sampler = texture::Texture::create_sampler(ctx, wgpu::AddressMode::ClampToEdge, wgpu::FilterMode::Linear, None);

        Self {
            render_pipeline,
            uniform_bind_group_layout,
            texture_bind_group_layout,
            sampler,
        }
    }

    pub fn render(
        &self,
        ctx: &engine::Context,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        texture: Option<&texture::Texture>,
        target: &wgpu::TextureView,
    ) {
        let render_bundle_builder = builders::RenderBundleBuilder::new(ctx, "ui");
        let uniform_buffer = render_bundle_builder.create_uniform_buffer_init(bytemuck::cast_slice(&[Uniforms {
            viewport_width: ctx.viewport.width as f32 / ctx.viewport.ui_scale,
            viewport_height: ctx.viewport.height as f32 / ctx.viewport.ui_scale,
            has_texture: if texture.is_some() { 1 } else { 0 },
        }]));

        let mut primitive_bundle = builders::PrimitiveBuilder::new(ctx, "ui")
            .with_vertices(bytemuck::cast_slice(vertices.as_slice()))
            .with_indices(bytemuck::cast_slice(&indices.as_slice()))
            .with_length(indices.len() as u32);

        if let Some(texture) = texture {
            primitive_bundle = primitive_bundle.with_texture_bind_group(
                &self.texture_bind_group_layout,
                &[
                    builders::RenderBundleBuilder::create_entry(0, wgpu::BindingResource::TextureView(&texture.view)),
                    builders::RenderBundleBuilder::create_entry(1, wgpu::BindingResource::Sampler(&self.sampler)),
                ],
            );
        }

        let render_bundle = render_bundle_builder
            .with_pipeline(&self.render_pipeline)
            .with_uniform_bind_group(&self.uniform_bind_group_layout, &uniform_buffer)
            .with_primitive(primitive_bundle)
            .build();

        builders::RenderTargetBuilder::new(ctx, "ui")
            .with_color_attachment(&target, wgpu::LoadOp::Load)
            .execute_bundles(vec![&render_bundle]);
    }
}
