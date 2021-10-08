use crate::{engine, world};
use cgmath::*;
use rand::{prelude::StdRng, Rng, SeedableRng};

mod block;

#[derive(Clone)]
struct Room {
    entrances: [bool; 4],
    x: i32,
    z: i32,
}

impl Default for Room {
    fn default() -> Self {
        Self {
            entrances: [false, false, false, false],
            x: 0,
            z: 0,
        }
    }
}

pub struct Map {
    seed: u64,
    block: block::Block,
    grid_size: usize,
    number_of_rooms: i32,
}

impl Map {
    pub fn new(engine: &engine::Engine, seed: u64, grid_size: usize) -> Self {
        let block = block::Block::new(engine, 16.0);
        let number_of_rooms = 150;
        Self {
            block,
            seed,
            grid_size,
            number_of_rooms,
        }
    }

    pub fn generate(&self, engine: &engine::Engine, world: &mut world::World) {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let mut rooms = self.create_rooms(&mut rng);
        self.add_doors(&mut rooms);

        for x in 0..(self.grid_size * 2) {
            for z in 0..(self.grid_size * 2) {
                if let Some(room) = &mut rooms[x][z] {
                    print!("X");
                    self.block.build(engine, world, room.x, room.z, &room.entrances);
                } else {
                    print!(" ")
                }
            }
            println!("");
        }
    }

    fn create_rooms(&self, rng: &mut StdRng) -> Vec<Vec<Option<Room>>> {
        let mut rooms = vec![vec![None; self.grid_size * 2]; self.grid_size * 2];
        let mut taken_positions = vec![(0, 0)];

        rooms[self.grid_size][self.grid_size] = Some(Room {
            x: 0,
            z: 0,
            entrances: [false, false, false, false],
        });

        let random_compare_start = 0.2f32;
        let random_compare_end = 0.01f32;

        for i in 0..(self.number_of_rooms - 1) {
            let random_perc = (i as f32) / (self.number_of_rooms - 1) as f32;
            let random_compare = vec1(random_compare_start).lerp(vec1(random_compare_end), random_perc).x;
            let mut check_pos = self.new_position(rng, &taken_positions);

            if self.number_of_neighbors(&check_pos, &taken_positions) > 1 && rng.gen::<f32>() > random_compare * 0.5 {
                for _ in 0..100 {
                    check_pos = self.selective_new_position(rng, &taken_positions);
                    if self.number_of_neighbors(&check_pos, &taken_positions) == 0 {
                        break;
                    }
                }
            }

            rooms[check_pos.0 as usize + self.grid_size][check_pos.1 as usize + self.grid_size] = Some(Room {
                x: check_pos.0,
                z: check_pos.1,
                entrances: [false, false, false, false],
            });

            taken_positions.insert(0, check_pos);
        }

        rooms
    }

    fn add_doors(&self, rooms: &mut Vec<Vec<Option<Room>>>) {
        for x in 0..(self.grid_size * 2) {
            for z in 0..(self.grid_size * 2) {
                self.add_door(rooms, x as i32, z as i32, 0, -1, 0);
                self.add_door(rooms, x as i32, z as i32, 1, 0, 1);
                self.add_door(rooms, x as i32, z as i32, 0, 1, 2);
                self.add_door(rooms, x as i32, z as i32, -1, 0, 3);
            }
        }
    }

    fn add_door(&self, rooms: &mut Vec<Vec<Option<Room>>>, x: i32, z: i32, ox: i32, oz: i32, entrance: usize) {
        let existing = rooms.clone();

        if let Some(room) = &mut rooms[x as usize][z as usize] {
            if z + oz >= 0 && z + oz < self.grid_size as i32 * 2 && x + ox >= 0 && x + ox < self.grid_size as i32 * 2 {
                if existing[(x + ox) as usize][(z + oz) as usize].is_some() {
                    room.entrances[entrance] = true;
                }
            }
        }
    }

    fn selective_new_position(&self, rng: &mut StdRng, taken_positions: &Vec<(i32, i32)>) -> (i32, i32) {
        let mut check_pos: (i32, i32);

        loop {
            let mut index = 0;
            for _ in 0..100 {
                index = rng.gen_range(0..taken_positions.len());
                if self.number_of_neighbors(&taken_positions[index], taken_positions) <= 1 {
                    break;
                }
            }

            let (mut x, mut z) = taken_positions[index];
            let up_down = rng.gen::<f32>() < 0.5;
            let positive = rng.gen::<f32>() < 0.5;
            if up_down {
                if positive {
                    z += 1;
                } else {
                    z -= 1;
                }
            } else {
                if positive {
                    x += 1;
                } else {
                    x -= 1;
                }
            }

            check_pos = (x, z);

            let gs = self.grid_size as i32;
            if !taken_positions.iter().any(|t| t == &check_pos) && x >= -gs && x < gs && z >= -gs && z < gs {
                break;
            }
        }

        check_pos
    }

    fn number_of_neighbors(&self, pos: &(i32, i32), taken_positions: &Vec<(i32, i32)>) -> usize {
        let (x, z) = pos;
        taken_positions.iter().filter(|(tx, tz)| tx == &(x + 1) && tz == z).count()
            + taken_positions.iter().filter(|(tx, tz)| tx == &(x - 1) && tz == z).count()
            + taken_positions.iter().filter(|(tx, tz)| tx == x && tz == &(z + 1)).count()
            + taken_positions.iter().filter(|(tx, tz)| tx == x && tz == &(z - 1)).count()
    }

    fn new_position(&self, rng: &mut StdRng, taken_positions: &Vec<(i32, i32)>) -> (i32, i32) {
        let mut checking_pos: (i32, i32);

        loop {
            let index = rng.gen_range(0..taken_positions.len());
            let (mut x, mut z) = taken_positions[index];

            let up_down = rng.gen::<f32>() < 0.5;
            let positive = rng.gen::<f32>() < 0.5;

            if up_down {
                if positive {
                    z += 1;
                } else {
                    z -= 1;
                }
            } else {
                if positive {
                    x += 1;
                } else {
                    x -= 1;
                }
            }

            checking_pos = (x as i32, z as i32);

            let gs = self.grid_size as i32;
            if !taken_positions.iter().any(|t| t == &checking_pos) && x >= -gs && x < gs && z >= -gs && z < gs {
                break;
            }
        }

        checking_pos
    }
}
