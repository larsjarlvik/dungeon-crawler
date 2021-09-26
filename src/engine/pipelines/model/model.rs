use std::mem;

use crate::engine::{
    self, bounding_box,
    pipelines::{
        self, builders,
        model::{uniforms::PrimitiveUniforms, Uniforms},
    },
};

pub struct Model {
    pub mesh_name: String,
    pub uniform_buffer: wgpu::Buffer,
    pub primitive_buffers: Vec<wgpu::Buffer>,
    pub render_bundle: wgpu::RenderBundle,
    pub bounding_box: bounding_box::BoundingBox,
}

impl Model {
    pub fn new(ctx: &engine::Context, pipeline: &pipelines::ModelPipeline, model: &engine::model::GltfModel, mesh_name: &str) -> Self {
        let builder = builders::RenderBundleBuilder::new(ctx, mesh_name);
        let uniform_buffer = builder.create_uniform_buffer(mem::size_of::<Uniforms>() as u64);

        let mut builder = builder
            .with_pipeline(&pipeline.render_pipeline)
            .with_uniform_bind_group(&pipeline.uniform_bind_group_layout, &uniform_buffer);

        let mut primitive_buffers = vec![];
        let mesh = model.get_mesh_by_name(mesh_name);

        for primitive in mesh.primitives.iter() {
            let material = model.get_material(primitive.material);
            let uniform_buffer = builder.create_uniform_buffer_init(bytemuck::cast_slice(&[PrimitiveUniforms {
                orm_factor: [1.0, material.roughness_factor, material.metallic_factor, 0.0],
            }]));

            primitive_buffers.push(uniform_buffer);
        }

        for (i, primitive) in mesh.primitives.iter().enumerate() {
            let material = model.get_material(primitive.material);
            let texture_entries = &[
                builders::RenderBundleBuilder::create_entry(0, wgpu::BindingResource::TextureView(&material.base_color_texture.view)),
                builders::RenderBundleBuilder::create_entry(1, wgpu::BindingResource::TextureView(&material.normal_texture.view)),
                builders::RenderBundleBuilder::create_entry(2, wgpu::BindingResource::TextureView(&material.orm_texture.view)),
                builders::RenderBundleBuilder::create_entry(3, wgpu::BindingResource::Sampler(&pipeline.sampler)),
            ];

            builder = builder.with_primitive(
                builders::PrimitiveBuilder::new(ctx, mesh_name)
                    .with_uniform_bind_group(&pipeline.primitive_uniform_bind_group_layout, &primitive_buffers[i])
                    .with_texture_bind_group(&pipeline.texture_bind_group_layout, texture_entries)
                    .with_vertices(bytemuck::cast_slice(primitive.vertices.as_slice()))
                    .with_indices(bytemuck::cast_slice(&primitive.indices.as_slice()))
                    .with_length(primitive.length),
            );
        }

        let render_bundle = builder.build();
        Model {
            mesh_name: mesh_name.to_string(),
            uniform_buffer,
            primitive_buffers,
            render_bundle,
            bounding_box: mesh.bounding_box.clone(),
        }
    }
}
