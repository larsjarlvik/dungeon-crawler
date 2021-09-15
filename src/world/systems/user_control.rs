use crate::world::*;
use cgmath::*;
use winit::event::VirtualKeyCode;

pub struct UserControl;

impl<'a> System<'a> for UserControl {
    type SystemData = (
        Read<'a, resources::Input>,
        ReadStorage<'a, components::UserControl>,
        WriteStorage<'a, components::Movement>,
    );

    fn run(&mut self, (input, _, mut movement): Self::SystemData) {
        for movement in (&mut movement).join() {
            if input.keys.contains(&VirtualKeyCode::W) {
                movement.towards(vec3(0.0, 0.0, -1.0));
            }
            if input.keys.contains(&VirtualKeyCode::A) {
                movement.towards(vec3(-1.0, 0.0, 0.0));
            }
            if input.keys.contains(&VirtualKeyCode::S) {
                movement.towards(vec3(0.0, 0.0, 1.0));
            }
            if input.keys.contains(&VirtualKeyCode::D) {
                movement.towards(vec3(1.0, 0.0, 0.0));
            }
        }
    }
}
