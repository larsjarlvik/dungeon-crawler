use super::{pipeline_builder, render_bundle_builder};
use crate::engine;
use wgpu::util::DeviceExt;

pub struct PrimitiveBuilder<'a> {
    ctx: &'a engine::Context,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub bind_groups: Vec<render_bundle_builder::MappedBindGroup>,
    pub buffers: Vec<&'a wgpu::Buffer>,
    pub length: u32,
    label: &'a str,
}

impl<'a> PrimitiveBuilder<'a> {
    pub fn new(ctx: &'a engine::Context, label: &'a str) -> Self {
        Self {
            ctx,
            vertex_buffer: None,
            index_buffer: None,
            bind_groups: vec![],
            buffers: vec![],
            length: 0,
            label,
        }
    }

    pub fn with_vertices(mut self, contents: &[u8]) -> Self {
        self.vertex_buffer = Some(self.ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(format!("{}_vertex_buffer", self.label).as_str()),
            contents,
            usage: wgpu::BufferUsage::VERTEX,
        }));
        self
    }

    pub fn with_indices(mut self, contents: &[u8]) -> Self {
        self.index_buffer = Some(self.ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(format!("{}_index_buffer", self.label).as_str()),
            contents,
            usage: wgpu::BufferUsage::INDEX,
        }));
        self
    }

    pub fn with_texture_bind_group(
        mut self,
        bind_group_layout: &'a pipeline_builder::MappedBindGroupLayout,
        entries: &[wgpu::BindGroupEntry],
    ) -> Self {
        let bind_group = self.ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(format!("{}_texture_bind_group", self.label).as_str()),
            layout: &bind_group_layout.layout,
            entries,
        });

        self.bind_groups.push(render_bundle_builder::MappedBindGroup {
            bind_group,
            index: bind_group_layout.index,
        });
        self
    }

    pub fn with_uniform_bind_group(
        mut self,
        bind_group_layout: &pipeline_builder::MappedBindGroupLayout,
        uniform_buffer: &'a wgpu::Buffer,
    ) -> Self {
        self.buffers.push(uniform_buffer);

        let bind_group = self.ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout.layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some(format!("{}_uniform_bind_group", self.label).as_str()),
        });

        self.bind_groups.push(render_bundle_builder::MappedBindGroup {
            bind_group,
            index: bind_group_layout.index,
        });

        self
    }

    pub fn with_length(mut self, length: u32) -> Self {
        self.length = length as u32;
        self
    }
}
