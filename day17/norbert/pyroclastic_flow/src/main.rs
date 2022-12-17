extern crate core;

use crate::Error::InvalidInput;
use crate::Flow::{Left, Right};

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
const FLOW_CYCLE: usize = 10091;

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

    // Build chamber
    let mut chamber: Vec<[bool; CHAMBER_WIDTH]> = vec![];
    let mut height: u64 = 0;

    // Let block fall
    for block_nr in 0..max_blocks {
        if block_nr % 100000000 == 0 {
            println!("block_nr={block_nr}");
        }

        let mut pos: (usize, usize) = (2, chamber.len() + 3);
        let mut used_flows = 0;
        let block = next_block(block_nr);
        let mut settled = false;
        while !settled {
            // Blowing in the wind
            match flow.next().expect("Cyclic iterator ran out of items") {
                Left => {
                    if can_move_left(pos, &block, &chamber) {
                        pos.0 -= 1;
                    }
                }
                Right => {
                    if can_move_right(pos, &block, &chamber) {
                        pos.0 += 1;
                    }
                }
            }
            used_flows += 1;
            used_flows %= FLOW_CYCLE;

            // Falling down
            if can_move_down(pos, &block, &chamber) {
                pos.1 -= 1;
            } else {
                settled = true;
                while chamber.len() < pos.1 + block_height(&block) {
                    const EMPTY_LINE: [bool; CHAMBER_WIDTH] =
                        [false, false, false, false, false, false, false];
                    chamber.push(EMPTY_LINE);
                    height += 1;
                }
                add_block(pos, &block, &mut chamber);

                // Print height if we hit full cycle, i.e. we are back to first block and first flow
                if (block_nr + 1) % NUM_BLOCKS == 0 && used_flows == 0 {
                    println!("Height at full cycle: {height}");
                }

                // Forget old chamber lines
                const MAX_CHAMBER_SIZE: usize = 100000000;
                const REDUCED_CHAMBER_SIZE: usize = 10000;
                if chamber.len() > MAX_CHAMBER_SIZE {
                    println!("Draining chamber");
                    chamber = chamber
                        .drain((chamber.len() - REDUCED_CHAMBER_SIZE)..)
                        .collect();
                }
            }
        }
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
    mut pos: (usize, usize),
    block: &Vec<Vec<bool>>,
    chamber: &Vec<[bool; CHAMBER_WIDTH]>,
) -> bool {
    if pos.0 == 0 {
        return false;
    }
    pos.0 -= 1;
    collision(pos, block, chamber)
}

pub fn can_move_right(
    mut pos: (usize, usize),
    block: &Vec<Vec<bool>>,
    chamber: &Vec<[bool; CHAMBER_WIDTH]>,
) -> bool {
    if pos.0 + block_width(block) >= CHAMBER_WIDTH {
        return false;
    }
    pos.0 += 1;
    collision(pos, block, chamber)
}

pub fn can_move_down(
    mut pos: (usize, usize),
    block: &Vec<Vec<bool>>,
    chamber: &Vec<[bool; CHAMBER_WIDTH]>,
) -> bool {
    if pos.1 == 0 {
        return false;
    }
    pos.1 -= 1;
    collision(pos, block, chamber)
}

pub fn collision(pos: (usize, usize), block: &Vec<Vec<bool>>, chamber: &Vec<[bool; 7]>) -> bool {
    for block_y in 0..block_height(block) {
        let chamber_y = block_y + pos.1;
        if chamber_y < chamber.len() {
            let chamber_line = chamber[chamber_y];
            for block_x in 0..block_width(block) {
                let chamber_x = block_x + pos.0;
                if block[block_y][block_x] && chamber_line[chamber_x] {
                    return false;
                }
            }
        }
    }
    true
}

fn add_block(pos: (usize, usize), block: &Vec<Vec<bool>>, chamber: &mut Vec<[bool; 7]>) -> bool {
    for block_y in 0..block_height(block) {
        let chamber_y = block_y + pos.1;
        if chamber_y >= chamber.len() {
            panic!("Chamber not high enough.");
        }
        let chamber_line = &mut chamber[chamber_y];
        for block_x in 0..block_width(block) {
            let chamber_x = block_x + pos.0;
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
        assert_eq!(height, 3157);
    }
}
