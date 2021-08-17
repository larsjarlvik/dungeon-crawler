use super::{pipeline_builder, primitive_builder};
use crate::{config, engine};

pub struct MappedBindGroup {
    pub bind_group: wgpu::BindGroup,
    pub index: u32,
}

pub struct RenderBundleBuilder<'a> {
    ctx: &'a engine::Context,
    pipeline: Option<&'a wgpu::RenderPipeline>,
    bind_groups: Vec<(u32, wgpu::BindGroup)>,
    primitives: Vec<primitive_builder::PrimitiveBuilder<'a>>,
    pub buffers: Vec<&'a wgpu::Buffer>,
    label: &'a str,
}

impl<'a> RenderBundleBuilder<'a> {
    pub fn new(ctx: &'a engine::Context, label: &'a str) -> Self {
        Self {
            ctx,
            pipeline: None,
            bind_groups: vec![],
            primitives: vec![],
            buffers: vec![],
            label,
        }
    }

    pub fn with_pipeline(mut self, pipeline: &'a wgpu::RenderPipeline) -> Self {
        self.pipeline = Some(&pipeline);
        self
    }

    pub fn create_uniform_buffer(&self, size: u64) -> wgpu::Buffer {
        self.ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(format!("{}_uniform_buffer", self.label).as_str()),
            size,
            mapped_at_creation: false,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        })
    }

    pub fn with_uniform_bind_group(mut self, layout: &pipeline_builder::MappedBindGroupLayout, uniform_buffer: &'a wgpu::Buffer) -> Self {
        self.buffers.push(uniform_buffer);

        let uniform_bind_group = self.ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout.layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some(format!("{}_uniform_bind_group", self.label).as_str()),
        });

        self.bind_groups.push((layout.index, uniform_bind_group));
        self
    }

    pub fn with_primitive(mut self, primitive_builder: primitive_builder::PrimitiveBuilder<'a>) -> Self {
        self.primitives.push(primitive_builder);
        self
    }

    pub fn create_entry(binding: u32, resource: wgpu::BindingResource<'a>) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry { binding, resource }
    }

    pub fn build(self) -> wgpu::RenderBundle {
        let mut encoder = self.ctx.device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
            label: Some(format!("{}_encoder", self.label).as_str()),
            color_formats: &[config::COLOR_TEXTURE_FORMAT],
            depth_stencil_format: Some(config::DEPTH_FORMAT),
            sample_count: 1,
        });

        encoder.set_pipeline(&self.pipeline.unwrap());

        for (index, bind_group) in self.bind_groups.iter() {
            encoder.set_bind_group(*index, &bind_group, &[]);
        }

        for primitive in &self.primitives {
            for bind_group in primitive.bind_groups.iter() {
                encoder.set_bind_group(bind_group.index, &bind_group.bind_group, &[]);
            }

            if let Some(vertex_buffer) = &primitive.vertex_buffer {
                encoder.set_vertex_buffer(0, vertex_buffer.slice(..));
            }
            if let Some(index_buffer) = &primitive.index_buffer {
                encoder.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            }

            encoder.draw_indexed(0..primitive.length, 0, 0..1);
        }

        encoder.finish(&wgpu::RenderBundleDescriptor {
            label: Some(format!("{}_render_bundle", self.label).as_str()),
        })
    }
}
