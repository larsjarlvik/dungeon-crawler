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
        if ui_state.clicked(&new_game_button.key, true).is_some() {
            self.sub_menu = SubMenu::None;
            world.init(engine);
            world.game_state = GameState::Running;
        }

        let resume_button = Button::new("resume_button");
        if ui_state.clicked(&resume_button.key, true).is_some() {
            self.sub_menu = SubMenu::None;
            world.game_state = GameState::Running;
        }

        let settings_button = Button::new("settings_button");
        if ui_state.clicked(&settings_button.key, true).is_some() {
            self.sub_menu = SubMenu::Settings;
        }

        let exit_button = Button::new("exit_button");
        if ui_state.clicked(&exit_button.key, true).is_some() {
            world.game_state = GameState::Terminated;
        }

        let mut menu_panel = DisplayWidget::new(
            DisplayWidgetProps { ..Default::default() },
            Style {
                flex_direction: FlexDirection::Column,
                padding: Rect {
                    left: length(style::SM),
                    right: length(style::SL),
                    top: length(0.0),
                    bottom: length(0.0),
                },
                max_size: Size {
                    width: length(800.0),
                    height: auto(),
                },
                min_size: Size {
                    width: length(500.0),
                    height: auto(),
                },
                size: Size {
                    width: percent(0.3),
                    height: percent(1.0),
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
                Rect {
                    left: length(0.0),
                    right: length(0.0),
                    top: length(style::SS),
                    bottom: length(style::SL),
                },
                AlignItems::FlexStart,
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

        DisplayWidget::new(
            DisplayWidgetProps {
                background: style::PALETTE_BROWN.extend(0.5),
                gradient: Some(Gradient {
                    background_end: style::PALETTE_GRAY.extend(0.5),
                    angle: 90.0,
                }),
                ..Default::default()
            },
            Style {
                padding: Rect {
                    left: length(style::SL),
                    right: length(style::SL),
                    top: length(style::SL),
                    bottom: length(style::SM),
                },
                size: Size {
                    width: percent(1.0),
                    height: percent(1.0),
                },
                ..Default::default()
            },
        )
        .with_key("main_menu")
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
        margin: Rect {
            left: length(0.0),
            right: length(0.0),
            top: length(0.0),
            bottom: length(style::SM),
        },
        padding: Rect {
            left: length(style::SS),
            right: length(style::SS),
            top: length(style::SM),
            bottom: length(style::SM),
        },
        shadow_color: style::PALETTE_LIGHT_GOLD.extend(1.0),
        shadow_radius: length(style::SHADOW_S),
        border_radius: length(style::RADIUS_M),
        ..Default::default()
    }
}
