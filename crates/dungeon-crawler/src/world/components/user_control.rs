use bevy_ecs::prelude::*;
use engine::ecs::resources::input::mouse::PressState;
use fxhash::FxHashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UiActionCode {
    Attack,
    Health,
}

#[derive(Component)]
pub struct UserControl {
    pub ui_actions: FxHashMap<UiActionCode, PressState>,
}

impl Default for UserControl {
    fn default() -> Self {
        Self {
            ui_actions: Default::default(),
        }
    }
}

impl UserControl {
    pub fn set_from_ui(&mut self, action_code: UiActionCode, pressed: bool) {
        match pressed {
            true => self
                .ui_actions
                .insert(action_code, PressState::Pressed(self.ui_actions.contains_key(&action_code))),
            false => self.ui_actions.remove(&action_code),
        };
    }
}
