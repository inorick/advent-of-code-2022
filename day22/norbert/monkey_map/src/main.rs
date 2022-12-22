extern crate core;

use crate::Error::InvalidInput;
use crate::Tile::{Blocked, Free};
use std::collections::{HashMap, VecDeque};

fn main() {}

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Free,
    Blocked,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub enum Turn {
    Left,
    Right,
}

pub fn solve(input: String, cube_size: Option<usize>) -> Result<u32, Error> {
    let lines: Vec<_> = input.lines().take(input.lines().count() - 2).collect();
    let dirs = input.lines().last().ok_or(InvalidInput)?;
    let mut map = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => Some(Free),
                '#' => Some(Blocked),
                _ => None,
            }
            .map(|tile| map.insert((x, y), tile));
        }
    }
    let start = *map
        .iter()
        .filter(|((_, y), t)| *y == 0 && **t == Free)
        .min_by_key(|((x, _), _)| x)
        .ok_or(InvalidInput)?
        .0;
    let mut steps: VecDeque<_> = dirs
        .split(|c: char| !c.is_numeric())
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();
    let mut turns: VecDeque<_> = dirs
        .chars()
        .filter_map(|c| match c {
            'L' => Some(Turn::Left),
            'R' => Some(Turn::Right),
            _ => None,
        })
        .collect();
    if steps.len() != turns.len() + 1 {
        return Err(InvalidInput);
    }

    let mut pos = start;
    let mut dir = Direction::Right;
    println!("pos={pos:?}, dir={dir:?}");
    while !steps.is_empty() {
        let steps = steps.pop_front().unwrap();
        for _ in 0..steps {
            (pos, dir) = next_pos(pos.0, pos.1, dir, cube_size, &map);
        }
        if let Some(turn) = turns.pop_front() {
            dir = match turn {
                Turn::Left => match dir {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                },
                Turn::Right => match dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                },
            }
        }
        println!("pos={pos:?}, dir={dir:?}");
    }
    let score = 1000 * (pos.1 + 1)
        + 4 * (pos.0 + 1)
        + match dir {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        };
    println!("Final score: {score}");
    println!();
    Ok(score as u32)
}

pub fn next_pos(
    x: usize,
    y: usize,
    dir: Direction,
    cube_size: Option<usize>,
    map: &HashMap<(usize, usize), Tile>,
) -> ((usize, usize), Direction) {
    match dir {
        Direction::Up => {
            if y == 0 || map.get(&(x, y - 1)).is_none() {
                let wrapped_pos = if let Some(cube_size) = cube_size {
                    wrap_around_cube(x, y, dir, cube_size, map)
                } else {
                    (wrap_around(x, y, dir, map), dir)
                };
                match map.get(&wrapped_pos.0).unwrap() {
                    Free => wrapped_pos,
                    Blocked => ((x, y), dir),
                }
            } else {
                match map.get(&(x, y - 1)).unwrap() {
                    Free => ((x, y - 1), dir),
                    Blocked => ((x, y), dir),
                }
            }
        }
        Direction::Down => {
            if map.get(&(x, y + 1)).is_none() {
                let wrapped_pos = if let Some(cube_size) = cube_size {
                    wrap_around_cube(x, y, dir, cube_size, map)
                } else {
                    (wrap_around(x, y, dir, map), dir)
                };
                match map.get(&wrapped_pos.0).unwrap() {
                    Free => wrapped_pos,
                    Blocked => ((x, y), dir),
                }
            } else {
                match map.get(&(x, y + 1)).unwrap() {
                    Free => ((x, y + 1), dir),
                    Blocked => ((x, y), dir),
                }
            }
        }
        Direction::Left => {
            if x == 0 || map.get(&(x - 1, y)).is_none() {
                let wrapped_pos = if let Some(cube_size) = cube_size {
                    wrap_around_cube(x, y, dir, cube_size, map)
                } else {
                    (wrap_around(x, y, dir, map), dir)
                };
                match map.get(&wrapped_pos.0).unwrap() {
                    Free => wrapped_pos,
                    Blocked => ((x, y), dir),
                }
            } else {
                match map.get(&(x - 1, y)).unwrap() {
                    Free => ((x - 1, y), dir),
                    Blocked => ((x, y), dir),
                }
            }
        }
        Direction::Right => {
            if map.get(&(x + 1, y)).is_none() {
                let wrapped_pos = if let Some(cube_size) = cube_size {
                    wrap_around_cube(x, y, dir, cube_size, map)
                } else {
                    (wrap_around(x, y, dir, map), dir)
                };
                match map.get(&wrapped_pos.0).unwrap() {
                    Free => wrapped_pos,
                    Blocked => ((x, y), dir),
                }
            } else {
                match map.get(&(x + 1, y)).unwrap() {
                    Free => ((x + 1, y), dir),
                    Blocked => ((x, y), dir),
                }
            }
        }
    }
}

pub fn wrap_around(
    mut x: usize,
    mut y: usize,
    dir: Direction,
    map: &HashMap<(usize, usize), Tile>,
) -> (usize, usize) {
    match dir {
        Direction::Up => {
            while map.get(&(x, y + 1)).is_some() {
                y += 1;
            }
        }
        Direction::Down => {
            while y > 0 && map.get(&(x, y - 1)).is_some() {
                y -= 1;
            }
        }
        Direction::Left => {
            while map.get(&(x + 1, y)).is_some() {
                x += 1;
            }
        }
        Direction::Right => {
            while x > 0 && map.get(&(x - 1, y)).is_some() {
                x -= 1;
            }
        }
    }
    (x, y)
}

pub fn wrap_around_cube(
    x: usize,
    y: usize,
    dir: Direction,
    cube_size: usize,
    map: &HashMap<(usize, usize), Tile>,
) -> ((usize, usize), Direction) {
    match dir {
        Direction::Up => {
            if x < cube_size {
                return if y == 2 * cube_size {
                    ((cube_size, cube_size + x), Direction::Right)
                } else if map.get(&(x, y - 1)).is_some() {
                    ((x, y - 1), dir)
                } else {
                    ((x, y), dir)
                };
            } else if x < 2 * cube_size {
                return if y == 0 {
                    ((0, 3 * cube_size + (x - cube_size)), Direction::Right)
                } else if map.get(&(x, y - 1)).is_some() {
                    ((x, y - 1), dir)
                } else {
                    ((x, y), dir)
                };
            } else if x < 3 * cube_size {
                return if y == 0 {
                    ((x - 2 * cube_size, 4 * cube_size - 1), Direction::Up)
                } else if map.get(&(x, y - 1)).is_some() {
                    ((x, y - 1), dir)
                } else {
                    ((x, y), dir)
                };
            } else {
                panic!("Invalid value of x: {x}");
            }
        }
        Direction::Down => {
            if x < cube_size {
                return if y == 4 * cube_size - 1 {
                    ((x + 2 * cube_size, 0), Direction::Down)
                } else if map.get(&(x, y + 1)).is_some() {
                    ((x, y + 1), dir)
                } else {
                    ((x, y), dir)
                };
            } else if x < 2 * cube_size {
                return if y == 3 * cube_size - 1 {
                    (
                        (cube_size - 1, 3 * cube_size + (x - cube_size)),
                        Direction::Left,
                    )
                } else if map.get(&(x, y + 1)).is_some() {
                    ((x, y + 1), dir)
                } else {
                    ((x, y), dir)
                };
            } else if x < 3 * cube_size {
                return if y == cube_size - 1 {
                    (
                        (2 * cube_size - 1, cube_size + (x - 2 * cube_size)),
                        Direction::Left,
                    )
                } else if map.get(&(x, y + 1)).is_some() {
                    ((x, y + 1), dir)
                } else {
                    ((x, y), dir)
                };
            } else {
                panic!("Invalid value of x: {x}");
            }
        }
        Direction::Left => {
            if y < cube_size {
                return if x == cube_size {
                    ((0, 3 * cube_size - 1 - y), Direction::Right)
                } else if map.get(&(x - 1, y)).is_some() {
                    ((x - 1, y), dir)
                } else {
                    ((x, y), dir)
                };
            } else if y < 2 * cube_size {
                return if x == cube_size {
                    ((y - cube_size, 2 * cube_size), Direction::Down)
                } else if map.get(&(x - 1, y)).is_some() {
                    ((x - 1, y), dir)
                } else {
                    ((x, y), dir)
                };
            } else if y < 3 * cube_size {
                return if x == 0 {
                    (
                        (cube_size, cube_size - 1 - (y - 2 * cube_size)),
                        Direction::Right,
                    )
                } else if map.get(&(x - 1, y)).is_some() {
                    ((x - 1, y), dir)
                } else {
                    ((x, y), dir)
                };
            } else if y < 4 * cube_size {
                return if x == 0 {
                    ((cube_size + (y - 3 * cube_size), 0), Direction::Down)
                } else if map.get(&(x - 1, y)).is_some() {
                    ((x - 1, y), dir)
                } else {
                    ((x, y), dir)
                };
            } else {
                panic!("Invalid value of x: {x}");
            }
        }
        Direction::Right => {
            if y < cube_size {
                return if x == 3 * cube_size - 1 {
                    ((2 * cube_size - 1, 3 * cube_size - 1 - y), Direction::Left)
                } else if map.get(&(x + 1, y)).is_some() {
                    ((x + 1, y), dir)
                } else {
                    ((x, y), dir)
                };
            } else if y < 2 * cube_size {
                return if x == 2 * cube_size - 1 {
                    (
                        (2 * cube_size + (y - cube_size), cube_size - 1),
                        Direction::Up,
                    )
                } else if map.get(&(x + 1, y)).is_some() {
                    ((x + 1, y), dir)
                } else {
                    ((x, y), dir)
                };
            } else if y < 3 * cube_size {
                return if x == 2 * cube_size - 1 {
                    (
                        (3 * cube_size - 1, cube_size - 1 - (y - 2 * cube_size)),
                        Direction::Left,
                    )
                } else if map.get(&(x + 1, y)).is_some() {
                    ((x + 1, y), dir)
                } else {
                    ((x, y), dir)
                };
            } else if y < 4 * cube_size {
                return if x == cube_size - 1 {
                    (
                        (cube_size + (y - 3 * cube_size), 3 * cube_size - 1),
                        Direction::Up,
                    )
                } else if map.get(&(x + 1, y)).is_some() {
                    ((x + 1, y), dir)
                } else {
                    ((x, y), dir)
                };
            } else {
                panic!("Invalid value of x: {x}");
            }
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
        let sol = crate::solve(input, None).expect("failed to solve");
        assert_eq!(sol, 6032);
    }

    #[test]
    fn solve_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve(input, None).expect("failed to solve");
        assert_eq!(sol, 117054);
    }

    #[test]
    fn solve_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve(input, Some(50)).expect("failed to solve");
        assert_eq!(sol, 162096);
    }
}
