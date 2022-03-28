use crate::world::*;
use bevy_ecs::prelude::*;
use cgmath::*;
use winit::event::VirtualKeyCode;

pub fn user_control(
    input: Res<resources::Input>,
    mut query: Query<(&components::UserControl, &mut components::Movement, &mut components::Action)>,
) {
    let rot = cgmath::Quaternion::from_angle_y(Deg(config::CAMERA_ROTATION));

    for (_, mut movement, mut action) in query.iter_mut() {
        if let Some(joystick) = &input.joystick {
            if let Some(current) = joystick.current {
                movement.towards(rot.rotate_vector(vec3(current.x, 0.0, current.y)));
            }
        }

        if input.is_pressed(VirtualKeyCode::Space) || input.ui.contains_key(&resources::input::UiActionCode::Attack) {
            action.set_action(components::CurrentAction::Attack(2.0), 1.0, 0.35, false);
        } else {
            if let Some(joystick) = &input.joystick {
                if action.current == components::CurrentAction::None {
                    movement.velocity = joystick.strength * 8.0 / config::UPDATES_PER_SECOND;
                }
            }
        }
    }
}
