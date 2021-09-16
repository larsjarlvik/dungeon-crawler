use crate::world::components;
use specs::*;
use std::time::Instant;

pub struct Fps;

impl<'a> System<'a> for Fps {
    type SystemData = (WriteStorage<'a, components::Text>, WriteStorage<'a, components::Fps>);

    fn run(&mut self, (mut text, mut fps): Self::SystemData) {
        for (text, fps) in (&mut text, &mut fps).join() {
            if fps.last_update.elapsed().as_millis() >= 1000 {
                text.text = format!("FPS: {0}", fps.fps);
                fps.last_update = Instant::now();
                fps.fps = 0;
            }
        }
    }
}
