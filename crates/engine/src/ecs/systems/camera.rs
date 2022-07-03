use crate::ecs::{components, resources};
use crate::interpolated_value::Interpolate;
use bevy_ecs::prelude::*;

pub fn camera(
    mut camera: ResMut<resources::Camera>,
    time: Res<resources::Time>,
    query: Query<&components::Transform, With<components::Follow>>,
) {
    for transform in query.iter() {
        let t = transform.translation.get(time.alpha);
        camera.set(t);
    }
}
