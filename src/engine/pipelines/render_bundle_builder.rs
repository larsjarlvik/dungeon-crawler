use wgpu::util::DeviceExt;

use crate::{config, engine};

pub struct RenderBundleBuilder<'a> {
    ctx: &'a engine::Context,
    pipeline: Option<&'a wgpu::RenderPipeline>,
    bind_groups: Vec<wgpu::BindGroup>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    pub buffers: Vec<&'a wgpu::Buffer>,
    length: u32,
}

impl<'a> RenderBundleBuilder<'a> {
    pub fn new(ctx: &'a engine::Context) -> Self {
        Self {
            ctx,
            pipeline: None,
            bind_groups: vec![],
            vertex_buffer: None,
            index_buffer: None,
            buffers: vec![],
            length: 0,
        }
    }

    pub fn with_pipeline(mut self, pipeline: &'a wgpu::RenderPipeline) -> Self {
        self.pipeline = Some(&pipeline);
        self
    }

    pub fn create_uniform_buffer(&self, size: u64) -> wgpu::Buffer {
        self.ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("uniform_buffer"),
            size,
            mapped_at_creation: false,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        })
    }

    pub fn with_uniform_bind_group(mut self, layout: &wgpu::BindGroupLayout, uniform_buffer: &'a wgpu::Buffer) -> Self {
        self.buffers.push(uniform_buffer);

        let uniform_bind_group = self.ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        self.bind_groups.push(uniform_bind_group);
        self
    }

    pub fn with_texture_bind_group(mut self, layout: &wgpu::BindGroupLayout, entries: &[wgpu::BindGroupEntry]) -> Self {
        let texture_bind_group = self.ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("texture_bind_group"),
            layout,
            entries,
        });

        self.bind_groups.push(texture_bind_group);
        self
    }

    pub fn with_vertices(mut self, contents: &[u8]) -> Self {
        self.vertex_buffer = Some(self.ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("model_vertex_buffer"),
            contents,
            usage: wgpu::BufferUsage::VERTEX,
        }));
        self
    }

    pub fn with_indices(mut self, contents: &[u8]) -> Self {
        self.index_buffer = Some(self.ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("model_index_buffer"),
            contents,
            usage: wgpu::BufferUsage::INDEX,
        }));
        self
    }

    pub fn with_length(mut self, length: u32) -> Self {
        self.length = length as u32;
        self
    }

    pub fn create_entry(binding: u32, resource: wgpu::BindingResource<'a>) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry { binding, resource }
    }

    pub fn build(self) -> wgpu::RenderBundle {
        let mut encoder = self.ctx.device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
            label: Some("model_bundle"),
            color_formats: &[config::COLOR_TEXTURE_FORMAT],
            depth_stencil_format: Some(config::DEPTH_FORMAT),
            sample_count: 1,
        });

        encoder.set_pipeline(&self.pipeline.unwrap());

        for (index, bind_group) in self.bind_groups.iter().enumerate() {
            encoder.set_bind_group(index as u32, &bind_group, &[]);
        }

        let vertex_buffer = self.vertex_buffer.unwrap();
        let index_buffer = self.index_buffer.unwrap();

        encoder.set_vertex_buffer(0, vertex_buffer.slice(..));
        encoder.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        encoder.draw_indexed(0..self.length, 0, 0..1);

        encoder.finish(&wgpu::RenderBundleDescriptor {
            label: Some("model_render_bundle"),
        })
    }
}
