use components::*;
use stretch::geometry::Size;
use stretch::style::*;
pub mod components;

pub fn build_ui() {
    let mut stretch = stretch::node::Stretch::new();

    let text = Text::new("this is the text");
    let node = Node::new(
        Style {
            size: Size {
                width: Dimension::Points(100.0),
                height: Dimension::Points(100.0),
            },
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        vec![&text],
    );

    let root = node.render(&mut stretch).expect("Failed to generate layout!");
    let result = node.get_layout(&mut stretch, Size::undefined());
}
