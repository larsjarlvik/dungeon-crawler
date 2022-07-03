use super::base;
use stretch::geometry::Size;
use stretch::{style::*, Stretch};

pub struct Text {
    pub value: String,
    node: Option<stretch::node::Node>,
}

impl Text {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
            node: None,
        }
    }
}

impl base::BaseComponent for Text {
    fn render(&self, stretch: &mut Stretch) -> Result<stretch::node::Node, stretch::Error> {
        stretch.new_node(
            Style {
                size: Size {
                    width: Dimension::Points(100.0),
                    height: Dimension::Points(100.0),
                },
                ..Default::default()
            },
            vec![],
        )
    }

    fn get_layout(
        &self,
        stretch: &mut Stretch,
        size: stretch::geometry::Size<stretch::number::Number>,
    ) -> Vec<Result<stretch::result::Layout, stretch::Error>> {
        todo!()
    }
}
