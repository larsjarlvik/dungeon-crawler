use crate::world::*;

pub struct Parent;

impl<'a> System<'a> for Parent {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, components::Delete>,
        ReadStorage<'a, components::Child>,
    );

    fn run(&mut self, (entities, lazy, delete, child): Self::SystemData) {
        let mut ids = vec![];

        for (entity, _deletable) in (&entities, &delete).join() {
            entities.delete(entity).unwrap();
            ids.push(entity.id());
        }

        for (entity, child) in (&entities, &child).join() {
            if ids.contains(&child.parent_id) {
                lazy.insert(entity, components::Delete);
            }
        }
    }
}
