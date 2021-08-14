use crate::world::components;
use cgmath::*;
use specs::*;

pub struct Render;

impl<'a> System<'a> for Render {
    type SystemData = (
        WriteStorage<'a, components::Render>,
        ReadStorage<'a, components::Camera>,
        ReadStorage<'a, components::Position>,
    );

    fn run(&mut self, (mut render, camera, pos): Self::SystemData) {
        for (render, camera, pos) in (&mut render, &camera, &pos).join() {
            let view = cgmath::Matrix4::look_at_rh(camera.eye, camera.target, camera.up);
            let proj = cgmath::perspective(cgmath::Deg(camera.fovy), camera.aspect, camera.znear, camera.zfar);

            render.view_proj = proj * view;
            render.model_matrix = Matrix4::from_translation(pos.0);
        }
    }
}
