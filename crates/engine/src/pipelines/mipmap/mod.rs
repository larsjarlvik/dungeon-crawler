use super::builders;
use crate::Context;
use std::num::NonZeroU32;

pub fn generate_mipmaps(ctx: &Context, encoder: &mut wgpu::CommandEncoder, texture: &wgpu::Texture, mip_count: u32) {
    let builder = builders::PipelineBuilder::new(&ctx, "mipmap");

    let texture_bind_group_layout = builder.create_bindgroup_layout(
        0,
        "texture_bind_group_layout",
        &[
            builder.create_texture_entry(0, wgpu::ShaderStages::FRAGMENT, true),
            builder.create_sampler_entry(1, wgpu::ShaderStages::FRAGMENT, false),
        ],
    );

    let pipeline = builder
        .with_shader("shaders/mipmap.wgsl")
        .with_primitve_topology(wgpu::PrimitiveTopology::TriangleStrip)
        .with_bind_group_layout(&texture_bind_group_layout)
        .with_color_targets(vec![wgpu::TextureFormat::Rgba8Unorm])
        .build();

    let sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("mipmap_sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    let views = (0..mip_count)
        .map(|mip| {
            texture.create_view(&wgpu::TextureViewDescriptor {
                label: Some("mipmap_view"),
                base_mip_level: mip,
                mip_level_count: NonZeroU32::new(1),
                ..Default::default()
            })
        })
        .collect::<Vec<_>>();

    for target_mip in 1..mip_count as usize {
        let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout.layout,
            entries: &[
                builders::RenderBundleBuilder::create_entry(0, wgpu::BindingResource::TextureView(&views[target_mip - 1])),
                builders::RenderBundleBuilder::create_entry(1, wgpu::BindingResource::Sampler(&sampler)),
            ],
            label: None,
        });

        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &views[target_mip],
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        rpass.set_pipeline(&pipeline.render_pipeline);
        rpass.set_bind_group(0, &bind_group, &[]);
        rpass.draw(0..4, 0..1);
    }
}
