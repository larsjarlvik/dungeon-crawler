use crate::{config, world::*};
mod gltf;
mod uniforms;
mod vertex;

pub use gltf::GltfModel;
use specs::WorldExt;
pub use uniforms::Uniforms;
pub use vertex::Vertex;

pub struct Model {
    pub render_pipeline: wgpu::RenderPipeline,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
}

impl Model {
    pub fn new(device: &wgpu::Device) -> Self {
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("model_shader"),
            flags: wgpu::ShaderFlags::all(),
            source: wgpu::ShaderSource::Wgsl(include_str!("model.wgsl").into()),
        });

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("model_render_pipeline_layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
            depth_stencil: None,
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

    pub fn render(&self, device: &wgpu::Device, queue: &wgpu::Queue, components: &specs::World, view: &wgpu::TextureView) {
        let models = components.read_storage::<components::Model>();
        let render = components.read_storage::<components::Render>();
        let mut bundles = vec![];

        for (model, render) in (&models, &render).join() {
            let uniforms = pipelines::model::Uniforms {
                view_proj: render.view_proj.into(),
                model: render.model_matrix.into(),
            };

            queue.write_buffer(&model.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
            bundles.push(&model.render_bundle);
        }

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("model_encoder"),
        });

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
                depth_stencil_attachment: None,
            })
            .execute_bundles(bundles.into_iter());

        queue.submit(std::iter::once(encoder.finish()));
    }
}
