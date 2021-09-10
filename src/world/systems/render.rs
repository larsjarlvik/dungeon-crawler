use crate::world::{components, resources};
use cgmath::*;
use specs::*;

pub struct Render;

impl<'a> System<'a> for Render {
    type SystemData = (
        Read<'a, resources::Camera>,
        WriteStorage<'a, components::Render>,
        ReadStorage<'a, components::Position>,
        ReadStorage<'a, components::Rotation>,
    );

    fn run(&mut self, (camera, mut render, pos, rot): Self::SystemData) {
        for (render, pos, rot) in (&mut render, &pos, &rot).join() {
            render.view_proj = camera.view_proj;
            render.model_matrix = Matrix4::from_angle_y(Deg(rot.0.y)) * Matrix4::from_translation(pos.0);
        }
    }
}
