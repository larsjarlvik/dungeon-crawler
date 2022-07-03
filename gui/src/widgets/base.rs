use taffy::prelude::*;

pub trait BaseWidget {
    fn render(&mut self, taffy: &mut Taffy) -> Node;
    fn get_nodes(&self) -> Vec<Node>;
}
