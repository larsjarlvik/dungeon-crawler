use super::settings::Settings;
use crate::{
    ui::style,
    world::{self, GameState},
};
use ui::{components::*, prelude::*, widgets::*};

enum SubMenu {
    Settings,
    None,
}

pub struct MainMenu {
    sub_menu: SubMenu,
    settings: Settings,
}

impl MainMenu {
    pub fn new(ctx: &engine::Context) -> Self {
        Self {
            sub_menu: SubMenu::None,
            settings: Settings::new(ctx),
        }
    }

    pub fn draw(&mut self, engine: &mut engine::Engine, ui_state: &mut ui::State, world: &mut world::World) -> Box<dyn BaseWidget> {
        let new_game_button = Button::new("new_game_button");
        if ui_state.clicked(&new_game_button.key).is_some() {
            self.sub_menu = SubMenu::None;
            world.init(engine);
            world.game_state = GameState::Running;
        }

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

        let mut menu_panel = PanelWidget::new(
            None,
            AssetData { ..Default::default() },
            Style {
                flex_direction: FlexDirection::Column,
                padding: Rect::<Dimension>::from_points(style::SM, style::SL, 0.0, 0.0),
                max_size: Size {
                    width: Dimension::Points(800.0),
                    height: Dimension::Undefined,
                },
                min_size: Size {
                    width: Dimension::Points(500.0),
                    height: Dimension::Undefined,
                },
                size: Size {
                    width: Dimension::Percent(0.3),
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
            new_game_button.draw(menu_button_props("New Game")),
            settings_button.draw(menu_button_props("Settings")),
            exit_button.draw(menu_button_props("Exit Game")),
        ]);

        if !world.is_dead() {
            menu_panel.children.insert(2, resume_button.draw(menu_button_props("Resume")));
        }

        let mut children: Vec<Box<dyn BaseWidget>> = vec![menu_panel];
        match self.sub_menu {
            SubMenu::Settings => children.push(self.settings.draw(ui_state, world)),
            SubMenu::None => {}
        }

        PanelWidget::new(
            Some("main_menu".into()),
            AssetData {
                background: style::PALETTE_BROWN.extend(0.5),
                gradient: Some(Gradient {
                    background_end: style::PALETTE_GRAY.extend(0.5),
                    angle: 90.0,
                }),
                ..Default::default()
            },
            Style {
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
        background: style::PALETTE_GOLD.extend(0.3),
        gradient: Some(Gradient {
            background_end: style::PALETTE_BROWN.extend(0.0),
            angle: 90.0,
        }),
        text: Some((text.into(), style::BODY1)),
        margin: Rect::<Dimension>::from_points(0.0, 0.0, 0.0, style::SM),
        padding: Rect::<Dimension>::from_points(style::SS, style::SS, style::SM, style::SM),
        shadow_color: style::PALETTE_LIGHT_GOLD.extend(1.0),
        shadow_radius: Dimension::Points(style::SHADOW_S),
        border_radius: Dimension::Points(style::RADIUS_M),
        ..Default::default()
    }
}
