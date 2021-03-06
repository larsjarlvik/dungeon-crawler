use crate::world::*;
use bevy_ecs::prelude::*;
use cgmath::*;

pub fn actions(
    mut commands: Commands,
    time: Res<engine::ecs::resources::Time>,
    mut query: Query<(
        &engine::ecs::components::Model,
        &components::Stats,
        &mut components::Movement,
        &mut engine::ecs::components::Transform,
        &mut engine::ecs::components::Animations,
        &mut components::Action,
        Option<&components::Weapon>,
        Option<&components::Collision>,
    )>,
) {
    for (model, stats, mut movement, mut transform, mut animation, mut action, weapon, collision) in query.iter_mut() {
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
                        animation.set_animation(
                            &model.model,
                            "base",
                            "run",
                            engine::ecs::components::AnimationSpeed::Speed(animation_velocity * 0.4),
                            engine::ecs::components::AnimationRunType::Repeat,
                        );
                    } else if animation_velocity > 0.3 {
                        animation.set_animation(
                            &model.model,
                            "base",
                            "walk",
                            engine::ecs::components::AnimationSpeed::Speed(animation_velocity),
                            engine::ecs::components::AnimationRunType::Repeat,
                        );
                    }
                } else {
                    transform.translation.freeze();
                    animation.set_animation(
                        &model.model,
                        "base",
                        "idle",
                        engine::ecs::components::AnimationSpeed::Original,
                        engine::ecs::components::AnimationRunType::Repeat,
                    );
                }

                movement.velocity *= 0.9;
            }
            components::CurrentAction::Attack => {
                transform.translation.freeze();
                transform.rotation.set(current_rot.slerp(new_rot, 0.2), time.frame);
                movement.velocity *= 0.85;

                if action.should_execute() {
                    if let Some(collision) = collision {
                        animation.set_animation(
                            &model.model,
                            "base",
                            "attack",
                            engine::ecs::components::AnimationSpeed::Length(action.length),
                            engine::ecs::components::AnimationRunType::Default,
                        );

                        if let Some(weapon) = weapon {
                            let dir = vec3(movement.direction.sin(), 0.0, movement.direction.cos()) * 0.5;

                            let base = stats.get_attack_damage();
                            let weapon = weapon.damage.clone();

                            commands.spawn().insert_bundle((
                                components::Attack {
                                    collision_key: collision.key.clone(),
                                    damage: (base.start * weapon.start)..(base.end * weapon.end),
                                },
                                engine::ecs::components::Transform::from_translation(transform.translation.current + dir),
                            ));
                        }
                    }
                }
            }
            components::CurrentAction::Hit => {
                if action.should_execute() {
                    animation.set_animation(
                        &model.model,
                        "base",
                        "hit",
                        engine::ecs::components::AnimationSpeed::Length(action.length),
                        engine::ecs::components::AnimationRunType::Default,
                    );
                }
            }
            components::CurrentAction::Death => {
                if action.should_execute() {
                    animation.set_animation(
                        &model.model,
                        "base",
                        "death",
                        engine::ecs::components::AnimationSpeed::Original,
                        engine::ecs::components::AnimationRunType::Default,
                    );
                }
            }
        }
    }
}
