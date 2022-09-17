use crate::file;
use crate::{config, ecs::resources};
use cgmath::*;
use rand::seq::SliceRandom;
use std::{collections::HashMap, io::Cursor};

pub struct SoundEffects {
    sounds: HashMap<String, Vec<Vec<u8>>>,
    sinks: HashMap<String, rodio::SpatialSink>,
    handle: rodio::OutputStreamHandle,
    _stream: rodio::OutputStream,
}

impl Default for SoundEffects {
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

impl SoundEffects {
    pub fn load(&mut self, sounds: &Vec<String>) {
        for key in sounds.iter() {
            let mut sounds = vec![];

            loop {
                let path = format!("sounds/{}-{}.ogg", key, sounds.len() + 1);
                if !file::exists(path.as_str()) {
                    break;
                }

                sounds.push(file::read_bytes(path.as_str()));
            }

            self.sounds.insert(key.clone(), sounds);
        }
    }

    pub fn play(&mut self, sink: &String, sound: &String, camera: &resources::Camera, position: Option<Vector3<f32>>) {
        let position = match position {
            Some(position) => position.into(),
            None => [0.0, 0.0, 0.0],
        };

        let left = camera.target + Vector3::new(-config::EAR_DISTANCE, config::EAR_HEIGHT, 0.0);
        let right = camera.target + Vector3::new(config::EAR_DISTANCE, config::EAR_HEIGHT, 0.0);

        let sink = self
            .sinks
            .entry(sink.clone())
            .or_insert(rodio::SpatialSink::try_new(&self.handle, position, left.into(), right.into()).unwrap());

        let sound = self
            .sounds
            .get(sound.into())
            .expect(format!("Could not find sound {}!", sound).as_str());

        let file = Cursor::new(sound.choose(&mut rand::thread_rng()).unwrap().clone());
        sink.append(rodio::Decoder::new(file).unwrap());
    }

    pub fn set_position(&mut self, sink: &String, camera: &resources::Camera, position: Vector3<f32>) {
        if let Some(sink) = &mut self.sinks.get(sink) {
            let left = camera.target + Vector3::new(-config::EAR_DISTANCE, config::EAR_HEIGHT, 0.0);
            let right = camera.target + Vector3::new(config::EAR_DISTANCE, config::EAR_HEIGHT, 0.0);

            sink.set_left_ear_position(left.into());
            sink.set_right_ear_position(right.into());
            sink.set_emitter_position(position.into());
        }
    }

    pub fn set_speed(&mut self, sink: &String, speed: f32) {
        if let Some(sink) = &mut self.sinks.get(sink) {
            sink.set_speed(speed)
        }
    }

    pub fn is_playing(&self, sink: &String) -> bool {
        let sink = self.sinks.get(sink);
        match sink {
            Some(sink) => sink.is_paused() || !sink.empty(),
            None => false,
        }
    }
}
