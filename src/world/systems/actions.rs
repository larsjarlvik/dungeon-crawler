use crate::world::*;
use bevy_ecs::prelude::*;
use cgmath::*;

pub fn actions(
    mut commands: Commands,
    time: Res<resources::Time>,
    mut movement_query: Query<(
        &mut components::Movement,
        &mut components::Transform,
        &mut components::Animations,
        &mut components::Action,
        &components::Collision,
    )>,
) {
    for (mut movement, mut transform, mut animation, mut action, collision) in movement_query.iter_mut() {
        let new_rot = cgmath::Quaternion::from_angle_y(Rad(movement.direction));
        let current_rot = transform.rotation.current;
        let current_trans = transform.translation.current;

        if action.set.elapsed().as_secs_f32() > action.length {
            action.reset();
        }

        match action.current {
            components::CurrentAction::None => {
                transform.rotation.set(current_rot.slerp(new_rot, 0.2), time.frame);

                let velocity = vec2(movement.velocity_dir.x, movement.velocity_dir.z).distance(Vector2::zero());
                if velocity > 0.01 {
                    transform.translation.set(current_trans + movement.velocity_dir, time.frame);

                    let animation_velocity = velocity / 0.04;
                    if animation_velocity > 2.5 {
                        animation.set_animation("base", "run", animation_velocity * 0.4, true);
                    } else if animation_velocity > 0.3 {
                        animation.set_animation("base", "walk", animation_velocity, true);
                    }
                } else {
                    transform.translation.freeze();
                    animation.set_animation("base", "idle", 1.0, true);
                }

                movement.velocity *= 0.9;
            }
            components::CurrentAction::Attack => {
                transform.rotation.set(current_rot.slerp(new_rot, 0.2), time.frame);
                animation.set_animation("base", "attack", 2.0, false);
                movement.velocity *= 0.85;

                if action.should_execute() {
                    // TODO: Range
                    let dir = vec3(movement.direction.sin(), 0.0, movement.direction.cos()) * 0.5;

                    commands.spawn().insert_bundle((
                        components::Attack {
                            collision_key: collision.key.clone(),
                            damage: 1.0,
                        },
                        components::Transform::from_translation(transform.translation.current + dir),
                    ));
                }
            }
            components::CurrentAction::Hit => {
                animation.set_animation("base", "hit", 2.0, false);
            }
            components::CurrentAction::Death => {
                animation.set_animation("base", "death", 2.0, false);
            }
        }
    }
}
