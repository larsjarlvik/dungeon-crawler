use specs::{Component, VecStorage};
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CurrentAction {
    None,
    Attack(f32),
}

pub struct Action {
    pub current: CurrentAction,
    pub set: Option<Instant>,
    pub length: f32,
}

impl Component for Action {
    type Storage = VecStorage<Self>;
}

impl Action {
    pub fn new() -> Self {
        Self {
            current: CurrentAction::None,
            set: None,
            length: 0.0,
        }
    }

    pub fn set_action(&mut self, action: CurrentAction, min_action_time: f32) {
        if self.current != CurrentAction::None {
            return;
        }

        self.current = action;
        self.set = Some(Instant::now());
        self.length = min_action_time;
    }

    pub fn reset(&mut self) {
        self.current = CurrentAction::None;
    }
}
