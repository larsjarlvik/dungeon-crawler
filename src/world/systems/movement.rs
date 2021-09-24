use crate::world::*;
use cgmath::*;

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (
        WriteStorage<'a, components::Movement>,
        WriteStorage<'a, components::Transform>,
        WriteStorage<'a, components::Animations>,
    );

    fn run(&mut self, (mut movement, mut transform, mut animation): Self::SystemData) {
        for (movement, transform, animation) in (&mut movement, &mut transform, (&mut animation).maybe()).join() {
            let velocity_dir = vec3(movement.direction.sin(), 0.0, movement.direction.cos()) * movement.velocity;

            transform.translation.set(transform.translation.current + velocity_dir);
            transform.rotation.set(cgmath::Quaternion::from_angle_y(Rad(movement.direction)));

            if let Some(animation) = animation {
                let walking = movement.velocity.abs() > 0.01;
                if walking {
                    animation.set_animation("base", "walk");
                } else {
                    animation.set_animation("base", "idle");
                }
            }

            movement.velocity *= 0.9;
        }
    }
}
