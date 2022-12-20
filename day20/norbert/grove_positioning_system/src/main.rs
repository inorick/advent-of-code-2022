extern crate core;

use crate::Error::InvalidInput;

fn main() {}

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

pub fn solve(input: String, part2: bool) -> Result<i64, Error> {
    const DECRYPTION_KEY: i64 = 811589153;
    let effective_key = if part2 { DECRYPTION_KEY } else { 1 };

    let mut numbers: Vec<_> = input
        .lines()
        .filter_map(|line| line.parse::<i64>().ok())
        .map(|n| n * effective_key)
        .enumerate()
        .collect();
    let modulus = numbers.len();

    let max_rounds = if part2 { 10 } else { 1 };
    for _ in 0..max_rounds {
        for initial_i in 0..modulus {
            let src_i = numbers.iter().position(|(i, _)| *i == initial_i).unwrap();
            let number = numbers.remove(src_i);
            let dest_i = modulo_pos((src_i as i64) + number.1, modulus - 1);
            numbers.insert(dest_i, number);
        }
    }

    let zero_index = numbers.iter().position(|n| n.1 == 0).ok_or(InvalidInput)?;
    let i1 = modulo_pos((zero_index + 1000) as i64, modulus);
    let i2 = modulo_pos((zero_index + 2000) as i64, modulus);
    let i3 = modulo_pos((zero_index + 3000) as i64, modulus);
    let n1 = numbers.get(i1).unwrap().1;
    let n2 = numbers.get(i2).unwrap().1;
    let n3 = numbers.get(i3).unwrap().1;

    Ok(n1 + n2 + n3)
}

fn modulo_pos(n: i64, m: usize) -> usize {
    ((n % m as i64) + m as i64) as usize % m
}

#[cfg(test)]
pub mod test {
    #[test]
    fn solve_example() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve(input, false).expect("failed to solve");
        assert_eq!(sol, 3);
    }

    #[test]
    fn solve_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve(input, false).expect("failed to solve");
        assert_eq!(sol, 7228);
    }

    #[test]
    fn solve_example_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve(input, true).expect("failed to solve");
        assert_eq!(sol, 1623178306);
    }

    #[test]
    fn solve_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve(input, true).expect("failed to solve");
        assert_eq!(sol, 4526232706281);
    }
}
