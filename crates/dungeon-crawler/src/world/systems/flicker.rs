use crate::{utils::fbm, world::*};
use bevy_ecs::prelude::*;
use cgmath::vec3;

pub fn flicker(
    time: Res<engine::ecs::resources::Time>,
    mut query: Query<(
        &mut components::Flicker,
        Option<&mut engine::ecs::components::Light>,
        Option<&mut engine::ecs::components::Particle>,
    )>,
) {
    for (mut flicker, mut light, mut particle) in query.iter_mut() {
        let f = fbm(flicker.last, 3) * flicker.amount;
        flicker.last += flicker.speed;

        let amount = (flicker.amount / 2.0) + f;

        if let Some(light) = &mut light {
            let intensity = light.base_intensity;
            let orig_offset = light.orig_offset;

            light.intensity.set(intensity - amount, time.frame);

            let movement = ((flicker.amount - (flicker.amount / 2.0)) * 2.0) * 0.5;
            light.offset.set(
                orig_offset
                    + vec3(
                        fbm(flicker.last, 3) * movement,
                        fbm(flicker.last, 3) * movement,
                        fbm(flicker.last, 3) * movement,
                    ),
                time.frame,
            );
        }

        if let Some(particle) = &mut particle {
            let base_strength = particle.base_strength;
            particle.strength.set(base_strength - (amount * base_strength), time.frame);
        }
    }
}
