use super::*;
use std::mem;
use wgpu::util::DeviceExt;

pub struct Model {
    pub uniform_buffer: wgpu::Buffer,
    pub render_bundle: wgpu::RenderBundle,
}

impl Model {
    pub fn new(ctx: &engine::Context, pipeline: &engine::pipelines::model::ModelPipeline) -> Model {
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

        let vertex_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("model_vertex_buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let uniform_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("uniform_buffer"),
            size: mem::size_of::<Uniforms>() as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let uniform_bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &pipeline.uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        let mut encoder = ctx.device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
            label: Some("model_bundle"),
            color_formats: &[config::COLOR_TEXTURE_FORMAT],
            depth_stencil_format: if_some!(ctx.depth_texture, Some(config::DEPTH_FORMAT)),
            sample_count: 1,
        });
        encoder.set_pipeline(&pipeline.render_pipeline);
        encoder.set_bind_group(0, &uniform_bind_group, &[]);
        encoder.set_vertex_buffer(0, vertex_buffer.slice(..));
        encoder.draw(0..3, 0..1);
        let render_bundle = encoder.finish(&wgpu::RenderBundleDescriptor {
            label: Some("model_render_bundle"),
        });

        Self {
            uniform_buffer,
            render_bundle,
        }
    }
}
