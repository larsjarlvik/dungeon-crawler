use crate::ui::style;
use crate::world;
use crate::world::GameState;
use ui::components::*;
use ui::prelude::*;
use ui::widgets::*;

pub struct Settings {
    settings: engine::Settings,
    scroll: Scroll,
}

impl Settings {
    pub fn new(ctx: &engine::Context) -> Settings {
        Self {
            settings: ctx.settings,
            scroll: Scroll::new("settings_scroll", 0.0),
        }
    }

    pub fn draw(&mut self, ui_state: &mut ui::State, world: &mut world::World) -> Box<dyn BaseWidget> {
        let contrast = create_slider(ui_state, "contrast", self.settings.contrast, 10.0, |val| {
            self.settings.contrast = (val * 20.0).round() / 2.0;
        });

        let render_scale = create_slider(ui_state, "render_scale", self.settings.render_scale * 100.0, 100.0, |val| {
            self.settings.render_scale = ((val * 20.0).round() / 20.0).max(0.05);
        });

        let ui_scale = create_slider(ui_state, "ui_scale", self.settings.ui_scale, 2.0, |val| {
            self.settings.ui_scale = (val * 8.0).round() / 4.0;
        });

        let shadow_quality = create_slider(ui_state, "shadow_quality", self.settings.shadow_map_scale, 4.0, |val| {
            self.settings.shadow_map_scale = (val * 8.0).round() / 2.0;
        });

        let anti_aliasing = create_checkbox(ui_state, "anti_aliasing", self.settings.smaa, || {
            self.settings.smaa = !self.settings.smaa;
        });

        let sharpen = create_checkbox(ui_state, "sharpen", self.settings.sharpen, || {
            self.settings.sharpen = !self.settings.sharpen;
        });

        let show_fps = create_checkbox(ui_state, "show_fps", self.settings.show_fps, || {
            self.settings.show_fps = !self.settings.show_fps;
        });

        let audio_effects = create_slider(ui_state, "audio_effects", self.settings.audio_effects, 1.0, |val| {
            self.settings.audio_effects = (val * 20.0).round() / 20.0;
        });

        let audio_ambient = create_slider(ui_state, "audio_ambient", self.settings.audio_ambient, 1.0, |val| {
            self.settings.audio_ambient = (val * 20.0).round() / 20.0;
        });

        self.scroll.handle_state(ui_state);
        let apply_settings = Button::new("apply_settings");
        if ui_state.clicked(&apply_settings.key).is_some() {
            self.settings.store();
            world.game_state = GameState::Reload;
        }

        let reset_settings = Button::new("reset_settings");
        if ui_state.clicked(&reset_settings.key).is_some() {
            self.settings = engine::Settings::load();
        }

        NodeWidget::new(Style {
            flex_direction: FlexDirection::Column,
            margin: Rect::from_points(0.0, 0.0, style::SM, 0.0),
            size: Size {
                width: Dimension::Percent(1.0),
                height: Dimension::Percent(1.0),
            },
            ..Default::default()
        })
        .with_children(vec![
            self.scroll.draw(
                ScrollProps::default(),
                vec![
                    TextWidget::new(
                        TextData {
                            size: style::HEADING2,
                            text: "Graphics".into(),
                        },
                        Rect::from_points(0.0, 0.0, 0.0, style::SM),
                        AlignSelf::FlexStart,
                    ),
                    setting("Contrast:", contrast.draw(), Some(format!("{:.2}", contrast.value))),
                    setting(
                        "Render scale:",
                        render_scale.draw(),
                        Some(format!("{:.0}%", render_scale.value)),
                    ),
                    setting("UI scale:", ui_scale.draw(), Some(format!("{:.0}%", ui_scale.value * 100.0))),
                    setting(
                        "Shadow quality:",
                        shadow_quality.draw(),
                        Some(format!("{:.2}", shadow_quality.value)),
                    ),
                    setting("Anti aliasing:", anti_aliasing.draw(), None),
                    setting("Sharpen:", sharpen.draw(), None),
                    setting("Show FPS:", show_fps.draw(), None),
                    TextWidget::new(
                        TextData {
                            size: style::HEADING2,
                            text: "Sound".into(),
                        },
                        Rect::from_points(0.0, 0.0, style::SL, style::SM),
                        AlignSelf::FlexStart,
                    ),
                    setting(
                        "Effects:",
                        audio_effects.draw(),
                        Some(format!("{:.0}%", audio_effects.value * 100.0)),
                    ),
                    setting(
                        "Ambient:",
                        audio_ambient.draw(),
                        Some(format!("{:.0}%", audio_ambient.value * 100.0)),
                    ),
                ],
            ),
            NodeWidget::new(Style {
                margin: Rect::from_points(0.0, 0.0, style::SL, style::SL),
                ..Default::default()
            })
            .with_children(vec![
                reset_settings.draw(ButtonProps {
                    text: Some(("Reset".into(), style::BODY2)),
                    padding: Rect::from_points(style::SM, style::SM, style::SS, style::SS),
                    margin: Rect::from_points(0.0, style::SM, 0.0, 0.0),
                    background: style::PALETTE_LIGHT_GRAY.extend(0.6),
                    border_radius: Dimension::Points(style::RADIUS_M),
                    ..Default::default()
                }),
                apply_settings.draw(ButtonProps {
                    text: Some(("Apply".into(), style::BODY2)),
                    padding: Rect::from_points(style::SM, style::SM, style::SS, style::SS),
                    background: style::PALETTE_LIGHT_GOLD.extend(0.6),
                    border_radius: Dimension::Points(style::RADIUS_M),
                    ..Default::default()
                }),
            ]),
        ])
    }
}

fn setting(label: &str, control: Box<dyn BaseWidget>, value: Option<String>) -> Box<NodeWidget> {
    NodeWidget::new(Style {
        align_items: AlignItems::Center,
        margin: Rect::from_points(0.0, 0.0, 0.0, style::SM),
        ..Default::default()
    })
    .with_children(vec![
        NodeWidget::new(Style {
            flex_direction: FlexDirection::Column,
            size: Size {
                width: Dimension::Points(300.0),
                height: Dimension::Auto,
            },
            ..Default::default()
        })
        .with_children(vec![TextWidget::new(
            TextData {
                size: style::BODY1,
                text: label.into(),
            },
            Default::default(),
            AlignSelf::FlexStart,
        )]),
        control,
        NodeWidget::new(Style {
            flex_direction: FlexDirection::Column,
            margin: Rect::from_points(style::SL, 0.0, 0.0, 0.0),
            ..Default::default()
        })
        .with_children(if let Some(value) = value {
            vec![TextWidget::new(
                TextData {
                    size: style::BODY2,
                    text: value,
                },
                Default::default(),
                AlignSelf::FlexStart,
            )]
        } else {
            vec![]
        }),
    ])
}

fn create_slider<F: FnOnce(f32)>(ui_state: &mut ui::State, key: &str, value: f32, max_value: f32, handle: F) -> Slider {
    if let Some(mouse) = ui_state.mouse_down(key) {
        handle(mouse.x.max(0.0).min(1.0));
    }

    Slider {
        key: key.into(),
        value,
        max_value,
    }
}

fn create_checkbox<F: FnOnce()>(ui_state: &mut ui::State, key: &str, checked: bool, handle: F) -> Checkbox {
    if ui_state.clicked(key).is_some() {
        handle();
    }

    Checkbox { key: key.into(), checked }
}
