use super::style;
use crate::world::{self, GameState};
use cgmath::Vector4;
use ui::{components::*, prelude::*, widgets::*};

pub fn main_menu(ui_state: &mut ui::State, world: &mut world::World) -> Box<dyn BaseWidget> {
    let resume_button = Button::new("resume_button");
    if ui_state.clicked(&resume_button.key) {
        world.game_state = GameState::Running;
    }

    let settings_button = Button::new("settings_button");
    if ui_state.clicked(&settings_button.key) {
        world.game_state = GameState::Running;
    }

    let exit_button = Button::new("exit_button");
    if ui_state.clicked(&exit_button.key) {
        world.game_state = GameState::Terminated;
    }

    let menu_panel = PanelWidget::new(
        AssetData {
            background: Vector4::new(0.0, 0.0, 0.0, 0.6),
            ..Default::default()
        },
        FlexboxLayout {
            flex_direction: FlexDirection::Column,
            padding: Rect::<Dimension>::from_points(style::SM, style::SM, style::SM, style::SM),
            max_size: Size {
                width: Dimension::Points(400.0),
                height: Dimension::Undefined,
            },
            size: Size {
                width: Dimension::Percent(0.35),
                height: Dimension::Percent(1.0),
            },
            ..Default::default()
        },
        vec![
            TextWidget::new(
                TextData {
                    size: style::HEADING1,
                    text: "Dungeon Crawler".into(),
                },
                Rect::<Dimension>::from_points(0.0, 0.0, style::SS, style::SL),
                AlignSelf::Center,
            ),
            settings_button.draw(ButtonProps {
                background: style::PRIMARY_BACKGROUND,
                text: Some(("Settings".into(), style::BODY1)),
                margin: Rect::<Dimension>::from_points(0.0, 0.0, 0.0, style::SS),
                variant: Variant::Border,
                ..Default::default()
            }),
            resume_button.draw(ButtonProps {
                background: style::PRIMARY_BACKGROUND,
                text: Some(("Resume".into(), style::BODY1)),
                margin: Rect::<Dimension>::from_points(0.0, 0.0, 0.0, style::SS),
                variant: Variant::Border,
                ..Default::default()
            }),
            exit_button.draw(ButtonProps {
                background: style::PRIMARY_BACKGROUND,
                text: Some(("Exit Game".into(), style::BODY1)),
                margin: Rect::<Dimension>::from_points(0.0, 0.0, 0.0, style::SS),
                variant: Variant::Border,
                ..Default::default()
            }),
        ],
    );

    PanelWidget::new(
        AssetData {
            background: Vector4::new(0.0, 0.0, 0.0, 0.5),
            ..Default::default()
        },
        FlexboxLayout {
            size: Size {
                width: Dimension::Percent(1.0),
                height: Dimension::Percent(1.0),
            },
            ..Default::default()
        },
        vec![menu_panel],
    )
}
