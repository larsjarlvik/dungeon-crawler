#[derive(Clone)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
    pub render_scale: f32,
    pub ui_scale: f32,
}

impl Viewport {
    pub fn new(width: u32, height: u32, ui_scale: f32, render_scale: f32) -> Self {
        Self {
            width,
            height,
            ui_scale,
            render_scale,
        }
    }

    pub fn get_aspect(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    pub fn get_render_size(&self) -> (u32, u32) {
        (
            (self.width as f32 * self.render_scale) as u32,
            (self.height as f32 * self.render_scale) as u32,
        )
    }
}
