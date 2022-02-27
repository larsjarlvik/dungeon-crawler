use crate::engine::{
    bounding_box,
    collision::{self, Polygon},
};
use bevy_ecs::prelude::Component;
use cgmath::*;

pub enum TileState {
    Active,
    Destroyed,
}

pub struct DecorLight {
    pub color: Vector3<f32>,
    pub intensity: f32,
    pub radius: Option<f32>,
    pub offset: Vector3<f32>,
    pub bloom: f32,
    pub position: Vector3<f32>,
    pub rotation: f32,
    pub flicker: Option<f32>,
}

pub struct DecorEmitter {
    pub emitter_id: String,
    pub position: Vector3<f32>,
    pub rotation: f32,
    pub start_color: Vector3<f32>,
    pub end_color: Vector3<f32>,
    pub size: f32,
    pub strength: f32,
    pub flicker: Option<f32>,
}

pub struct Decor {
    pub mesh_id: String,
    pub lights: Vec<DecorLight>,
    pub emitters: Vec<DecorEmitter>,
    pub rotation: f32,
    pub position: Vector3<f32>,
    pub collisions: Vec<collision::Polygon>,
}

#[derive(Component)]
pub struct Tile {
    pub mesh_id: String,
    pub state: TileState,
    pub center: Vector3<f32>,
    pub rotation: f32,
    pub decor: Vec<Decor>,
    pub bounding_box: bounding_box::BoundingBox,
    pub collisions: Vec<Polygon>,
}

impl Tile {
    pub fn new(mesh_id: String, collisions: Vec<Polygon>, center: Vector3<f32>, size: f32, rotation: f32, decor: Vec<Decor>) -> Self {
        let h_size = size / 2.0;

        Self {
            mesh_id,
            collisions,
            state: TileState::Destroyed,
            center,
            bounding_box: bounding_box::BoundingBox {
                min: point3(center.x - h_size, 0.0, center.z - h_size),
                max: point3(center.x + h_size, 2.5, center.z + h_size),
            },
            rotation,
            decor,
        }
    }
}
