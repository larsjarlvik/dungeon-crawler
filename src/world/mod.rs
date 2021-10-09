use crate::{config, utils::Interpolate};
use specs::*;
use std::time::Instant;
pub mod components;
pub mod resources;
pub mod systems;

pub struct World {
    pub components: specs::World,
    pub dispatcher: specs::Dispatcher<'static, 'static>,
    update_time: f32,
    last_frame: std::time::Instant,
}

impl<'a> World {
    pub fn new() -> Self {
        let mut components = specs::World::new();
        components.register::<components::Render>();
        components.register::<components::Model>();
        components.register::<components::Transform>();
        components.register::<components::Text>();
        components.register::<components::Fps>();
        components.register::<components::Light>();
        components.register::<components::Animations>();
        components.register::<components::UserControl>();
        components.register::<components::Movement>();
        components.register::<components::Follow>();
        components.register::<components::Collider>();
        components.register::<components::Collision>();

        let dispatcher = DispatcherBuilder::new()
            .with(systems::Fps, "fps", &[])
            .with(systems::UserControl, "user_control", &[])
            .with(systems::Movement, "movement", &[])
            .build();

        Self {
            components,
            dispatcher,
            update_time: 0.0,
            last_frame: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        self.update_time += self.last_frame.elapsed().as_secs_f32();
        self.update_time = self.update_time.min(5.0);

        while self.update_time >= 0.0 {
            self.dispatcher.setup(&mut self.components);
            self.dispatcher.dispatch_par(&mut self.components);
            self.components.maintain();
            self.update_time -= config::time_step().as_secs_f32();

            {
                let mut time = self.components.write_resource::<resources::Time>();
                time.reset();
            }
        }

        let mut fps = self.components.write_storage::<components::Fps>();
        for fps in (&mut fps).join() {
            fps.fps += 1;
        }

        {
            let mut camera = self.components.write_resource::<resources::Camera>();
            let mut time = self.components.write_resource::<resources::Time>();
            time.freeze();

            let follow = self.components.read_storage::<components::Follow>();
            let transform = self.components.read_storage::<components::Transform>();

            for (transform, _) in (&transform, &follow).join() {
                camera.set(transform.translation.get(time.last_frame));
            }

            let mut animations = self.components.write_storage::<components::Animations>();
            for animation in (&mut animations).join() {
                for (_, channel) in animation.channels.iter_mut() {
                    channel.current.elapsed += self.last_frame.elapsed().as_secs_f32() * channel.current.speed;
                    if let Some(previous) = &mut channel.prev {
                        previous.elapsed += self.last_frame.elapsed().as_secs_f32() * previous.speed;
                    }
                }
            }
        }

        self.last_frame = Instant::now();
    }
}
