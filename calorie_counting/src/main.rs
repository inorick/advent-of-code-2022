type ElfNr = usize;
type Cal = u32;

fn main() {}

#[derive(Debug)]
pub enum Error {
    InvalidList,
}

/// Calculate the calorie sums of each individual elf
fn calculate_sums(cal_list: String) -> Result<Vec<Cal>, Error> {
    let lines: Vec<&str> = cal_list.lines().collect();
    let lists: Vec<&[&str]> = lines.split(|s| s == &"").collect();
    if lists.iter().any(|list| list.is_empty()) {
        return Err(Error::InvalidList); // Adjacent empty lines
    }
    let sums: Vec<Cal> = lists
        .iter()
        .map(|list| list.iter().map(|cal| cal.parse::<Cal>()).collect())
        .collect::<Result<Vec<Vec<Cal>>, _>>()
        .map_err(|_| Error::InvalidList)?
        .iter()
        .map(|list| list.iter().sum())
        .collect();
    Ok(sums)
}

pub fn find_fattest_elf(cal_list: String) -> Result<(ElfNr, Cal), Error> {
    let sums = calculate_sums(cal_list)?;
    return match sums.iter().enumerate().max_by_key(|(_, cal)| *cal) {
        None => Err(Error::InvalidList),
        Some((elf_nr, cal)) => Ok((elf_nr + 1, *cal)),
    };
}

pub fn find_calsum_of_top3_fattest_elfs(cal_list: String) -> Result<Cal, Error> {
    let mut sums = calculate_sums(cal_list)?;
    sums.sort_by(|cal1, cal2| cal2.cmp(cal1)); // Descending order
    if sums.len() < 3 {
        return Err(Error::InvalidList);
    }
    return Ok(sums[0..=2].iter().sum());
}

#[cfg(test)]
pub mod test {
    const EXAMPLE_LIST: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn example_list() {
        assert_eq!(
            crate::find_fattest_elf(EXAMPLE_LIST.to_string()).unwrap(),
            (4, 24000)
        );
    }

    #[test]
    fn empty_list() {
        let list: &str = "";
        assert!(crate::find_fattest_elf(list.to_string()).is_err());
    }

    #[test]
    fn only_line_breaks() {
        let list: &str = "\n\n\n\n\n";
        assert!(crate::find_fattest_elf(list.to_string()).is_err());
    }

    #[test]
    fn multiple_line_breaks() {
        let list: &str = "1000
2000
3000


4000";
        assert!(crate::find_fattest_elf(list.to_string()).is_err());
    }

    #[test]
    fn input_file1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input_nf.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let (fattest_elf, cals) =
            crate::find_fattest_elf(input).expect("failed to find fattest elf");
        assert_eq!(fattest_elf, 213);
        assert_eq!(cals, 68467);
    }

    #[test]
    fn second_challenge_example_list() {
        assert_eq!(
            crate::find_calsum_of_top3_fattest_elfs(EXAMPLE_LIST.to_string()).unwrap(),
            45000
        );
    }

    #[test]
    fn input_file2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input_nf.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let calsum = crate::find_calsum_of_top3_fattest_elfs(input)
            .expect("failed to find calculate sum of fattest elfs");
        assert_eq!(calsum, 203420);
    }
}
