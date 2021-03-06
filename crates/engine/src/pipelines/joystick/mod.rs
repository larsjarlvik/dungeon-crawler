mod uniforms;
use super::builders;
use crate::{config, ecs::resources, Context};
use cgmath::*;
pub use uniforms::Uniforms;

pub struct JoystickPipeline {
    render_bundle: wgpu::RenderBundle,
    uniform_buffer: wgpu::Buffer,
    is_visible: bool,
}

impl JoystickPipeline {
    pub fn new(ctx: &Context) -> Self {
        let builder = builders::PipelineBuilder::new(&ctx, "joystick");

        let uniform_bind_group_layout = builder.create_bindgroup_layout(
            0,
            "uniform_bind_group_layout",
            &[builder.create_uniform_entry(0, wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT)],
        );

        let render_pipeline = builder
            .with_shader("shaders/joystick.wgsl")
            .with_primitve_topology(wgpu::PrimitiveTopology::TriangleStrip)
            .with_color_targets(vec![config::COLOR_TEXTURE_FORMAT])
            .with_blend(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    operation: wgpu::BlendOperation::Add,
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                },
                alpha: wgpu::BlendComponent::REPLACE,
            })
            .with_bind_group_layout(&uniform_bind_group_layout)
            .build();

        let render_bundle_builder = builders::RenderBundleBuilder::new(ctx, "joystick");
        let uniform_buffer = render_bundle_builder.create_uniform_buffer_init(bytemuck::cast_slice(&[Uniforms {
            center: [0.0, 0.0],
            current: [0.0, 0.0],
            radius: config::JOYSTICK_RADIUS,
            aspect: 1.0,
        }]));

        let render_bundle = render_bundle_builder
            .with_pipeline(&render_pipeline)
            .with_uniform_bind_group(&uniform_bind_group_layout, &uniform_buffer)
            .with_primitive(builders::PrimitiveBuilder::new(ctx, "joystick").with_length(4))
            .build();

        Self {
            render_bundle,
            uniform_buffer,
            is_visible: false,
        }
    }

    pub fn update(
        &mut self,
        ctx: &Context,
        components: &bevy_ecs::world::World,
        center: Option<Point2<f32>>,
        current: Option<Point2<f32>>,
        is_touch: bool,
    ) {
        self.is_visible = false;
        let camera = components.get_resource::<resources::Camera>().unwrap();

        if let Some(center) = center {
            if let Some(current) = current {
                let uniforms = uniforms::Uniforms {
                    center: center.into(),
                    current: current.into(),
                    radius: config::JOYSTICK_RADIUS,
                    aspect: camera.aspect,
                };

                ctx.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
                self.is_visible = is_touch;
            }
        }
    }

    pub fn render(&self, ctx: &Context, target: &wgpu::TextureView) {
        if self.is_visible {
            builders::RenderTargetBuilder::new(ctx, "joystick")
                .with_color_attachment(&target, wgpu::LoadOp::Load)
                .execute_bundles(vec![&self.render_bundle]);
        }
    }
}
