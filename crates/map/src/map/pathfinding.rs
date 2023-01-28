use super::grid::{self, Direction, Grid, Position};
use pathfinding::prelude::astar;

fn get_successors(grid: &Grid, position: &grid::Position) -> Vec<Position> {
    let mut successors = Vec::new();

    if let Some(tile) = grid.get(position) {
        if tile.edges.north.iter().any(|e| e > &0) {
            if let Some(next) = grid.move_position(position, Direction::North) {
                successors.push(next);
            }
        }
        if tile.edges.east.iter().any(|e| e > &0) {
            if let Some(next) = grid.move_position(position, Direction::East) {
                successors.push(next);
            }
        }
        if tile.edges.south.iter().any(|e| e > &0) {
            if let Some(next) = grid.move_position(position, Direction::South) {
                successors.push(next);
            }
        }
        if tile.edges.west.iter().any(|e| e > &0) {
            if let Some(next) = grid.move_position(position, Direction::West) {
                successors.push(next);
            }
        }
    }

    successors
}

fn distance((px, py): grid::Position, (gx, gy): grid::Position) -> usize {
    ((px as i32 - gx as i32).abs() + (py as i32 - gy as i32).abs()) as usize
}

pub fn test(grid: &Grid, entrance: grid::Position, exit: grid::Position) -> Option<(Vec<grid::Position>, usize)> {
    astar(
        &entrance,
        |p| get_successors(grid, p).iter().map(|s| (*s, 1)).collect::<Vec<_>>(),
        |p| distance(*p, exit),
        |p| *p == exit,
    )
}
