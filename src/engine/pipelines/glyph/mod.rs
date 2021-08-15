use crate::{config, engine, world::components};
use specs::{Join, WorldExt};
use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text};

pub struct GlyphPipeline {
    brush: GlyphBrush<()>,
    staging_belt: wgpu::util::StagingBelt,
}

impl GlyphPipeline {
    pub fn new(ctx: &engine::Context) -> Self {
        let font = ab_glyph::FontArc::try_from_slice(include_bytes!("inconsolata-regular.ttf")).unwrap();
        let brush = GlyphBrushBuilder::using_font(font).build(&ctx.device, config::COLOR_TEXTURE_FORMAT);
        let staging_belt = wgpu::util::StagingBelt::new(1024);

        Self { brush, staging_belt }
    }

    pub fn render(&mut self, ctx: &engine::Context, components: &specs::World, view: &wgpu::TextureView) {
        let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("glyph_encoder"),
        });

        let texts = components.read_storage::<components::Text>();
        for text in texts.join() {
            self.brush.queue(Section {
                screen_position: (
                    text.position.x * ctx.viewport.dpi as f32,
                    text.position.y * ctx.viewport.dpi as f32,
                ),
                bounds: (ctx.viewport.width as f32, ctx.viewport.height as f32),
                text: vec![Text::new(text.text.as_str())
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(25.0 * ctx.viewport.dpi as f32)],
                ..Section::default()
            });
        }

        self.brush
            .draw_queued(
                &ctx.device,
                &mut self.staging_belt,
                &mut encoder,
                view,
                ctx.viewport.width,
                ctx.viewport.height,
            )
            .expect("Draw queued");

        self.staging_belt.finish();
        ctx.queue.submit(Some(encoder.finish()));
    }
}
