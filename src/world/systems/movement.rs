use crate::world::*;
use cgmath::*;

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (
        WriteStorage<'a, components::Movement>,
        WriteStorage<'a, components::Transform>,
        WriteStorage<'a, components::Animation>,
    );

    fn run(&mut self, (mut movement, mut transform, mut animation): Self::SystemData) {
        for (movement, transform, animation) in (&mut movement, &mut transform, (&mut animation).maybe()).join() {
            let velocity_dir = vec3(movement.direction.sin(), 0.0, movement.direction.cos()) * movement.velocity;

            transform.translation.set(transform.translation.current + velocity_dir);
            transform.rotation.set(cgmath::Quaternion::from_angle_y(Rad(movement.direction)));

            if let Some(animation) = animation {
                let animate = movement.velocity.abs() > 0.01;
                animation.set_animation("walk", "legs", animate);
            }

            movement.velocity *= 0.9;
        }
    }
}
