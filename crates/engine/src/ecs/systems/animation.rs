use crate::{
    config,
    ecs::{components, resources},
};
use bevy_ecs::prelude::*;

pub fn animation(time: Res<resources::Time>, mut query: Query<(&mut components::Animations, &components::Model)>) {
    for (mut animation, model) in query.iter_mut() {
        for (_, channel) in animation.channels.iter_mut() {
            for animation in channel.queue.iter_mut() {
                animate_channel(&model, animation, time.last_frame);
            }

            cleanup_channel(channel);
        }
    }
}

fn animate_channel(model: &components::Model, animation: &mut components::Animation, last_frame: f32) {
    let total_time = *model
        .animation_times
        .get(&animation.name.to_string())
        .expect(format!("Could not find animation: {}", &animation.name).as_str());

    animation.elapsed = match animation.run_type {
        components::AnimationRunType::Default => {
            let new_elapsed = animation.elapsed + last_frame * animation.speed;

            if new_elapsed >= total_time {
                animation.run_type = components::AnimationRunType::Stopped;
                total_time
            } else {
                new_elapsed
            }
        }
        components::AnimationRunType::Repeat => (animation.elapsed + last_frame * animation.speed) % total_time,
        components::AnimationRunType::Stopped => total_time,
    };
}

fn cleanup_channel(channel: &mut components::Channel) {
    channel.queue = channel
        .queue
        .iter()
        .enumerate()
        .filter(|(index, _)| {
            if let Some(next) = channel.queue.get(index + 1) {
                next.started.elapsed().as_secs_f32() < config::ANIMATION_BLEND_SECONDS
            } else {
                true
            }
        })
        .map(|(_, animation)| animation)
        .cloned()
        .collect();
}
