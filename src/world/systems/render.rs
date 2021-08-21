use crate::world::{components, resources};
use cgmath::*;
use specs::*;

pub struct Render;

impl<'a> System<'a> for Render {
    type SystemData = (
        Read<'a, resources::Camera>,
        WriteStorage<'a, components::Render>,
        ReadStorage<'a, components::Position>,
    );

    fn run(&mut self, (camera, mut render, pos): Self::SystemData) {
        for (render, pos) in (&mut render, &pos).join() {
            render.view_proj = camera.view_proj;
            render.model_matrix = Matrix4::from_translation(pos.0);
        }
    }
}
