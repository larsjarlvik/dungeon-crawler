use crate::world::*;
use cgmath::*;

pub struct UserControl;

impl<'a> System<'a> for UserControl {
    type SystemData = (
        Read<'a, resources::Input>,
        ReadStorage<'a, components::UserControl>,
        WriteStorage<'a, components::Movement>,
    );

    fn run(&mut self, (input, _, mut movement): Self::SystemData) {
        for movement in (&mut movement).join() {
            if input.mouse.pressed {
                movement.towards(vec3(input.mouse.relative.x, 0.0, input.mouse.relative.y));
                movement.velocity = 3.0 / config::UPDATES_PER_SECOND;
            } else {
                movement.velocity = 0.0;
            }
        }
    }
}
