use crate::file;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use oddio::*;

pub struct Player {}

impl Default for Player {
    fn default() -> Self {
        Self {}
    }
}

impl Player {
    pub fn play(&mut self, sound: &String) {}
}
