use taffy::prelude::*;

#[derive(Debug, Clone)]
pub enum RenderWidget {
    Text(String),
    None,
}

#[derive(Debug, Clone)]
pub struct NodeLayout {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
}

pub trait BaseWidget {
    fn render(&mut self, taffy: &mut Taffy) -> Node;
    fn get_nodes(&self, taffy: &Taffy, parent_layout: &NodeLayout) -> Vec<(NodeLayout, RenderWidget)>;
}
