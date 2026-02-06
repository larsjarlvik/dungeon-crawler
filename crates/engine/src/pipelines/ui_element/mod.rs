pub mod uniforms;
use crate::{pipelines::builders, Context};
use std::mem;
pub mod context;

const MAX_UI_ELEMENTS: usize = 256;

pub struct UiElementPipeline {
    render_pipeline: builders::Pipeline,
    texture_bind_group_layout: builders::MappedBindGroupLayout,
    uniform_bind_group_layout: builders::MappedBindGroupLayout,
    render_pipeline_textured: builders::Pipeline,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl UiElementPipeline {
    pub fn new(ctx: &Context) -> Self {
        let builder = builders::PipelineBuilder::new(ctx, "ui_element");

        let uniform_bind_group_layout = builder.create_bindgroup_layout(
            0,
            "uniform_bind_group_layout",
            &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: None,
                },
                count: None,
            }],
        );

        let texture_bind_group_layout = builder.create_bindgroup_layout(
            1,
            "texture_bind_group_layout",
            &[
                builder.create_texture_entry(0, wgpu::ShaderStages::FRAGMENT, true),
                builder.create_sampler_entry(1, wgpu::ShaderStages::FRAGMENT, false),
            ],
        );

        let blend_state = wgpu::BlendState {
            color: wgpu::BlendComponent {
                operation: wgpu::BlendOperation::Add,
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
            },
            alpha: wgpu::BlendComponent::OVER,
        };

        let render_pipeline = builder
            .with_shader("shaders/ui-element.wgsl")
            .with_primitve_topology(wgpu::PrimitiveTopology::TriangleStrip)
            .with_blend(blend_state)
            .with_color_targets(vec![ctx.color_format])
            .with_bind_group_layout(&uniform_bind_group_layout)
            .build();

        let builder = builders::PipelineBuilder::new(ctx, "ui_element_textured");
        let render_pipeline_textured = builder
            .with_shader("shaders/ui-element-textured.wgsl")
            .with_primitve_topology(wgpu::PrimitiveTopology::TriangleStrip)
            .with_blend(blend_state)
            .with_color_targets(vec![ctx.color_format])
            .with_bind_group_layout(&uniform_bind_group_layout)
            .with_bind_group_layout(&texture_bind_group_layout)
            .build();

        let element_size = mem::size_of::<uniforms::Uniforms>() as u64;
        let uniform_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ui_element_uniform_buffer"),
            size: MAX_UI_ELEMENTS as u64 * element_size,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ui_element_uniform_bind_group"),
            layout: &uniform_bind_group_layout.layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &uniform_buffer,
                    offset: 0,
                    size: std::num::NonZeroU64::new(element_size),
                }),
            }],
        });

        Self {
            render_pipeline,
            render_pipeline_textured,
            texture_bind_group_layout,
            uniform_bind_group_layout,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    pub fn render(&self, ctx: &mut Context, target: &wgpu::TextureView) {
        if ctx.images.queue.is_empty() {
            return;
        }

        ctx.images.queue.sort_by_key(|e| e.texture_id.is_some());

        let element_size = mem::size_of::<uniforms::Uniforms>();
        let uniforms: Vec<uniforms::Uniforms> = ctx.images.queue.iter().map(|e| e.uniforms).collect();
        ctx.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&uniforms));

        let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("ui_element_encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ui_element_render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            for (i, elem) in ctx.images.queue.iter().enumerate() {
                let dynamic_offset = (i * element_size) as u32;

                if let Some(id) = &elem.texture_id {
                    let asset = ctx.images.textures.get(id).expect("Could not find texture!");
                    render_pass.set_pipeline(&self.render_pipeline_textured.render_pipeline);
                    render_pass.set_bind_group(self.texture_bind_group_layout.index as u32, asset, &[]);
                } else {
                    render_pass.set_pipeline(&self.render_pipeline.render_pipeline);
                }

                if let Some(clip) = elem.clip {
                    render_pass.set_scissor_rect(
                        clip[0],
                        clip[1],
                        clip[2].min(ctx.viewport.width - clip[0]),
                        clip[3].min(ctx.viewport.height - clip[1]),
                    );
                } else {
                    render_pass.set_scissor_rect(0, 0, ctx.viewport.width, ctx.viewport.height);
                }

                render_pass.set_bind_group(
                    self.uniform_bind_group_layout.index as u32,
                    &self.uniform_bind_group,
                    &[dynamic_offset],
                );
                render_pass.draw(0..4, 0..1);
            }
        }

        ctx.queue.submit(std::iter::once(encoder.finish()));
        ctx.images.queue.clear();
    }
}
