use super::{settings, style};
use crate::world::{self, GameState};
use ui::{components::*, prelude::*, widgets::*};

enum SubMenu {
    Settings,
    None,
}

pub struct MainMenu {
    sub_menu: SubMenu,
}

impl MainMenu {
    pub fn new() -> Self {
        Self { sub_menu: SubMenu::None }
    }

    pub fn draw(&mut self, ctx: &mut engine::Context, ui_state: &mut ui::State, world: &mut world::World) -> Box<dyn BaseWidget> {
        let resume_button = Button::new("resume_button");
        if ui_state.clicked(&resume_button.key).is_some() {
            self.sub_menu = SubMenu::None;
            world.game_state = GameState::Running;
        }

        let settings_button = Button::new("settings_button");
        if ui_state.clicked(&settings_button.key).is_some() {
            self.sub_menu = SubMenu::Settings;
        }

        let exit_button = Button::new("exit_button");
        if ui_state.clicked(&exit_button.key).is_some() {
            world.game_state = GameState::Terminated;
        }

        let menu_panel = PanelWidget::new(
            AssetData { ..Default::default() },
            FlexboxLayout {
                flex_direction: FlexDirection::Column,
                padding: Rect::<Dimension>::from_points(0.0, style::SM, 0.0, 0.0),
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
        )
        .with_children(vec![
            TextWidget::new(
                TextData {
                    size: style::HEADING1,
                    text: "Dungeon Crawler".into(),
                },
                Rect::<Dimension>::from_points(0.0, 0.0, style::SS, style::SL),
                AlignSelf::FlexStart,
            ),
            settings_button.draw(menu_button_props("Settings")),
            resume_button.draw(menu_button_props("Resume")),
            exit_button.draw(menu_button_props("Exit Game")),
        ]);

        let mut children: Vec<Box<dyn BaseWidget>> = vec![menu_panel];
        match self.sub_menu {
            SubMenu::Settings => children.push(settings::settings(ctx, ui_state)),
            SubMenu::None => {}
        }

        PanelWidget::new(
            AssetData {
                background: style::PALETTE_BROWN.extend(0.5),
                gradient: Some(Gradient {
                    background_end: style::PALETTE_GRAY.extend(0.5),
                    angle: 90.0,
                }),
                ..Default::default()
            },
            FlexboxLayout {
                padding: Rect::<Dimension>::from_points(style::SL, style::SM, style::SL, style::SM),
                size: Size {
                    width: Dimension::Percent(1.0),
                    height: Dimension::Percent(1.0),
                },
                ..Default::default()
            },
        )
        .with_children(children)
    }
}

fn menu_button_props(text: &str) -> ButtonProps {
    ButtonProps {
        background: style::PALETTE_GOLD.extend(0.15),
        gradient: Some(Gradient {
            background_end: style::PALETTE_BROWN.extend(0.0),
            angle: 90.0,
        }),
        text: Some((text.into(), style::BODY1)),
        margin: Rect::<Dimension>::from_points(0.0, 0.0, 0.0, style::SM),
        padding: Rect::<Dimension>::from_points(style::SS, style::SS, style::SM, style::SM),
        shadow_color: style::PALETTE_LIGHT_GOLD.extend(1.0),
        shadow_radius: Dimension::Points(style::SHADOW_S),
        border_radius: Dimension::Points(5.0),
        ..Default::default()
    }
}
