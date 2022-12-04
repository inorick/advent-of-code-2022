#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

fn main() {}

pub fn find_overlap_part1(input: String) -> Result<u32, Error> {
    let mut sum = 0;
    let list = input.split('\n');
    for line in list {
        if line.is_empty() {
            continue;
        }
        let ranges: Vec<_> = line.split(',').collect();
        let left_range = ranges[0].split('-').collect::<Vec<_>>();
        let right_range = ranges[1].split('-').collect::<Vec<_>>();
        let (x1, x2) = (left_range[0], left_range[1]);
        let (y1, y2) = (right_range[0], right_range[1]);
        let (x1, x2) = (x1.parse::<u32>().unwrap(), x2.parse::<u32>().unwrap());
        let (y1, y2) = (y1.parse::<u32>().unwrap(), y2.parse::<u32>().unwrap());
        let is_left_in_right = x1 >= y1 && x2 <= y2;
        let is_right_in_left = y1 >= x1 && y2 <= x2;
        if is_left_in_right || is_right_in_left {
            sum += 1;
        }
    }
    Ok(sum)
}

pub fn find_overlap_part2(input: String) -> Result<u32, Error> {
    let mut sum = 0;
    let list = input.split('\n');
    for line in list {
        if line.is_empty() {
            continue;
        }
        let ranges: Vec<_> = line.split(',').collect();
        let left_range = ranges[0].split('-').collect::<Vec<_>>();
        let right_range = ranges[1].split('-').collect::<Vec<_>>();
        let (x1, x2) = (left_range[0], left_range[1]);
        let (y1, y2) = (right_range[0], right_range[1]);
        let (x1, x2) = (x1.parse::<u32>().unwrap(), x2.parse::<u32>().unwrap());
        let (y1, y2) = (y1.parse::<u32>().unwrap(), y2.parse::<u32>().unwrap());
        let first_overlaps_from_left = x1 <= y1 && x2 >= y1;
        let second_overlaps_from_left = y1 <= x1 && y2 >= x1;
        if first_overlaps_from_left || second_overlaps_from_left {
            sum += 1;
        }
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
        let overlapping = crate::find_overlap_part1(input).expect("failed to find overlap");
        assert_eq!(overlapping, 507);
    }

    #[test]
    fn part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let overlapping = crate::find_overlap_part2(input).expect("failed to calculate score");
        assert_eq!(overlapping, 897);
    }
}
