use bevy_ecs::prelude::*;

use crate::ecs::{components, resources};

pub fn audio(mut commands: Commands, mut player: ResMut<resources::Player>, query: Query<(Entity, &components::Sound)>) {
    for (entity, sound) in query.iter() {
        player.play(&sound.name);

        commands.entity(entity).remove::<components::Sound>();
    }
}
