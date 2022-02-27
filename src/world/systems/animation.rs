use crate::world::*;
use bevy_ecs::prelude::*;

pub fn animation(time: Res<resources::Time>, mut query: Query<&mut components::Animations>) {
    for mut animation in query.iter_mut() {
        for (_, channel) in animation.channels.iter_mut() {
            channel.current.elapsed += time.frame_time * channel.current.speed;
            if let Some(previous) = &mut channel.prev {
                previous.elapsed += time.frame_time * previous.speed;
            }
        }
    }
}
