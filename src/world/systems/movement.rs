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
            transform.rotation.set(cgmath::Quaternion::from_angle_y(Rad(movement.direction)));

            if let Some(animation) = animation {
                if movement.velocity.abs() > 0.01 {
                    transform.translation.set(transform.translation.current + velocity_dir);

                    let animation_velocity = movement.velocity.abs() / 0.05;
                    if animation_velocity > 1.6 {
                        animation.set_animation("base", "run", animation_velocity);
                    } else {
                        animation.set_animation("base", "walk", animation_velocity);
                    }
                } else {
                    transform.translation.freeze();
                    animation.set_animation("base", "idle", 1.0);
                }
            }

            movement.velocity *= 0.9;
        }
    }
}
