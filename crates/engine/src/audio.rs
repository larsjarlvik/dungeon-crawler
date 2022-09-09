use std::{io::Cursor, thread};

use bevy_ecs::prelude::*;

use crate::{ecs, file};

pub struct Player {
    sound: Vec<u8>,
}

impl Player {
    pub fn new() -> Self {
        let sound = file::read_bytes("sounds/steps-stone.ogg");
        Self { sound }
    }

    pub fn play(&mut self, components: &mut bevy_ecs::world::World) {
        let mut entities = vec![];

        for (entity, _sound) in components.query::<(Entity, &ecs::components::Sound)>().iter(components) {
            let sound = self.sound.clone();
            thread::Builder::new()
                .spawn(move || {
                    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
                    let file = Cursor::new(sound);
                    let sink = handle.play_once(file).unwrap();
                    sink.sleep_until_end();
                })
                .unwrap();

            entities.push(entity);
        }

        for entity in entities {
            components.get_entity_mut(entity).unwrap().remove::<ecs::components::Sound>();
        }
    }
}
