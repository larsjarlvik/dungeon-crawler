use crate::world::*;
use cgmath::*;
use winit::event::VirtualKeyCode;

pub struct UserControl;

impl<'a> System<'a> for UserControl {
    type SystemData = (
        Read<'a, resources::Input>,
        ReadStorage<'a, components::UserControl>,
        WriteStorage<'a, components::Movement>,
        WriteStorage<'a, components::Action>,
    );

    fn run(&mut self, (input, _, mut movement, mut action): Self::SystemData) {
        let rot = cgmath::Quaternion::from_angle_y(Deg(config::CAMERA_ROTATION));

        for (movement, action) in (&mut movement, &mut action).join() {
            if let Some(joystick) = &input.joystick {
                if let Some(current) = joystick.current {
                    movement.towards(rot.rotate_vector(vec3(current.x, 0.0, current.y)));
                }
            }

            if input.is_pressed(VirtualKeyCode::Space) || input.ui.contains_key(&resources::input::UiActionCode::Attack) {
                action.set_action(components::CurrentAction::Attack(2.0), 1.0); // TODO: Weapon damage
            } else {
                if let Some(joystick) = &input.joystick {
                    if action.current == components::CurrentAction::None {
                        movement.velocity = joystick.strength * 8.0 / config::UPDATES_PER_SECOND;
                    }
                }
            }
        }
    }
}
