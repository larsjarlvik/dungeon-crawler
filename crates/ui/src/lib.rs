use std::collections::HashMap;
use taffy::prelude::*;
use widgets::{BaseWidget, NodeLayout, RenderWidget};
pub mod widgets;
pub use taffy::prelude;

pub struct Ui {
    pub textures: HashMap<String, engine::texture::Texture>,
}

impl Ui {
    pub fn new() -> Self {
        Self { textures: HashMap::new() }
    }

    pub fn add_texture(&mut self, ctx: &engine::Context, key: &str, data: Vec<u8>) {
        let img = image::load_from_memory(data.as_slice()).unwrap();
        let pixels = img.as_bytes();

        let texture = engine::texture::Texture::create_view(ctx, pixels, img.width(), img.height(), true);
        self.textures.insert(key.to_string(), texture);
    }

    pub fn render(
        &self,
        ctx: &mut engine::Context,
        root: &mut widgets::NodeWidget,
        width: f32,
        height: f32,
    ) -> Vec<(NodeLayout, RenderWidget)> {
        let mut taffy = Taffy::new();
        let root_node = root.render(ctx, &mut taffy);
        let root_layout = NodeLayout {
            x: 0.0,
            y: 0.0,
            width,
            height,
        };

        taffy
            .compute_layout(
                root_node,
                Size {
                    width: Some(width),
                    height: Some(height),
                },
            )
            .unwrap();

        root.get_nodes(&taffy, &root_layout)
            .iter()
            .map(|(node, widget)| (node.clone(), widget.clone()))
            .collect()
    }
}
