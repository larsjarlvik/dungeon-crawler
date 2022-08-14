use super::{AssetData, TextData};

#[derive(Debug, Clone)]
pub enum RenderWidgetType<'a> {
    Text(&'a TextData),
    Asset(&'a AssetData),
    None,
}

#[derive(Debug, Clone)]
pub enum RenderWidgetState {
    None,
    Hover,
    Pressed,
    Clicked,
}

#[derive(Debug, Clone)]
pub struct RenderWidget<'a> {
    pub widget: RenderWidgetType<'a>,
    pub state: RenderWidgetState,
    pub key: Option<String>,
}

impl<'a> RenderWidget<'a> {
    pub fn new(key: Option<String>, widget: RenderWidgetType<'a>) -> Self {
        Self {
            key,
            widget,
            state: RenderWidgetState::None,
        }
    }
}
