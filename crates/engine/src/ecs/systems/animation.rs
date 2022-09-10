use crate::{
    config,
    ecs::{
        components::{self, Sound},
        resources,
    },
};
use bevy_ecs::prelude::*;

pub fn animation(
    time: Res<resources::Time>,
    mut query: Query<(
        &mut components::Animations,
        &components::Model,
        Option<&mut components::SoundEffects>,
    )>,
) {
    for (mut animation, model, mut sound_effects) in query.iter_mut() {
        for (_, channel) in animation.channels.iter_mut() {
            for animation in channel.queue.iter_mut() {
                let total_time = *model
                    .animation_times
                    .get(&animation.name)
                    .expect(format!("Could not find animation: {}", &animation.name).as_str());

                let new_elapsed = animate_channel(animation, total_time, time.last_frame);

                // TODO: Separate system?
                if let Some(sound_effects) = &mut sound_effects {
                    if let Some(sound_effect) = model.animation_sound_effects.get(&animation.name) {
                        sound_effect
                            .timestamps
                            .iter()
                            .filter(|t| t > &&(animation.elapsed % total_time) && t <= &&(new_elapsed % total_time))
                            .for_each(|_t| {
                                let sink = format!("{}_{}_{}", &model.key, &animation.name, &sound_effect.name);
                                sound_effects.set(sink, Sound::new(sound_effect.name.as_str()));
                            });
                    }
                }

                animation.elapsed = new_elapsed;
            }

            cleanup_channel(channel);
        }
    }
}

fn animate_channel(animation: &mut components::Animation, total_time: f32, last_frame: f32) -> f32 {
    let speed = match animation.speed {
        components::AnimationSpeed::Original => 1.0,
        components::AnimationSpeed::Length(length) => total_time / length,
        components::AnimationSpeed::Speed(speed) => speed,
    };

    let new_elapsed = match animation.status {
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

    new_elapsed
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
