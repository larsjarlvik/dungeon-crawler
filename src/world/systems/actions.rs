use crate::world::*;
use bevy_ecs::prelude::*;
use cgmath::*;

pub fn actions(
    mut commands: Commands,
    time: Res<resources::Time>,
    mut query: Query<(
        &mut components::Movement,
        &mut components::Transform,
        &mut components::Animations,
        &mut components::Action,
        Option<&components::Weapon>,
        Option<&components::Collision>,
    )>,
) {
    for (mut movement, mut transform, mut animation, mut action, weapon, collision) in query.iter_mut() {
        let new_rot = cgmath::Quaternion::from_angle_y(Rad(movement.direction));
        let current_rot = transform.rotation.current;
        let current_trans = transform.translation.current;

        if action.set.elapsed().as_secs_f32() >= action.length {
            action.reset();
        }

        match &action.current {
            components::CurrentAction::None => {
                transform.rotation.set(current_rot.slerp(new_rot, 0.2), time.frame);

                let velocity = vec2(movement.velocity_dir.x, movement.velocity_dir.z).distance(Vector2::zero());
                if velocity > 0.01 {
                    transform.translation.set(current_trans + movement.velocity_dir, time.frame);

                    let animation_velocity = velocity / 0.04;
                    if animation_velocity > 2.5 {
                        animation.set_animation("base", "run", animation_velocity * 0.4, components::AnimationRunType::Repeat);
                    } else if animation_velocity > 0.3 {
                        animation.set_animation("base", "walk", animation_velocity, components::AnimationRunType::Repeat);
                    }
                } else {
                    transform.translation.freeze();
                    animation.set_animation("base", "idle", 1.0, components::AnimationRunType::Repeat);
                }

                movement.velocity *= 0.9;
            }
            components::CurrentAction::Attack => {
                transform.translation.freeze();
                transform.rotation.set(current_rot.slerp(new_rot, 0.2), time.frame);
                movement.velocity *= 0.85;

                if action.should_execute() {
                    if let Some(collision) = collision {
                        animation.set_animation("base", "attack", 2.2, components::AnimationRunType::Default);

                        if let Some(weapon) = weapon {
                            let dir = vec3(movement.direction.sin(), 0.0, movement.direction.cos()) * 0.5;

                            commands.spawn().insert_bundle((
                                components::Attack {
                                    collision_key: collision.key.clone(),
                                    min: weapon.min,
                                    max: weapon.max,
                                },
                                components::Transform::from_translation(transform.translation.current + dir),
                            ));
                        }
                    }
                }
            }
            components::CurrentAction::Hit => {
                if action.should_execute() {
                    animation.set_animation("base", "hit", 2.2, components::AnimationRunType::Default);
                }
            }
            components::CurrentAction::Death => {
                if action.should_execute() {
                    animation.set_animation("base", "death", 1.0, components::AnimationRunType::Default);
                }
            }
        }
    }
}
