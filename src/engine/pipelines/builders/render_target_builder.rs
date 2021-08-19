use crate::engine;

pub struct RenderTargetBuilder<'a> {
    ctx: &'a engine::Context,
    color_attachments: Vec<wgpu::RenderPassColorAttachment<'a>>,
    depth_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'a>>,
    label: &'a str,
}

impl<'a> RenderTargetBuilder<'a> {
    pub fn new(ctx: &'a engine::Context, label: &'a str) -> Self {
        Self {
            ctx,
            color_attachments: vec![],
            depth_attachment: None,
            label,
        }
    }

    pub fn with_color_attachment(mut self, view: &'a wgpu::TextureView, load: wgpu::LoadOp<wgpu::Color>) -> Self {
        self.color_attachments.push(wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: wgpu::Operations { load, store: true },
        });
        self
    }

    pub fn with_depth_attachment(mut self, view: &'a wgpu::TextureView, load: wgpu::LoadOp<f32>) -> Self {
        self.depth_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
            view,
            depth_ops: Some(wgpu::Operations { load, store: true }),
            stencil_ops: None,
        });
        self
    }

    pub fn execute_bundles(self, bundles: Vec<&wgpu::RenderBundle>) {
        let mut encoder = self.ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some(format!("{}_encoder", self.label).as_str()),
        });

        encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some(format!("{}_render_pass", self.label).as_str()),
                color_attachments: &self.color_attachments,
                depth_stencil_attachment: self.depth_attachment,
            })
            .execute_bundles(bundles.into_iter());

        self.ctx.queue.submit(std::iter::once(encoder.finish()));
    }
}
