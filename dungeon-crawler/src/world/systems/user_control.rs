use std::time::Duration;

use crate::world::*;
use bevy_ecs::prelude::*;
use cgmath::*;
use winit::event::VirtualKeyCode;

pub fn user_control(
    input: Res<resources::Input>,
    mut query: QuerySet<(
        QueryState<
            (
                &engine::ecs::components::Transform,
                &mut components::Movement,
                &mut components::Action,
                &mut components::Stats,
                Option<&components::Weapon>,
            ),
            With<components::UserControl>,
        >,
        QueryState<(&components::Agressor, &engine::ecs::components::Transform)>,
    )>,
) {
    let rot = cgmath::Quaternion::from_angle_y(Deg(config::CAMERA_ROTATION));
    let targets: Vec<Vector3<f32>> = query.q1().iter().map(|(_, t)| t.translation.current.clone()).collect();

    for (transform, mut movement, mut action, mut stats, weapon) in query.q0().iter_mut() {
        if let Some(joystick) = &input.joystick {
            movement.velocity = joystick.strength * 8.0 / config::UPDATES_PER_SECOND;

            if let Some(current) = joystick.current {
                movement.towards(rot.rotate_vector(vec3(current.x, 0.0, current.y)));
            }
        }

        if input.is_pressed(VirtualKeyCode::Space) || input.ui.contains_key(&resources::input::UiActionCode::Attack) {
            let closest_target: Option<&Vector3<f32>> = targets.iter().filter(|t| t.distance(transform.translation.current) < 1.0).next();

            if let Some(closest_target) = closest_target {
                let direction = closest_target - transform.translation.current;
                let direction = direction.x.atan2(direction.z);

                // ~90 degrees
                if (direction - movement.direction).abs() < 1.57 {
                    movement.direction = direction;
                }
            };

            if let Some(weapon) = weapon {
                action.set_action(
                    components::CurrentAction::Attack,
                    (weapon.time + stats.get_attack_time()) / 2.0,
                    (weapon.time + stats.get_attack_time()) / 2.0 * 0.3,
                    false,
                );
            }
        }

        if input.is_pressed(VirtualKeyCode::H) || input.ui.contains_key(&resources::input::UiActionCode::Health) {
            if stats.health.changes.len() == 0 {
                // TODO: Health value
                stats.health.changes.push(components::HealthChange::new(
                    2.0,
                    components::HealthChangeType::OverTime(Duration::from_secs(10)),
                ));
            }
        }
    }
}
