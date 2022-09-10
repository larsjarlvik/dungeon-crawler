use crate::ecs::{components, resources};
use bevy_ecs::prelude::*;

pub fn player(mut player: NonSendMut<resources::Player>, mut query: Query<&mut components::SoundEffects>) {
    for mut effects in query.iter_mut() {
        effects.sounds.retain(|sink, sound| {
            if !player.is_playing(sink) {
                player.play(sink, &sound.name);
            }

            false
        });
    }
}
