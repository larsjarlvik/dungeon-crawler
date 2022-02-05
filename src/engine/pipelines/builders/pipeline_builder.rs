use std::borrow::Cow;

use crate::{config, engine, utils};

pub struct MappedBindGroupLayout {
    pub layout: wgpu::BindGroupLayout,
    pub index: u64,
}

pub struct PipelineBuilder<'a> {
    ctx: &'a engine::Context,
    shader: Option<wgpu::ShaderModule>,
    bind_group_layouts: Vec<&'a wgpu::BindGroupLayout>,
    color_targets: Vec<wgpu::TextureFormat>,
    depth_target: Option<wgpu::RenderBundleDepthStencil>,
    depth_bias: Option<wgpu::DepthBiasState>,
    buffer_layouts: Vec<wgpu::VertexBufferLayout<'a>>,
    primitve_topology: wgpu::PrimitiveTopology,
    depth_write: bool,
    blend: Option<wgpu::BlendState>,
    label: &'a str,
}

pub struct Pipeline {
    pub render_pipeline: wgpu::RenderPipeline,
    pub color_targets: Vec<wgpu::TextureFormat>,
    pub depth_target: Option<wgpu::RenderBundleDepthStencil>,
}

impl<'a> PipelineBuilder<'a> {
    pub fn new(ctx: &'a engine::Context, label: &'a str) -> Self {
        Self {
            ctx,
            shader: None,
            bind_group_layouts: vec![],
            color_targets: vec![],
            depth_target: None,
            depth_bias: None,
            buffer_layouts: vec![],
            primitve_topology: wgpu::PrimitiveTopology::TriangleList,
            blend: None,
            depth_write: true,
            label,
        }
    }

    pub fn with_shader(mut self, path: &str) -> Self {
        self.shader = Some(self.ctx.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some(format!("{}_shader", self.label).as_str()),
            source: wgpu::ShaderSource::Wgsl(Cow::from(utils::read_string(path).as_str())),
        }));
        self
    }

    pub fn with_bind_group_layout(mut self, bind_group_layout: &'a MappedBindGroupLayout) -> Self {
        self.bind_group_layouts.push(&bind_group_layout.layout);
        self
    }

    pub fn create_bindgroup_layout(&self, index: u64, label: &str, entries: &[wgpu::BindGroupLayoutEntry]) -> MappedBindGroupLayout {
        let layout = self.ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(label),
            entries,
        });

        MappedBindGroupLayout { index, layout }
    }

    pub fn create_uniform_entry(&self, binding: u32, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }

    pub fn create_texture_entry(&self, binding: u32, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Texture {
                multisampled: false,
                view_dimension: wgpu::TextureViewDimension::D2,
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
            },
            count: None,
        }
    }

    pub fn create_shadow_texture_entry(&self, binding: u32, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Texture {
                multisampled: false,
                sample_type: wgpu::TextureSampleType::Depth,
                view_dimension: wgpu::TextureViewDimension::D2,
            },
            count: None,
        }
    }

    pub fn create_sampler_entry(&self, binding: u32, visibility: wgpu::ShaderStages, comparison: bool) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Sampler(if comparison {
                wgpu::SamplerBindingType::Comparison
            } else {
                wgpu::SamplerBindingType::Filtering
            }),
            count: None,
        }
    }

    pub fn with_color_targets(mut self, formats: Vec<wgpu::TextureFormat>) -> Self {
        self.color_targets = formats;
        self
    }

    pub fn with_primitve_topology(mut self, primitve_topology: wgpu::PrimitiveTopology) -> Self {
        self.primitve_topology = primitve_topology;
        self
    }

    pub fn with_depth_target(mut self, format: wgpu::TextureFormat) -> Self {
        self.depth_target = Some(wgpu::RenderBundleDepthStencil {
            format,
            depth_read_only: false,
            stencil_read_only: false,
        });
        self
    }

    pub fn with_buffer_layouts(mut self, buffer_layouts: Vec<wgpu::VertexBufferLayout<'a>>) -> Self {
        self.buffer_layouts = buffer_layouts;
        self
    }

    pub fn with_blend(mut self, blend: wgpu::BlendState) -> Self {
        self.blend = Some(blend);
        self
    }

    pub fn with_depth_bias(mut self) -> Self {
        self.depth_bias = Some(wgpu::DepthBiasState {
            constant: 6,
            slope_scale: 2.0,
            clamp: 0.0,
        });
        self
    }

    pub fn with_depth_write(mut self, depth_write: bool) -> Self {
        self.depth_write = depth_write;
        self
    }

    pub fn build(self) -> Pipeline {
        let shader = self.shader.unwrap();

        let render_pipeline_layout = self.ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(format!("{}_render_pipeline_layout", self.label).as_str()),
            bind_group_layouts: self.bind_group_layouts.as_slice(),
            push_constant_ranges: &[],
        });

        let blend = self.blend;
        let color_targets: Vec<wgpu::ColorTargetState> = self
            .color_targets
            .iter()
            .map(|format| wgpu::ColorTargetState {
                format: *format,
                blend,
                write_mask: wgpu::ColorWrites::ALL,
            })
            .collect();

        let depth_stencil = if self.depth_target.is_some() {
            Some(wgpu::DepthStencilState {
                format: config::DEPTH_FORMAT,
                depth_write_enabled: self.depth_write,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: if let Some(depth_bias) = self.depth_bias {
                    depth_bias
                } else {
                    wgpu::DepthBiasState::default()
                },
            })
        } else {
            None
        };

        let fragment = Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "frag_main",
            targets: color_targets.as_slice(),
        });

        let render_pipeline = self.ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(format!("{}_render_pipeline", self.label).as_str()),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vert_main",
                buffers: &self.buffer_layouts,
            },
            fragment,
            primitive: wgpu::PrimitiveState {
                topology: self.primitve_topology,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Pipeline {
            render_pipeline,
            color_targets: self.color_targets,
            depth_target: self.depth_target,
        }
    }
}
