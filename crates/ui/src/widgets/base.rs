use core::fmt;

use super::TextData;
use cgmath::*;
use taffy::prelude::*;

pub type Callback = fn();

pub struct Callbacks {
    pub on_click: Option<Callback>,
}

impl fmt::Debug for Callbacks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Callbacks").finish()
    }
}

#[derive(Debug)]
pub struct AssetData {
    pub key: Option<String>,
    pub asset_id: Option<String>,
    pub background: Vector4<f32>,
    pub foreground: Vector4<f32>,
    pub background_hover: Option<Vector4<f32>>,
    pub background_pressed: Option<Vector4<f32>>,
    pub callbacks: Callbacks,
}

impl Default for AssetData {
    fn default() -> Self {
        Self {
            key: None,
            asset_id: None,
            background: Vector4::new(0.0, 0.0, 0.0, 0.0),
            foreground: Vector4::new(0.0, 0.0, 0.0, 0.0),
            background_hover: None,
            background_pressed: None,
            callbacks: Callbacks { on_click: None },
        }
    }
}

#[derive(Debug, Clone)]
pub enum RenderWidget<'a> {
    Text(&'a TextData),
    Asset(&'a AssetData),
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
    fn get_nodes<'a>(&self, taffy: &Taffy, parent_layout: &NodeLayout) -> Vec<(NodeLayout, RenderWidget)>;
}
