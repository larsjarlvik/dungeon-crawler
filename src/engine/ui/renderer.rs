use crate::engine::{self, pipelines::ui::Vertex};
use raui::prelude::*;
use raui_core::{
    layout::{CoordsMapping, Layout},
    renderer::Renderer,
    widget::unit::WidgetUnit,
};

#[derive(Debug, Clone)]
pub enum Error {
    WidgetHasNoLayout(WidgetId),
    UnsupportedImageMaterial(ImageBoxMaterial),
}

pub struct WgpuRenderer<'a> {
    ctx: &'a engine::Context,
    ui_pipeline: &'a engine::pipelines::UiPipeline,
    glyph_pipeline: &'a mut engine::pipelines::GlyphPipeline,
    target: &'a wgpu::TextureView,
}

impl<'a> WgpuRenderer<'a> {
    pub fn new(
        ctx: &'a engine::Context,
        ui_pipeline: &'a engine::pipelines::UiPipeline,
        glyph_pipeline: &'a mut engine::pipelines::GlyphPipeline,
        target: &'a wgpu::TextureView,
    ) -> Self {
        Self {
            ctx,
            ui_pipeline,
            glyph_pipeline,
            target,
        }
    }

    fn transform_rect(rect: Rect, transform: &Transform) -> (Vec2, Scalar, Vec2, Rect) {
        let offset = Vec2 {
            x: lerp(rect.left, rect.right, transform.pivot.x),
            y: lerp(rect.top, rect.bottom, transform.pivot.y),
        };
        let rect = Rect {
            left: rect.left - offset.x,
            right: rect.right - offset.x,
            top: rect.top - offset.y,
            bottom: rect.bottom - offset.y,
        };
        (offset, transform.rotation, transform.scale, rect)
    }

    fn render_node(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        unit: &WidgetUnit,
        mapping: &CoordsMapping,
        layout: &Layout,
    ) -> Result<(), Error> {
        match unit {
            WidgetUnit::None | WidgetUnit::PortalBox(_) => Ok(()),
            WidgetUnit::AreaBox(unit) => self.render_node(encoder, &unit.slot, mapping, layout),
            WidgetUnit::ContentBox(unit) => {
                let mut items = unit.items.iter().map(|item| (item.layout.depth, item)).collect::<Vec<_>>();
                items.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
                for (_, item) in items {
                    self.render_node(encoder, &item.slot, mapping, layout)?;
                }
                Ok(())
            }
            WidgetUnit::FlexBox(unit) => {
                for item in &unit.items {
                    self.render_node(encoder, &item.slot, mapping, layout)?;
                }
                Ok(())
            }
            WidgetUnit::GridBox(unit) => {
                for item in &unit.items {
                    self.render_node(encoder, &item.slot, mapping, layout)?;
                }
                Ok(())
            }
            WidgetUnit::SizeBox(unit) => self.render_node(encoder, &unit.slot, mapping, layout),
            WidgetUnit::ImageBox(unit) => match &unit.material {
                ImageBoxMaterial::Color(image) => {
                    if let Some(item) = layout.items.get(&unit.id) {
                        let scale = mapping.scale();
                        let color = [image.color.r, image.color.g, image.color.b, image.color.a];
                        let rect = mapping.virtual_to_real_rect(item.ui_space, false);

                        // TODO
                        // let (offset, rotation, scaling, rect) = Self::transform_rect(rect, &unit.transform);

                        match &image.scaling {
                            ImageBoxImageScaling::Stretch => {
                                let vertices = vec![
                                    Vertex::new([rect.left, rect.top], [0.0, 0.0], color),
                                    Vertex::new([rect.right, rect.top], [0.0, 0.0], color),
                                    Vertex::new([rect.right, rect.bottom], [0.0, 0.0], color),
                                    Vertex::new([rect.left, rect.bottom], [0.0, 0.0], color),
                                ];
                                let indices = vec![0, 1, 2, 2, 3, 0];
                                self.ui_pipeline.render(&self.ctx, vertices, indices, &self.target);
                            }
                            ImageBoxImageScaling::Frame(frame) => {
                                let vl = frame.destination.left * scale.x;
                                let vr = frame.destination.right * scale.x;
                                let vt = frame.destination.top * scale.y;
                                let vb = frame.destination.bottom * scale.y;
                                let vertices = vec![
                                    Vertex::new([rect.left, rect.top], [0.0, 0.0], color),
                                    Vertex::new([rect.left + vl, rect.top], [0.0, 0.0], color),
                                    Vertex::new([rect.right - vr, rect.top], [0.0, 0.0], color),
                                    Vertex::new([rect.right, rect.top], [0.0, 0.0], color),
                                    Vertex::new([rect.left, rect.top + vt], [0.0, 0.0], color),
                                    Vertex::new([rect.left + vl, rect.top + vt], [0.0, 0.0], color),
                                    Vertex::new([rect.right - vr, rect.top + vt], [0.0, 0.0], color),
                                    Vertex::new([rect.right, rect.top + vt], [0.0, 0.0], color),
                                    Vertex::new([rect.left, rect.bottom - vb], [0.0, 0.0], color),
                                    Vertex::new([rect.left + vl, rect.bottom - vb], [0.0, 0.0], color),
                                    Vertex::new([rect.right - vr, rect.bottom - vb], [0.0, 0.0], color),
                                    Vertex::new([rect.right, rect.bottom - vb], [0.0, 0.0], color),
                                    Vertex::new([rect.left, rect.bottom], [0.0, 0.0], color),
                                    Vertex::new([rect.left + vl, rect.bottom], [0.0, 0.0], color),
                                    Vertex::new([rect.right - vr, rect.bottom], [0.0, 0.0], color),
                                    Vertex::new([rect.right, rect.bottom], [0.0, 0.0], color),
                                ];
                                if frame.frame_only {
                                    let indices = vec![
                                        0, 1, 5, 5, 4, 0, 1, 2, 6, 6, 5, 1, 2, 3, 7, 7, 6, 2, 4, 5, 9, 9, 8, 4, 6, 7, 11, 11, 10, 6, 8, 9,
                                        13, 13, 12, 8, 9, 10, 14, 14, 13, 9, 10, 11, 15, 15, 14, 10,
                                    ];
                                    self.ui_pipeline.render(&self.ctx, vertices, indices, &self.target);
                                } else {
                                    let indices = vec![
                                        0, 1, 5, 5, 4, 0, 1, 2, 6, 6, 5, 1, 2, 3, 7, 7, 6, 2, 4, 5, 9, 9, 8, 4, 5, 6, 10, 10, 9, 5, 6, 7,
                                        11, 11, 10, 6, 8, 9, 13, 13, 12, 8, 9, 10, 14, 14, 13, 9, 10, 11, 15, 15, 14, 10,
                                    ];
                                    self.ui_pipeline.render(&self.ctx, vertices, indices, &self.target);
                                }
                            }
                        }

                        Ok(())
                    } else {
                        Err(Error::WidgetHasNoLayout(unit.id.to_owned()))
                    }
                }
                ImageBoxMaterial::Image(_image) => {
                    // if let Some(item) = layout.items.get(&unit.id) {
                    //     if let Some(resource) = self.resources.images.get(&image.id) {
                    //         let scale = mapping.scale();
                    //         let color = [image.tint.r, image.tint.g, image.tint.b, image.tint.a];
                    //         let source = image.source_rect.unwrap_or(Rect {
                    //             left: 0.0,
                    //             right: 1.0,
                    //             top: 0.0,
                    //             bottom: 1.0,
                    //         });
                    //         let sfx = source.left;
                    //         let stx = source.right;
                    //         let sfy = source.top;
                    //         let sty = source.bottom;
                    //         let rect = if let Some(aspect) = unit.content_keep_aspect_ratio {
                    //             let ox = item.ui_space.left;
                    //             let oy = item.ui_space.top;
                    //             let rw = resource.width() as Scalar;
                    //             let rh = resource.height() as Scalar;
                    //             let iw = item.ui_space.width();
                    //             let ih = item.ui_space.height();
                    //             let ra = rw / rh;
                    //             let ia = iw / ih;
                    //             let scale = if (ra >= ia) != aspect.outside { iw / rw } else { ih / rh };
                    //             let w = rw * scale;
                    //             let h = rh * scale;
                    //             let ow = lerp(0.0, iw - w, aspect.horizontal_alignment);
                    //             let oh = lerp(0.0, ih - h, aspect.vertical_alignment);
                    //             Rect {
                    //                 left: ox + ow,
                    //                 right: ox + ow + w,
                    //                 top: oy + oh,
                    //                 bottom: oy + oh + h,
                    //             }
                    //         } else {
                    //             item.ui_space
                    //         };
                    //         let rect = mapping.virtual_to_real_rect(rect, false);
                    //         let (offset, rotation, scaling, rect) = Self::transform_rect(rect, &unit.transform);

                    //         match &image.scaling {
                    //             ImageBoxImageScaling::Stretch => {
                    //                 let vertices = &[
                    //                     Vertex::new([rect.left, rect.top], [lerp(sfx, stx, 0.0), lerp(sfy, sty, 0.0)], color),
                    //                     Vertex::new([rect.right, rect.top], [lerp(sfx, stx, 1.0), lerp(sfy, sty, 0.0)], color),
                    //                     Vertex::new([rect.right, rect.bottom], [lerp(sfx, stx, 1.0), lerp(sfy, sty, 1.0)], color),
                    //                     Vertex::new([rect.left, rect.bottom], [lerp(sfx, stx, 0.0), lerp(sfy, sty, 1.0)], color),
                    //                 ];
                    //                 let indices = &[0, 1, 2, 2, 3, 0];
                    //                 self.engine
                    //                     .ui_pipeline
                    //                     .render(&self.engine.ctx, vertices.to_vec(), indices.to_vec());
                    //             }
                    //             ImageBoxImageScaling::Frame(frame) => {
                    //                 let fl = frame.source.left / resource.width() as Scalar;
                    //                 let fr = 1.0 - (frame.source.right / resource.width() as Scalar);
                    //                 let ft = frame.source.top / resource.height() as Scalar;
                    //                 let fb = 1.0 - (frame.source.bottom / resource.height() as Scalar);
                    //                 let vl = frame.destination.left * scale.x;
                    //                 let vr = frame.destination.right * scale.x;
                    //                 let vt = frame.destination.top * scale.y;
                    //                 let vb = frame.destination.bottom * scale.y;
                    //                 let vertices = &[
                    //                     Vertex::new([rect.left, rect.top], [lerp(sfx, stx, 0.0), lerp(sfy, sty, 0.0)], color),
                    //                     Vertex::new([rect.left + vl, rect.top], [lerp(sfx, stx, fl), lerp(sfy, sty, 0.0)], color),
                    //                     Vertex::new([rect.right - vr, rect.top], [lerp(sfx, stx, fr), lerp(sfy, sty, 0.0)], color),
                    //                     Vertex::new([rect.right, rect.top], [lerp(sfx, stx, 1.0), lerp(sfy, sty, 0.0)], color),
                    //                     Vertex::new([rect.left, rect.top + vt], [lerp(sfx, stx, 0.0), lerp(sfy, sty, ft)], color),
                    //                     Vertex::new([rect.left + vl, rect.top + vt], [lerp(sfx, stx, fl), lerp(sfy, sty, ft)], color),
                    //                     Vertex::new(
                    //                         [rect.right - vr, rect.top + vt],
                    //                         [lerp(sfx, stx, fr), lerp(sfy, sty, ft)],
                    //                         color,
                    //                     ),
                    //                     Vertex::new([rect.right, rect.top + vt], [lerp(sfx, stx, 1.0), lerp(sfy, sty, ft)], color),
                    //                     Vertex::new([rect.left, rect.bottom - vb], [lerp(sfx, stx, 0.0), lerp(sfy, sty, fb)], color),
                    //                     Vertex::new(
                    //                         [rect.left + vl, rect.bottom - vb],
                    //                         [lerp(sfx, stx, fl), lerp(sfy, sty, fb)],
                    //                         color,
                    //                     ),
                    //                     Vertex::new(
                    //                         [rect.right - vr, rect.bottom - vb],
                    //                         [lerp(sfx, stx, fr), lerp(sfy, sty, fb)],
                    //                         color,
                    //                     ),
                    //                     Vertex::new([rect.right, rect.bottom - vb], [lerp(sfx, stx, 1.0), lerp(sfy, sty, fb)], color),
                    //                     Vertex::new([rect.left, rect.bottom], [lerp(sfx, stx, 0.0), lerp(sfy, sty, 1.0)], color),
                    //                     Vertex::new([rect.left + vl, rect.bottom], [lerp(sfx, stx, fl), lerp(sfy, sty, 1.0)], color),
                    //                     Vertex::new([rect.right - vr, rect.bottom], [lerp(sfx, stx, fr), lerp(sfy, sty, 1.0)], color),
                    //                     Vertex::new([rect.right, rect.bottom], [lerp(sfx, stx, 1.0), lerp(sfy, sty, 1.0)], color),
                    //                 ];
                    //                 if frame.frame_only {
                    //                     let indices = &[
                    //                         0, 1, 5, 5, 4, 0, 1, 2, 6, 6, 5, 1, 2, 3, 7, 7, 6, 2, 4, 5, 9, 9, 8, 4, 6, 7, 11, 11, 10, 6, 8,
                    //                         9, 13, 13, 12, 8, 9, 10, 14, 14, 13, 9, 10, 11, 15, 15, 14, 10,
                    //                     ];
                    //                     self.engine
                    //                         .ui_pipeline
                    //                         .render(&self.engine.ctx, vertices.to_vec(), indices.to_vec());
                    //                 } else {
                    //                     let indices = &[
                    //                         0, 1, 5, 5, 4, 0, 1, 2, 6, 6, 5, 1, 2, 3, 7, 7, 6, 2, 4, 5, 9, 9, 8, 4, 5, 6, 10, 10, 9, 5, 6,
                    //                         7, 11, 11, 10, 6, 8, 9, 13, 13, 12, 8, 9, 10, 14, 14, 13, 9, 10, 11, 15, 15, 14, 10,
                    //                     ];
                    //                     self.engine
                    //                         .ui_pipeline
                    //                         .render(&self.engine.ctx, vertices.to_vec(), indices.to_vec());
                    //                 }
                    //             }
                    //         }
                    //         if let Ok(mesh) = builder.build(self.context) {
                    //             let params = graphics::DrawParam::default()
                    //                 .rotation(rotation)
                    //                 .scale([scaling.x, scaling.y])
                    //                 .dest([offset.x, offset.y]);
                    //             if graphics::draw(self.context, &mesh, params).is_ok() {
                    //                 Ok(())
                    //             } else {
                    //                 Err(Error::CouldNotDrawImage(unit.id.to_owned()))
                    //             }
                    //         } else {
                    //             Err(Error::CouldNotBuildImageMesh(unit.id.to_owned()))
                    //         }
                    //     } else {
                    //         Err(Error::ImageResourceNotFound(unit.id.to_owned(), image.id.to_owned()))
                    //     }
                    // } else {
                    //     Err(Error::WidgetHasNoLayout(unit.id.to_owned()))
                    // }
                    Ok(())
                }
                _ => Err(Error::UnsupportedImageMaterial(unit.material.clone())),
            },
            WidgetUnit::TextBox(unit) => {
                if let Some(item) = layout.items.get(&unit.id) {
                    let rect = mapping.virtual_to_real_rect(item.ui_space, false);
                    let (offset, _, _, rect) = Self::transform_rect(rect, &unit.transform);
                    let color = [unit.color.r, unit.color.g, unit.color.b, unit.color.a];

                    let h_align = match unit.horizontal_align {
                        TextBoxHorizontalAlign::Left => wgpu_glyph::HorizontalAlign::Left,
                        TextBoxHorizontalAlign::Center => wgpu_glyph::HorizontalAlign::Center,
                        TextBoxHorizontalAlign::Right => wgpu_glyph::HorizontalAlign::Right,
                    };
                    let v_align = match unit.vertical_align {
                        TextBoxVerticalAlign::Bottom => wgpu_glyph::VerticalAlign::Bottom,
                        TextBoxVerticalAlign::Middle => wgpu_glyph::VerticalAlign::Center,
                        TextBoxVerticalAlign::Top => wgpu_glyph::VerticalAlign::Top,
                    };

                    self.glyph_pipeline.queue(wgpu_glyph::Section {
                        screen_position: (offset.x, offset.y),
                        bounds: (rect.width() as f32, rect.height() as f32),
                        text: vec![wgpu_glyph::Text::new(unit.text.as_str())
                            .with_color(color)
                            .with_scale(unit.font.size)],
                        layout: wgpu_glyph::Layout::default_wrap().h_align(h_align).v_align(v_align),
                    });

                    Ok(())
                } else {
                    Err(Error::WidgetHasNoLayout(unit.id.to_owned()))
                }
            }
        }
    }
}

impl<'a> Renderer<(), Error> for WgpuRenderer<'a> {
    fn render(&mut self, tree: &WidgetUnit, mapping: &CoordsMapping, layout: &Layout) -> Result<(), Error> {
        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("ui_encoder") });

        let result = self.render_node(&mut encoder, tree, mapping, layout);

        self.glyph_pipeline.render_queued(&self.ctx, &mut encoder, &self.target);
        self.ctx.queue.submit(Some(encoder.finish()));
        result
    }
}
