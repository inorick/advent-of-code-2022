extern crate core;

use crate::Error::InvalidInput;
use itertools::Itertools;
use std::collections::HashSet;

fn main() {}

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}
pub fn solve_part1(input: String) -> Result<u32, Error> {
    Ok(count_surface(&parse_input(input)))
}

pub fn solve_part2(input: String) -> Result<u32, Error> {
    let blocks: HashSet<_> = parse_input(input);
    let (x_min, y_min, z_min, x_max, y_max, z_max) = get_min_max(&blocks)?;
    let mut water = HashSet::new();
    let mut next_wave = create_box(
        x_min - 1,
        y_min - 1,
        z_min - 1,
        x_max + 1,
        y_max + 1,
        z_max + 1,
    );
    while !next_wave.is_empty() {
        water = water.union(&next_wave).cloned().collect();
        next_wave = water
            .iter()
            .flat_map(neighbours)
            .filter(|w| {
                w.0 >= x_min
                    && w.0 <= x_max
                    && w.1 >= y_min
                    && w.1 <= y_max
                    && w.2 >= z_min
                    && w.2 <= z_max
            })
            .filter(|w| !water.contains(w))
            .filter(|w| !blocks.contains(w))
            .collect();
    }
    let water_surface: u32 = count_surface(&water);
    let box_x: u32 = ((x_max + 1) - (x_min - 1)) as u32 + 1;
    let box_y: u32 = ((y_max + 1) - (y_min - 1)) as u32 + 1;
    let box_z: u32 = ((z_max + 1) - (z_min - 1)) as u32 + 1;
    let outer_surface = 2 * ((box_x * box_y) + box_x * box_z + box_y * box_z);
    Ok(water_surface - outer_surface)
}

fn count_surface(blocks: &HashSet<(i32, i32, i32)>) -> u32 {
    blocks
        .iter()
        .map(|block| neighbours(block).difference(blocks).count() as u32)
        .sum()
}

fn neighbours(block: &(i32, i32, i32)) -> HashSet<(i32, i32, i32)> {
    HashSet::from_iter(
        vec![
            (block.0 - 1, block.1, block.2),
            (block.0 + 1, block.1, block.2),
            (block.0, block.1 - 1, block.2),
            (block.0, block.1 + 1, block.2),
            (block.0, block.1, block.2 - 1),
            (block.0, block.1, block.2 + 1),
        ]
        .iter()
        .cloned(),
    )
}

fn create_box(
    x_min: i32,
    y_min: i32,
    z_min: i32,
    x_max: i32,
    y_max: i32,
    z_max: i32,
) -> HashSet<(i32, i32, i32)> {
    let mut blocks = HashSet::new();
    for y in y_min..=y_max {
        for z in z_min..=z_max {
            blocks.insert((x_min, y, z));
            blocks.insert((x_max, y, z));
        }
    }
    for x in x_min..=x_max {
        for z in z_min..=z_max {
            blocks.insert((x, y_min, z));
            blocks.insert((x, y_max, z));
        }
    }
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            blocks.insert((x, y, z_min));
            blocks.insert((x, y, z_max));
        }
    }
    blocks
}

fn get_min_max(blocks: &HashSet<(i32, i32, i32)>) -> Result<(i32, i32, i32, i32, i32, i32), Error> {
    let x_min = blocks
        .iter()
        .map(|block| block.0)
        .min()
        .ok_or(InvalidInput)?;
    let y_min = blocks
        .iter()
        .map(|block| block.1)
        .min()
        .ok_or(InvalidInput)?;
    let z_min = blocks
        .iter()
        .map(|block| block.2)
        .min()
        .ok_or(InvalidInput)?;
    let x_max = blocks
        .iter()
        .map(|block| block.0)
        .max()
        .ok_or(InvalidInput)?;
    let y_max = blocks
        .iter()
        .map(|block| block.1)
        .max()
        .ok_or(InvalidInput)?;
    let z_max = blocks
        .iter()
        .map(|block| block.2)
        .max()
        .ok_or(InvalidInput)?;
    Ok((x_min, y_min, z_min, x_max, y_max, z_max))
}

fn parse_input(input: String) -> HashSet<(i32, i32, i32)> {
    input
        .lines()
        .filter_map(|line| {
            line.split(',')
                .filter_map(|n| n.parse::<i32>().ok())
                .tuples::<(_, _, _)>()
                .next()
        })
        .collect()
}

#[cfg(test)]
pub mod test {
    #[test]
    fn solve_mini_example() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/mini_example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve_part1(input).expect("failed to solve");
        assert_eq!(max_released, 10);
    }

    #[test]
    fn solve_example() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve_part1(input).expect("failed to solve");
        assert_eq!(max_released, 64);
    }

    #[test]
    fn solve_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve_part1(input).expect("failed to solve");
        assert_eq!(max_released, 3454);
    }

    #[test]
    fn solve_mini_example_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/mini_example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve_part2(input).expect("failed to solve");
        assert_eq!(max_released, 10);
    }

    #[test]
    fn solve_example_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve_part2(input).expect("failed to solve");
        assert_eq!(max_released, 58);
    }

    #[test]
    fn solve_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve_part2(input).expect("failed to solve");
        assert_eq!(max_released, 2014);
    }
}
