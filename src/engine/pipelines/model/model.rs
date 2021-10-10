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
            let (has_textures, orm_factor) = if let Some(material) = material {
                (
                    material.textures.is_some(),
                    [1.0, material.roughness_factor, material.metallic_factor, 0.0],
                )
            } else {
                (false, [1.0, 0.5, 0.5, 0.0])
            };

            let uniform_buffer = builder.create_uniform_buffer_init(bytemuck::cast_slice(&[PrimitiveUniforms {
                has_textures: has_textures.into(),
                orm_factor,
            }]));

            primitive_buffers.push(uniform_buffer);
        }

        for (i, primitive) in mesh.primitives.iter().enumerate() {
            let material = model.get_material(primitive.material);

            let mut primitive_builder = builders::PrimitiveBuilder::new(ctx, mesh_name)
                .with_uniform_bind_group(&pipeline.primitive_uniform_bind_group_layout, &primitive_buffers[i])
                .with_vertices(bytemuck::cast_slice(primitive.vertices.as_slice()))
                .with_indices(bytemuck::cast_slice(&primitive.indices.as_slice()))
                .with_length(primitive.length);

            if let Some(material) = material {
                if let Some(textures) = &material.textures {
                    let texture_entries = &[
                        builders::RenderBundleBuilder::create_entry(
                            0,
                            wgpu::BindingResource::TextureView(&textures.base_color_texture.view),
                        ),
                        builders::RenderBundleBuilder::create_entry(1, wgpu::BindingResource::TextureView(&textures.normal_texture.view)),
                        builders::RenderBundleBuilder::create_entry(2, wgpu::BindingResource::TextureView(&textures.orm_texture.view)),
                        builders::RenderBundleBuilder::create_entry(3, wgpu::BindingResource::Sampler(&pipeline.sampler)),
                    ];
                    primitive_builder = primitive_builder.with_texture_bind_group(&pipeline.texture_bind_group_layout, texture_entries);
                }
            }

            builder = builder.with_primitive(primitive_builder);
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
