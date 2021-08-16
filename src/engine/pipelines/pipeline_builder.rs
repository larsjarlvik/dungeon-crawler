use crate::{config, engine};

pub struct PipelineBuilder<'a> {
    ctx: &'a engine::Context,
    shader: Option<wgpu::ShaderModule>,
    bind_group_layouts: Vec<&'a wgpu::BindGroupLayout>,
}

impl<'a> PipelineBuilder<'a> {
    pub fn new(ctx: &'a engine::Context) -> Self {
        Self {
            ctx,
            shader: None,
            bind_group_layouts: vec![],
        }
    }

    pub fn with_shader(mut self, source: wgpu::ShaderSource) -> Self {
        self.shader = Some(self.ctx.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("model_shader"),
            flags: wgpu::ShaderFlags::all(),
            source,
        }));
        self
    }

    pub fn with_bind_group_layout(mut self, bind_group_layout: &'a wgpu::BindGroupLayout) -> Self {
        self.bind_group_layouts.push(bind_group_layout);
        self
    }

    pub fn create_bindgroup_layout(&self, label: &str, entries: &[wgpu::BindGroupLayoutEntry]) -> wgpu::BindGroupLayout {
        self.ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(label),
            entries,
        })
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

    pub fn create_sampler(&self) -> wgpu::Sampler {
        self.ctx.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        })
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

    pub fn build(self) -> wgpu::RenderPipeline {
        let shader = self.shader.unwrap();

        let render_pipeline_layout = self.ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("model_render_pipeline_layout"),
            bind_group_layouts: self.bind_group_layouts.as_slice(),
            push_constant_ranges: &[],
        });

        self.ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("model_render_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "main",
                buffers: &[engine::model::Vertex::desc()],
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: config::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        })
    }
}
