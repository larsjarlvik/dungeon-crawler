use specs::*;
pub mod components;
pub mod resources;
pub mod systems;

pub struct World {
    pub components: specs::World,
    pub dispatcher: specs::Dispatcher<'static, 'static>,
}

impl<'a> World {
    pub fn new() -> Self {
        let mut components = specs::World::new();
        components.register::<components::Render>();
        components.register::<components::Model>();
        components.register::<components::Position>();
        components.register::<components::Rotation>();
        components.register::<components::Bouce>();
        components.register::<components::Text>();
        components.register::<components::Fps>();
        components.register::<components::Light>();
        components.register::<components::Animation>();

        let dispatcher = DispatcherBuilder::new()
            .with(systems::Bounce, "bounce", &[])
            .with(systems::Rotate, "rotate", &[])
            .with(systems::Render, "render", &[])
            .with(systems::Fps, "fps", &[])
            .with(systems::Animation, "animation", &[])
            .build();

        Self { components, dispatcher }
    }

    pub fn update(&mut self) {
        {
            let mut time = self.components.write_resource::<resources::Time>();
            time.set();
        }

        self.dispatcher.setup(&mut self.components);
        self.dispatcher.dispatch(&mut self.components);
        self.components.maintain();
    }
}
