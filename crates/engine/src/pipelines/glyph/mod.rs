use crate::Context;
use cgmath::{Point2, Vector2, Vector4};
use wgpu_glyph::{ab_glyph::Rect, GlyphCruncher, Section, Text};

pub struct GlyphPipeline {
    staging_belt: wgpu::util::StagingBelt,
}

pub struct GlyphProps {
    pub position: Point2<f32>,
    pub bounds: Vector2<f32>,
    pub text: String,
    pub size: f32,
    pub color: Vector4<f32>,
}

impl Default for GlyphProps {
    fn default() -> Self {
        Self {
            position: Point2::new(0.0, 0.0),
            bounds: Vector2::new(f32::INFINITY, f32::INFINITY),
            text: Default::default(),
            size: Default::default(),
            color: Vector4::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}

impl GlyphPipeline {
    pub fn queue(ctx: &mut Context, props: GlyphProps) {
        ctx.glyph_brush.queue(Section {
            screen_position: props.position.into(),
            bounds: props.bounds.into(),
            text: vec![Text::new(props.text.as_str()).with_color(props.color).with_scale(props.size)],
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

impl Default for GlyphPipeline {
    fn default() -> Self {
        let staging_belt = wgpu::util::StagingBelt::new(1024);
        Self { staging_belt }
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
