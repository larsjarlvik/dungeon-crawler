use egui::*;

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct Bar {
    progress: f32,
    desired_width: Option<f32>,
    text: WidgetText,
    color: Color32,
}

impl Bar {
    pub fn new(progress: f32, text: impl Into<WidgetText>, color: Color32) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            desired_width: None,
            text: text.into(),
            color,
        }
    }

    pub fn desired_width(mut self, desired_width: f32) -> Self {
        self.desired_width = Some(desired_width);
        self
    }
}

impl Widget for Bar {
    fn ui(self, ui: &mut Ui) -> Response {
        let Bar {
            progress,
            desired_width,
            text,
            color,
        } = self;

        let desired_width = desired_width.unwrap_or_else(|| ui.available_size_before_wrap().x.at_least(96.0));
        let height = ui.spacing().interact_size.y;
        let (outer_rect, response) = ui.allocate_exact_size(vec2(desired_width, height), Sense::hover());

        if ui.is_rect_visible(response.rect) {
            let visuals = ui.style().visuals.clone();
            let rounding = outer_rect.height() / 6.0;
            ui.painter().rect(outer_rect, rounding, visuals.extreme_bg_color, Stroke::none());
            let inner_rect = Rect::from_min_size(outer_rect.min, vec2(outer_rect.width() * progress, outer_rect.height()));

            ui.painter().rect(inner_rect, rounding, Color32::from(color), Stroke::none());

            let galley = text.into_galley(ui, Some(false), f32::INFINITY, TextStyle::Button);
            let text_pos = outer_rect.center() - Vec2::new(galley.size().x / 2.0, galley.size().y / 2.0);
            let text_color = Color32::from_rgb(180, 180, 180);
            galley.paint_with_color_override(&ui.painter().sub_region(outer_rect), text_pos, text_color);
        }

        response
    }
}
