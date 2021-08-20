use crate::{config, engine};

pub struct MappedBindGroupLayout {
    pub layout: wgpu::BindGroupLayout,
    pub index: u32,
}

pub struct PipelineBuilder<'a> {
    ctx: &'a engine::Context,
    shader: Option<wgpu::ShaderModule>,
    bind_group_layouts: Vec<&'a wgpu::BindGroupLayout>,
    color_targets: Vec<wgpu::TextureFormat>,
    depth_target: Option<wgpu::TextureFormat>,
    buffer_layouts: Vec<wgpu::VertexBufferLayout<'a>>,
    label: &'a str,
}

pub struct Pipeline {
    pub render_pipeline: wgpu::RenderPipeline,
    pub color_targets: Vec<wgpu::TextureFormat>,
    pub depth_target: Option<wgpu::TextureFormat>,
}

impl<'a> PipelineBuilder<'a> {
    pub fn new(ctx: &'a engine::Context, label: &'a str) -> Self {
        Self {
            ctx,
            shader: None,
            bind_group_layouts: vec![],
            color_targets: vec![],
            depth_target: None,
            buffer_layouts: vec![],
            label,
        }
    }

    pub fn with_shader(mut self, source: wgpu::ShaderSource) -> Self {
        self.shader = Some(self.ctx.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some(format!("{}_shader", self.label).as_str()),
            flags: wgpu::ShaderFlags::all(),
            source,
        }));
        self
    }

    pub fn with_bind_group_layout(mut self, bind_group_layout: &'a MappedBindGroupLayout) -> Self {
        self.bind_group_layouts.push(&bind_group_layout.layout);
        self
    }

    pub fn create_bindgroup_layout(&self, index: u32, label: &str, entries: &[wgpu::BindGroupLayoutEntry]) -> MappedBindGroupLayout {
        let layout = self.ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(label),
            entries,
        });

        MappedBindGroupLayout { index, layout }
    }

    pub fn create_uniform_entry(&self, binding: u32, visibility: wgpu::ShaderStage) -> wgpu::BindGroupLayoutEntry {
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

    pub fn create_texture_entry(&self, binding: u32, visibility: wgpu::ShaderStage) -> wgpu::BindGroupLayoutEntry {
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

    pub fn create_sampler_entry(&self, binding: u32, visibility: wgpu::ShaderStage) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Sampler {
                comparison: false,
                filtering: true,
            },
            count: None,
        }
    }

    pub fn with_color_targets(mut self, formats: Vec<wgpu::TextureFormat>) -> Self {
        self.color_targets = formats;
        self
    }

    pub fn with_depth_target(mut self, depth_target: wgpu::TextureFormat) -> Self {
        self.depth_target = Some(depth_target);
        self
    }

    pub fn with_buffer_layouts(mut self, buffer_layouts: Vec<wgpu::VertexBufferLayout<'a>>) -> Self {
        self.buffer_layouts = buffer_layouts;
        self
    }

    pub fn build(self) -> Pipeline {
        let shader = self.shader.unwrap();

        let render_pipeline_layout = self.ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(format!("{}_render_pipeline_layout", self.label).as_str()),
            bind_group_layouts: self.bind_group_layouts.as_slice(),
            push_constant_ranges: &[],
        });

        let color_targets: Vec<wgpu::ColorTargetState> = self
            .color_targets
            .iter()
            .map(|format| wgpu::ColorTargetState {
                format: *format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrite::ALL,
            })
            .collect();

        let depth_stencil = if self.depth_target.is_some() {
            Some(wgpu::DepthStencilState {
                format: config::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            })
        } else {
            None
        };

        let render_pipeline = self.ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(format!("{}_render_pipeline", self.label).as_str()),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "main",
                buffers: &self.buffer_layouts,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "main",
                targets: color_targets.as_slice(),
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
            depth_stencil,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        Pipeline {
            render_pipeline,
            color_targets: self.color_targets,
            depth_target: self.depth_target,
        }
    }
}
