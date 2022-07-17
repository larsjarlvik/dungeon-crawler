use crate::world::{
    self, components,
    resources::{self, input::UiActionCode},
    GameState,
};
use bevy_ecs::prelude::*;
use cgmath::*;
use ui::{components::*, prelude::*, widgets::*};

use super::style;

fn status_bar(label: &str, value: f32, max_value: f32, color: Vector3<f32>) -> Box<PanelWidget> {
    Bar::new().draw(
        label,
        BarProps {
            width: Dimension::Percent(1.0),
            value,
            max_value,
            border_color: style::PALETTE_LIGHT_GOLD.extend(1.0),
            margin: Rect::<Dimension>::from_points(0.0, 0.0, 0.0, style::SS),
            color: color.extend(1.0),
            gradient: Some(Gradient {
                background_end: (color * 0.4).extend(1.0),
                angle: 180.0,
            }),
            ..Default::default()
        },
    )
}

fn action_button(button: &Button, icon: &str, foreground: Vector3<f32>, icon_size: f32, padding: f32) -> Box<PanelWidget> {
    button.draw(ButtonProps {
        icon: Some((icon.into(), icon_size)),
        border_radius: Dimension::Percent(0.5),
        background: vec4(0.0, 0.0, 0.0, 0.7),
        foreground: foreground.extend(1.0),
        margin: Rect::<Dimension>::from_points(style::SS, 0.0, 0.0, 0.0),
        padding: Rect::<Dimension>::from_points(padding, padding, padding, padding),
        ..Default::default()
    })
}

fn top_bar(ctx: &mut engine::Context, world: &mut world::World) -> Box<NodeWidget> {
    let mut top_left: Vec<Box<dyn BaseWidget>> = vec![];
    for stats in world
        .components
        .query_filtered::<&components::Stats, With<components::UserControl>>()
        .iter(&world.components)
    {
        let max = stats.get_base_health();
        top_left.push(status_bar(
            &format!("{} / {}", stats.health.current.floor(), max),
            stats.health.current,
            max,
            style::PALETTE_LIGHT_RED,
        ));

        let level = components::stats::get_level(stats.experience);
        let level_experience = components::stats::get_level_experience(level);
        let next_level_experience = components::stats::get_level_experience(level + 1);
        top_left.push(status_bar(
            &format!("Level: {}", level),
            (stats.experience - level_experience) as f32,
            (next_level_experience - level_experience) as f32,
            style::PALETTE_LIGHT_GOLD,
        ));
    }

    if ctx.settings.show_fps {
        let fps = world.components.get_resource::<resources::Fps>().unwrap();
        top_left.push(NodeWidget::new(FlexboxLayout::default()).with_children(vec![TextWidget::new(
            TextData {
                text: format!("FPS: {}", fps.fps),
                size: style::BODY2,
            },
            Default::default(),
            AlignSelf::FlexStart,
        )]));
    }

    let mut top_right: Vec<Box<dyn BaseWidget>> = vec![];
    for (stats, name) in world
        .components
        .query_filtered::<(&components::Stats, &components::Name), With<components::Display>>()
        .iter(&world.components)
    {
        top_right.push(status_bar(
            &name.name,
            stats.health.current,
            stats.get_base_health(),
            style::PALETTE_LIGHT_RED,
        ));
    }

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
                width: Dimension::Percent(0.25),
                height: Dimension::Auto,
            },
            max_size: Size {
                width: Dimension::Points(250.0),
                height: Dimension::Auto,
            },
            ..Default::default()
        })
        .with_children(top_left),
        NodeWidget::new(FlexboxLayout {
            flex_direction: FlexDirection::Column,
            size: Size {
                width: Dimension::Percent(0.25),
                height: Dimension::Auto,
            },
            max_size: Size {
                width: Dimension::Points(250.0),
                height: Dimension::Auto,
            },
            ..Default::default()
        })
        .with_children(top_right),
    ])
}

pub fn game(ctx: &mut engine::Context, ui_state: &mut ui::State, world: &mut world::World) -> Box<dyn BaseWidget> {
    let mut input = world.components.get_resource_mut::<resources::Input>().unwrap();
    let menu_button = Button::new("menu_button");
    if ui_state.clicked(&menu_button.key) {
        world.game_state = GameState::MainMenu;
    }

    let attack_button = Button::new("attack_button");
    input.set_from_ui(UiActionCode::Attack, ui_state.mouse_down(&attack_button.key));

    let health_button = Button::new("health_button");
    input.set_from_ui(UiActionCode::Health, ui_state.mouse_down(&health_button.key));

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
        top_bar(ctx, world),
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
            action_button(&menu_button, "menu", style::TEXT, style::ICON_M, style::SM),
            action_button(&attack_button, "attack", style::PALETTE_LIGHT_GOLD, style::ICON_L, style::SL),
            action_button(&health_button, "health", style::PALETTE_LIGHT_RED, style::ICON_L, style::SL),
        ]),
    ])
}
