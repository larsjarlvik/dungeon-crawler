mod uniforms;
use crate::{pipelines::builders, Context};
pub mod context;

pub struct UiElementPipeline {
    render_pipeline: builders::Pipeline,
    texture_bind_group_layout: builders::MappedBindGroupLayout,
    uniform_bind_group_layout: builders::MappedBindGroupLayout,
    render_pipeline_textured: builders::Pipeline,
}

impl UiElementPipeline {
    pub fn new(ctx: &Context) -> Self {
        let builder = builders::PipelineBuilder::new(ctx, "ui_element");

        let uniform_bind_group_layout = builder.create_bindgroup_layout(
            0,
            "uniform_bind_group_layout",
            &[builder.create_uniform_entry(0, wgpu::ShaderStages::VERTEX_FRAGMENT)],
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

        Self {
            render_pipeline,
            render_pipeline_textured,
            texture_bind_group_layout,
            uniform_bind_group_layout,
        }
    }

    pub fn render(&self, ctx: &mut Context, target: &wgpu::TextureView) {
        let mut command_buffers = vec![];
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
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            for (id, data) in ctx.images.queue.iter() {
                if let Some(id) = id {
                    let asset = ctx.images.textures.get(id).expect("Could not find texture!");

                    render_pass.set_pipeline(&self.render_pipeline_textured.render_pipeline);
                    render_pass.set_bind_group(self.texture_bind_group_layout.index as u32, asset, &[]);
                } else {
                    render_pass.set_pipeline(&self.render_pipeline.render_pipeline);
                }

                if let Some(clip) = data.clip {
                    render_pass.set_scissor_rect(
                        clip[0],
                        clip[1],
                        clip[2].min(ctx.viewport.width - clip[0]),
                        clip[3].min(ctx.viewport.height - clip[1]),
                    );
                } else {
                    render_pass.set_scissor_rect(0, 0, ctx.viewport.width, ctx.viewport.height);
                }

                render_pass.set_bind_group(self.uniform_bind_group_layout.index as u32, &data.uniform_bind_group, &[]);
                render_pass.draw(0..4, 0..1);
            }
        }

        command_buffers.push(encoder.finish());

        ctx.queue.submit(command_buffers);
        ctx.images.queue.clear();
    }
}
