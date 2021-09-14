use crate::world::{components, resources};
use cgmath::*;
use specs::*;

pub struct Render;

impl<'a> System<'a> for Render {
    type SystemData = (
        Read<'a, resources::Camera>,
        WriteStorage<'a, components::Render>,
        ReadStorage<'a, components::Transform>,
    );

    fn run(&mut self, (camera, mut render, transform): Self::SystemData) {
        for (render, transform) in (&mut render, &transform).join() {
            render.view_proj = camera.view_proj;
            render.model_matrix = Matrix4::from_translation(transform.translation) * Matrix4::from_angle_y(Deg(transform.rotation));
        }
    }
}
