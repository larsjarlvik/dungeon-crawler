use cgmath::*;
use taffy::prelude::*;

#[derive(Debug)]
pub struct Gradient {
    pub background_end: Vector4<f32>,
    pub angle: f32,
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

pub type Clip = [u32; 4];

pub struct RenderParams {
    pub scale: Point2<f32>,
    pub opacity: f32,
    pub frame_time: f32,
    pub clip: Clip,
}

pub trait BaseWidget {
    fn calculate_layout(&mut self, engine: &mut engine::Engine, taffy: &mut Taffy) -> Node;
    fn render(
        &self,
        taffy: &Taffy,
        engine: &mut engine::Engine,
        input: &mut engine::ecs::resources::Input,
        state: &mut crate::state::State,
        parent_layout: &NodeLayout,
        params: &mut RenderParams,
    );
}
