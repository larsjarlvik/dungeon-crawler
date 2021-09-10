use crate::world::*;
use cgmath::*;

pub struct Rotate;

impl<'a> System<'a> for Rotate {
    type SystemData = (Read<'a, resources::Time>, WriteStorage<'a, components::Rotation>);

    fn run(&mut self, (time, mut rotation): Self::SystemData) {
        for rotation in (&mut rotation).join() {
            rotation.0 += vec3(0.0, time.elapsed.as_micros() as f32 * 0.00003, 0.0);
        }
    }
}
