use winit::{event::WindowEvent, window::Window};

use crate::{camera, config, model};

pub struct State {
    pub size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    camera: camera::Camera,
    model: model::Model,
    model_instances: Vec<model::ModelInstance>,
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

        let camera = camera::Camera::new(size.width as u32, size.height as u32);
        let model = model::Model::new(&device);
        let model_instances = vec![model.load(&device)];

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            camera,
            model,
            model_instances,
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

    pub fn update(&mut self, elapsed: u64) {
        for model_instance in &mut self.model_instances {
            self.model.update(&self.queue, model_instance, &self.camera, elapsed);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("state_encoder"),
        });

        {
            self.model.render(&mut encoder, &self.model_instances, &frame.view);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}
