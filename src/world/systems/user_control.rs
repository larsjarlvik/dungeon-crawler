use crate::world::*;
use bevy_ecs::prelude::*;
use cgmath::*;
use winit::event::VirtualKeyCode;

pub fn user_control(
    input: Res<resources::Input>,
    mut query: QuerySet<(
        QueryState<(
            &components::UserControl,
            &components::Transform,
            &mut components::Movement,
            &mut components::Action,
        )>,
        QueryState<(&components::Agressor, &components::Transform)>,
    )>,
) {
    let rot = cgmath::Quaternion::from_angle_y(Deg(config::CAMERA_ROTATION));
    let targets: Vec<Vector3<f32>> = query.q1().iter().map(|(_, t)| t.translation.current.clone()).collect();

    for (_, transform, mut movement, mut action) in query.q0().iter_mut() {
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

            action.set_action(components::CurrentAction::Attack, 1.0, 0.35, false);
        }
    }
}
