use crate::{config, viewport, world::components};
use specs::{Join, WorldExt};
use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text};

pub struct Glyph {
    brush: GlyphBrush<()>,
    staging_belt: wgpu::util::StagingBelt,
}

impl Glyph {
    pub fn new(device: &wgpu::Device) -> Self {
        let font = ab_glyph::FontArc::try_from_slice(include_bytes!("inconsolata-regular.ttf")).unwrap();
        let brush = GlyphBrushBuilder::using_font(font).build(&device, config::COLOR_TEXTURE_FORMAT);
        let staging_belt = wgpu::util::StagingBelt::new(1024);

        Self { brush, staging_belt }
    }

    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        components: &specs::World,
        viewport: &viewport::Viewport,
        view: &wgpu::TextureView,
    ) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("glyph_encoder"),
        });

        let texts = components.read_storage::<components::Text>();
        for text in texts.join() {
            self.brush.queue(Section {
                screen_position: (text.position.x * viewport.dpi as f32, text.position.y * viewport.dpi as f32),
                bounds: (viewport.width as f32, viewport.height as f32),
                text: vec![Text::new(text.text.as_str())
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(25.0 * viewport.dpi as f32)],
                ..Section::default()
            });
        }

        self.brush
            .draw_queued(
                &device,
                &mut self.staging_belt,
                &mut encoder,
                view,
                viewport.width,
                viewport.height,
            )
            .expect("Draw queued");

        self.staging_belt.finish();
        queue.submit(Some(encoder.finish()));
    }
}
