extern crate core;

use crate::Error::InvalidInput;

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
        .map(snafu_to_dec)
        .collect::<Result<Vec<_>, Error>>()?;
    Ok(dec_to_snafu(decimals.iter().sum()).iter().collect())
}

fn snafu_to_dec(line: &Vec<char>) -> Result<i64, Error> {
    let mut sum = 0;
    let mut n = BASE.pow((line.len() - 1) as u32) as i64;
    for c in line {
        sum += n * match c {
            '2' => Ok(2),
            '1' => Ok(1),
            '0' => Ok(0),
            '-' => Ok(-1),
            '=' => Ok(-2),
            _ => Err(InvalidInput),
        }?;
        n /= BASE as i64;
    }
    Ok(sum)
}

fn dec_to_snafu(mut n: i64) -> Vec<char> {
    let mut max_digit = 0;
    loop {
        let pow = BASE.pow(max_digit);
        let sum_smaller_digit = 2 * (pow - 1) / 4;
        if ((n.unsigned_abs() - sum_smaller_digit) + (pow - 1)) / pow <= 2 {
            break;
        }
        max_digit += 1;
    }
    let mut snafu = vec![];

    for exp in (0..=max_digit).rev() {
        let pow = BASE.pow(exp) as i64;
        let sum_smaller_digit = 2 * (pow - 1) / 4;
        let quotient = ((n.abs() - sum_smaller_digit) + (pow - 1)) / pow;
        let quotient = quotient * n.signum();
        match quotient {
            2 => snafu.push('2'),
            1 => snafu.push('1'),
            0 => snafu.push('0'),
            -1 => snafu.push('-'),
            -2 => snafu.push('='),
            _ => panic!("Unexpect quotient"),
        }
        n -= quotient * pow;
    }
    snafu
}

#[cfg(test)]
pub mod test {
    use crate::{dec_to_snafu, snafu_to_dec};

    #[test]
    fn snafu_to_dec1() {
        let dec = snafu_to_dec(&"1=-0-2".chars().collect()).expect("failed to convert number");
        assert_eq!(dec, 1747);
    }

    #[test]
    fn dec_to_snafu1() {
        let snafu: String = dec_to_snafu(12).iter().collect();
        assert_eq!(snafu, "22");
    }

    #[test]
    fn dec_to_snafu2() {
        let snafu: String = dec_to_snafu(13).iter().collect();
        assert_eq!(snafu, "1==");
    }

    #[test]
    fn dec_to_snafu3() {
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
