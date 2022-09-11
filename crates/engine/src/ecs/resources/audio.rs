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
        let mut sounds = HashMap::new();

        sounds.insert(
            "human-steps".to_string(),
            vec![
                file::read_bytes("sounds/human-steps-1.ogg"),
                file::read_bytes("sounds/human-steps-2.ogg"),
            ],
        );
        sounds.insert(
            "human-attack".to_string(),
            vec![
                file::read_bytes("sounds/human-attack-1.ogg"),
                file::read_bytes("sounds/human-attack-2.ogg"),
            ],
        );
        sounds.insert("human-hit".to_string(), vec![file::read_bytes("sounds/human-hit-1.ogg")]);
        sounds.insert("human-death".to_string(), vec![file::read_bytes("sounds/human-death-1.ogg")]);

        sounds.insert(
            "skeleton-steps".to_string(),
            vec![
                file::read_bytes("sounds/skeleton-steps-1.ogg"),
                file::read_bytes("sounds/skeleton-steps-2.ogg"),
            ],
        );
        sounds.insert(
            "skeleton-attack".to_string(),
            vec![file::read_bytes("sounds/skeleton-attack-1.ogg")],
        );
        sounds.insert("skeleton-hit".to_string(), vec![file::read_bytes("sounds/skeleton-hit-1.ogg")]);
        sounds.insert(
            "skeleton-death".to_string(),
            vec![file::read_bytes("sounds/skeleton-death-1.ogg")],
        );

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
