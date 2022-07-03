use crate::{config, Context};
use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text};

pub struct GlyphPipeline {
    brush: GlyphBrush<()>,
    staging_belt: wgpu::util::StagingBelt,
}

impl GlyphPipeline {
    pub fn new(ctx: &Context, font_data: Vec<u8>) -> Self {
        let font = ab_glyph::FontArc::try_from_vec(font_data).expect("Failed to load font!");
        let brush = GlyphBrushBuilder::using_font(font.clone()).build(&ctx.device, config::COLOR_TEXTURE_FORMAT);
        let staging_belt = wgpu::util::StagingBelt::new(1024);
        Self { brush, staging_belt }
    }

    pub fn queue(&mut self, text: String, screen_position: (f32, f32), bounds: (f32, f32)) {
        self.brush.queue(Section {
            screen_position,
            bounds,
            text: vec![Text::new(text.as_str()).with_color([1.0, 1.0, 1.0, 1.0]).with_scale(20.0)],
            layout: wgpu_glyph::Layout::default_single_line()
                .h_align(wgpu_glyph::HorizontalAlign::Left)
                .v_align(wgpu_glyph::VerticalAlign::Top),
            ..Section::default()
        });
    }

    pub fn draw_queued(&mut self, ctx: &Context, target: &wgpu::TextureView) {
        let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("glyph_encoder"),
        });

        self.brush
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
