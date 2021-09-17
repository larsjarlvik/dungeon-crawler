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
        self.frame_time += self.last_frame.elapsed().as_secs_f32();
        self.frame_time = self.frame_time.min(5.0);

        while self.frame_time >= 0.0 {
            self.dispatcher.setup(&mut self.components);
            self.dispatcher.dispatch(&mut self.components);
            self.components.maintain();
            self.frame_time -= config::time_step().as_secs_f32();

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
            let time = self.components.read_resource::<resources::Time>();
            let mut camera = self.components.write_resource::<resources::Camera>();
            camera.update(time.last_frame);
        }

        self.last_frame = Instant::now();
    }
}
