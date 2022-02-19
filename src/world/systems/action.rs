use crate::world::*;

pub struct Action;

impl<'a> System<'a> for Action {
    type SystemData = WriteStorage<'a, components::Action>;

    fn run(&mut self, mut action: Self::SystemData) {
        for action in (&mut action).join() {
            if let Some(action_changed) = action.set {
                if action_changed.elapsed().as_secs_f32() > action.length {
                    action.reset();
                }
            }
        }
    }
}
