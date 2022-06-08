use crate::{engine, utils};
use egui_wgpu_backend::RenderPass;

pub fn load_image(ctx: &engine::Context, render_pass: &mut RenderPass, path: &str) -> egui::TextureId {
    let image_data = utils::read_bytes(path);
    let image = image::load_from_memory(image_data.as_slice()).unwrap();

    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();

    let texture = engine::texture::Texture::create_view(ctx, pixels.as_slice(), image.width(), image.height(), false);
    render_pass.egui_texture_from_wgpu_texture(&ctx.device, &texture.view, wgpu::FilterMode::Linear)
}
