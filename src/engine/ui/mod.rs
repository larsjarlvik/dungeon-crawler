use super::texture::Texture;
use raui::prelude::*;
use std::collections::HashMap;
mod renderer;

pub struct Ui {
    application: Application,
    interactions: DefaultInteractionsEngine,
    resources: HashMap<String, Texture>,
}

impl Ui {
    pub fn new(ctx: &super::Context) -> Self {
        let mut application = Application::new();

        let tree = make_widget!(content_box).listed_slot(
            make_widget!(content_box)
                .with_props(ContentBoxItemLayout {
                    margin: 64.0.into(),
                    ..Default::default()
                })
                .listed_slot(make_widget!(image_box).with_props(ImageBoxProps::colored(Color {
                    r: 0.25,
                    g: 0.25,
                    b: 0.25,
                    a: 1.0,
                })))
                .listed_slot(
                    make_widget!(flex_box)
                        .with_props(FlexBoxProps {
                            direction: FlexBoxDirection::VerticalTopToBottom,
                            separation: 10.0,
                            ..Default::default()
                        })
                        .listed_slot(
                            make_widget!(text_box)
                                .with_props(TextBoxProps {
                                    text: "RAUI text box example".to_owned(),
                                    font: TextBoxFont {
                                        name: "default".to_owned(),
                                        size: 18.0,
                                    },
                                    color: Color {
                                        r: 1.0,
                                        g: 1.0,
                                        b: 1.0,
                                        a: 1.0,
                                    },
                                    vertical_align: TextBoxVerticalAlign::Top,
                                    horizontal_align: TextBoxHorizontalAlign::Left,
                                    height: TextBoxSizeValue::Exact(18.0),
                                    ..Default::default()
                                })
                                .with_props(FlexBoxItemLayout {
                                    grow: 0.0,
                                    ..Default::default()
                                }),
                        )
                        .listed_slot(
                            make_widget!(text_box)
                                .with_props(TextBoxProps {
                                    text: "RAUI text box example".to_owned(),
                                    font: TextBoxFont {
                                        name: "default".to_owned(),
                                        size: 18.0,
                                    },
                                    color: Color {
                                        r: 1.0,
                                        g: 1.0,
                                        b: 1.0,
                                        a: 1.0,
                                    },
                                    vertical_align: TextBoxVerticalAlign::Top,
                                    horizontal_align: TextBoxHorizontalAlign::Center,
                                    height: TextBoxSizeValue::Exact(18.0),
                                    ..Default::default()
                                })
                                .with_props(FlexBoxItemLayout {
                                    grow: 0.0,
                                    ..Default::default()
                                }),
                        )
                        .listed_slot(
                            make_widget!(text_box)
                                .with_props(TextBoxProps {
                                    text: "RAUI text box example".to_owned(),
                                    font: TextBoxFont {
                                        name: "default".to_owned(),
                                        size: 18.0,
                                    },
                                    color: Color {
                                        r: 1.0,
                                        g: 1.0,
                                        b: 1.0,
                                        a: 1.0,
                                    },
                                    vertical_align: TextBoxVerticalAlign::Top,
                                    horizontal_align: TextBoxHorizontalAlign::Right,
                                    height: TextBoxSizeValue::Exact(18.0),
                                    ..Default::default()
                                })
                                .with_props(FlexBoxItemLayout {
                                    grow: 0.0,
                                    ..Default::default()
                                }),
                        )
                        .listed_slot(
                            make_widget!(image_box)
                                .with_props(FlexBoxProps {
                                    direction: FlexBoxDirection::VerticalTopToBottom,
                                    separation: 10.0,
                                    ..Default::default()
                                })
                                .with_props(ImageBoxProps {
                                    material: ImageBoxMaterial::Image(ImageBoxImage {
                                        id: "test".to_owned(),
                                        ..Default::default()
                                    }),
                                    content_keep_aspect_ratio: Some(ImageBoxAspectRatio {
                                        horizontal_alignment: 1.0,
                                        vertical_alignment: 1.0,
                                        outside: false,
                                    }),
                                    ..Default::default()
                                }),
                        ),
                ),
        );

        application.setup(setup);
        let interactions = DefaultInteractionsEngine::new();

        let app = WidgetNode::Component(tree);
        application.apply(app);

        let mut resources = HashMap::new();

        let texture = Texture::from_resource(ctx, "textures/test.png", false);
        resources.insert("test".to_owned(), texture);

        Self {
            application,
            interactions,
            resources,
        }
    }

    pub fn render(
        &mut self,
        ctx: &super::Context,
        ui_pipeline: &super::pipelines::UiPipeline,
        glyph_pipeline: &mut super::pipelines::GlyphPipeline,
        target: &wgpu::TextureView,
    ) {
        self.application.process();

        let mapping = CoordsMapping::new(Rect {
            left: 0.0,
            right: ctx.viewport.width as f32 / ctx.viewport.ui_scale,
            top: 0.0,
            bottom: ctx.viewport.height as f32 / ctx.viewport.ui_scale,
        });

        self.interactions.interact(Interaction::PointerMove(Vec2 { x: 200.0, y: 100.0 }));
        self.interactions
            .interact(Interaction::PointerDown(PointerButton::Trigger, Vec2 { x: 200.0, y: 100.0 }));

        self.application.layout(&mapping, &mut DefaultLayoutEngine).unwrap();
        self.application.interact(&mut self.interactions).unwrap();

        let mut renderer = renderer::WgpuRenderer::new(&ctx, &ui_pipeline, glyph_pipeline, &self.resources, &target);
        self.application.render(&mapping, &mut renderer).unwrap();
    }
}
