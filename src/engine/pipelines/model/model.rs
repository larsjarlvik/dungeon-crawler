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
    pub bounding_box: bounding_box::BoundingBox,
    pub display_render_bundle: wgpu::RenderBundle,
    pub display_uniform_buffer: wgpu::Buffer,
    pub display_primitive_buffers: Vec<wgpu::Buffer>,
    pub shadow_uniform_buffer: wgpu::Buffer,
    pub shadow_render_bundle: wgpu::RenderBundle,
}

impl Model {
    pub fn new(ctx: &engine::Context, pipeline: &pipelines::ModelPipeline, model: &engine::model::GltfModel, mesh_name: &str) -> Self {
        let mesh = model.get_mesh_by_name(mesh_name);

        let builder = builders::RenderBundleBuilder::new(ctx, mesh_name);
        let shadow_builder_name = format!("{}_shadows", mesh_name);
        let shadow_builder = builders::RenderBundleBuilder::new(ctx, &shadow_builder_name.as_str());

        let display_uniform_buffer = builder.create_uniform_buffer(mem::size_of::<Uniforms>() as u64);
        let shadow_uniform_buffer = builder.create_uniform_buffer(mem::size_of::<Uniforms>() as u64);

        let mut display_builder = builder
            .with_pipeline(&pipeline.display.render_pipeline)
            .with_uniform_bind_group(&pipeline.display.uniform_bind_group_layout, &display_uniform_buffer);

        let mut shadow_builder = shadow_builder
            .with_pipeline(&pipeline.shadows.render_pipeline)
            .with_uniform_bind_group(&pipeline.shadows.uniform_bind_group_layout, &shadow_uniform_buffer);

        let display_primitive_buffers: Vec<wgpu::Buffer> = mesh
            .primitives
            .iter()
            .map(|primitive| {
                let material = model.get_material(primitive.material);

                display_builder.create_uniform_buffer_init(bytemuck::cast_slice(&[if let Some(material) = material {
                    PrimitiveUniforms {
                        has_textures: material.textures.is_some().into(),
                        base_color_factor: material.base_color_factor.into(),
                        orm_factor: [1.0, material.roughness_factor, material.metallic_factor, 0.0],
                    }
                } else {
                    PrimitiveUniforms {
                        has_textures: 0,
                        base_color_factor: [1.0, 1.0, 1.0, 1.0],
                        orm_factor: [1.0, 1.0, 0.5, 0.0],
                    }
                }]))
            })
            .collect();

        for (i, primitive) in mesh.primitives.iter().enumerate() {
            let material = model.get_material(primitive.material);

            let mut primitive_builder = builders::PrimitiveBuilder::new(ctx, mesh_name)
                .with_uniform_bind_group(
                    &pipeline.display.primitive_uniform_bind_group_layout,
                    &display_primitive_buffers[i],
                )
                .with_vertices(bytemuck::cast_slice(primitive.vertices.as_slice()))
                .with_indices(bytemuck::cast_slice(&primitive.indices.as_slice()))
                .with_length(primitive.length);

            let shadow_primitive_builder = builders::PrimitiveBuilder::new(ctx, mesh_name)
                .with_vertices(bytemuck::cast_slice(primitive.vertices_position.as_slice()))
                .with_indices(bytemuck::cast_slice(&primitive.indices.as_slice()))
                .with_length(primitive.length);

            if let Some(material) = material {
                if let Some(textures) = &material.textures {
                    primitive_builder = primitive_builder.with_texture_bind_group(
                        &pipeline.display.texture_bind_group_layout,
                        &[
                            builders::RenderBundleBuilder::create_entry(0, wgpu::BindingResource::TextureView(&textures.base_color.view)),
                            builders::RenderBundleBuilder::create_entry(1, wgpu::BindingResource::TextureView(&textures.normal.view)),
                            builders::RenderBundleBuilder::create_entry(2, wgpu::BindingResource::TextureView(&textures.orm.view)),
                            builders::RenderBundleBuilder::create_entry(3, wgpu::BindingResource::Sampler(&pipeline.display.sampler)),
                        ],
                    );
                }
            }

            display_builder = display_builder.with_primitive(primitive_builder);
            shadow_builder = shadow_builder.with_primitive(shadow_primitive_builder);
        }

        Model {
            mesh_name: mesh_name.to_string(),
            bounding_box: mesh.bounding_box.clone(),
            display_render_bundle: display_builder.build(),
            display_uniform_buffer,
            display_primitive_buffers,
            shadow_render_bundle: shadow_builder.build(),
            shadow_uniform_buffer,
        }
    }
}
