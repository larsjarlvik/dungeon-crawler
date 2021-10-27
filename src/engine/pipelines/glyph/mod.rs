use crate::{
    config, engine,
    utils::Interpolate,
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

        self.draw_3d(ctx, components, &mut encoder, target);
        self.draw_2d(ctx, components, &mut encoder, target);

        self.staging_belt.finish();
        ctx.queue.submit(Some(encoder.finish()));
    }

    fn draw_2d(
        &mut self,
        ctx: &engine::Context,
        components: &specs::World,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
    ) {
        let texts = components.read_storage::<components::Text>();
        let transforms = components.read_storage::<components::Transform2d>();
        let time = components.read_resource::<resources::Time>();

        for (text, transform) in (&texts, &transforms).join() {
            let translation = transform.translation.get(time.last_frame);
            let scale = transform.scale.get(time.last_frame);

            self.brush.queue(Section {
                screen_position: (translation.x * ctx.viewport.ui_scale, translation.y * ctx.viewport.ui_scale),
                bounds: (ctx.viewport.width as f32, ctx.viewport.height as f32),
                text: vec![Text::new(text.text.as_str())
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(scale.y * ctx.viewport.ui_scale as f32)],
                ..Section::default()
            });
        }

        self.brush
            .draw_queued(
                &ctx.device,
                &mut self.staging_belt,
                encoder,
                target,
                ctx.viewport.width,
                ctx.viewport.height,
            )
            .expect("Draw queued");
    }

    fn draw_3d(
        &mut self,
        ctx: &engine::Context,
        components: &specs::World,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
    ) {
        let camera = components.read_resource::<resources::Camera>();
        let transforms = components.read_storage::<components::Transform>();
        let texts = components.read_storage::<components::Text>();
        let time = components.read_resource::<resources::Time>();
        let (width, height) = ctx.viewport.get_render_size();
        let (width, height) = (width as f32 / 2.0, height as f32 / 2.0);

        for (text, transform) in (&texts, &transforms).join() {
            let scale = transform.scale.get(time.last_frame);
            let screen_position = camera.view_proj * transform.to_matrix(time.last_frame) * vec4(0.0, 0.0, 0.0, 1.0);

            let screen_position_x = width * (1.0 + screen_position.x / screen_position.w);
            let screen_position_y = height * (1.0 - screen_position.y / screen_position.w);

            self.brush.queue(Section {
                screen_position: (screen_position_x, screen_position_y),
                bounds: (ctx.viewport.width as f32, ctx.viewport.height as f32),
                text: vec![Text::new(text.text.as_str()).with_color([1.0, 1.0, 1.0, 1.0]).with_scale(scale.y)],
                layout: wgpu_glyph::Layout::default_single_line()
                    .h_align(wgpu_glyph::HorizontalAlign::Center)
                    .v_align(wgpu_glyph::VerticalAlign::Bottom),
                ..Section::default()
            });
        }

        self.brush
            .draw_queued(
                &ctx.device,
                &mut self.staging_belt,
                encoder,
                target,
                ctx.viewport.width,
                ctx.viewport.height,
            )
            .expect("Draw queued");
    }
}
