use crate::world::*;
use bevy_ecs::prelude::*;

pub fn camera(
    mut camera: ResMut<resources::Camera>,
    time: Res<resources::Time>,
    query: Query<(&components::Transform, &components::Follow)>,
) {
    for (transform, _) in query.iter() {
        let t = transform.translation.get(time.last_frame);
        camera.set(t);
    }
}
