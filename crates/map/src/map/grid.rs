use super::Tile;

pub type Position = (usize, usize);

#[derive(Debug, Clone, Hash)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

#[derive(Debug, Default, Clone)]
pub struct Grid {
    pub size: usize,
    pub tiles: Vec<Option<Tile>>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            tiles: vec![None; size * size],
        }
    }

    pub fn get(&self, (x, y): &Position) -> &Option<Tile> {
        &self.tiles[y * self.size + x]
    }

    pub fn get_mut(&mut self, (x, y): &Position) -> &mut Option<Tile> {
        &mut self.tiles[y * self.size + x]
    }

    pub fn set(&mut self, (x, y): &Position, tile: Option<Tile>) {
        self.tiles[y * self.size + x] = tile;
    }

    pub fn move_position(&self, (x, y): &Position, direction: Direction) -> Option<Position> {
        match direction {
            Direction::North => {
                if y > &0 {
                    return Some((*x, y - 1));
                }
            }
            Direction::East => {
                if x + 1 < self.size {
                    return Some((x + 1, *y));
                }
            }
            Direction::South => {
                if y + 1 < self.size {
                    return Some((*x, y + 1));
                }
            }
            Direction::West => {
                if x > &0 {
                    return Some((x - 1, *y));
                }
            }
        }

        None
    }

    pub fn get_by_asset(&self, asset: usize) -> Vec<Position> {
        let mut matches = vec![];
        for y in 0..self.size {
            for x in 0..self.size {
                if let Some(tile) = self.get(&(x, y)) {
                    if tile.asset == asset {
                        matches.push((x, y));
                    }
                }
            }
        }

        matches
    }
}
