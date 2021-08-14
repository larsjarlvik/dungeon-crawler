use cgmath::*;
use specs::{Component, VecStorage};

pub struct Render {
    pub view_proj: Matrix4<f32>,
    pub model_matrix: Matrix4<f32>,
}

impl Default for Render {
    fn default() -> Self {
        Render {
            view_proj: Matrix4::identity(),
            model_matrix: Matrix4::identity(),
        }
    }
}

impl Component for Render {
    type Storage = VecStorage<Self>;
}
