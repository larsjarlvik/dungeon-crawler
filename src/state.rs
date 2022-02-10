use crate::{
    engine::{self},
    ui::{self},
    world::{self, resources::input::KeyState},
};
use cgmath::*;
use specs::WorldExt;
use std::time::Instant;
use winit::{event::VirtualKeyCode, window::Window};

pub struct State {
    pub engine: engine::Engine,
    pub world: world::World,
    pub ui: ui::Ui,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let start = Instant::now();

        let engine = engine::Engine::new(window).await;
        let world = world::World::new(&engine);
        let ui = ui::Ui::new(&engine.ctx, &window);

        let mut state = Self { engine, world, ui };
        println!("Startup {} ms", start.elapsed().as_millis());

        state.reset();
        state
    }

    pub fn reset(&mut self) {
        let start = Instant::now();

        self.engine.init();

        println!("Init {} ms", start.elapsed().as_millis());
    }

    pub fn resize(&mut self, window: &Window, active: bool) {
        if active {
            self.engine.set_viewport(window);
            self.engine.deferred_pipeline.resize(&self.engine.ctx);

            let mut camera = self.world.components.write_resource::<world::resources::Camera>();
            *camera = world::resources::Camera::new(self.engine.ctx.viewport.get_aspect());
        } else {
            self.engine.ctx.surface = None;
        }
    }

    pub fn keyboard(&mut self, keyboard_input: &winit::event::KeyboardInput) {
        let r = {
            let mut input = self.world.components.write_resource::<world::resources::Input>();
            input.keyboard(keyboard_input);
            input.key_state(VirtualKeyCode::R)
        };

        if r == KeyState::Pressed(false) {
            self.reset();
        }
    }

    pub fn mouse_move(&mut self, id: u64, x: f32, y: f32) {
        let mut input = self.world.components.write_resource::<world::resources::Input>();
        input.mouse_move(
            id,
            Point2::new(x, y),
            self.engine.ctx.viewport.width,
            self.engine.ctx.viewport.height,
        );
    }

    pub fn mouse_press(&mut self, id: u64, touch: bool, pressed: bool) {
        let mut input = self.world.components.write_resource::<world::resources::Input>();

        if !pressed || !self.ui.is_blocking(input.mouse.position) {
            input.mouse_set_pressed(id, touch, pressed);
        }
    }

    pub fn is_ui_blocking(&self) -> bool {
        let input = self.world.components.read_resource::<world::resources::Input>();
        self.ui.is_blocking(input.mouse.position)
    }

    pub fn update(&mut self, window: &Window) {
        self.world.update(&self.engine);
        self.engine.deferred_pipeline.update(&self.engine.ctx, &self.world.components);
        self.engine.joystick_pipeline.update(&self.engine.ctx, &self.world.components);
        self.ui.update(window, &mut self.world)
    }

    pub fn render(&mut self, window: &Window) -> Result<(), wgpu::SurfaceError> {
        if let Some(frame) = self.engine.get_output_frame() {
            let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

            self.engine
                .model_pipeline
                .render(&self.engine.ctx, &self.world.components, &self.engine.deferred_pipeline);

            self.engine
                .deferred_pipeline
                .render(&self.engine.ctx, &self.engine.scaling_pipeline.texture.view);

            self.engine.particle_pipeline.render(
                &self.engine.ctx,
                &self.world.components,
                &self.engine.scaling_pipeline.texture.view,
                &self.engine.deferred_pipeline.depth_texture.view,
            );

            self.engine.scaling_pipeline.render(&self.engine.ctx, &view);
            self.engine.joystick_pipeline.render(&self.engine.ctx, &view);

            self.ui.render(&self.engine.ctx, &window, &view);
            frame.present();
        }

        Ok(())
    }
}
