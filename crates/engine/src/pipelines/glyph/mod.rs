use crate::Context;
use cgmath::{Point2, Vector2, Vector4};
use fxhash::FxHashMap;
use std::collections::hash_map::Entry;
use wgpu_glyph::{
    ab_glyph::{FontArc, Rect},
    GlyphBrushBuilder, GlyphCruncher, Region, Section, Text,
};

pub struct GlyphPipeline {
    staging_belt: wgpu::util::StagingBelt,
    brush: wgpu_glyph::GlyphBrush<()>,
    regions: FxHashMap<Option<[u32; 4]>, Vec<GlyphProps>>,
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
    pub fn new(ctx: &Context, font_data: Vec<u8>) -> Self {
        let staging_belt = wgpu::util::StagingBelt::new(1024);
        let font = FontArc::try_from_vec(font_data).expect("Failed to load font!");
        let glyph_brush = GlyphBrushBuilder::using_font(font).build(&ctx.device, ctx.color_format);

        Self {
            staging_belt,
            regions: FxHashMap::<Option<[u32; 4]>, Vec<GlyphProps>>::default(),
            brush: glyph_brush,
        }
    }

    pub fn queue(&mut self, region: Option<[u32; 4]>, props: GlyphProps) {
        let region = match self.regions.entry(region) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(vec![]),
        };

        region.push(props);
    }

    pub fn draw_queued(&mut self, ctx: &mut Context, target: &wgpu::TextureView) {
        let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("glyph_encoder"),
        });

        for (region, props) in self.regions.drain() {
            for props in props {
                self.brush.queue(Section {
                    screen_position: props.position.into(),
                    bounds: props.bounds.into(),
                    text: vec![Text::new(props.text.as_str()).with_color(props.color).with_scale(props.size)],
                    layout: wgpu_glyph::Layout::default_single_line()
                        .h_align(wgpu_glyph::HorizontalAlign::Left)
                        .v_align(wgpu_glyph::VerticalAlign::Top),
                });
            }

            let region = if let Some(region) = region {
                Region {
                    x: region[0],
                    y: region[1],
                    width: region[2].min(ctx.viewport.width - region[0]),
                    height: region[3].min(ctx.viewport.height - region[1]),
                }
            } else {
                Region {
                    x: 0,
                    y: 0,
                    width: ctx.viewport.width,
                    height: ctx.viewport.height,
                }
            };

            let transform = wgpu_glyph::orthographic_projection(ctx.viewport.width, ctx.viewport.height);
            self.brush
                .draw_queued_with_transform_and_scissoring(&ctx.device, &mut self.staging_belt, &mut encoder, target, transform, region)
                .expect("Draw queued");
        }

        self.staging_belt.finish();
        ctx.queue.submit(Some(encoder.finish()));
    }

    pub fn get_bounds(&mut self, text: &str, scale: f32) -> Rect {
        self.brush
            .glyph_bounds(Section {
                text: vec![Text::new(text).with_color([1.0, 1.0, 1.0, 1.0]).with_scale(scale)],
                layout: wgpu_glyph::Layout::default_single_line()
                    .h_align(wgpu_glyph::HorizontalAlign::Left)
                    .v_align(wgpu_glyph::VerticalAlign::Top),
                ..Default::default()
            })
            .unwrap()
    }
}
