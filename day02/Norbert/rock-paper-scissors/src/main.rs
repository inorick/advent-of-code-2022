#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

fn main() {}

pub fn calculate_score_part1(list: String) -> Result<u32, Error> {
    let list = list.split(|c| c == '\n');
    let mut sum = 0;
    for line in list {
        let line: Vec<&str> = line.split(' ').collect();
        if line.len() != 2 {
            return Err(Error::InvalidInput);
        }
        let opp = line[0];
        let own = line[1];

        sum += match opp {
            "A" => match own {
                "X" => 1 + 3,
                "Y" => 2 + 6,
                "Z" => 3 + 0,
                _ => {
                    return Err(Error::InvalidInput);
                }
            },
            "B" => match own {
                "X" => 1 + 0,
                "Y" => 2 + 3,
                "Z" => 3 + 6,
                _ => {
                    return Err(Error::InvalidInput);
                }
            },
            "C" => match own {
                "X" => 1 + 6,
                "Y" => 2 + 0,
                "Z" => 3 + 3,
                _ => {
                    return Err(Error::InvalidInput);
                }
            },
            _ => {
                return Err(Error::InvalidInput);
            }
        };
    }
    Ok(sum)
}

pub fn calculate_score_part2(list: String) -> Result<u32, Error> {
    let list = list.split(|c| c == '\n');
    let mut sum = 0;
    for line in list {
        let line: Vec<&str> = line.split(' ').collect();
        if line.len() != 2 {
            return Err(Error::InvalidInput);
        }
        let opp = line[0];
        let own = line[1];

        sum += match opp {
            "A" => match own {
                "X" => 3 + 0,
                "Y" => 1 + 3,
                "Z" => 2 + 6,
                _ => {
                    return Err(Error::InvalidInput);
                }
            },
            "B" => match own {
                "X" => 1 + 0,
                "Y" => 2 + 3,
                "Z" => 3 + 6,
                _ => {
                    return Err(Error::InvalidInput);
                }
            },
            "C" => match own {
                "X" => 2 + 0,
                "Y" => 3 + 3,
                "Z" => 1 + 6,
                _ => {
                    return Err(Error::InvalidInput);
                }
            },
            _ => {
                return Err(Error::InvalidInput);
            }
        };
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
