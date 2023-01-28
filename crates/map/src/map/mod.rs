use itertools::Itertools;
use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::ops::Range;
use std::time::Instant;
mod grid;
mod pathfinding;
mod tile;
use self::grid::{Grid, Position};
pub use grid::Direction;
pub use tile::Edges;
pub use tile::Path;
pub use tile::Tile;
mod config;
pub use config::*;

#[derive(Debug)]
pub struct MapData {
    pub size: usize,
    pub path_length: Range<usize>,
    pub history: Vec<Grid>,
    pub variants: Vec<Tile>,
}

impl MapData {
    pub fn new(size: usize, path_length: Range<usize>) -> Self {
        Self {
            size,
            path_length,
            history: vec![],
            variants: vec![],
        }
    }

    fn pick_entrance_exit(&self, rng: &mut StdRng, grid: &Grid, variants: Vec<usize>) -> grid::Position {
        if !variants.is_empty() {
            let candidates = grid.get_by_asset(variants[rng.gen_range(0..variants.len())]);
            candidates[rng.gen_range(0..candidates.len())]
        } else {
            (rng.gen_range(0..grid.size - 1), rng.gen_range(0..grid.size - 1))
        }
    }

    pub fn build(&mut self, config: &config::Config, log_history: bool) {
        let mut rng = StdRng::seed_from_u64(config.seed);
        self.history.clear();
        let time = Instant::now();

        self.load_config(config);
        self.history.push(Grid {
            size: self.variants.len(),
            tiles: self.variants.iter().map(|v| Some(v.clone())).collect(),
        });

        let mut tries = 0;

        loop {
            tries += 1;
            let map_ok = self.generate_map(&mut rng, log_history);

            if map_ok {
                let mut grid = self.history.last().unwrap().clone();

                let entrance = self.pick_entrance_exit(
                    &mut rng,
                    &grid,
                    config
                        .variants
                        .iter()
                        .filter_map(|v| if v.entrance { Some(v.index) } else { None })
                        .collect(),
                );

                let exit = self.pick_entrance_exit(
                    &mut rng,
                    &grid,
                    config
                        .variants
                        .iter()
                        .filter_map(|v| if v.exit { Some(v.index) } else { None })
                        .collect(),
                );

                if let Some((tiles, length)) = pathfinding::test(&grid, entrance, exit) {
                    if self.path_length.contains(&length) {
                        for index in tiles {
                            grid.get_mut(&index).as_mut().unwrap().path = if index == entrance {
                                Path::Entrance
                            } else if index == exit {
                                Path::Exit
                            } else {
                                Path::Track
                            };
                        }

                        self.history.push(grid.clone());

                        for x in 0..grid.size {
                            for y in 0..grid.size {
                                if pathfinding::test(&grid, (x, y), exit).is_none() {
                                    grid.set(&(x, y), None);
                                }
                            }
                        }

                        self.history.push(grid.clone());
                        break;
                    }
                }
            }
        }

        let elapsed = time.elapsed().as_secs_f32();

        println!("Map generated after {} tries", tries);
        println!("Time taken: {}", elapsed);
        println!("Per try: {}", elapsed / tries as f32);
    }

    fn neighbors_from_image(&self, config: &config::Config) -> Vec<(usize, Direction, Edges)> {
        let mut variants = vec![];
        let tile_size = config.tile_size;

        for x in 0..(config.tiles.width() / tile_size) {
            for y in 0..(config.tiles.height() / tile_size) {
                let index = (y * self.size as u32 + x) as usize;
                let mut variant_img = config.tiles.clone().crop(x * tile_size, y * tile_size, tile_size, tile_size);
                let mut direction = Direction::North;

                for _ in 0..4 {
                    variants.push((index, direction.clone(), variant_img.clone()));
                    variant_img = variant_img.rotate90();
                    direction = match direction {
                        Direction::North => Direction::East,
                        Direction::East => Direction::South,
                        Direction::South => Direction::West,
                        Direction::West => Direction::North,
                    };
                }
            }
        }

        variants.sort_by(|(_, _, a), (_, _, b)| a.as_bytes().cmp(b.as_bytes()));
        variants.dedup_by(|(a, _, ai), (b, _, bi)| a == b && ai.as_bytes() == bi.as_bytes());
        variants
            .into_iter()
            .map(|(index, direction, image)| (index, direction, tile::get_edges(&image)))
            .collect()
    }

    fn load_config(&mut self, config: &config::Config) {
        self.variants.clear();

        let neighbors = self.neighbors_from_image(config);

        for (asset, direction, edges) in neighbors {
            self.variants.push(Tile {
                asset,
                direction,
                edges,
                weight: 1.0,
                path: Path::None,
            });
        }

        for variant in config.variants.iter() {
            if let Some(existing) = self.variants.iter_mut().find(|v| v.asset == variant.index) {
                existing.weight = variant.weight;
            }
        }

        if self.variants.is_empty() {
            panic!("No variants set for map!");
        }
    }

    fn generate_map(&mut self, rng: &mut StdRng, step_by_step: bool) -> bool {
        let mut grid = self.clear(rng);
        if step_by_step {
            self.history.push(grid.clone());
        }

        loop {
            if grid.tiles.iter().all(|tile| tile.is_some()) {
                self.history.push(grid);
                return true;
            }

            let free_neighbors = self.get_free_neighbors(&grid);
            let least_entropy = free_neighbors
                .iter()
                .min_by(|(_, a_tile), (_, b_tile)| {
                    let a_sum: f32 = a_tile.iter().map(|a| self.variants[*a].weight).sum();
                    let b_sum: f32 = b_tile.iter().map(|b| self.variants[*b].weight).sum();
                    a_sum.partial_cmp(&b_sum).unwrap()
                })
                .map(|(_, tile)| tile);

            if let Some(least_entropy) = least_entropy {
                let possibilties: Vec<&(Position, Vec<usize>)> = free_neighbors
                    .iter()
                    .filter(|(position, _)| self.get_possible_variants(&grid, position).len() == least_entropy.len())
                    .collect();

                let (next_pos, next_tile) = possibilties[rng.gen_range(0..possibilties.len())];
                if next_tile.is_empty() {
                    return false;
                }

                grid.set(next_pos, Some(self.weighted_variant(rng, next_tile)));

                if step_by_step {
                    self.history.push(grid.clone());
                }
            } else {
                return false;
            }
        }
    }

    fn weighted_variant(&self, rng: &mut StdRng, variants: &[usize]) -> Tile {
        let weights: Vec<f32> = variants.iter().map(|v| self.variants[*v].weight).collect();
        let dist = WeightedIndex::new(&weights).unwrap();

        self.variants[variants[dist.sample(rng)]].clone()
    }

    pub fn get_free_neighbors(&self, grid: &Grid) -> Vec<(Position, Vec<usize>)> {
        (0..grid.tiles.len())
            .into_iter()
            .filter(|index| grid.tiles[*index].is_some())
            .flat_map(|index| {
                let pos = &(index % grid.size, index / grid.size);
                let neighbors = [
                    grid.move_position(pos, Direction::North),
                    grid.move_position(pos, Direction::East),
                    grid.move_position(pos, Direction::South),
                    grid.move_position(pos, Direction::West),
                ];

                neighbors.into_iter().flatten().filter_map(|position| {
                    if grid.get(&position).is_none() {
                        Some((position, self.get_possible_variants(grid, &position)))
                    } else {
                        None
                    }
                })
            })
            .unique()
            .collect()
    }

    fn clear(&mut self, rng: &mut StdRng) -> Grid {
        let mut grid = Grid::new(self.size);
        grid.tiles[rng.gen_range(0..(grid.size * grid.size))] = Some(self.variants[rng.gen_range(0..self.variants.len())].clone());
        grid
    }

    fn get_possible_variants(&self, grid: &Grid, position: &grid::Position) -> Vec<usize> {
        let variants = self
            .variants
            .iter()
            .enumerate()
            .filter_map(|(variant_index, variant)| {
                if let Some(north) = grid.move_position(position, Direction::North) {
                    if let Some(tile) = grid.get(&north) {
                        if variant.edges.north != tile.edges.south {
                            return None;
                        }
                    }
                } else if variant.edges.north.iter().any(|e| e > &0) {
                    return None;
                }

                if let Some(east) = grid.move_position(position, Direction::East) {
                    if let Some(tile) = grid.get(&east) {
                        if variant.edges.east != tile.edges.west {
                            return None;
                        }
                    }
                } else if variant.edges.east.iter().any(|e| e > &0) {
                    return None;
                }

                if let Some(south) = grid.move_position(position, Direction::South) {
                    if let Some(tile) = grid.get(&south) {
                        if variant.edges.south != tile.edges.north {
                            return None;
                        }
                    }
                } else if variant.edges.south.iter().any(|e| e > &0) {
                    return None;
                }

                if let Some(west) = grid.move_position(position, Direction::West) {
                    if let Some(tile) = grid.get(&west) {
                        if variant.edges.west != tile.edges.east {
                            return None;
                        }
                    }
                } else if variant.edges.west.iter().any(|e| e > &0) {
                    return None;
                }

                Some(variant_index)
            })
            .collect();

        variants
    }
}
