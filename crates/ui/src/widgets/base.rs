use super::TextData;
use cgmath::*;
use taffy::prelude::*;

#[derive(Debug, Clone)]
pub struct AssetData {
    pub key: Option<String>,
    pub asset_id: Option<String>,
    pub background: Vector4<f32>,
    pub background_hover: Option<Vector4<f32>>,
    pub background_pressed: Option<Vector4<f32>>,
}

impl Default for AssetData {
    fn default() -> Self {
        Self {
            key: None,
            asset_id: None,
            background: Vector4::new(1.0, 1.0, 1.0, 1.0),
            background_hover: None,
            background_pressed: None,
        }
    }
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
