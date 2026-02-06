use crate::ui::style;
use crate::world;
use crate::world::GameState;
use ui::prelude::*;
use ui::widgets::*;

pub fn dead(ui_state: &mut ui::State, world: &mut world::World) -> Box<dyn BaseWidget> {
    let key = "dead_screen".to_string();

    let screen = DisplayWidget::new(
        DisplayWidgetProps {
            background: style::PALETTE_BROWN.extend(0.5),
            ..Default::default()
        },
        Style {
            flex_direction: FlexDirection::Column,
            align_items: Some(AlignItems::Center),
            justify_content: Some(JustifyContent::Center),
            size: Size {
                width: percent(1.0),
                height: percent(1.0),
            },
            ..Default::default()
        },
    )
    .with_key(key.as_str())
    .with_children(vec![
        TextWidget::new(
            TextData {
                size: style::DISPLAY1,
                text: "You Died!".into(),
            },
            Rect {
                left: length(0.0),
                right: length(0.0),
                top: length(0.0),
                bottom: length(style::SL),
            },
            AlignItems::Center,
        ),
        TextWidget::new(
            TextData {
                size: style::BODY1,
                text: "Tap screen to return to main menu...".into(),
            },
            Rect {
                left: length(0.0),
                right: length(0.0),
                top: length(0.0),
                bottom: length(style::SL),
            },
            AlignItems::Center,
        ),
    ]);

    if ui_state.clicked(&key, true).is_some() {
        world.game_state = GameState::MainMenu;
    }

    screen
}
