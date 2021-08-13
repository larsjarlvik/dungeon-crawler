use crate::{camera, config, model::vertex::Vertex};
use cgmath::*;
use std::mem;
use wgpu::util::DeviceExt;

mod vertex;

pub struct Model {
    render_pipeline: wgpu::RenderPipeline,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelInstanceUniforms {
    view_proj: [[f32; 4]; 4],
    model: [[f32; 4]; 4],
}

pub struct ModelInstance {
    vertex_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl Model {
    pub fn new(device: &wgpu::Device) -> Self {
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("model_shader"),
            flags: wgpu::ShaderFlags::all(),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
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

    pub fn load(&self, device: &wgpu::Device) -> ModelInstance {
        const VERTICES: &[Vertex] = &[
            Vertex {
                position: [0.0, 0.5, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [0.0, 0.0, 1.0],
            },
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("model_vertex_buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("uniform_buffer"),
            size: mem::size_of::<ModelInstanceUniforms>() as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        ModelInstance {
            vertex_buffer,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    pub fn update(&self, queue: &wgpu::Queue, model_instance: &mut ModelInstance, camera: &camera::Camera, elapsed: u64) {
        let uniforms = ModelInstanceUniforms {
            view_proj: camera.build_view_projection_matrix().into(),
            model: (Matrix4::from_translation(vec3(((elapsed as f32) * 0.005).sin(), 0.0, 0.0))
                * Matrix4::from_angle_y(cgmath::Deg(elapsed as f32 * 0.3)))
            .into(),
        };

        queue.write_buffer(&model_instance.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }

    pub fn render(&self, encoder: &mut wgpu::CommandEncoder, model_instances: &Vec<ModelInstance>, view: &wgpu::TextureView) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("model_render_pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.192,
                        g: 0.204,
                        b: 0.220,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        for model_instance in model_instances.into_iter() {
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, model_instance.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &model_instance.uniform_bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }
    }
}
