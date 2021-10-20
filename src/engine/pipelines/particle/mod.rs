use std::mem;

use super::builders;
use crate::{
    config, engine,
    world::{components, resources},
};
use rand::Rng;
use specs::{Join, WorldExt};
mod uniforms;
mod vertex;

pub struct ParticleEmitter {
    pub render_bundle: wgpu::RenderBundle,
    pub uniform_buffer: wgpu::Buffer,
}

pub struct ParticlePipeline {
    render_pipeline: builders::Pipeline,
    uniform_bind_group_layout: builders::MappedBindGroupLayout,
    vertex: Vec<vertex::Vertex>,
}

impl ParticlePipeline {
    pub fn new(ctx: &engine::Context) -> Self {
        let builder = builders::PipelineBuilder::new(&ctx, "particle");
        let uniform_bind_group_layout = builder.create_bindgroup_layout(
            0,
            "uniform_bind_group_layout",
            &[builder.create_uniform_entry(0, wgpu::ShaderStages::VERTEX_FRAGMENT)],
        );

        let render_pipeline = builder
            .with_shader("shaders/particle.wgsl")
            .with_primitve_topology(wgpu::PrimitiveTopology::TriangleStrip)
            .with_buffer_layouts(vec![vertex::Vertex::desc(), vertex::Instance::desc()])
            .with_depth_target(config::DEPTH_FORMAT)
            .with_blend(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    operation: wgpu::BlendOperation::Add,
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                },
                alpha: wgpu::BlendComponent::REPLACE,
            })
            .with_color_targets(vec![config::COLOR_TEXTURE_FORMAT])
            .with_bind_group_layout(&uniform_bind_group_layout)
            .build();

        let size = 0.02;
        let vertex = vec![
            vertex::Vertex { position: [size, -size] },
            vertex::Vertex { position: [size, size] },
            vertex::Vertex { position: [-size, -size] },
            vertex::Vertex { position: [-size, size] },
        ];

        Self {
            render_pipeline,
            uniform_bind_group_layout,
            vertex,
        }
    }

    pub fn render(&self, ctx: &engine::Context, components: &specs::World, target: &wgpu::TextureView, depth_target: &wgpu::TextureView) {
        let camera = components.read_resource::<resources::Camera>();
        let time = components.read_resource::<resources::Time>();
        let particle = components.read_storage::<components::Particle>();
        let transform = components.read_storage::<components::Transform>();
        let mut bundles = vec![];

        for (particle, transform) in (&particle, &transform).join() {
            let uniforms = uniforms::Uniforms {
                view: camera.view.into(),
                proj: camera.proj.into(),
                model: transform.to_matrix(time.last_frame).into(),
                start_color: particle.start_color.extend(1.0).into(),
                end_color: particle.end_color.extend(1.0).into(),
                life: [0.001, time.total_time.elapsed().as_secs_f32(), 0.0, 0.0],
            };

            ctx.queue
                .write_buffer(&particle.emitter.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

            bundles.push(&particle.emitter.render_bundle);
        }

        builders::RenderTargetBuilder::new(ctx, "particle")
            .with_color_attachment(&target, wgpu::LoadOp::Load)
            .with_depth_attachment(&depth_target, wgpu::LoadOp::Load)
            .execute_bundles(bundles);
    }

    pub fn create_emitter(&self, ctx: &engine::Context) -> ParticleEmitter {
        let mut particles = vec![];
        let mut rng = rand::thread_rng();
        for _ in 0..500 {
            let spread = 0.6 - 0.3;
            particles.push(vertex::Instance {
                data: [
                    rng.gen::<f32>(),                        // lifetime
                    rng.gen::<f32>() * 0.4 + 0.2,            // Speed
                    (rng.gen::<f32>() * 2.0 - 1.0) * spread, // spread X
                    (rng.gen::<f32>() * 2.0 - 1.0) * spread, // spread Z
                ],
            });
        }

        let builder = builders::RenderBundleBuilder::new(ctx, "deferred");
        let uniform_buffer = builder.create_uniform_buffer(mem::size_of::<uniforms::Uniforms>() as u64);

        let render_bundle = builders::RenderBundleBuilder::new(ctx, "particle")
            .with_pipeline(&self.render_pipeline)
            .with_uniform_bind_group(&self.uniform_bind_group_layout, &uniform_buffer)
            .with_primitive(
                builders::PrimitiveBuilder::new(ctx, "particle")
                    .with_vertices(bytemuck::cast_slice(self.vertex.as_slice()))
                    .with_instances(bytemuck::cast_slice(particles.as_slice()))
                    .with_instance_count(particles.len() as u32)
                    .with_length(4),
            )
            .build();

        ParticleEmitter {
            render_bundle,
            uniform_buffer,
        }
    }
}
