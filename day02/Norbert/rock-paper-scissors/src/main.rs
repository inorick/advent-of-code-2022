#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

fn main() {}

pub fn calculate_score_part1(list: String) -> Result<u32, Error> {
    Ok(list
        .split(|c| c == '\n')
        .into_iter()
        .map(|line| match line {
            "A X" => Ok(1 + 3),
            "A Y" => Ok(2 + 6),
            "A Z" => Ok(3 + 0),
            "B X" => Ok(1 + 0),
            "B Y" => Ok(2 + 3),
            "B Z" => Ok(3 + 6),
            "C X" => Ok(1 + 6),
            "C Y" => Ok(2 + 0),
            "C Z" => Ok(3 + 3),
            _ => Err(Error::InvalidInput),
        })
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|_| Error::InvalidInput)?
        .iter()
        .sum())
}

pub fn calculate_score_part2(list: String) -> Result<u32, Error> {
    Ok(list
        .split(|c| c == '\n')
        .into_iter()
        .map(|line| match line {
            "A X" => Ok(3 + 0),
            "A Y" => Ok(1 + 3),
            "A Z" => Ok(2 + 6),
            "B X" => Ok(1 + 0),
            "B Y" => Ok(2 + 3),
            "B Z" => Ok(3 + 6),
            "C X" => Ok(2 + 0),
            "C Y" => Ok(3 + 3),
            "C Z" => Ok(1 + 6),
            _ => Err(Error::InvalidInput),
        })
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|_| Error::InvalidInput)?
        .iter()
        .sum())
}

#[cfg(test)]
pub mod test {
    #[test]
    fn part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let score = crate::calculate_score_part1(input).expect("failed to calculate score");
        assert_eq!(score, 13809);
    }

    #[test]
    fn part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let score = crate::calculate_score_part2(input).expect("failed to calculate score");
        assert_eq!(score, 12316);
    }
}
