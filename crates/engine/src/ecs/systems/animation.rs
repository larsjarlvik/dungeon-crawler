use crate::{
    config,
    ecs::{components, resources},
};
use bevy_ecs::prelude::*;

pub fn animation(time: Res<resources::Time>, mut query: Query<(&mut components::Animations, &components::Model)>) {
    for (mut animation, model) in query.iter_mut() {
        for (_, channel) in animation.channels.iter_mut() {
            for animation in channel.queue.iter_mut() {
                animate_channel(model, animation, time.last_frame);
            }

            cleanup_channel(channel);
        }
    }
}

fn animate_channel(model: &components::Model, animation: &mut components::Animation, last_frame: f32) {
    let total_time = *model
        .animation_times
        .get(&animation.name.to_string())
        .unwrap_or_else(|| panic!("Could not find animation: {}", &animation.name));

    let speed = match animation.speed {
        components::AnimationSpeed::Original => 1.0,
        components::AnimationSpeed::Length(length) => total_time / length,
        components::AnimationSpeed::Speed(speed) => speed,
    };

    animation.elapsed = match animation.status {
        components::AnimationStatus::Default => {
            let new_elapsed = animation.elapsed + last_frame * speed;

            if new_elapsed >= total_time {
                animation.status = components::AnimationStatus::Stopped;
                total_time
            } else {
                new_elapsed
            }
        }
        components::AnimationStatus::Repeat => (animation.elapsed + last_frame * speed) % total_time,
        components::AnimationStatus::Stopped => total_time,
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
