use crate::world::{
    self,
    resources::{self},
};
use ui::{components::*, prelude::*, widgets::*};

pub fn game(ctx: &mut engine::Context, world: &world::World) -> Box<dyn BaseWidget> {
    let mut top_left = NodeWidget::new(
        FlexboxLayout {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        vec![],
    );

    if ctx.settings.show_fps {
        let fps = world.components.get_resource::<resources::Fps>().unwrap();
        top_left.children.push(TextWidget::new(
            TextData {
                text: format!("FPS: {}", fps.fps),
                size: 16.0,
            },
            Default::default(),
        ));
    }

    NodeWidget::new(
        FlexboxLayout {
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::SpaceBetween,
            padding: Rect::<Dimension>::from_points(10.0, 10.0, 10.0, 10.0),
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
            button(
                "button",
                || {
                    dbg!("hej");
                },
                ButtonProps {
                    icon: Some("menu".into()),
                    ..Default::default()
                },
            ),
        ],
    )
}
