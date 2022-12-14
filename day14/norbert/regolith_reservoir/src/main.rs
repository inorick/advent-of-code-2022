use crate::Error::InvalidInput;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

fn main() {
    println!("Hello, world!");
}

pub fn solve<const PART2: bool>(input: String) -> Result<u32, Error> {
    // Parse input
    let mut map = HashSet::new();
    let lines = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    point
                        .split(',')
                        .filter_map(|coord| coord.parse::<u32>().ok())
                        .tuples::<(_, _)>()
                        .next()
                        .ok_or(InvalidInput)
                })
                .collect()
        })
        .collect::<Result<Vec<Vec<(_, _)>>, Error>>()?;

    // Build rock map
    for line in lines {
        for ((x1, y1), (x2, y2)) in line.iter().tuple_windows() {
            // P1 is to the upper left of P2
            let (p1, p2): ((u32, u32), (u32, u32)) = if x1 <= x2 && y1 <= y2 {
                ((*x1, *y1), (*x2, *y2))
            } else {
                ((*x2, *y2), (*x1, *y1))
            };
            for i in 0..=p2.0 - p1.0 {
                map.insert((p1.0 + i, p1.1)); // Draw horizontal line
            }
            for i in 0..=p2.1 - p1.1 {
                map.insert((p1.0, p1.1 + i)); // Draw vertical line
            }
        }
    }

    // Draw floor for part 2 of challenge
    if PART2 {
        let y_max = *map.iter().map(|(_, y)| y).max().ok_or(InvalidInput)?;
        for x in 0..=1000 {
            map.insert((x, y_max + 2));
        }
    }

    // Drop sand
    const SAND_ORIGIN: (u32, u32) = (500, 0);
    const ABYSS_THRESHOLD: u32 = 1000;
    let mut sand = SAND_ORIGIN;
    let mut counter = 0;
    loop {
        if map.get(&SAND_ORIGIN).is_some() || sand.1 >= ABYSS_THRESHOLD {
            break;
        }
        let below = (sand.0, sand.1 + 1);
        let left_below = (sand.0 - 1, sand.1 + 1);
        let right_below = (sand.0 + 1, sand.1 + 1);
        if map.get(&below).is_none() {
            sand = below;
        } else if map.get(&left_below).is_none() {
            sand = left_below;
        } else if map.get(&right_below).is_none() {
            sand = right_below;
        } else {
            map.insert(sand); // Sand settled here
            counter += 1;
            sand = SAND_ORIGIN;
        }
    }
    Ok(counter)
}

#[cfg(test)]
pub mod test {
    #[test]
    fn solve_example() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sum = crate::solve::<false>(input).expect("failed to solve");
        assert_eq!(sum, 24);
    }

    #[test]
    fn solve_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sum = crate::solve::<false>(input).expect("failed to solve");
        assert_eq!(sum, 979);
    }

    #[test]
    fn solve_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sum = crate::solve::<true>(input).expect("failed to solve");
        assert_eq!(sum, 29044);
    }
}
