use crate::{
    config,
    engine::{self, pipelines},
    world::*,
};
mod model;
mod uniforms;
mod vertex;

pub use model::Model;
use specs::{Join, WorldExt};
pub use uniforms::Uniforms;
pub use vertex::Vertex;

pub struct ModelPipeline {
    pub render_pipeline: wgpu::RenderPipeline,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
}

impl ModelPipeline {
    pub fn new(ctx: &engine::Context) -> Self {
        let shader = ctx.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("model_shader"),
            flags: wgpu::ShaderFlags::all(),
            source: wgpu::ShaderSource::Wgsl(include_str!("model.wgsl").into()),
        });

        let uniform_bind_group_layout = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("model_uniform_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let render_pipeline_layout = ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("model_render_pipeline_layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("model_render_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: config::COLOR_TEXTURE_FORMAT,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: if_some!(
                ctx.depth_texture,
                Some(wgpu::DepthStencilState {
                    format: config::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                })
            ),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        Self {
            render_pipeline,
            uniform_bind_group_layout,
        }
    }

    pub fn render(&self, ctx: &engine::Context, components: &specs::World, view: &wgpu::TextureView) {
        let models = components.read_storage::<components::Model>();
        let render = components.read_storage::<components::Render>();
        let mut bundles = vec![];

        for (model, render) in (&models, &render).join() {
            let uniforms = pipelines::model::Uniforms {
                view_proj: render.view_proj.into(),
                model: render.model_matrix.into(),
            };

            ctx.queue.write_buffer(&model.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
            bundles.push(&model.render_bundle);
        }

        let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("model_encoder"),
        });

        let depth_stencil_attachment = if let Some(depth_texture) = &ctx.depth_texture {
            if_some!(
                ctx.depth_texture,
                Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                })
            )
        } else {
            None
        };

        encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("model_render_pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(config::CLEAR_COLOR),
                        store: true,
                    },
                }],
                depth_stencil_attachment,
            })
            .execute_bundles(bundles.into_iter());

        ctx.queue.submit(std::iter::once(encoder.finish()));
    }
}
