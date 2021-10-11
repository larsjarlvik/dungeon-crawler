use crate::{utils::fbm, world::*};

pub struct LightFlicker;

impl<'a> System<'a> for LightFlicker {
    type SystemData = (WriteStorage<'a, components::Flicker>, WriteStorage<'a, components::Light>);

    fn run(&mut self, (mut flicker, mut light): Self::SystemData) {
        for (flicker, light) in (&mut flicker, &mut light).join() {
            let f = fbm(flicker.last, 3) * flicker.amount;
            flicker.last += flicker.speed;
            light.intensity.set(light.base_intensity - (flicker.amount / 2.0) + f);
        }
    }
}
