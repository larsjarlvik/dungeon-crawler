use crate::{utils::fbm, world::*};

pub struct Flicker;

impl<'a> System<'a> for Flicker {
    type SystemData = (WriteStorage<'a, components::Flicker>, WriteStorage<'a, components::Light>, WriteStorage<'a, components::Particle>);

    fn run(&mut self, (mut flicker, mut light, mut particle): Self::SystemData) {
        for (flicker, light, particle) in (&mut flicker, (&mut light).maybe(), (&mut particle).maybe()).join() {
            let f = fbm(flicker.last, 3) * flicker.amount;
            flicker.last += flicker.speed;

            let amount = (flicker.amount / 2.0) + f;

            if let Some(light) = light {
                light.intensity.set(light.base_intensity - amount);
            }

            if let Some(particle) = particle {
                particle.strength.set(1.0 - amount);
            }
        }
    }
}
