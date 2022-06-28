use crate::ecs::{components, resources};
use bevy_ecs::prelude::*;

pub fn animation(time: Res<resources::Time>, mut query: Query<&mut components::Animations>) {
    for mut animation in query.iter_mut() {
        for (_, channel) in animation.channels.iter_mut() {
            animate_channel(&mut channel.current, time.last_frame);

            if let Some(previous) = &mut channel.prev {
                animate_channel(previous, time.last_frame);
            }
        }
    }
}

fn animate_channel(channel: &mut components::Animation, last_frame: f32) {
    channel.elapsed = match channel.run_type {
        components::AnimationRunType::Default => {
            let new_elapsed = channel.elapsed + last_frame * channel.speed;

            if new_elapsed >= channel.total_time {
                channel.run_type = components::AnimationRunType::Stopped;
                channel.total_time
            } else {
                new_elapsed
            }
        }
        components::AnimationRunType::Repeat => (channel.elapsed + last_frame * channel.speed) % channel.total_time,
        components::AnimationRunType::Stopped => channel.total_time,
    };
}
