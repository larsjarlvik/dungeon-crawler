use crate::world::{components::Action, *};
use bevy_ecs::prelude::*;
use cgmath::*;
use engine::{
    ecs::components::{AnimationSpeed, AnimationStatus},
    utils,
};

pub fn actions(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &components::Stats,
        &engine::ecs::components::Transform,
        &mut components::Movement,
        &mut engine::ecs::components::Animations,
        &mut components::ActionExecutor,
        Option<&components::Weapon>,
        Option<&components::Collision>,
    )>,
) {
    for (entity, stats, transform, mut movement, mut animation, mut action, weapon, collision) in query.iter_mut() {
        match &action.get() {
            Action::None => {
                movement.velocity = vec1(movement.velocity).lerp(vec1(movement.target_velocity), 0.1).x;

                if movement.velocity.abs() <= 0.01 {
                    movement.velocity = 0.0;
                    animation.set_animation("base", "idle", AnimationSpeed::Original, AnimationStatus::Repeat);
                } else {
                    if movement.velocity > 0.08 {
                        animation.set_animation("base", "run", AnimationSpeed::Speed(1.0), AnimationStatus::Repeat);
                    } else {
                        animation.set_animation("base", "walk", AnimationSpeed::Speed(1.0), AnimationStatus::Repeat);
                    }
                }
            }
            Action::Attack => {
                movement.velocity *= 0.85;

                if movement.velocity < 0.01 {
                    animation.set_animation("base", "attack", AnimationSpeed::Length(action.length), AnimationStatus::Repeat);
                }

                if action.should_execute() {
                    if let Some(collision) = collision {
                        if let Some(weapon) = weapon {
                            let dir = vec3(movement.direction.sin(), 0.0, movement.direction.cos()) * 0.5;
                            let damage_base = stats.get_attack_damage();
                            let damage_weapon = weapon.damage.clone();

                            commands.spawn().insert_bundle((
                                components::Attack {
                                    collision_key: collision.key.clone(),
                                    damage: (damage_base.start * damage_weapon.start)..(damage_base.end * damage_weapon.end),
                                },
                                engine::ecs::components::Transform::from_translation(transform.translation.current + dir),
                            ));
                        }
                    }
                }
            }
            Action::Hit => {
                movement.velocity *= 0.6;

                if action.should_execute() {
                    utils::vibrate(engine::config::VIBRATION_LENGTH * 2.0);
                    animation.set_animation("base", "hit", AnimationSpeed::Length(action.length), AnimationStatus::Default);
                }
            }
            Action::Death => {
                movement.velocity *= 0.0;

                if action.should_execute() {
                    utils::vibrate(engine::config::VIBRATION_LENGTH * 2.0);
                    commands.entity(entity).remove::<components::Movement>();
                    animation.set_animation("base", "death", AnimationSpeed::Original, AnimationStatus::Default);
                }
            }
        }

        movement.to = vec3(movement.direction.sin(), 0.0, movement.direction.cos()) * movement.velocity;
    }
}
