extern crate core;

use crate::Error::InvalidInput;
use crate::Item::{ClosingBracket, Number, OpeningBracket};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

pub enum Item {
    Number(u32),
    OpeningBracket,
    ClosingBracket,
}

fn main() {}

pub fn solve(input: String) -> Result<u32, Error> {
    Ok(input
        .lines()
        .filter(|line| !line.is_empty())
        .tuples::<(_, _)>()
        .enumerate()
        .filter(|(_, (left, right))| is_right_order(left, right))
        .map(|(index, (_, _))| index + 1)
        .sum::<usize>() as u32)
}

pub fn add_and_sort_marks(mut input: String) -> Result<u32, Error> {
    const DIVIDER1: &str = "[[2]]";
    const DIVIDER2: &str = "[[6]]";
    input += "\n";
    input += DIVIDER1;
    input += "\n";
    input += DIVIDER2;

    let sorted: Vec<_> = input
        .lines()
        .filter(|line| !line.is_empty())
        .sorted_by(|line1, line2| {
            if is_right_order(line1, line2) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .collect();
    let mark1 = sorted.iter().position(|line| line == &DIVIDER1);
    let mark2 = sorted.iter().position(|line| line == &DIVIDER2);
    match (mark1, mark2) {
        (Some(m1), Some(m2)) => Ok(((m1 + 1) * (m2 + 1)) as u32),
        _ => Err(InvalidInput),
    }
}

fn is_right_order(left: &str, right: &str) -> bool {
    let mut left = preprocess(left).expect("failed to preprocess data");
    let mut right = preprocess(right).expect("failed to preprocess data");

    loop {
        let l_next = left.pop_front();
        let r_next = right.pop_front();
        match (l_next, r_next) {
            (None, _) => {
                return true; // Left side ran out of items, so inputs are in the right order
            }
            (_, None) => {
                return false; // Right side ran out of items, so inputs are not in the right order
            }
            (Some(Number(l_n)), Some(Number(r_n))) => {
                if l_n < r_n {
                    return true;
                }
                if l_n > r_n {
                    return false;
                }
                // Do nothing with this pair and continue as numbers are same
            }
            (Some(Number(l_n)), Some(OpeningBracket)) => {
                // Wrap number into list with one element and retry
                left.push_front(ClosingBracket);
                left.push_front(Number(l_n));
                left.push_front(OpeningBracket);
                right.push_front(OpeningBracket);
            }
            (Some(OpeningBracket), Some(Number(r_n))) => {
                // Wrap number into list with one element and retry
                right.push_front(ClosingBracket);
                right.push_front(Number(r_n));
                right.push_front(OpeningBracket);
                left.push_front(OpeningBracket);
            }
            (Some(ClosingBracket), Some(ClosingBracket)) => {}
            (Some(OpeningBracket), Some(OpeningBracket)) => {}
            (Some(ClosingBracket), _) => {
                return true; // Left side ran out of items, so inputs are in the right order
            }
            (_, Some(ClosingBracket)) => {
                return false; // Right side ran out of items, so inputs are not in the right order
            }
        }
    }
}

fn preprocess(input: &str) -> Result<VecDeque<Item>, Error> {
    let mut ret = VecDeque::new();
    let mut digits = VecDeque::new();
    for c in input.chars() {
        match c {
            '[' => ret.push_back(OpeningBracket),
            ']' => {
                push_digits(&mut ret, &mut digits)?;
                ret.push_back(ClosingBracket)
            }
            ',' => {
                push_digits(&mut ret, &mut digits)?;
            }
            _ => {
                if c.is_numeric() {
                    digits.push_back(c)
                } else {
                    return Err(InvalidInput);
                }
            }
        }
    }
    Ok(ret)
}

fn push_digits(ret: &mut VecDeque<Item>, digits: &mut VecDeque<char>) -> Result<(), Error> {
    if !digits.is_empty() {
        let number: String = digits.iter().collect();
        ret.push_back(Number(number.parse().map_err(|_| InvalidInput)?));
        digits.clear();
    }
    Ok(())
}

#[cfg(test)]
pub mod test {
    #[test]
    fn solve_example() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sum = crate::solve(input).expect("failed to solve");
        assert_eq!(sum, 13);
    }

    #[test]
    fn solve_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sum = crate::solve(input).expect("failed to solve");
        assert_eq!(sum, 5905);
    }

    #[test]
    fn solve_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let product = crate::add_and_sort_marks(input).expect("failed to sort");
        assert_eq!(product, 21691);
    }
}
