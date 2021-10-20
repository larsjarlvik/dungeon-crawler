use crate::config;
use wgpu::util::DeviceExt;

use super::pipelines;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

impl Texture {
    pub fn create_depth_texture(ctx: &super::Context, width: u32, height: u32, label: &str) -> Self {
        let texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: config::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self { texture, view }
    }

    pub fn create_texture(ctx: &super::Context, format: wgpu::TextureFormat, width: u32, height: u32, label: &str) -> Self {
        let texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::RENDER_ATTACHMENT,
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self { texture, view }
    }

    pub fn create_mipmapped_view(ctx: &super::Context, pixels: &[u8], width: u32, height: u32) -> Self {
        let mip_level_count = (1f32 + ((width as f32).max(height as f32)).log2().floor()) as u32;
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
            size,
            mip_level_count,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_DST,
            label: None,
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = ctx.device.create_command_encoder(&Default::default());
        encoder.copy_buffer_to_texture(
            wgpu::ImageCopyBuffer {
                buffer: &ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("mipmap_buffer"),
                    contents: pixels,
                    usage: wgpu::BufferUsages::COPY_SRC,
                }),
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(4 * width),
                    rows_per_image: std::num::NonZeroU32::new(height),
                },
            },
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                aspect: wgpu::TextureAspect::All,
                origin: wgpu::Origin3d::ZERO,
            },
            size,
        );

        pipelines::mipmap::generate_mipmaps(ctx, &mut encoder, &texture, mip_level_count);
        ctx.queue.submit(std::iter::once(encoder.finish()));

        Self { texture, view }
    }

    pub fn create_sampler(
        ctx: &super::Context,
        address_mode: wgpu::AddressMode,
        filter_mode: wgpu::FilterMode,
        compare: Option<wgpu::CompareFunction>,
    ) -> wgpu::Sampler {
        ctx.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: address_mode,
            address_mode_v: address_mode,
            address_mode_w: address_mode,
            mag_filter: filter_mode,
            min_filter: filter_mode,
            mipmap_filter: filter_mode,
            compare,
            ..Default::default()
        })
    }
}
