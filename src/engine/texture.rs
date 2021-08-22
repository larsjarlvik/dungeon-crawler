use wgpu::util::DeviceExt;
use wgpu_mipmap::{MipmapGenerator, RecommendedMipmapGenerator};

use crate::config;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

impl Texture {
    pub fn create_depth_texture(ctx: &super::Context, label: &str) -> Self {
        let size = wgpu::Extent3d {
            width: ctx.viewport.width,
            height: ctx.viewport.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: config::DEPTH_FORMAT,
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT | wgpu::TextureUsage::SAMPLED,
        };
        let texture = ctx.device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self { texture, view }
    }

    pub fn create_texture(ctx: &super::Context, format: wgpu::TextureFormat, label: &str) -> Self {
        let size = wgpu::Extent3d {
            width: ctx.viewport.width,
            height: ctx.viewport.height,
            depth_or_array_layers: 1,
        };
        let texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::RENDER_ATTACHMENT,
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self { texture, view }
    }

    pub fn create_mipmapped_view(ctx: &super::Context, pixels: &[u8], width: u32, height: u32) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let generator = RecommendedMipmapGenerator::new_with_format_hints(&ctx.device, &[wgpu::TextureFormat::Rgba8Unorm]);
        let texture_descriptor = wgpu::TextureDescriptor {
            size,
            mip_level_count: 9,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::RENDER_ATTACHMENT | wgpu::TextureUsage::COPY_DST,
            label: None,
        };

        let texture = ctx.device.create_texture(&texture_descriptor);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = ctx.device.create_command_encoder(&Default::default());
        encoder.copy_buffer_to_texture(
            wgpu::ImageCopyBuffer {
                buffer: &ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("mipmap_buffer"),
                    contents: pixels,
                    usage: wgpu::BufferUsage::COPY_SRC,
                }),
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(4 * size.width),
                    rows_per_image: std::num::NonZeroU32::new(size.height),
                },
            },
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            size,
        );

        generator
            .generate(&ctx.device, &mut encoder, &texture, &texture_descriptor)
            .unwrap();
        ctx.queue.submit(std::iter::once(encoder.finish()));

        Self { texture, view }
    }

    pub fn create_sampler(ctx: &super::Context) -> wgpu::Sampler {
        ctx.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        })
    }
}
