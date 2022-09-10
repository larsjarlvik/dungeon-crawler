use crate::ecs::{components, resources};
use bevy_ecs::prelude::*;

pub fn player(mut commands: Commands, mut player: NonSendMut<resources::Player>, query: Query<(Entity, &components::Sound)>) {
    for (entity, _sound) in query.iter() {
        player.play();
        commands.entity(entity).remove::<components::Sound>();
    }
}
