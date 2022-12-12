use crate::world::{components::UiActionCode, *};
use bevy_ecs::prelude::*;
use cgmath::*;
use engine::ecs::resources::Input;
use std::time::Duration;
use winit::event::VirtualKeyCode;

struct Target {
    position: Vector3<f32>,
    health: f32,
    max_health: f32,
    name: String,
}

pub fn user_control(
    mut commands: Commands,
    input: Res<Input>,
    mut query: ParamSet<(
        Query<(
            Entity,
            &engine::ecs::components::Transform,
            &mut components::Movement,
            &mut components::ActionExecutor,
            &mut components::Stats,
            &components::UserControl,
            Option<&components::Weapon>,
        )>,
        Query<(
            &components::Name,
            &components::Agressor,
            &engine::ecs::components::Transform,
            &components::Stats,
        )>,
    )>,
) {
    let rot = cgmath::Quaternion::from_angle_y(Deg(config::CAMERA_ROTATION));
    let targets: Vec<Target> = query
        .p1()
        .iter()
        .map(|(n, _, t, s)| Target {
            position: t.translation.current,
            health: s.health.current,
            max_health: s.get_base_health(),
            name: n.name.clone(),
        })
        .collect();

    for (entity, transform, mut movement, mut action, mut stats, user_control, weapon) in query.p0().iter_mut() {
        commands.entity(entity).remove::<components::DisplayTarget>();

        let mut targets: Vec<&Target> = targets
            .iter()
            .filter(|target| target.position.distance(transform.translation.current) < 8.0)
            .collect();

        targets.sort_by(|a, b| {
            let direction_a = a.position - transform.translation.current;
            let direction_a = direction_a.x.atan2(direction_a.z);

            let direction_b = b.position - transform.translation.current;
            let direction_b = direction_b.x.atan2(direction_b.z);

            (direction_a - movement.direction)
                .abs()
                .partial_cmp(&(direction_b - movement.direction).abs())
                .unwrap()
        });

        let focus_target = if let Some(target) = targets.first() {
            let direction = target.position - transform.translation.current;
            let direction = direction.x.atan2(direction.z);

            // ~90 degrees
            if (direction - movement.direction).abs() < 1.57 {
                Some(target)
            } else {
                None
            }
        } else {
            None
        };

        movement.target_velocity = 0.0;

        if let Some(joystick) = &input.joystick {
            if let Some((direction, strength)) = joystick.get_direction_strength(&input.mouse) {
                movement.target_velocity = strength * 8.0 / config::UPDATES_PER_SECOND;
                movement.towards(rot.rotate_vector(vec3(direction.x, 0.0, direction.y)));
            }
        }

        if let Some(target) = focus_target {
            commands.entity(entity).insert(components::DisplayTarget {
                name: target.name.clone(),
                current_health: target.health,
                max_health: target.max_health,
            });
        }

        if input.is_pressed(VirtualKeyCode::Space) || user_control.ui_actions.contains_key(&UiActionCode::Attack) {
            if let Some(target) = focus_target {
                let direction = target.position - transform.translation.current;
                let direction = direction.x.atan2(direction.z);
                movement.direction = direction;
            };

            if let Some(weapon) = weapon {
                action.set_action(components::Action::Attack, weapon.time * stats.get_attack_time(), 0.25);
            }
        }

        if (input.is_pressed(VirtualKeyCode::H) || user_control.ui_actions.contains_key(&UiActionCode::Health))
            && stats.health.changes.is_empty()
        {
            // TODO: Health value
            stats.health.changes.push(components::HealthChange::new(
                2.0,
                components::HealthChangeType::OverTime(Duration::from_secs(10)),
            ));
        }
    }
}
