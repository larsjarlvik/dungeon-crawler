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
            if let Some(joystick) = &input.joystick {
                movement.towards(vec3(joystick.current.x, 0.0, joystick.current.y));
                movement.velocity = joystick.strength * 6.0 / config::UPDATES_PER_SECOND;
            }
        }
    }
}
