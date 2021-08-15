#[derive(Clone)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
    pub dpi: f64,
}

impl Viewport {
    pub fn new(width: u32, height: u32, dpi: f64) -> Self {
        // TODO: Winit doesn't support DPI on Android, need to find another way to determine it
        let dpi = if cfg!(target_os = "android") { 2.0 } else { dpi };
        Self { width, height, dpi }
    }
}
