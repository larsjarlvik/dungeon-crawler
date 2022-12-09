use crate::file;
use crate::{config, ecs::resources};
use cgmath::*;
use fxhash::FxHashMap;
use rand::seq::SliceRandom;
use std::io::Cursor;

pub struct SoundEffects {
    sounds: FxHashMap<String, Vec<Vec<u8>>>,
    sinks: FxHashMap<String, rodio::SpatialSink>,
    handle: rodio::OutputStreamHandle,
    _stream: rodio::OutputStream,
}

struct Positions {
    left: [f32; 3],
    right: [f32; 3],
    emitter: [f32; 3],
}

impl Default for SoundEffects {
    fn default() -> Self {
        let sounds = FxHashMap::default();
        let (stream, handle) = rodio::OutputStream::try_default().unwrap();

        Self {
            sounds,
            sinks: FxHashMap::default(),
            handle,
            _stream: stream,
        }
    }
}

impl SoundEffects {
    pub fn load(&mut self, sounds: &[String]) {
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

    fn get_left_right_emit(&self, camera: &resources::Camera, position: Option<Vector3<f32>>) -> Positions {
        let position = match position {
            Some(position) => (position * config::SOUND_DISTANCE_SCALE).into(),
            None => [0.0, 0.0, 0.0],
        };

        let left = camera.target * config::SOUND_DISTANCE_SCALE + Vector3::new(-config::EAR_DISTANCE, config::EAR_HEIGHT, 0.0);
        let right = camera.target * config::SOUND_DISTANCE_SCALE + Vector3::new(config::EAR_DISTANCE, config::EAR_HEIGHT, 0.0);

        Positions {
            left: left.into(),
            right: right.into(),
            emitter: position,
        }
    }

    pub fn play(&mut self, sink: &str, sound: &String, camera: &resources::Camera, position: Option<Vector3<f32>>) {
        let positions = self.get_left_right_emit(camera, position);

        let sink = self
            .sinks
            .entry(sink.into())
            .or_insert_with(|| rodio::SpatialSink::try_new(&self.handle, positions.emitter, positions.left, positions.right).unwrap());

        let sound = self.sounds.get(sound).unwrap_or_else(|| panic!("Could not find sound {}!", sound));

        let file = Cursor::new(sound.choose(&mut rand::thread_rng()).unwrap().clone());
        sink.append(rodio::Decoder::new(file).unwrap());
    }

    pub fn set_position(&mut self, sink: &String, camera: &resources::Camera, position: Vector3<f32>) {
        if let Some(sink) = &mut self.sinks.get(sink) {
            let positions = self.get_left_right_emit(camera, Some(position));

            sink.set_left_ear_position(positions.left);
            sink.set_right_ear_position(positions.right);
            sink.set_emitter_position(positions.emitter);
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
