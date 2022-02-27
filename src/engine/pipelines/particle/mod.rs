use std::{f32::consts, mem};

use super::builders;
use crate::{
    config, engine,
    utils::Interpolate,
    world::{components, resources},
};
use bevy_ecs::prelude::World;
use rand::Rng;
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
            .with_depth_write(false)
            .with_blend(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    operation: wgpu::BlendOperation::Add,
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::One,
                },
                alpha: wgpu::BlendComponent::OVER,
            })
            .with_color_targets(vec![config::COLOR_TEXTURE_FORMAT])
            .with_bind_group_layout(&uniform_bind_group_layout)
            .build();

        let vertex = vec![
            vertex::Vertex { position: [1.0, -1.0] },
            vertex::Vertex { position: [1.0, 1.0] },
            vertex::Vertex { position: [-1.0, -1.0] },
            vertex::Vertex { position: [-1.0, 1.0] },
        ];

        Self {
            render_pipeline,
            uniform_bind_group_layout,
            vertex,
        }
    }

    pub fn render(&self, ctx: &engine::Context, components: &mut World, target: &wgpu::TextureView, depth_target: &wgpu::TextureView) {
        let (view, proj, frustum) = {
            let camera = components.get_resource::<resources::Camera>().unwrap();
            (camera.view, camera.proj, camera.frustum)
        };
        let (last_frame, total_time) = {
            let time = components.get_resource::<resources::Time>().unwrap();
            (time.last_frame, time.total_time)
        };
        let mut bundles = vec![];

        for (particle, transform, render) in components
            .query::<(&components::Particle, &components::Transform, &components::Render)>()
            .iter(&components)
        {
            if render.cull_frustum {
                let transformed_bb = particle.bounding_box.transform(transform.to_matrix(last_frame).into());
                if !frustum.test_bounding_box(&transformed_bb) {
                    continue;
                }
            }

            let uniforms = uniforms::Uniforms {
                view: view.into(),
                proj: proj.into(),
                model: transform.to_matrix(last_frame).into(),
                start_color: particle.start_color.extend(1.0).into(),
                end_color: particle.end_color.extend(1.0).into(),
                life: [
                    total_time.elapsed().as_secs_f32(),
                    particle.strength.get(last_frame),
                    particle.size,
                    0.0,
                ],
            };

            let emitter = ctx
                .emitter_instances
                .get(&particle.emitter)
                .expect("Could not find particle emitter!");

            ctx.queue
                .write_buffer(&emitter.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

            bundles.push(&emitter.render_bundle);
        }

        builders::RenderTargetBuilder::new(ctx, "particle")
            .with_color_attachment(&target, wgpu::LoadOp::Load)
            .with_depth_attachment(&depth_target, wgpu::LoadOp::Load)
            .execute_bundles(bundles);
    }

    pub fn create_emitter(&self, ctx: &engine::Context, count: u32, life_time: f32, spread: f32, speed: f32) -> ParticleEmitter {
        let mut particles = vec![];
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let angle = rng.gen::<f32>() * consts::PI * 2.0;
            let dist = (rng.gen::<f32>() * (spread / 2.0)).max(rng.gen::<f32>() * (spread * 0.8 / 2.0));
            let x = dist * angle.sin();
            let z = dist * angle.cos();

            let life = life_time * (1.0 - (dist / spread));

            particles.push(vertex::Instance {
                life_speed: [
                    rng.gen::<f32>() * life,                  // lifetime
                    rng.gen::<f32>() * speed + (speed * 0.5), // Speed
                ],
                pos: [x, (rng.gen::<f32>() - 0.5) * life_time * 0.8, z],
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
