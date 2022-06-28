pub enum State {
    Running,
    Menu,
}

pub struct GameState {
    pub state: State,
}

impl Default for GameState {
    fn default() -> Self {
        Self { state: State::Running }
    }
}
