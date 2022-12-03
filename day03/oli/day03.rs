use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{self, BufRead},
};

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
fn priority(c: char) -> usize {
    let value = if c.is_uppercase() {
        c.to_lowercase().last().unwrap() as usize + 26
    } else {
        c as usize
    };
    value - 'a' as usize + 1
}

fn common_item(halfs: (&str, &str)) -> Option<char> {
    let set: HashSet<char> = halfs.0.chars().collect();
    halfs.1.chars().into_iter().find(|&c| set.contains(&c))
}

fn split(s: &str) -> (&str, &str) {
    let half = s.len() / 2_usize;
    (&s[0..half], &s[half..])
}

fn score(s: &str) -> usize {
    let common = common_item(split(s)).expect("could not find out common element");
    priority(common)
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path).unwrap_or_else(|_| panic!("could not read file {file_path}"));
    let mut sum = 0usize;
    io::BufReader::new(file).lines().for_each(|line| {
        if let Ok(line) = line {
            sum += score(&line);
        }
    });
    println!("{sum}");
}
#[test]
fn test_priority() {
    assert_eq!(priority('p'), 16);
    assert_eq!(priority('L'), 38);
    assert_eq!(priority('P'), 42);
    assert_eq!(priority('v'), 22);
    assert_eq!(priority('t'), 20);
    assert_eq!(priority('s'), 19);
}

#[test]
fn test_common_item() {
    assert_eq!(common_item(("vJrwpWtwJgWr", "hcsFMMfFFhFp")), Some('p'));
    assert_eq!(
        common_item(("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL")),
        Some('L')
    );
    assert_eq!(common_item(("PmmdzqPrV", "PmmdzqPrV")), Some('P'));
}

#[test]
fn test_split() {
    assert_eq!(
        split("vJrwpWtwJgWrhcsFMMfFFhFp"),
        ("vJrwpWtwJgWr", "hcsFMMfFFhFp")
    );
    assert_eq!(
        split("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
        ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL")
    );
    assert_eq!(split("PmmdzqPrVPmmdzqPrV"), ("PmmdzqPrV", "PmmdzqPrV"));
}
