use cgmath::vec3;

use crate::{utils::fbm, world::*};

pub struct Flicker;

impl<'a> System<'a> for Flicker {
    type SystemData = (
        WriteStorage<'a, components::Flicker>,
        WriteStorage<'a, components::Light>,
        WriteStorage<'a, components::Particle>,
    );

    fn run(&mut self, (mut flicker, mut light, mut particle): Self::SystemData) {
        for (flicker, light, particle) in (&mut flicker, (&mut light).maybe(), (&mut particle).maybe()).join() {
            let f = fbm(flicker.last, 3) * flicker.amount;
            flicker.last += flicker.speed;

            let amount = (flicker.amount / 2.0) + f;

            if let Some(light) = light {
                light.intensity.set(light.base_intensity - amount);

                let movement = ((flicker.amount - (flicker.amount / 2.0)) * 2.0) * 0.5;
                light.offset.set(
                    light.orig_offset
                        + vec3(
                            fbm(flicker.last, 3) * movement,
                            fbm(flicker.last, 3) * movement,
                            fbm(flicker.last, 3) * movement,
                        ),
                );
            }

            if let Some(particle) = particle {
                particle.strength.set(particle.base_strength - (amount * particle.base_strength));
            }
        }
    }
}
