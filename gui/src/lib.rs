use taffy::prelude::*;
use widgets::BaseWidget;
mod widgets;

pub fn build_ui() {
    let mut taffy = Taffy::new();

    let mut root = widgets::NodeWidget::new(
        FlexboxLayout {
            size: Size {
                width: Dimension::Percent(1.0),
                height: Dimension::Percent(1.0),
            },
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        vec![
            Box::new(widgets::TextWidget::new("This is a text")),
            Box::new(widgets::TextWidget::new("This is a text")),
        ],
    );

    let root_node = root.render(&mut taffy);

    taffy
        .compute_layout(
            root_node,
            Size {
                height: Some(100.0),
                width: Some(100.0),
            },
        )
        .unwrap();

    for node in root.get_nodes() {
        dbg!(taffy.layout(node).unwrap());
    }
}
