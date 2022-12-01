type ElfNr = usize;
type Cal = u32;

fn main() {}

#[derive(Debug)]
pub enum Error {
    InvalidList,
}

pub fn find_fattest_elf(cal_list: String) -> Result<(ElfNr, Cal), Error> {
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
    return match sums.iter().enumerate().max_by_key(|(_, cal)| *cal) {
        None => Err(Error::InvalidList),
        Some((elf_nr, cal)) => Ok((elf_nr + 1, *cal)),
    };
}

#[cfg(test)]
pub mod test {

    #[test]
    fn example_list() {
        let list: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(
            crate::find_fattest_elf(list.to_string()).unwrap(),
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
    fn input_file() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let (fattest_elf, cals) =
            crate::find_fattest_elf(input).expect("failed to find fattest elf");
        assert_eq!(fattest_elf, 213);
        assert_eq!(cals, 68467);
    }
}
