use crate::world::*;

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (
        Read<'a, resources::Time>,
        WriteStorage<'a, components::Movement>,
        WriteStorage<'a, components::Transform>,
        WriteStorage<'a, components::Animation>,
    );

    fn run(&mut self, (time, mut movement, mut transform, mut animation): Self::SystemData) {
        let elapsed = time.elapsed.as_secs_f32();
        let acceleration = elapsed * 10.0;

        for (movement, transform, animation) in (&mut movement, &mut transform, (&mut animation).maybe()).join() {
            let velocity = movement.velocity * acceleration;

            movement.velocity *= 1.0 - (elapsed / 0.5);
            transform.translation += movement.velocity * elapsed;
            transform.rotation = velocity.x.atan2(velocity.z);

            if let Some(animation) = animation {
                let animate = movement.velocity.x.abs() > 0.1 || movement.velocity.z.abs() > 0.1;
                animation.set_animation("walk", "legs", animate);
            }
        }
    }
}
