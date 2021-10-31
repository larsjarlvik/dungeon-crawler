use specs::{Component, VecStorage};

pub struct Text {
    pub text: String,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

impl Component for Text {
    type Storage = VecStorage<Self>;
}
