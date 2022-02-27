use crate::{
    engine::{self},
    ui::{self},
    world::{self, resources::input::KeyState},
};
use cgmath::*;
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

        println!("Startup {} ms", start.elapsed().as_millis());
        Self { engine, world, ui }
    }

    pub fn resize(&mut self, window: &Window, active: bool) {
        if active {
            self.engine.set_viewport(window);
            self.engine.deferred_pipeline.resize(&self.engine.ctx);

            let size = window.inner_size();
            let pos = window.inner_position().unwrap_or(winit::dpi::PhysicalPosition::new(100, 100));
            let fullscreen = window.fullscreen().is_some();

            if !fullscreen {
                self.engine.ctx.settings.window_size = [size.width, size.height];
                self.engine.ctx.settings.window_pos = [pos.x, pos.y];
            }
            self.engine.ctx.settings.fullscreen = window.fullscreen().is_some();
            self.engine.ctx.settings.store();

            self.world.components.remove_resource::<world::resources::Camera>().unwrap();
            self.world
                .components
                .insert_resource(world::resources::Camera::new(self.engine.ctx.viewport.get_aspect()));
        } else {
            self.engine.ctx.surface = None;
        }
    }

    pub fn keyboard(&mut self, keyboard_input: &winit::event::KeyboardInput) {
        let r = {
            let mut input = self.world.components.get_resource_mut::<world::resources::Input>().unwrap();
            input.keyboard(keyboard_input);
            input.key_state(VirtualKeyCode::R)
        };

        if r == KeyState::Pressed(false) {
            self.world.init(&self.engine);
        }
    }

    pub fn mouse_move(&mut self, id: u64, x: f32, y: f32) {
        let mut input = self.world.components.get_resource_mut::<world::resources::Input>().unwrap();
        input.mouse_move(
            id,
            Point2::new(x, y),
            self.engine.ctx.viewport.width,
            self.engine.ctx.viewport.height,
        );
    }

    pub fn mouse_press(&mut self, id: u64, touch: bool, pressed: bool) {
        let mut input = self.world.components.get_resource_mut::<world::resources::Input>().unwrap();

        if !pressed || !self.ui.is_blocking(input.mouse.position) {
            input.mouse_set_pressed(id, touch, pressed);
        }
    }

    pub fn is_ui_blocking(&self) -> bool {
        let input = self.world.components.get_resource::<world::resources::Input>().unwrap();
        self.ui.is_blocking(input.mouse.position)
    }

    pub fn update(&mut self, window: &Window) {
        self.world.update(&self.engine);
        self.engine.deferred_pipeline.update(&self.engine.ctx, &mut self.world.components);
        self.engine.joystick_pipeline.update(&self.engine.ctx, &self.world.components);
        self.ui.update(window, &self.engine.ctx, &mut self.world)
    }

    pub fn render(&mut self, window: &Window) -> Result<(), wgpu::SurfaceError> {
        if let Some(frame) = self.engine.get_output_frame() {
            let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

            self.engine
                .model_pipeline
                .render(&self.engine.ctx, &mut self.world.components, &self.engine.deferred_pipeline);

            self.engine
                .deferred_pipeline
                .render(&self.engine.ctx, &self.engine.scaling_pipeline.texture.view);

            self.engine.particle_pipeline.render(
                &self.engine.ctx,
                &mut self.world.components,
                &self.engine.scaling_pipeline.texture.view,
                &self.engine.deferred_pipeline.depth_texture.view,
            );

            self.engine.scaling_pipeline.render(&self.engine.ctx, &view);
            self.engine
                .glyph_pipeline
                .render(&self.engine.ctx, &mut self.world.components, &view);
            self.engine.joystick_pipeline.render(&self.engine.ctx, &view);

            self.ui.render(&self.engine.ctx, &window, &view);
            frame.present();
        }

        Ok(())
    }
}
