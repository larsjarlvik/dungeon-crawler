use crate::{config, world};
use cgmath::*;
use rand::Rng;
use specs::{Builder, WorldExt};
use winit::window::Window;

pub struct State {
    pub size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    world: world::World,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::VULKAN);
        let (size, surface) = unsafe {
            let size = window.inner_size();
            let surface = instance.create_surface(window);
            (size, surface)
        };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("No suitable GPU adapters found on the system!");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: config::COLOR_TEXTURE_FORMAT,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let mut world = world::World::new(&device);

        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let model = world.load_model(&device);

            world
                .components
                .create_entity()
                .with(world::components::Camera::new(size.width as u32, size.height as u32))
                .with(world::components::Model::from(model))
                .with(world::components::Position(vec3(
                    rng.gen::<f32>() * 4.0 - 2.0,
                    rng.gen::<f32>() * 4.0 - 2.0,
                    rng.gen::<f32>() * 4.0 - 2.0,
                )))
                .with(world::components::Bouce(vec3(
                    rng.gen::<f32>() * 0.06 - 0.03,
                    rng.gen::<f32>() * 0.06 - 0.03,
                    rng.gen::<f32>() * 0.06 - 0.03,
                )))
                .with(world::components::Render::default())
                .build();
        }

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            world,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.sc_desc.width = new_size.width;
            self.sc_desc.height = new_size.height;
            self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        }
    }

    pub fn update(&mut self, _elapsed: u64) {
        self.world.update();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;
        self.world.render(&self.device, &self.queue, &frame.view);
        Ok(())
    }
}
