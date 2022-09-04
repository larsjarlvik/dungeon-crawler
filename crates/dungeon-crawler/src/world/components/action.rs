use crate::config;
use bevy_ecs::prelude::*;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Action {
    None,
    Attack,
    Hit,
    Death,
}

#[derive(Component)]
pub struct ActionExecutor {
    current: Action,
    pub set: Instant,
    pub length: f32,
    pub activation_time: f32,
    pub executed: bool,
}

impl ActionExecutor {
    pub fn new() -> Self {
        Self {
            current: Action::None,
            set: Instant::now(),
            length: 0.0,
            activation_time: 0.0,
            executed: false,
        }
    }

    pub fn get(&self) -> Action {
        if self.set.elapsed().as_secs_f32() <= self.length + config::time_step().as_secs_f32() {
            self.current
        } else {
            Action::None
        }
    }

    pub fn set_action(&mut self, action: Action, length: f32, activation_time: f32) {
        if !is_forced(action) && self.set.elapsed().as_secs_f32() <= self.length {
            return;
        }

        self.current = action;
        self.set = Instant::now();
        self.length = length;
        self.activation_time = length * activation_time;
        self.executed = false;
    }

    pub fn should_execute(&mut self) -> bool {
        let activated = self.set.elapsed().as_secs_f32() >= self.activation_time;
        if activated && !self.executed {
            self.executed = true;
            return true;
        }

        false
    }
}

fn is_forced(action: Action) -> bool {
    match action {
        Action::None | Action::Attack => false,
        Action::Hit | Action::Death => true,
    }
}
