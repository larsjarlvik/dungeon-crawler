use cgmath::*;
use rand::{prelude::StdRng, Rng};

#[derive(Clone, Default)]
pub struct Tile {
    pub entrances: [bool; 4],
}

pub fn generate(rng: &mut StdRng, grid_size: usize, number_of_tiles: usize) -> Vec<Vec<Option<Tile>>> {
    let mut tiles = vec![vec![None; grid_size * 2]; grid_size * 2];
    let mut taken_positions = vec![(0, 0)];

    tiles[grid_size][grid_size] = Some(Tile {
        entrances: [false, false, false, false],
    });

    let random_compare_start = 0.2f32;
    let random_compare_end = 0.01f32;

    for i in 0..(number_of_tiles - 1) {
        let random_perc = (i as f32) / (number_of_tiles - 1) as f32;
        let random_compare = vec1(random_compare_start).lerp(vec1(random_compare_end), random_perc).x;
        let mut check_pos = new_position(grid_size, rng, &taken_positions);

        if number_of_neighbors(&check_pos, &taken_positions) > 1 && rng.gen::<f32>() > random_compare * 0.5 {
            for _ in 0..100 {
                check_pos = selective_new_position(grid_size, rng, &taken_positions);
                if number_of_neighbors(&check_pos, &taken_positions) == 0 {
                    break;
                }
            }
        }
        tiles[(check_pos.0 + grid_size as i32) as usize][(check_pos.1 + grid_size as i32) as usize] = Some(Tile {
            entrances: [false, false, false, false],
        });

        taken_positions.insert(0, check_pos);
    }

    add_entrances(&mut tiles, grid_size);
    tiles
}

fn add_entrances(tiles: &mut [Vec<Option<Tile>>], grid_size: usize) {
    for x in 0..(grid_size * 2) {
        for z in 0..(grid_size * 2) {
            add_entrance(grid_size, tiles, x as i32, z as i32, 0, -1, 0);
            add_entrance(grid_size, tiles, x as i32, z as i32, 1, 0, 1);
            add_entrance(grid_size, tiles, x as i32, z as i32, 0, 1, 2);
            add_entrance(grid_size, tiles, x as i32, z as i32, -1, 0, 3);
        }
    }
}

fn add_entrance(grid_size: usize, tiles: &mut [Vec<Option<Tile>>], x: i32, z: i32, ox: i32, oz: i32, entrance: usize) {
    let existing = tiles.to_owned();

    if let Some(tile) = &mut tiles[x as usize][z as usize] {
        if z + oz >= 0
            && z + oz < grid_size as i32 * 2
            && x + ox >= 0
            && x + ox < grid_size as i32 * 2
            && existing[(x + ox) as usize][(z + oz) as usize].is_some()
        {
            tile.entrances[entrance] = true;
        }
    }
}

fn selective_new_position(grid_size: usize, rng: &mut StdRng, taken_positions: &Vec<(i32, i32)>) -> (i32, i32) {
    let mut check_pos: (i32, i32);

    loop {
        let mut index = 0;
        for _ in 0..100 {
            index = rng.gen_range(0..taken_positions.len());
            if number_of_neighbors(&taken_positions[index], taken_positions) <= 1 {
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
        } else if positive {
            x += 1;
        } else {
            x -= 1;
        }

        check_pos = (x, z);

        let gs = grid_size as i32;
        if !taken_positions.iter().any(|t| t == &check_pos) && x >= -gs && x < gs && z >= -gs && z < gs {
            break;
        }
    }

    check_pos
}

fn number_of_neighbors(pos: &(i32, i32), taken_positions: &[(i32, i32)]) -> usize {
    let (x, z) = pos;
    taken_positions.iter().filter(|(tx, tz)| tx == &(x + 1) && tz == z).count()
        + taken_positions.iter().filter(|(tx, tz)| tx == &(x - 1) && tz == z).count()
        + taken_positions.iter().filter(|(tx, tz)| tx == x && tz == &(z + 1)).count()
        + taken_positions.iter().filter(|(tx, tz)| tx == x && tz == &(z - 1)).count()
}

fn new_position(grid_size: usize, rng: &mut StdRng, taken_positions: &Vec<(i32, i32)>) -> (i32, i32) {
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
        } else if positive {
            x += 1;
        } else {
            x -= 1;
        }

        checking_pos = (x, z);

        let gs = grid_size as i32;
        if !taken_positions.iter().any(|t| t == &checking_pos) && x >= -gs && x < gs && z >= -gs && z < gs {
            break;
        }
    }

    checking_pos
}
