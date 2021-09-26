use crate::world::{self::*, resources::input};
pub struct Input;

impl<'a> System<'a> for Input {
    type SystemData = (Write<'a, resources::Input>,);

    fn run(&mut self, mut input: Self::SystemData) {
        for (_, mouse) in input.0.mouse.iter_mut() {
            mouse.state = match mouse.state {
                resources::input::PressedState::Pressed => input::PressedState::Repeat,
                _ => mouse.state,
            };
        }
    }
}
