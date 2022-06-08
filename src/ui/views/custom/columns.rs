use egui::*;

pub struct Columns {
    max_rect: Rect,
    offset: Vec2,
}

impl Columns {
    pub fn new(ui: &Ui, columns: usize) -> Self {
        let item_spacing = ui.spacing().item_spacing.x;
        let mut max_rect = ui.max_rect();
        max_rect.set_height(0.0);
        max_rect.set_width(max_rect.width() - columns as f32 * item_spacing);

        let width = max_rect.width() / columns as f32;
        let offset = vec2(width + item_spacing, 0.0);

        max_rect.set_width(width);

        Self { max_rect, offset }
    }

    pub fn show<R>(mut self, ui: &mut Ui, child_ui: impl FnOnce(&mut Ui) -> R) -> Self {
        ui.allocate_ui_at_rect(self.max_rect, |ui| {
            child_ui(ui);
        });
        self.max_rect = self.max_rect.translate(self.offset);
        self
    }
}
