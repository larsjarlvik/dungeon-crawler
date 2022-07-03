use super::base;
use stretch::{style::*, Stretch};

pub struct Node<'a> {
    pub style: Style,
    pub children: Vec<&'a dyn base::BaseComponent>,
    node: Option<stretch::node::Node>,
}

impl<'a> Node<'a> {
    pub fn new(style: Style, children: Vec<&'a dyn base::BaseComponent>) -> Self {
        Self {
            style,
            children,
            node: None,
        }
    }
}

impl<'a> base::BaseComponent for Node<'a> {
    fn render(&self, stretch: &mut Stretch) -> Result<stretch::node::Node, stretch::Error> {
        let children: Vec<stretch::node::Node> = self.children.iter().map(|c| c.render(stretch).unwrap()).collect();
        let node = stretch.new_node(self.style, children).unwrap();
        self.node = Some(node);
        Ok(node)
    }

    fn get_layout(
        &self,
        stretch: &mut Stretch,
        size: stretch::geometry::Size<stretch::number::Number>,
    ) -> Vec<Result<stretch::result::Layout, stretch::Error>> {
        todo!()
    }
}
