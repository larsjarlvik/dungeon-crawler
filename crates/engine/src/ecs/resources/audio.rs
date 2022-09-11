use rand::seq::SliceRandom;

use crate::file;
use std::{collections::HashMap, io::Cursor};

pub struct Player {
    sounds: HashMap<String, Vec<Vec<u8>>>,
    sinks: HashMap<String, rodio::Sink>,
    handle: rodio::OutputStreamHandle,
    _stream: rodio::OutputStream,
}

impl Default for Player {
    fn default() -> Self {
        let sounds = HashMap::new();
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
    pub fn load_sounds(&mut self, sounds: &Vec<String>) {
        for key in sounds.iter() {
            let mut effect_sounds = vec![];

            loop {
                let path = format!("sounds/{}-{}.ogg", key, effect_sounds.len() + 1);
                if !file::exists(path.as_str()) {
                    break;
                }

                effect_sounds.push(file::read_bytes(path.as_str()));
            }

            self.sounds.insert(key.clone(), effect_sounds);
        }
    }

    pub fn play(&mut self, sink: &String, sound: &String) {
        let sink = self
            .sinks
            .entry(sink.clone())
            .or_insert(rodio::Sink::try_new(&self.handle).unwrap());

        let sound = self
            .sounds
            .get(sound.into())
            .expect(format!("Could not find sound {}!", sound).as_str());

        let file = Cursor::new(sound.choose(&mut rand::thread_rng()).unwrap().clone());
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
