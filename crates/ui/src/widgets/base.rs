use cgmath::*;
use taffy::prelude::*;

#[derive(Debug)]
pub struct Gradient {
    pub background_end: Vector4<f32>,
    pub angle: f32,
}

#[derive(Debug)]
pub struct AssetData {
    pub asset_id: Option<String>,
    pub background: Vector4<f32>,
    pub gradient: Option<Gradient>,
    pub foreground: Vector4<f32>,
    pub background_hover: Option<Vector4<f32>>,
    pub background_pressed: Option<Vector4<f32>>,
    pub border_radius: Dimension,
    pub shadow_radius: Dimension,
    pub shadow_offset: Option<Vector2<f32>>,
    pub shadow_color: Vector4<f32>,
    pub visible: bool,
}

impl Default for AssetData {
    fn default() -> Self {
        Self {
            asset_id: None,
            background: Vector4::new(0.0, 0.0, 0.0, 0.0),
            foreground: Vector4::new(0.0, 0.0, 0.0, 0.0),
            background_hover: None,
            background_pressed: None,
            border_radius: Dimension::default(),
            shadow_radius: Dimension::default(),
            shadow_offset: None,
            gradient: None,
            shadow_color: Vector4::new(0.0, 0.0, 0.0, 1.0),
            visible: true,
        }
    }
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

pub struct RenderParams {
    pub scale: Point2<f32>,
    pub opacity: f32,
    pub frame_time: f32,
}

pub trait BaseWidget {
    fn calculate_layout(&mut self, ctx: &mut engine::Context, taffy: &mut Taffy) -> Node;
    fn render(&self, taffy: &Taffy, engine: &mut engine::Engine, parent_layout: &NodeLayout, params: &RenderParams);
}
