extern crate core;

use crate::Direction::{Down, Left, Right, Up};
use crate::Error::InvalidInput;
use itertools::Itertools;
use pathfinding::prelude::astar;
use std::collections::HashMap;

fn main() {}

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn solve(input: String, part2: bool) -> Result<u32, Error> {
    let lines: Vec<_> = input.lines().collect();
    let first = lines.first().expect("no first line");
    let last = lines.last().expect("no last line");
    let width = first.len();
    let height = lines.len();
    let start = (
        first
            .chars()
            .enumerate()
            .find(|(_, c)| *c == '.')
            .expect("no start")
            .0,
        0,
    );
    let goal = (
        last.chars()
            .enumerate()
            .find(|(_, c)| *c == '.')
            .expect("no goal")
            .0,
        height - 1,
    );
    let mut blizzards = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '^' => blizzards.insert((x, y), Up),
                'v' => blizzards.insert((x, y), Down),
                '<' => blizzards.insert((x, y), Left),
                '>' => blizzards.insert((x, y), Right),
                _ => None,
            };
        }
    }
    if !part2 {
        shortest_path(width, height, start, goal, 0, &blizzards)
    } else {
        let steps1 = shortest_path(width, height, start, goal, 0, &blizzards)? as usize;
        let (start, goal) = (goal, start);
        let steps2 = shortest_path(width, height, start, goal, steps1, &blizzards)? as usize;
        let (start, goal) = (goal, start);
        let steps3 =
            shortest_path(width, height, start, goal, steps1 + steps2, &blizzards)? as usize;
        Ok((steps1 + steps2 + steps3) as u32)
    }
}

fn shortest_path(
    width: usize,
    height: usize,
    start: (usize, usize),
    goal: (usize, usize),
    init_step: usize,
    blizzards: &HashMap<(usize, usize), Direction>,
) -> Result<u32, Error> {
    let successors = |(x, y, step): &(usize, usize, usize)| {
        let next_blizzards = move_blizzards(blizzards, step + 1, width, height);
        let state = (*x, *y, *step);
        next(width, height, start, goal, state, &next_blizzards)
    };
    let heuristic = |(x, y, _step): &(usize, usize, usize)| x.abs_diff(goal.0) + y.abs_diff(goal.1);
    let success = |(x, y, _step): &(usize, usize, usize)| *x == goal.0 && *y == goal.1;
    let result = astar(
        &(start.0, start.1, init_step),
        successors,
        heuristic,
        success,
    );
    match &result {
        None => {
            println!("No solution found!");
            Err(InvalidInput)
        }
        Some(result) => {
            let steps = result.0.len() - 1;
            println!("Shortest path takes {steps} steps:");
            for step in &result.0 {
                println!("{step:?}");
            }
            Ok(steps as u32)
        }
    }
}

pub fn next(
    width: usize,
    height: usize,
    start: (usize, usize),
    goal: (usize, usize),
    (x, y, step): (usize, usize, usize),
    next_blizzards: &HashMap<(usize, usize), Direction>,
) -> Vec<((usize, usize, usize), usize)> {
    const COST: usize = 1;
    let mut next_positions = vec![];
    {
        let wait = (x, y);
        if !next_blizzards.keys().contains(&wait) {
            next_positions.push(((wait.0, wait.1, step + 1), COST));
        }
    }
    if y >= 1 {
        let up = (x, y - 1);
        let would_hit_border = y == 1 && up != start && up != goal;
        let would_hit_blizzard = next_blizzards.keys().contains(&up);
        if !would_hit_border && !would_hit_blizzard {
            next_positions.push(((up.0, up.1, step + 1), COST));
        }
    }
    if y <= height - 2 {
        let down = (x, y + 1);
        let would_hit_border = y == height - 2 && down != start && down != goal;
        let would_hit_blizzard = next_blizzards.keys().contains(&down);
        if !would_hit_border && !would_hit_blizzard {
            next_positions.push(((down.0, down.1, step + 1), COST));
        }
    }
    if x >= 2 && y != 0 && y != height - 1 {
        let left = (x - 1, y);
        let would_hit_blizzard = next_blizzards.keys().contains(&left);
        if !would_hit_blizzard {
            next_positions.push(((left.0, left.1, step + 1), COST));
        }
    }
    if x < width - 2 && y != 0 && y != height - 1 {
        let right = (x + 1, y);
        let would_hit_blizzard = next_blizzards.keys().contains(&right);
        if !would_hit_blizzard {
            next_positions.push(((right.0, right.1, step + 1), COST));
        }
    }
    next_positions
}

pub fn move_blizzards(
    blizzards: &HashMap<(usize, usize), Direction>,
    steps: usize,
    width: usize,
    height: usize,
) -> HashMap<(usize, usize), Direction> {
    let inner_width = width - 2;
    let inner_height = height - 2;
    let mut moved_blizzards = HashMap::new();
    for (pos, dir) in blizzards {
        let (x, y) = move_blizzard(steps, inner_width, inner_height, *pos, *dir);
        assert!(x >= 0 && y >= 0);
        moved_blizzards.insert((x as usize, y as usize), *dir);
    }
    moved_blizzards
}

pub fn move_blizzard(
    steps: usize,
    inner_width: usize,
    inner_height: usize,
    pos: (usize, usize),
    dir: Direction,
) -> (i32, i32) {
    let x = pos.0 as i32;
    let y = pos.1 as i32;
    let vert_steps = (steps % inner_height) as i32;
    let horiz_steps = (steps % inner_width) as i32;
    let inner_width = inner_width as i32;
    let inner_height = inner_height as i32;
    let (x, y) = match dir {
        Up => (x, (y - 1 - vert_steps + inner_height) % inner_height + 1),
        Down => (x, (y - 1 + vert_steps) % inner_height + 1),
        Left => ((x - 1 - horiz_steps + inner_width) % inner_width + 1, y),
        Right => ((x - 1 + horiz_steps) % inner_width + 1, y),
    };
    (x, y)
}

pub fn print_field(width: usize, height: usize, blizzards: &HashMap<(usize, usize), Direction>) {
    for y in 0..height {
        for x in 0..width {
            match blizzards.get(&(x, y)) {
                Some(dir) => match dir {
                    Up => {
                        print!("^");
                    }
                    Down => {
                        print!("v");
                    }
                    Left => {
                        print!("<");
                    }
                    Right => {
                        print!(">");
                    }
                },
                None => {
                    print!(".");
                }
            }
        }
        println!();
    }
}

#[cfg(test)]
pub mod test {
    use crate::move_blizzard;
    use crate::Direction::{Down, Left, Right, Up};

    #[test]
    fn move_blizzard_right() {
        let moved_pos = move_blizzard(0, 5, 5, (2, 3), Right);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(1, 5, 5, (2, 3), Right);
        assert_eq!(moved_pos.0, 3);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(2, 5, 5, (2, 3), Right);
        assert_eq!(moved_pos.0, 4);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(3, 5, 5, (2, 3), Right);
        assert_eq!(moved_pos.0, 5);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(4, 5, 5, (2, 3), Right);
        assert_eq!(moved_pos.0, 1);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(5, 5, 5, (2, 3), Right);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(6, 5, 5, (2, 3), Right);
        assert_eq!(moved_pos.0, 3);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(7, 5, 5, (2, 3), Right);
        assert_eq!(moved_pos.0, 4);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(8, 5, 5, (2, 3), Right);
        assert_eq!(moved_pos.0, 5);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(4, 5, 5, (2, 3), Right);
        assert_eq!(moved_pos.0, 1);
        assert_eq!(moved_pos.1, 3);
    }

    #[test]
    fn move_blizzard_left() {
        let moved_pos = move_blizzard(0, 5, 5, (2, 3), Left);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(1, 5, 5, (2, 3), Left);
        assert_eq!(moved_pos.0, 1);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(2, 5, 5, (2, 3), Left);
        assert_eq!(moved_pos.0, 5);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(3, 5, 5, (2, 3), Left);
        assert_eq!(moved_pos.0, 4);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(4, 5, 5, (2, 3), Left);
        assert_eq!(moved_pos.0, 3);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(5, 5, 5, (2, 3), Left);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(6, 5, 5, (2, 3), Left);
        assert_eq!(moved_pos.0, 1);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(7, 5, 5, (2, 3), Left);
        assert_eq!(moved_pos.0, 5);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(8, 5, 5, (2, 3), Left);
        assert_eq!(moved_pos.0, 4);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(4, 5, 5, (2, 3), Left);
        assert_eq!(moved_pos.0, 3);
        assert_eq!(moved_pos.1, 3);
    }

    #[test]
    fn move_blizzard_up() {
        let moved_pos = move_blizzard(0, 5, 5, (2, 3), Up);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(1, 5, 5, (2, 3), Up);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 2);
        let moved_pos = move_blizzard(2, 5, 5, (2, 3), Up);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 1);
        let moved_pos = move_blizzard(3, 5, 5, (2, 3), Up);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 5);
        let moved_pos = move_blizzard(4, 5, 5, (2, 3), Up);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 4);
        let moved_pos = move_blizzard(5, 5, 5, (2, 3), Up);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(6, 5, 5, (2, 3), Up);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 2);
        let moved_pos = move_blizzard(7, 5, 5, (2, 3), Up);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 1);
        let moved_pos = move_blizzard(8, 5, 5, (2, 3), Up);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 5);
        let moved_pos = move_blizzard(4, 5, 5, (2, 3), Up);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 4);
    }

    #[test]
    fn move_blizzard_down() {
        let moved_pos = move_blizzard(0, 5, 5, (2, 3), Down);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(1, 5, 5, (2, 3), Down);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 4);
        let moved_pos = move_blizzard(2, 5, 5, (2, 3), Down);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 5);
        let moved_pos = move_blizzard(3, 5, 5, (2, 3), Down);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 1);
        let moved_pos = move_blizzard(4, 5, 5, (2, 3), Down);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 2);
        let moved_pos = move_blizzard(5, 5, 5, (2, 3), Down);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 3);
        let moved_pos = move_blizzard(6, 5, 5, (2, 3), Down);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 4);
        let moved_pos = move_blizzard(7, 5, 5, (2, 3), Down);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 5);
        let moved_pos = move_blizzard(8, 5, 5, (2, 3), Down);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 1);
        let moved_pos = move_blizzard(4, 5, 5, (2, 3), Down);
        assert_eq!(moved_pos.0, 2);
        assert_eq!(moved_pos.1, 2);
    }

    #[test]
    fn solve_mini_example() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/mini_example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve(input, false).expect("failed to solve");
        assert_eq!(sol, 10);
    }

    #[test]
    fn solve_example() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve(input, false).expect("failed to solve");
        assert_eq!(sol, 18);
    }

    #[test]
    fn solve_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve(input, false).expect("failed to solve");
        assert_eq!(sol, 260);
    }

    #[test]
    fn solve_mini_example_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/mini_example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve(input, true).expect("failed to solve");
        assert_eq!(sol, 30);
    }

    #[test]
    fn solve_example_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve(input, true).expect("failed to solve");
        assert_eq!(sol, 54);
    }

    #[test]
    fn solve_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve(input, true).expect("failed to solve");
        assert_eq!(sol, 747);
    }
}
