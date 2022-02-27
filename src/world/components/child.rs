use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Child {
    pub parent_id: Entity,
}

impl Child {
    pub fn new(parent_id: Entity) -> Self {
        Self { parent_id }
    }
}
