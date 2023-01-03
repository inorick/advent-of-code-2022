extern crate core;

use crate::Error::InvalidInput;
use crate::Flow::{Left, Right};
use std::collections::HashMap;

fn main() {}

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

pub enum Flow {
    Left,
    Right,
}

const CHAMBER_WIDTH: usize = 7;
const NUM_BLOCKS: u64 = 5;
const FLOW_CYCLE: u64 = 10091;

pub fn solve(input: String, max_blocks: u64) -> Result<u64, Error> {
    let flow = input
        .lines()
        .next()
        .ok_or(InvalidInput)?
        .chars()
        .map(|c| match c {
            '<' => Ok(Left),
            '>' => Ok(Right),
            _ => Err(InvalidInput),
        })
        .collect::<Result<Vec<_>, Error>>()?;
    let mut flow = flow.iter().cycle();
    let mut used_flows: u64 = 0;

    // Build chamber
    let mut chamber: Vec<[bool; CHAMBER_WIDTH]> = vec![];
    let mut height: u64 = 0;

    // Cycle detection
    let mut prev_block_nr = 0;
    let mut prev_height = 0;
    let mut cycle_data = HashMap::new();

    // Let the blocks fall
    let mut block_nr = 0;
    while block_nr < max_blocks {
        let (mut x, mut y) = (2, chamber.len() + 3);
        let block = next_block(block_nr);
        loop {
            // Block is blowing in the wind
            match flow.next().expect("Cyclic iterator ran out of items") {
                Left => {
                    if can_move_left(&(x, y), &block, &chamber) {
                        x -= 1;
                    }
                }
                Right => {
                    if can_move_right(&(x, y), &block, &chamber) {
                        x += 1;
                    }
                }
            }
            used_flows += 1;
            used_flows %= FLOW_CYCLE;

            // Check for cycle
            if used_flows == 0 {
                let block_nr_diff = block_nr - prev_block_nr;
                let height_diff = height - prev_height;
                prev_block_nr = block_nr;
                prev_height = height;
                let count = *cycle_data
                    .entry((block_nr_diff, height_diff))
                    .and_modify(|e| *e += 1)
                    .or_insert(0);
                if count == 10 {
                    println!("Cycle detected. Skipping ahead ...");
                    println!(
                        "From: block_nr={block_nr}, block_nr_diff={block_nr_diff}, used_flows={used_flows}, height={height}, height_diff={height_diff}",
                    );
                    let skip_cycles = (max_blocks - block_nr) / block_nr_diff;
                    block_nr += block_nr_diff * skip_cycles;
                    height += height_diff * skip_cycles;
                    println!(
                        "To:   block_nr={block_nr}, block_nr_diff={block_nr_diff}, used_flows={used_flows}, height={height}, height_diff={height_diff}",
                    );
                }
            }

            // Falling down
            if can_move_down(&(x, y), &block, &chamber) {
                y -= 1;
            } else {
                while chamber.len() < y + block_height(&block) {
                    const EMPTY_LINE: [bool; CHAMBER_WIDTH] =
                        [false, false, false, false, false, false, false];
                    chamber.push(EMPTY_LINE);
                    height += 1;
                }
                add_block(&(x, y), &block, &mut chamber);
                break; // Block has settled. Continue with next one.
            }
        }
        block_nr += 1;
    }
    Ok(height)
}

pub fn block_height(block: &Vec<Vec<bool>>) -> usize {
    block.len()
}

pub fn block_width(block: &[Vec<bool>]) -> usize {
    block[0].len()
}

pub fn can_move_left(
    (x, y): &(usize, usize),
    block: &Vec<Vec<bool>>,
    chamber: &Vec<[bool; CHAMBER_WIDTH]>,
) -> bool {
    if x == &0 {
        return false;
    }
    collision(&(x - 1, *y), block, chamber)
}

pub fn can_move_right(
    (x, y): &(usize, usize),
    block: &Vec<Vec<bool>>,
    chamber: &Vec<[bool; CHAMBER_WIDTH]>,
) -> bool {
    if x + block_width(block) >= CHAMBER_WIDTH {
        return false;
    }
    collision(&(x + 1, *y), block, chamber)
}

pub fn can_move_down(
    (x, y): &(usize, usize),
    block: &Vec<Vec<bool>>,
    chamber: &Vec<[bool; CHAMBER_WIDTH]>,
) -> bool {
    if y == &0 {
        return false;
    }
    collision(&(*x, y - 1), block, chamber)
}

pub fn collision(
    (x, y): &(usize, usize),
    block: &Vec<Vec<bool>>,
    chamber: &Vec<[bool; 7]>,
) -> bool {
    for block_y in 0..block_height(block) {
        let chamber_y = block_y + y;
        if chamber_y < chamber.len() {
            let chamber_line = chamber[chamber_y];
            for block_x in 0..block_width(block) {
                let chamber_x = block_x + x;
                if block[block_y][block_x] && chamber_line[chamber_x] {
                    return false;
                }
            }
        }
    }
    true
}

fn add_block(
    (x, y): &(usize, usize),
    block: &Vec<Vec<bool>>,
    chamber: &mut Vec<[bool; 7]>,
) -> bool {
    for block_y in 0..block_height(block) {
        let chamber_y = block_y + y;
        if chamber_y >= chamber.len() {
            panic!("Chamber not high enough.");
        }
        let chamber_line = &mut chamber[chamber_y];
        for block_x in 0..block_width(block) {
            let chamber_x = block_x + x;
            chamber_line[chamber_x] |= block[block_y][block_x];
        }
    }
    true
}

pub fn next_block(block_nr: u64) -> Vec<Vec<bool>> {
    match block_nr % NUM_BLOCKS {
        0 => {
            vec![vec![true, true, true, true]]
        }
        1 => {
            vec![
                vec![false, true, false],
                vec![true, true, true],
                vec![false, true, false],
            ]
        }
        2 => {
            vec![
                vec![true, true, true],
                vec![false, false, true],
                vec![false, false, true],
            ]
        }
        3 => {
            vec![vec![true], vec![true], vec![true], vec![true]]
        }
        4 => {
            vec![vec![true, true], vec![true, true]]
        }
        _ => {
            panic!("Invalid block number.")
        }
    }
}

#[cfg(test)]
pub mod test {
    #[test]
    fn solve_example() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let height = crate::solve(input, 2022).expect("failed to solve");
        assert_eq!(height, 3068);
    }

    #[test]
    fn solve_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let height = crate::solve(input, 2022).expect("failed to solve");
        assert_eq!(height, 3157);
    }

    #[test]
    fn solve_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let height = crate::solve(input, 1000000000000).expect("failed to solve");
        assert_eq!(height, 1581449275319);
    }
}
