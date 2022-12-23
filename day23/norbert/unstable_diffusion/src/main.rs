extern crate core;

use crate::Direction::{East, North, South, West};
use std::collections::{HashMap, HashSet};

fn main() {}

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Clone, Debug)]
pub struct Elf {
    directions: Vec<Direction>,
}

impl Default for Elf {
    fn default() -> Self {
        Elf::new()
    }
}

impl Elf {
    pub fn new() -> Self {
        Elf {
            directions: vec![North, South, West, East],
        }
    }

    pub fn propose_next(
        &self,
        (x, y): (i32, i32),
        elves: &HashMap<(i32, i32), Elf>,
    ) -> ((i32, i32), Option<Direction>) {
        if has_no_neighbours((x, y), elves) {
            return ((x, y), None);
        }
        for dir in &self.directions {
            match dir {
                North => {
                    if is_free((x, y), *dir, elves) {
                        return ((x, y - 1), Some(*dir));
                    }
                }
                South => {
                    if is_free((x, y), *dir, elves) {
                        return ((x, y + 1), Some(*dir));
                    }
                }
                West => {
                    if is_free((x, y), *dir, elves) {
                        return ((x - 1, y), Some(*dir));
                    }
                }
                East => {
                    if is_free((x, y), *dir, elves) {
                        return ((x + 1, y), Some(*dir));
                    }
                }
            }
        }
        ((x, y), None)
    }

    pub fn rotate_dirs(&mut self) {
        self.directions.rotate_left(1);
    }
}

pub fn has_no_neighbours((x, y): (i32, i32), elves: &HashMap<(i32, i32), Elf>) -> bool {
    is_free((x, y), North, elves)
        && is_free((x, y), South, elves)
        && is_free((x, y), West, elves)
        && is_free((x, y), East, elves)
}

pub fn is_free((x, y): (i32, i32), dir: Direction, elves: &HashMap<(i32, i32), Elf>) -> bool {
    match dir {
        North => {
            if elves.get(&(x - 1, y - 1)).is_none()
                && elves.get(&(x, y - 1)).is_none()
                && elves.get(&(x + 1, y - 1)).is_none()
            {
                return true;
            }
        }
        South => {
            if elves.get(&(x - 1, y + 1)).is_none()
                && elves.get(&(x, y + 1)).is_none()
                && elves.get(&(x + 1, y + 1)).is_none()
            {
                return true;
            }
        }
        West => {
            if elves.get(&(x - 1, y - 1)).is_none()
                && elves.get(&(x - 1, y)).is_none()
                && elves.get(&(x - 1, y + 1)).is_none()
            {
                return true;
            }
        }
        East => {
            if elves.get(&(x + 1, y - 1)).is_none()
                && elves.get(&(x + 1, y)).is_none()
                && elves.get(&(x + 1, y + 1)).is_none()
            {
                return true;
            }
        }
    }
    false
}

pub fn solve_part1(input: String) -> Result<u32, Error> {
    let mut elves = parse_input(input);
    println!("== Initial State ==");
    print_elves(&elves);
    println!();

    for round in 1..=10 {
        let (new_elves, _) = do_round(&elves);
        elves = new_elves;
        println!("== End of Round {round} ==");
        print_elves(&elves);
        println!();
    }
    Ok(calc_empty_ground(&elves))
}

pub fn solve_part2(input: String) -> Result<u32, Error> {
    let mut elves = parse_input(input);
    let mut round = 0;
    loop {
        let (new_elves, num_elves_moved) = do_round(&elves);
        elves = new_elves;
        round += 1;
        println!("round={round}, num_elves_moved={num_elves_moved}");
        if num_elves_moved == 0 {
            break;
        }
    }
    Ok(round)
}

fn parse_input(input: String) -> HashMap<(i32, i32), Elf> {
    let mut elves = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                elves.insert((x as i32, y as i32), Elf::new());
            }
        })
    });
    elves
}

fn calc_empty_ground(elves: &HashMap<(i32, i32), Elf>) -> u32 {
    let (x_min, x_max, y_min, y_max) = calc_min_max(elves);
    let width = (x_max - x_min + 1) as u32;
    let height = (y_max - y_min + 1) as u32;
    let elves_in_rect = elves
        .keys()
        .filter(|(x, y)| x_min <= *x && *x <= x_max && y_min <= *y && *y <= y_max)
        .count() as u32;
    width * height - elves_in_rect
}

fn do_round(elves: &HashMap<(i32, i32), Elf>) -> (HashMap<(i32, i32), Elf>, u32) {
    let proposals: Vec<_> = elves
        .iter()
        .map(|(pos, elf)| {
            let (prop_pos, prop_dir) = elf.propose_next(*pos, elves);
            (*pos, prop_dir, prop_pos)
        })
        .collect();
    let mut prop_positions = HashSet::new();
    let mut conflicts = HashSet::new();
    for (_pos, prop_dir, prop_pos) in &proposals {
        if prop_dir.is_some() {
            if prop_positions.get(&prop_pos).is_some() {
                conflicts.insert(prop_pos);
            } else {
                prop_positions.insert(prop_pos);
            }
        }
    }
    let mut new_elves = HashMap::new();
    let mut num_elves_moved = 0;
    for (pos, _prop_dir, prop_pos) in &proposals {
        let mut elf = elves.get(pos).expect("failed to find elf").clone();
        elf.rotate_dirs();
        if conflicts.contains(prop_pos) {
            new_elves.insert(*pos, elf);
        } else {
            new_elves.insert(*prop_pos, elf.clone());
            if pos != prop_pos {
                num_elves_moved += 1;
            }
        }
    }
    (new_elves, num_elves_moved)
}

fn calc_min_max(elves: &HashMap<(i32, i32), Elf>) -> (i32, i32, i32, i32) {
    let x_min = elves
        .keys()
        .map(|(x, _)| x)
        .min()
        .expect("failed to determine minimum");
    let x_max = elves
        .keys()
        .map(|(x, _)| x)
        .max()
        .expect("failed to determine maximum");
    let y_min = elves
        .keys()
        .map(|(_, y)| y)
        .min()
        .expect("failed to determine minimum");
    let y_max = elves
        .keys()
        .map(|(_, y)| y)
        .max()
        .expect("failed to determine maximum");
    (*x_min, *x_max, *y_min, *y_max)
}

fn print_elves(elves: &HashMap<(i32, i32), Elf>) {
    let (x_min, x_max, y_min, y_max) = calc_min_max(elves);
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            match elves.get(&(x, y)) {
                None => {
                    print!(".")
                }
                Some(_) => {
                    print!("#")
                }
            }
        }
        println!();
    }
}

#[cfg(test)]
pub mod test {
    #[test]
    fn solve_mini_example() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/mini_example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve_part1(input).expect("failed to solve");
        assert_eq!(sol, 25);
    }

    #[test]
    fn solve_example() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve_part1(input).expect("failed to solve");
        assert_eq!(sol, 110);
    }

    #[test]
    fn solve_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve_part1(input).expect("failed to solve");
        assert_eq!(sol, 3849);
    }

    #[test]
    fn solve_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve_part2(input).expect("failed to solve");
        assert_eq!(sol, 995);
    }
}
