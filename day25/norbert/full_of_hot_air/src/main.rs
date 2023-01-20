extern crate core;

use crate::Error::InvalidInput;
use std::collections::VecDeque;

fn main() {}

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

const BASE: u64 = 5;

pub fn solve(input: String) -> Result<String, Error> {
    let lines: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let decimals = lines
        .iter()
        .map(|a| snafu_to_dec(a))
        .collect::<Result<Vec<_>, Error>>()?;
    Ok(dec_to_snafu(decimals.iter().sum()).iter().collect())
}

fn snafu_to_dec(line: &[char]) -> Result<i64, Error> {
    let mut sum = 0;
    for (i, c) in line.iter().rev().enumerate() {
        sum += BASE.pow(i as u32) as i64
            * match c {
                '2' => Ok(2),
                '1' => Ok(1),
                '0' => Ok(0),
                '-' => Ok(-1),
                '=' => Ok(-2),
                _ => Err(InvalidInput),
            }?;
    }
    Ok(sum)
}

fn dec_to_snafu(mut n: i64) -> VecDeque<char> {
    if n == 0 {
        return VecDeque::from(['0']);
    }
    let mut snafu = VecDeque::new();
    while n != 0 {
        match n % BASE as i64 {
            0 => {
                snafu.push_front('0');
            }
            1 => {
                snafu.push_front('1');
                n -= 1;
            }
            2 => {
                snafu.push_front('2');
                n -= 2;
            }
            3 => {
                snafu.push_front('=');
                n += 2;
            }
            4 => {
                snafu.push_front('-');
                n += 1;
            }
            _ => panic!("Unexpect remainder"),
        }
        n /= BASE as i64;
    }
    snafu
}

#[cfg(test)]
pub mod test {
    use crate::{dec_to_snafu, snafu_to_dec};

    #[test]
    fn snafu_to_dec1() {
        let dec = snafu_to_dec(&"1=-0-2".chars().collect::<Vec<char>>())
            .expect("failed to convert number");
        assert_eq!(dec, 1747);
    }

    #[test]
    fn dec_to_snafu0() {
        let snafu: String = dec_to_snafu(0).iter().collect();
        assert_eq!(snafu, "0");
    }

    #[test]
    fn dec_to_snafu1() {
        let snafu: String = dec_to_snafu(1).iter().collect();
        assert_eq!(snafu, "1");
    }

    #[test]
    fn dec_to_snafu2() {
        let snafu: String = dec_to_snafu(2).iter().collect();
        assert_eq!(snafu, "2");
    }

    #[test]
    fn dec_to_snafu3() {
        let snafu: String = dec_to_snafu(3).iter().collect();
        assert_eq!(snafu, "1=");
    }

    #[test]
    fn dec_to_snafu12() {
        let snafu: String = dec_to_snafu(12).iter().collect();
        assert_eq!(snafu, "22");
    }

    #[test]
    fn dec_to_snafu13() {
        let snafu: String = dec_to_snafu(13).iter().collect();
        assert_eq!(snafu, "1==");
    }

    #[test]
    fn dec_to_snafu1747() {
        let snafu: String = dec_to_snafu(1747).iter().collect();
        assert_eq!(snafu, "1=-0-2");
    }

    #[test]
    fn solve_example() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve(input).expect("failed to solve");
        assert_eq!(sol, "2=-1=0");
    }

    #[test]
    fn solve_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve(input).expect("failed to solve");
        assert_eq!(sol, "2=10---0===-1--01-20");
    }
}
