use crate::{
    bounding_box, model,
    pipelines::{self, builders},
    Context,
};
use std::mem;

use super::{
    uniforms::{DefaultUniforms, EnvironmentUniforms, PrimitiveUniforms},
    PbrUniforms,
};

pub struct DefaultRenderer {
    pub render_bundle: wgpu::RenderBundle,
    pub uniform_buffer: wgpu::Buffer,
}

pub struct PbrRenderer {
    pub render_bundle: wgpu::RenderBundle,
    pub uniform_buffer: wgpu::Buffer,
    pub environment_uniform_buffer: wgpu::Buffer,
    pub primitive_buffers: Vec<wgpu::Buffer>,
}

pub enum RenderType {
    Default(DefaultRenderer),
    PBR(PbrRenderer),
}

pub enum Method {
    Default,
    PBR,
}

pub struct Model {
    pub mesh_name: String,
    pub bounding_box: bounding_box::BoundingBox,
    pub render_type: RenderType,
    pub shadow_uniform_buffer: wgpu::Buffer,
    pub shadow_render_bundle: wgpu::RenderBundle,
}

impl Model {
    pub fn new(ctx: &Context, pipeline: &pipelines::ModelPipeline, model: &model::GltfModel, method: Method, mesh_name: &str) -> Self {
        let mesh = model.get_mesh_by_name(mesh_name);

        let builder = builders::RenderBundleBuilder::new(ctx, mesh_name);
        let shadow_builder_name = format!("{}_shadows", mesh_name);
        let shadow_builder = builders::RenderBundleBuilder::new(ctx, shadow_builder_name.as_str());
        let shadow_uniform_buffer = builder.create_uniform_buffer(mem::size_of::<PbrUniforms>() as u64);

        let render_type = match method {
            Method::Default => {
                let uniform_buffer = builder.create_uniform_buffer(mem::size_of::<DefaultUniforms>() as u64);
                let mut builder = builder
                    .with_pipeline(&pipeline.default.render_pipeline)
                    .with_uniform_bind_group(&pipeline.default.uniform_bind_group_layout, &uniform_buffer);

                for primitive in mesh.primitives.iter() {
                    builder = builder.with_primitive(
                        builders::PrimitiveBuilder::new(ctx, mesh_name)
                            .with_vertices(bytemuck::cast_slice(primitive.vertices.as_slice()))
                            .with_indices(bytemuck::cast_slice(primitive.indices.as_slice()))
                            .with_length(primitive.length),
                    );
                }

                RenderType::Default(DefaultRenderer {
                    render_bundle: builder.build(),
                    uniform_buffer,
                })
            }
            Method::PBR => {
                let uniform_buffer = builder.create_uniform_buffer(mem::size_of::<PbrUniforms>() as u64);
                let environment_uniform_buffer = builder.create_uniform_buffer(mem::size_of::<EnvironmentUniforms>() as u64);

                let mut builder = builder
                    .with_pipeline(&pipeline.pbr.render_pipeline)
                    .with_uniform_bind_group(&pipeline.pbr.uniform_bind_group_layout, &uniform_buffer)
                    .with_uniform_bind_group(&pipeline.pbr.environment_uniform_bind_group_layout, &environment_uniform_buffer);

                let primitive_buffers: Vec<wgpu::Buffer> = mesh
                    .primitives
                    .iter()
                    .map(|primitive| {
                        let material = model
                            .get_material(primitive.material)
                            .expect("Trying to render PBR primitive without materail!");

                        builder.create_uniform_buffer_init(bytemuck::cast_slice(&[PrimitiveUniforms {
                            base_color_factor: material.base_color_factor.into(),
                            orm_factor: [1.0, material.roughness_factor, material.metallic_factor, 0.0],
                        }]))
                    })
                    .collect();

                for (i, primitive) in mesh.primitives.iter().enumerate() {
                    let material = model
                        .get_material(primitive.material)
                        .expect("Trying to render PBR primitive without material!");

                    let t = material
                        .textures
                        .as_ref()
                        .expect("Trying to render PBR primitive without textures!");

                    builder = builder.with_primitive(
                        builders::PrimitiveBuilder::new(ctx, mesh_name)
                            .with_uniform_bind_group(&pipeline.pbr.primitive_uniform_bind_group_layout, &primitive_buffers[i])
                            .with_vertices(bytemuck::cast_slice(primitive.vertices.as_slice()))
                            .with_indices(bytemuck::cast_slice(primitive.indices.as_slice()))
                            .with_texture_bind_group(
                                &pipeline.pbr.texture_bind_group_layout,
                                &[
                                    builders::RenderBundleBuilder::create_entry(0, wgpu::BindingResource::TextureView(&t.base_color.view)),
                                    builders::RenderBundleBuilder::create_entry(1, wgpu::BindingResource::TextureView(&t.normal.view)),
                                    builders::RenderBundleBuilder::create_entry(2, wgpu::BindingResource::TextureView(&t.orm.view)),
                                    builders::RenderBundleBuilder::create_entry(3, wgpu::BindingResource::Sampler(&pipeline.pbr.sampler)),
                                ],
                            )
                            .with_length(primitive.length),
                    );
                }

                RenderType::PBR(PbrRenderer {
                    render_bundle: builder.build(),
                    uniform_buffer,
                    environment_uniform_buffer,
                    primitive_buffers,
                })
            }
        };

        let mut shadow_builder = shadow_builder
            .with_pipeline(&pipeline.shadows.render_pipeline)
            .with_uniform_bind_group(&pipeline.shadows.uniform_bind_group_layout, &shadow_uniform_buffer);

        for primitive in mesh.primitives.iter() {
            let shadow_primitive_builder = builders::PrimitiveBuilder::new(ctx, mesh_name)
                .with_vertices(bytemuck::cast_slice(primitive.vertices_position.as_slice()))
                .with_indices(bytemuck::cast_slice(primitive.indices.as_slice()))
                .with_length(primitive.length);

            shadow_builder = shadow_builder.with_primitive(shadow_primitive_builder);
        }

        Model {
            mesh_name: mesh_name.to_string(),
            bounding_box: mesh.bounding_box.clone(),
            shadow_render_bundle: shadow_builder.build(),
            shadow_uniform_buffer,
            render_type,
        }
    }
}
