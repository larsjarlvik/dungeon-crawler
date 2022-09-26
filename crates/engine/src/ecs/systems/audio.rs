use crate::ecs::{components, resources};
use crate::interpolated_value::Interpolate;
use bevy_ecs::prelude::*;

pub fn player(
    mut player: NonSendMut<resources::SoundEffects>,
    time: Res<resources::Time>,
    camera: ResMut<resources::Camera>,
    mut query: Query<(&mut components::SoundEffects, Option<&components::Transform>)>,
) {
    for (mut effects, transform) in query.iter_mut() {
        effects.sounds.retain(|sink, sound| {
            let position = transform.map(|transform| transform.translation.get(time.alpha));

            if !sound.started {
                player.play(sink, &sound.name, &camera, position);
                sound.started = true;
            } else if let Some(position) = position {
                player.set_position(sink, &camera, position);
            }

            player.is_playing(sink)
        });
    }
}
