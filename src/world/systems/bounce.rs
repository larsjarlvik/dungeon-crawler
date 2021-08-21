use crate::world::*;
use cgmath::*;

pub struct Bounce;

impl<'a> System<'a> for Bounce {
    type SystemData = (
        Read<'a, resources::Time>,
        WriteStorage<'a, components::Position>,
        WriteStorage<'a, components::Bouce>,
    );

    fn run(&mut self, (time, mut position, mut bounce): Self::SystemData) {
        for (position, bounce) in (&mut position, &mut bounce).join() {
            position.0 += vec3(
                bounce.0.x * time.elapsed.as_micros() as f32 * 0.000001,
                bounce.0.y * time.elapsed.as_micros() as f32 * 0.000001,
                bounce.0.z * time.elapsed.as_micros() as f32 * 0.000001,
            );

            if position.0.x > 4.0 || position.0.x < -4.0 {
                bounce.0.x = -bounce.0.x;
            }
            if position.0.y > 4.0 || position.0.y < -4.0 {
                bounce.0.y = -bounce.0.y;
            }
            if position.0.z > 4.0 || position.0.z < -4.0 {
                bounce.0.z = -bounce.0.z;
            }
        }
    }
}
