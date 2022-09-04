use crate::world::*;
use bevy_ecs::prelude::*;
use cgmath::*;
use engine::ecs::components::AnimationSpeed;

pub fn movement(
    time: Res<engine::ecs::resources::Time>,
    mut query: Query<(
        &components::Movement,
        &mut engine::ecs::components::Transform,
        &mut engine::ecs::components::Animations,
    )>,
) {
    for (movement, mut transform, mut animations) in query.iter_mut() {
        let new_rot = cgmath::Quaternion::from_angle_y(Rad(movement.direction));
        let current_rot = transform.rotation.current;
        let current_trans = transform.translation.current;

        transform.rotation.set(current_rot.slerp(new_rot, 0.2), time.frame);
        transform.translation.set(current_trans + movement.to, time.frame);

        animations.set_speed("base", "walk", AnimationSpeed::Speed(movement.velocity / 0.04));
        animations.set_speed("base", "run", AnimationSpeed::Speed(movement.velocity * 0.4 / 0.04));
    }
}
