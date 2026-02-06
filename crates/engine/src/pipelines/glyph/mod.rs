use crate::Context;
use cgmath::{Point2, Vector2, Vector4};
use fxhash::FxHashMap;
use glyphon::{
    Attrs, Buffer, Cache, Color, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache, TextArea, TextAtlas, TextBounds,
    TextRenderer, Viewport,
};
use std::collections::hash_map::Entry;

pub struct GlyphPipeline {
    font_system: FontSystem,
    swash_cache: SwashCache,
    atlas: TextAtlas,
    _cache: Cache,
    viewport: Viewport,
    text_renderer: TextRenderer,
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

pub struct GlyphBounds {
    pub width: f32,
    pub height: f32,
}

impl GlyphBounds {
    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }
}

impl GlyphPipeline {
    pub fn new(ctx: &Context, _font_data: Vec<u8>) -> Self {
        let mut font_system = FontSystem::new();
        font_system.db_mut().load_font_data(_font_data);
        let swash_cache = SwashCache::new();
        let cache = Cache::new(&ctx.device);
        let mut atlas = TextAtlas::new(&ctx.device, &ctx.queue, &cache, ctx.color_format);
        let viewport = Viewport::new(&ctx.device, &cache);
        let text_renderer = TextRenderer::new(&mut atlas, &ctx.device, wgpu::MultisampleState::default(), None);

        Self {
            font_system,
            swash_cache,
            atlas,
            _cache: cache,
            viewport,
            text_renderer,
            regions: FxHashMap::<Option<[u32; 4]>, Vec<GlyphProps>>::default(),
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
        self.viewport.update(
            &ctx.queue,
            Resolution {
                width: ctx.viewport.width,
                height: ctx.viewport.height,
            },
        );

        let mut text_areas: Vec<(Buffer, f32, f32, TextBounds)> = Vec::new();

        for (region, props_list) in self.regions.drain() {
            let bounds = if let Some(region) = region {
                let right = (region[0] + region[2]).min(ctx.viewport.width) as i32;
                let bottom = (region[1] + region[3]).min(ctx.viewport.height) as i32;
                TextBounds {
                    left: region[0] as i32,
                    top: region[1] as i32,
                    right,
                    bottom,
                }
            } else {
                TextBounds {
                    left: 0,
                    top: 0,
                    right: ctx.viewport.width as i32,
                    bottom: ctx.viewport.height as i32,
                }
            };

            for props in props_list {
                let mut buffer = Buffer::new(&mut self.font_system, Metrics::new(props.size, props.size * 1.2));
                buffer.set_size(&mut self.font_system, Some(props.bounds.x), Some(props.bounds.y));
                buffer.set_text(
                    &mut self.font_system,
                    &props.text,
                    &Attrs::new().family(Family::SansSerif).color(Color::rgba(
                        (props.color.x * 255.0) as u8,
                        (props.color.y * 255.0) as u8,
                        (props.color.z * 255.0) as u8,
                        (props.color.w * 255.0) as u8,
                    )),
                    Shaping::Advanced,
                );
                buffer.shape_until_scroll(&mut self.font_system, false);

                text_areas.push((buffer, props.position.x, props.position.y, bounds));
            }
        }

        let areas: Vec<TextArea> = text_areas
            .iter()
            .map(|(buffer, left, top, bounds)| TextArea {
                buffer,
                left: *left,
                top: *top,
                scale: 1.0,
                bounds: *bounds,
                default_color: Color::rgb(255, 255, 255),
                custom_glyphs: &[],
            })
            .collect();

        self.text_renderer
            .prepare(
                &ctx.device,
                &ctx.queue,
                &mut self.font_system,
                &mut self.atlas,
                &self.viewport,
                areas,
                &mut self.swash_cache,
            )
            .expect("Failed to prepare text");

        let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("glyph_encoder"),
        });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("glyph_render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.text_renderer
                .render(&self.atlas, &self.viewport, &mut pass)
                .expect("Failed to render text");
        }

        ctx.queue.submit(std::iter::once(encoder.finish()));
        self.atlas.trim();
    }

    pub fn get_bounds(&mut self, text: &str, scale: f32) -> GlyphBounds {
        let mut buffer = Buffer::new(&mut self.font_system, Metrics::new(scale, scale * 1.2));
        buffer.set_size(&mut self.font_system, None, None);
        buffer.set_text(
            &mut self.font_system,
            text,
            &Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );
        buffer.shape_until_scroll(&mut self.font_system, false);

        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;
        for run in buffer.layout_runs() {
            width = width.max(run.line_w);
            height += run.line_height;
        }

        GlyphBounds { width, height }
    }
}
