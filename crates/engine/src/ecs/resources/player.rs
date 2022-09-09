use std::{fs::File, io::Cursor};

use crate::file;

pub struct Player {}

impl Default for Player {
    fn default() -> Self {
        Self {}
    }
}

impl Player {
    pub fn play(&mut self, sound: &String) {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        let file = Cursor::new(file::read_bytes("sounds/steps-stone.ogg"));
        sink.append(rodio::Decoder::new(file).unwrap());

        sink.sleep_until_end();
    }
}
