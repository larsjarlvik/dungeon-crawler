use crate::Context;
use wgpu_glyph::{ab_glyph::Rect, GlyphCruncher, Section, Text};

pub struct GlyphPipeline {
    staging_belt: wgpu::util::StagingBelt,
}

impl GlyphPipeline {
    pub fn new() -> Self {
        let staging_belt = wgpu::util::StagingBelt::new(1024);
        Self { staging_belt }
    }

    pub fn queue(ctx: &mut Context, text: String, scale: f32, screen_position: (f32, f32), bounds: (f32, f32)) {
        ctx.glyph_brush.queue(Section {
            screen_position,
            bounds,
            text: vec![Text::new(text.as_str()).with_color([1.0, 1.0, 1.0, 1.0]).with_scale(scale)],
            layout: wgpu_glyph::Layout::default_single_line()
                .h_align(wgpu_glyph::HorizontalAlign::Left)
                .v_align(wgpu_glyph::VerticalAlign::Top),
        });
    }

    pub fn draw_queued(&mut self, ctx: &mut Context, target: &wgpu::TextureView) {
        let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("glyph_encoder"),
        });

        ctx.glyph_brush
            .draw_queued(
                &ctx.device,
                &mut self.staging_belt,
                &mut encoder,
                target,
                ctx.viewport.width,
                ctx.viewport.height,
            )
            .expect("Draw queued");

        self.staging_belt.finish();
        ctx.queue.submit(Some(encoder.finish()));
    }
}

pub fn get_bounds(ctx: &mut Context, text: &str, scale: f32) -> Rect {
    ctx.glyph_brush
        .glyph_bounds(Section {
            text: vec![Text::new(text).with_color([1.0, 1.0, 1.0, 1.0]).with_scale(scale)],
            layout: wgpu_glyph::Layout::default_single_line()
                .h_align(wgpu_glyph::HorizontalAlign::Left)
                .v_align(wgpu_glyph::VerticalAlign::Top),
            ..Default::default()
        })
        .unwrap()
}
