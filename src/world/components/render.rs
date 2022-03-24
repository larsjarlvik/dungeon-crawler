use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Render {
    pub cull_frustum: bool,
}
