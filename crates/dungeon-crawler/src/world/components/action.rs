use bevy_ecs::prelude::*;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CurrentAction {
    None,
    Attack,
    Hit,
    Death,
}

#[derive(Component)]
pub struct Action {
    pub current: CurrentAction,
    pub set: Instant,
    pub length: f32,
    pub activation_time: f32,
    pub executed: bool,
}

impl Action {
    pub fn new() -> Self {
        Self {
            current: CurrentAction::None,
            set: Instant::now(),
            length: 0.0,
            activation_time: 0.0,
            executed: false,
        }
    }

    pub fn set_action(&mut self, action: CurrentAction, min_action_time: f32, activation_time: f32, force: bool) {
        if !force && self.current != CurrentAction::None {
            return;
        }

        self.current = action;
        self.set = Instant::now();
        self.length = min_action_time;
        self.activation_time = min_action_time * activation_time;
        self.executed = false;
    }

    pub fn reset(&mut self) {
        self.current = CurrentAction::None;
    }

    pub fn should_execute(&mut self) -> bool {
        let activated = self.set.elapsed().as_secs_f32() > self.activation_time;
        if activated && !self.executed {
            self.executed = true;
            return true;
        }

        false
    }
}
