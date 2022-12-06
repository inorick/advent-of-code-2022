extern crate core;

use itertools::Itertools;

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

fn main() {}

pub fn solve<const WINDOW_SIZE: usize>(input: String) -> Result<usize, Error> {
    Ok(input
        .chars()
        .collect::<Vec<char>>()
        .as_slice()
        .windows(WINDOW_SIZE)
        .enumerate()
        .find(|(_, seq)| seq.iter().unique().count() == WINDOW_SIZE)
        .map(|(i, _)| i)
        .ok_or(Error::InvalidInput)?
        + WINDOW_SIZE)
}

#[cfg(test)]
pub mod test {
    #[test]
    fn solve_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve::<4>(input).expect("failed to solve");
        assert_eq!(sol, 1210);
    }

    #[test]
    fn solve_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve::<14>(input).expect("failed to solve");
        assert_eq!(sol, 3476);
    }
}
