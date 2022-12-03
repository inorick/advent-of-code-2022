use std::collections::HashSet;

#[derive(Debug)]
pub enum Error {
    InvalidInput,
    MultipleCommonItems,
}

fn main() {}

fn priority(c: char) -> Result<u8, Error> {
    return if c.is_ascii_lowercase() {
        Ok(c as u8 - 96)
    } else if c.is_ascii_uppercase() {
        Ok(c as u8 - 65 + 27)
    } else {
        Err(Error::InvalidInput)
    };
}

pub fn sum_priorities_part1(input: String) -> Result<u32, Error> {
    let lines = input.split('\n');
    let mut sum: u32 = 0;
    for line in lines {
        if line.len() == 0 || line.len() % 2 != 0 || line.chars().any(|c| !c.is_ascii_alphabetic())
        {
            return Err(Error::InvalidInput);
        }
        let (r1, r2) = line.split_at(line.len() / 2);
        let r1: HashSet<char> = HashSet::from_iter(r1.chars());
        let r2: HashSet<char> = HashSet::from_iter(r2.chars());
        let intersection: Vec<char> = r1.intersection(&r2).cloned().collect();
        if intersection.len() != 1 {
            return Err(Error::MultipleCommonItems);
        }
        sum += priority(*intersection.first().unwrap())? as u32;
    }
    Ok(sum)
}

pub fn sum_priorities_part2(input: String) -> Result<u32, Error> {
    let lines: Vec<_> = input.split('\n').collect();
    if lines.len() % 3 != 0
        || lines
            .iter()
            .any(|line| line.len() == 0 || line.chars().any(|c| !c.is_ascii_alphabetic()))
    {
        return Err(Error::InvalidInput);
    }
    let mut sum: u32 = 0;
    for group in lines.chunks(3) {
        let r1 = group[0];
        let r2 = group[1];
        let r3 = group[2];
        let r1: HashSet<char> = HashSet::from_iter(r1.chars());
        let r2: HashSet<char> = HashSet::from_iter(r2.chars());
        let r3: HashSet<char> = HashSet::from_iter(r3.chars());
        let intersection: Vec<char> = r1.intersection(&r2).cloned().collect();
        let intersection: HashSet<char> = HashSet::from_iter(intersection.iter().cloned());
        let intersection: Vec<char> = intersection.intersection(&r3).cloned().collect();
        if intersection.len() != 1 {
            return Err(Error::MultipleCommonItems);
        }
        sum += priority(*intersection.first().unwrap())? as u32;
    }

    Ok(sum)
}

#[cfg(test)]
pub mod test {
    #[test]
    fn part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let score = crate::sum_priorities_part1(input).expect("failed to calculate score");
        assert_eq!(score, 7821);
    }

    #[test]
    fn part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let score = crate::sum_priorities_part2(input).expect("failed to calculate score");
        assert_eq!(score, 2752);
    }
}
