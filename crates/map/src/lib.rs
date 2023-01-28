use cgmath::*;
use engine::{
    ecs::{self},
    pipelines::model,
};
mod map;
use map::Direction;
pub use map::MapData;

pub struct Map {
    data: map::MapData,
    gltf: engine::model::GltfModel,
}

impl Map {
    pub fn new(engine: &mut engine::Engine, tile_set: &str, seed: u64) -> Self {
        let config = map::Config::new(seed, tile_set, vec![]);

        let mut data = map::MapData::new(12, 20..40);
        data.build(&config, false);

        let gltf = engine::load_model(&engine.ctx, format!("models/{}_map.glb", tile_set).as_str());

        Self { data, gltf }
    }

    pub fn create_mini_map(&self, engine: &mut engine::Engine, components: &mut bevy_ecs::world::World) {
        let map = self.data.history.last().expect("No map data found!");
        let scale = 0.1;

        for x in 0..self.data.size {
            for y in 0..self.data.size {
                let tile = map.get(&(x, y));

                if let Some(tile) = tile {
                    let x = x as f32 * 5.75 * scale;
                    let y = y as f32 * 5.75 * scale;

                    let model = engine.initialize_model(&self.gltf, model::Method::Default, format!("{}", tile.asset).as_str());

                    let rotation = match tile.direction {
                        Direction::North => 0.0,
                        Direction::East => -90.0,
                        Direction::South => -180.0,
                        Direction::West => -270.0,
                    };

                    components.spawn((
                        model,
                        ecs::components::Render {
                            cull_frustum: false,
                            shadows: false,
                        },
                        ecs::components::Transform::from_translation_angle_scale(vec3(x, 3.0, y), rotation, scale),
                    ));
                }
            }
        }
    }
}
