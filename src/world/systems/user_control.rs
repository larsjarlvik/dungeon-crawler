use crate::world::*;
use winit::event::VirtualKeyCode;

pub struct UserControl;

impl<'a> System<'a> for UserControl {
    type SystemData = (
        Read<'a, resources::Input>,
        Read<'a, resources::Time>,
        ReadStorage<'a, components::UserControl>,
        WriteStorage<'a, components::Transform>,
        WriteStorage<'a, components::Animation>,
    );

    fn run(&mut self, (input, time, user_control, mut transform, mut animation): Self::SystemData) {
        let elapsed = time.elapsed.as_secs_f32() * 3.0;

        for (_, transform, animation) in (&user_control, &mut transform, (&mut animation).maybe()).join() {
            let mut animate = false;

            if input.keys.contains(&VirtualKeyCode::W) {
                transform.translation.z -= elapsed;
                animate = true;
            }
            if input.keys.contains(&VirtualKeyCode::A) {
                transform.translation.x -= elapsed;
                animate = true;
            }
            if input.keys.contains(&VirtualKeyCode::S) {
                transform.translation.z += elapsed;
                animate = true;
            }
            if input.keys.contains(&VirtualKeyCode::D) {
                transform.translation.x += elapsed;
                animate = true;
            }

            if let Some(animation) = animation {
                animation.set_animation("walk", "legs", animate);
            }
        }
    }
}
