use crate::file;
use std::{collections::HashMap, io::Cursor};

pub struct Player {
    sounds: HashMap<String, Vec<u8>>,
    sinks: HashMap<String, rodio::Sink>,
    handle: rodio::OutputStreamHandle,
    _stream: rodio::OutputStream,
}

impl Default for Player {
    fn default() -> Self {
        let mut sounds = HashMap::new();

        sounds.insert("step".to_string(), file::read_bytes("sounds/steps-stone.ogg"));
        sounds.insert("attack".to_string(), file::read_bytes("sounds/attack.ogg"));

        sounds.insert("skeleton_step".to_string(), file::read_bytes("sounds/steps-skeleton.ogg"));
        sounds.insert("skeleton_attack".to_string(), file::read_bytes("sounds/attack-skeleton.ogg"));

        let (stream, handle) = rodio::OutputStream::try_default().unwrap();

        Self {
            sounds,
            sinks: HashMap::new(),
            handle,
            _stream: stream,
        }
    }
}

impl Player {
    pub fn play(&mut self, sink: &String, sound: &String) {
        let sink = self
            .sinks
            .entry(sink.clone())
            .or_insert(rodio::Sink::try_new(&self.handle).unwrap());

        let sound = self
            .sounds
            .get(sound.into())
            .expect(format!("Could not find sound {}!", sound).as_str());

        let file = Cursor::new(sound.clone());
        sink.append(rodio::Decoder::new(file).unwrap());
    }

    pub fn set_speed(&mut self, sink: &String, speed: f32) {
        let sink = self
            .sinks
            .entry(sink.clone())
            .or_insert(rodio::Sink::try_new(&self.handle).unwrap());

        sink.set_speed(speed)
    }

    pub fn is_playing(&self, sink: &String) -> bool {
        let sink = self.sinks.get(sink);
        match sink {
            Some(sink) => sink.is_paused() || !sink.empty(),
            None => false,
        }
    }
}
