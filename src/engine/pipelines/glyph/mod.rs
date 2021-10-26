use crate::{
    config, engine,
    world::{components, resources},
};
use cgmath::*;
use specs::{Join, WorldExt};
use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text};

pub struct GlyphPipeline {
    brush: GlyphBrush<()>,
    staging_belt: wgpu::util::StagingBelt,
}

impl GlyphPipeline {
    pub fn new(ctx: &engine::Context) -> Self {
        let font = ab_glyph::FontArc::try_from_slice(include_bytes!("inconsolata-regular.ttf")).unwrap();
        let brush = GlyphBrushBuilder::using_font(font.clone()).build(&ctx.device, config::COLOR_TEXTURE_FORMAT);
        let staging_belt = wgpu::util::StagingBelt::new(1024);

        Self { brush, staging_belt }
    }

    pub fn render(&mut self, ctx: &engine::Context, components: &specs::World, target: &wgpu::TextureView) {
        let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("glyph_encoder"),
        });

        let texts = components.read_storage::<components::Text>();
        for text in texts.join() {
            self.brush.queue(Section {
                screen_position: (text.position.x * ctx.viewport.ui_scale, text.position.y * ctx.viewport.ui_scale),
                bounds: (ctx.viewport.width as f32, ctx.viewport.height as f32),
                text: vec![Text::new(text.text.as_str())
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(18.0 * ctx.viewport.ui_scale as f32)],
                ..Section::default()
            });
        }

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

        let camera = components.read_resource::<resources::Camera>();
        let texts3d = components.read_storage::<components::Text3d>();
        let scale_factor = 2400.0;

        for text in texts3d.join() {
            let mut mv = camera.view * Matrix4::from_translation(text.position);

            mv[0][0] = text.scale / scale_factor;
            mv[0][1] = 0.0;
            mv[0][2] = 0.0;
            mv[1][0] = 0.0;
            mv[1][1] = -text.scale / scale_factor;
            mv[1][2] = 0.0;
            mv[2][0] = 0.0;
            mv[2][1] = 0.0;
            mv[2][2] = text.scale / scale_factor;

            let c = camera.proj * mv;
            let mut arr: [f32; 16] = [0.0; 16];
            for i in 0..16 {
                arr[i] = c[i / 4][i % 4];
            }

            self.brush.queue(Section {
                screen_position: (0.0, 0.0),
                bounds: (ctx.viewport.width as f32, ctx.viewport.height as f32),
                text: vec![Text::new(text.text.as_str())
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(text.scale * 2.0)],
                layout: wgpu_glyph::Layout::default_single_line()
                    .h_align(wgpu_glyph::HorizontalAlign::Center)
                    .v_align(wgpu_glyph::VerticalAlign::Bottom),
                ..Section::default()
            });

            self.brush
                .draw_queued_with_transform(&ctx.device, &mut self.staging_belt, &mut encoder, target, arr)
                .expect("Draw queued");
        }

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
