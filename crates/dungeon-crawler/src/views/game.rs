use crate::world::{
    self,
    resources::{self, input::UiActionCode},
    GameState,
};
use cgmath::*;
use ui::{components::*, prelude::*, widgets::*};

use super::style;

pub fn game(ctx: &mut engine::Context, ui_state: &mut ui::State, world: &mut world::World) -> Box<dyn BaseWidget> {
    let mut top_left = NodeWidget::new(
        FlexboxLayout {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        vec![Bar::new().draw(
            "Health",
            BarProps {
                width: Dimension::Points(200.0),
                value: 50.0,
                max_value: 100.0,
                color: Vector4::new(0.8, 0.0, 0.0, 1.0),
            },
        )],
    );

    if ctx.settings.show_fps {
        let fps = world.components.get_resource::<resources::Fps>().unwrap();
        top_left.children.push(TextWidget::new(
            TextData {
                text: format!("FPS: {}", fps.fps),
                size: style::BODY2,
            },
            Default::default(),
            AlignSelf::FlexStart,
        ));
    }

    let mut input = world.components.get_resource_mut::<resources::Input>().unwrap();

    let menu_button = Button::new("menu_button");
    if ui_state.clicked(&menu_button.key) {
        world.game_state = GameState::MainMenu;
    }

    let attack_button = Button::new("attack_button");
    input.set_from_ui(UiActionCode::Attack, ui_state.mouse_down(&attack_button.key));

    let health_button = Button::new("health_button");
    input.set_from_ui(UiActionCode::Health, ui_state.mouse_down(&attack_button.key));

    NodeWidget::new(
        FlexboxLayout {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            padding: Rect::<Dimension>::from_points(style::SM, style::SM, style::SM, style::SM),
            size: Size {
                width: Dimension::Percent(1.0),
                height: Dimension::Percent(1.0),
            },
            ..Default::default()
        },
        vec![
            NodeWidget::new(
                FlexboxLayout {
                    justify_content: JustifyContent::SpaceBetween,
                    size: Size {
                        width: Dimension::Percent(1.0),
                        height: Dimension::Auto,
                    },
                    ..Default::default()
                },
                vec![
                    PanelWidget::new(
                        AssetData { ..Default::default() },
                        FlexboxLayout { ..Default::default() },
                        vec![top_left],
                    ),
                    menu_button.draw(ButtonProps {
                        icon: Some(("menu".into(), style::ICON_M)),
                        variant: Variant::Rounded,
                        margin: Rect::<Dimension>::from_points(0.0, 0.0, -style::SS, 0.0),
                        ..Default::default()
                    }),
                ],
            ),
            NodeWidget::new(
                FlexboxLayout {
                    justify_content: JustifyContent::FlexEnd,
                    size: Size {
                        width: Dimension::Percent(1.0),
                        height: Dimension::Auto,
                    },
                    ..Default::default()
                },
                vec![
                    attack_button.draw(ButtonProps {
                        icon: Some(("attack".into(), style::ICON_L)),
                        foreground: vec4(1.0, 0.5, 0.0, 1.0),
                        margin: Rect::<Dimension>::from_points(style::SS, 0.0, 0.0, 0.0),
                        variant: Variant::Rounded,
                        ..Default::default()
                    }),
                    health_button.draw(ButtonProps {
                        icon: Some(("health".into(), style::ICON_L)),
                        foreground: vec4(0.8, 0.0, 0.0, 1.0),
                        margin: Rect::<Dimension>::from_points(style::SS, 0.0, 0.0, 0.0),
                        variant: Variant::Rounded,
                        ..Default::default()
                    }),
                ],
            ),
        ],
    )
}
