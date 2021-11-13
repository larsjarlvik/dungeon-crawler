use raui::prelude::*;
mod renderer;

pub struct Ui {
    application: Application,
    interactions: DefaultInteractionsEngine,
}

impl Ui {
    pub fn new() -> Self {
        let mut application = Application::new();

        let tree = make_widget!(content_box).listed_slot(
            make_widget!(content_box)
                .with_props(ContentBoxItemLayout {
                    margin: 64.0.into(),
                    ..Default::default()
                })
                .listed_slot(make_widget!(image_box).with_props(ImageBoxProps::colored(Color {
                    r: 1.0,
                    g: 0.25,
                    b: 0.25,
                    a: 1.0,
                })))
                .listed_slot(
                    make_widget!(vertical_box)
                        .listed_slot(make_widget!(text_box).with_props(TextBoxProps {
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
                            height: TextBoxSizeValue::Exact(25.0),
                            ..Default::default()
                        }))
                        .listed_slot(make_widget!(text_box).with_props(TextBoxProps {
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
                            height: TextBoxSizeValue::Exact(25.0),
                            vertical_align: TextBoxVerticalAlign::Middle,
                            horizontal_align: TextBoxHorizontalAlign::Center,
                            ..Default::default()
                        })),
                ),
        );

        application.setup(setup);
        let interactions = DefaultInteractionsEngine::new();

        let app = WidgetNode::Component(tree);
        application.apply(app);
        Self { application, interactions }
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
            right: ctx.viewport.width as f32,
            top: 0.0,
            bottom: ctx.viewport.height as f32,
        });

        self.interactions.interact(Interaction::PointerMove(Vec2 { x: 200.0, y: 100.0 }));
        self.interactions
            .interact(Interaction::PointerDown(PointerButton::Trigger, Vec2 { x: 200.0, y: 100.0 }));

        self.application.layout(&mapping, &mut DefaultLayoutEngine).unwrap();
        self.application.interact(&mut self.interactions).unwrap();

        let mut renderer = renderer::WgpuRenderer::new(&ctx, &ui_pipeline, glyph_pipeline, &target);
        self.application.render(&mapping, &mut renderer).unwrap();
    }
}
