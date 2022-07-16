use crate::world::{
    self, components,
    resources::{self, input::UiActionCode},
    GameState,
};
use bevy_ecs::prelude::*;
use cgmath::*;
use ui::{components::*, prelude::*, widgets::*};

use super::style;

pub fn game(ctx: &mut engine::Context, ui_state: &mut ui::State, world: &mut world::World) -> Box<dyn BaseWidget> {
    let mut top_left: Vec<Box<dyn BaseWidget>> = vec![];
    let mut top_right: Vec<Box<dyn BaseWidget>> = vec![];

    for stats in world
        .components
        .query_filtered::<&components::Stats, With<components::UserControl>>()
        .iter(&world.components)
    {
        let max = stats.get_base_health();
        top_left.push(Bar::new().draw(
            &format!("{} / {}", stats.health.current.floor(), max),
            BarProps {
                width: Dimension::Percent(1.0),
                value: stats.health.current,
                max_value: max,
                border_color: style::PALETTE_LIGHT_GOLD.extend(1.0),
                color: style::PALETTE_RED.extend(1.0),
                gradient: Some(Gradient {
                    background_end: (style::PALETTE_RED * 0.2).extend(1.0),
                    angle: 180.0,
                }),
                ..Default::default()
            },
        ));

        let level = components::stats::get_level(stats.experience);
        let level_experience = components::stats::get_level_experience(level);
        let next_level_experience = components::stats::get_level_experience(level + 1);

        let current_experiance = (stats.experience - level_experience) as f32;
        let total_level_experiance = (next_level_experience - level_experience) as f32;

        top_left.push(Bar::new().draw(
            &format!("Level: {}", level),
            BarProps {
                width: Dimension::Percent(1.0),
                value: current_experiance,
                max_value: total_level_experiance,
                border_color: style::PALETTE_LIGHT_GOLD.extend(1.0),
                color: style::PALETTE_BROWN.extend(1.0),
                margin: Rect::<Dimension>::from_points(0.0, 0.0, style::SS, 0.0),
                gradient: Some(Gradient {
                    background_end: (style::PALETTE_BROWN * 0.2).extend(1.0),
                    angle: 180.0,
                }),
            },
        ));
    }

    for (stats, name) in world
        .components
        .query_filtered::<(&components::Stats, &components::Name), With<components::Display>>()
        .iter(&world.components)
    {
        top_right.push(Bar::new().draw(
            &name.name,
            BarProps {
                width: Dimension::Percent(1.0),
                value: stats.health.current,
                max_value: stats.get_base_health(),
                border_color: style::PALETTE_LIGHT_GOLD.extend(1.0),
                color: style::PALETTE_RED.extend(1.0),
                gradient: Some(Gradient {
                    background_end: (style::PALETTE_RED * 0.2).extend(1.0),
                    angle: 180.0,
                }),
                ..Default::default()
            },
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

    if ctx.settings.show_fps {
        let fps = world.components.get_resource::<resources::Fps>().unwrap();
        top_left.push(
            NodeWidget::new(FlexboxLayout {
                margin: Rect::<Dimension>::from_points(0.0, 0.0, style::SM, 0.0),
                ..Default::default()
            })
            .with_children(vec![TextWidget::new(
                TextData {
                    text: format!("FPS: {}", fps.fps),
                    size: style::BODY2,
                },
                Default::default(),
                AlignSelf::FlexStart,
            )]),
        );
    }

    NodeWidget::new(FlexboxLayout {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::SpaceBetween,
        padding: Rect::<Dimension>::from_points(style::SM, style::SM, style::SM, style::SM),
        size: Size {
            width: Dimension::Percent(1.0),
            height: Dimension::Percent(1.0),
        },
        ..Default::default()
    })
    .with_children(vec![
        NodeWidget::new(FlexboxLayout {
            justify_content: JustifyContent::SpaceBetween,
            size: Size {
                width: Dimension::Percent(1.0),
                height: Dimension::Auto,
            },
            ..Default::default()
        })
        .with_children(vec![
            NodeWidget::new(FlexboxLayout {
                flex_direction: FlexDirection::Column,
                size: Size {
                    width: Dimension::Percent(0.15),
                    height: Dimension::Auto,
                },
                ..Default::default()
            })
            .with_children(top_left),
            NodeWidget::new(FlexboxLayout {
                flex_direction: FlexDirection::Column,
                size: Size {
                    width: Dimension::Percent(0.15),
                    height: Dimension::Auto,
                },
                ..Default::default()
            })
            .with_children(top_right),
        ]),
        NodeWidget::new(FlexboxLayout {
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::FlexEnd,
            size: Size {
                width: Dimension::Percent(1.0),
                height: Dimension::Auto,
            },
            ..Default::default()
        })
        .with_children(vec![
            menu_button.draw(ButtonProps {
                icon: Some(("menu".into(), style::ICON_M)),
                border_radius: Dimension::Percent(0.5),
                background: vec4(0.0, 0.0, 0.0, 0.7),
                margin: Rect::<Dimension>::from_points(style::SS, 0.0, 0.0, 0.0),
                padding: Rect::<Dimension>::from_points(style::SM, style::SM, style::SM, style::SM),
                ..Default::default()
            }),
            attack_button.draw(ButtonProps {
                icon: Some(("attack".into(), style::ICON_L)),
                foreground: vec4(1.0, 0.5, 0.0, 1.0),
                background: vec4(0.0, 0.0, 0.0, 0.7),
                margin: Rect::<Dimension>::from_points(style::SS, 0.0, 0.0, 0.0),
                padding: Rect::<Dimension>::from_points(style::SL, style::SL, style::SL, style::SL),
                border_radius: Dimension::Percent(0.5),
                ..Default::default()
            }),
            health_button.draw(ButtonProps {
                icon: Some(("health".into(), style::ICON_L)),
                foreground: vec4(0.8, 0.0, 0.0, 1.0),
                background: vec4(0.0, 0.0, 0.0, 0.7),
                margin: Rect::<Dimension>::from_points(style::SS, 0.0, 0.0, 0.0),
                padding: Rect::<Dimension>::from_points(style::SL, style::SL, style::SL, style::SL),
                border_radius: Dimension::Percent(0.5),
                ..Default::default()
            }),
        ]),
    ])
}
