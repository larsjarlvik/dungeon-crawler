use crate::ecs::{components, resources};
use bevy_ecs::prelude::*;

pub fn animation(time: Res<resources::Time>, mut query: Query<(&mut components::Animations, &components::Model)>) {
    for (mut animation, model) in query.iter_mut() {
        for (_, channel) in animation.channels.iter_mut() {
            for animation in channel.queue.iter_mut() {
                animate_channel(&model, animation, time.last_frame);
            }
        }
    }
}

fn animate_channel(model: &components::Model, channel: &mut components::Animation, last_frame: f32) {
    let total_time = *model
        .animation_times
        .get(&channel.name.to_string())
        .expect(format!("Could not find animation: {}", &channel.name).as_str());

    channel.elapsed = match channel.run_type {
        components::AnimationRunType::Default => {
            let new_elapsed = channel.elapsed + last_frame * channel.speed;

            if new_elapsed >= total_time {
                channel.run_type = components::AnimationRunType::Stopped;
                total_time
            } else {
                new_elapsed
            }
        }
        components::AnimationRunType::Repeat => (channel.elapsed + last_frame * channel.speed) % total_time,
        components::AnimationRunType::Stopped => total_time,
    };
}
