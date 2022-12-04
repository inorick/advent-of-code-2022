use std::{
    fs::File,
    io::{self, BufRead},
    ops::RangeInclusive,
};

fn get_input(path: &str) -> Vec<String> {
    let file = File::open(path).expect("could not read file input.txt");
    let lines: Result<Vec<String>, std::io::Error> = io::BufReader::new(file).lines().collect();
    lines.unwrap()
}

fn into_range(a: &str) -> RangeInclusive<u32> {
    let xs: Vec<&str> = a.split('-').collect();
    xs[0].parse().expect("no parse")..=xs[1].parse().expect("no parse")
}

pub fn solve(path: &str) {
    let mut sum = 0usize;
    let mut sum2 = 0usize;
    get_input(path).iter().for_each(|line| {
        let xs: Vec<&str> = line.split(',').collect();
        let ranges = (into_range(xs[0]), into_range(xs[1]));
        sum += score(ranges.clone());
        sum2 += score2(ranges);
    });
    println!("{sum}");
    println!("{sum2}");
}

fn score(ranges: (RangeInclusive<u32>, RangeInclusive<u32>)) -> usize {
    usize::from(fully_contains(ranges.0, ranges.1))
}
fn score2(ranges: (RangeInclusive<u32>, RangeInclusive<u32>)) -> usize {
    usize::from(overlap(ranges.0, ranges.1))
}

fn fully_contains(a: RangeInclusive<u32>, b: RangeInclusive<u32>) -> bool {
    (a.start() >= b.start() && a.end() <= b.end()) || (b.start() >= a.start() && b.end() <= a.end())
}

fn overlap(a: RangeInclusive<u32>, b: RangeInclusive<u32>) -> bool {
    a.end() >= b.start() && a.start() <= b.end()
}

#[test]
fn test_overlap() {
    assert!(!overlap((2..=4), (6..=8)));
    assert!(!overlap((2..=3), (4..=7)));
    assert!(overlap((5..=7), (7..=9)));
    assert!(overlap((2..=8), (3..=7)));
    assert!(overlap((6..=6), (4..=6)));
    assert!(overlap((2..=6), (4..=8)));
}

#[test]
fn test_into_range() {
    assert_eq!(into_range("35-73"), (35..=73));
}

#[test]
fn test_contains() {
    assert!(!fully_contains((2..=4), (6..=8)));
    assert!(!fully_contains((2..=3), (4..=7)));
    assert!(!fully_contains((5..=7), (7..=9)));
    assert!(fully_contains((2..=8), (3..=7)));
    assert!(fully_contains((6..=6), (4..=6)));
    assert!(!fully_contains((2..=6), (4..=8)));
}

