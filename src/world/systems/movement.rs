use crate::world::*;

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (
        WriteStorage<'a, components::Movement>,
        WriteStorage<'a, components::Transform>,
        WriteStorage<'a, components::Animation>,
    );

    fn run(&mut self, (mut movement, mut transform, mut animation): Self::SystemData) {
        let elapsed = config::time_step().as_secs_f32();
        let acceleration = elapsed * 10.0;

        for (movement, transform, animation) in (&mut movement, &mut transform, (&mut animation).maybe()).join() {
            let velocity = movement.velocity * acceleration;

            movement.velocity *= 1.0 - (elapsed / 0.5);
            transform.set_translation(transform.translation.current + movement.velocity * elapsed);
            transform.set_rotation(velocity.x.atan2(velocity.z));

            if let Some(animation) = animation {
                let animate = movement.velocity.x.abs() > 0.1 || movement.velocity.z.abs() > 0.1;
                animation.set_animation("walk", "legs", animate);
            }
        }
    }
}
