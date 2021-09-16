use crate::config;
use specs::*;
use std::time::Instant;
pub mod components;
pub mod resources;
pub mod systems;

pub struct World {
    pub components: specs::World,
    pub dispatcher: specs::Dispatcher<'static, 'static>,
    frame_time: f32,
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
        components.register::<components::Animation>();
        components.register::<components::UserControl>();
        components.register::<components::Movement>();

        let dispatcher = DispatcherBuilder::new()
            .with(systems::Render, "render", &[])
            .with(systems::Fps, "fps", &[])
            .with(systems::UserControl, "user_control", &[])
            .with(systems::Movement, "movement", &[])
            .build();

        Self {
            components,
            dispatcher,
            frame_time: 0.0,
            last_frame: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        self.frame_time += self.last_frame.elapsed().as_millis() as f32;
        self.frame_time = self.frame_time.min(5000.0);

        while self.frame_time >= 0.0 {
            self.dispatcher.setup(&mut self.components);
            self.dispatcher.dispatch(&mut self.components);
            self.components.maintain();
            self.frame_time -= config::time_step().as_millis() as f32;
        }

        self.last_frame = Instant::now();
    }
}
