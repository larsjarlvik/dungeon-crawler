use crate::world::components;
use specs::*;

pub struct Bounce;

impl<'a> System<'a> for Bounce {
    type SystemData = (WriteStorage<'a, components::Position>, WriteStorage<'a, components::Bouce>);

    fn run(&mut self, (mut position, mut bounce): Self::SystemData) {
        for (position, bounce) in (&mut position, &mut bounce).join() {
            position.0 += bounce.0;

            if position.0.x > 40.0 || position.0.x < -40.0 {
                bounce.0.x = -bounce.0.x;
            }
            if position.0.y > 40.0 || position.0.y < -40.0 {
                bounce.0.y = -bounce.0.y;
            }
            if position.0.z > 40.0 || position.0.z < -40.0 {
                bounce.0.z = -bounce.0.z;
            }
        }
    }
}
