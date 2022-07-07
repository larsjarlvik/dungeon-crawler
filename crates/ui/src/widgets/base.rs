use super::TextData;
use cgmath::*;
use taffy::prelude::*;

#[derive(Debug, Clone)]
pub struct AssetData {
    pub asset_id: Option<String>,
    pub background: Option<Vector4<f32>>,
}

#[derive(Debug, Clone)]
pub enum RenderWidget {
    Text(TextData),
    Asset(AssetData),
    None,
}

#[derive(Debug, Clone)]
pub struct NodeLayout {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
}

impl NodeLayout {
    pub fn new(parent_layout: &NodeLayout, layout: &Layout) -> Self {
        Self {
            x: parent_layout.x + layout.location.x,
            y: parent_layout.y + layout.location.y,
            width: layout.size.width,
            height: layout.size.height,
        }
    }
}

pub trait BaseWidget {
    fn render(&mut self, ctx: &mut engine::Context, taffy: &mut Taffy) -> Node;
    fn get_nodes(&self, taffy: &Taffy, parent_layout: &NodeLayout) -> Vec<(NodeLayout, RenderWidget)>;
}
