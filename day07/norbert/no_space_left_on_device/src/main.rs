extern crate core;

use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

fn main() {}

pub fn solve(input: String) -> Result<(u32, u32), Error> {
    let lines: Vec<_> = input.lines().collect();
    let mut path: Vec<String> = vec![];
    let mut file_sizes: HashMap<Vec<String>, u32> = HashMap::new();

    println!("Collecting file sizes");
    for (nr, line) in lines.iter().enumerate() {
        println!("Line {nr}: {line}");
        if &line[0..4] == "$ ls" {
            continue;
        } else if &line[0..4] == "$ cd" {
            let name = &line[5..];
            if name == ".." {
                path.pop();
            } else {
                path.push(name.to_string());
            }
            println!("Changed path to: {}", path.join("/"));
        } else if &line[0..3] == "dir" {
            continue;
        } else {
            let words: Vec<_> = line.split(' ').collect();
            let size = words[0]
                .parse::<u32>()
                .expect("failed to convert file size to number");
            let name = words[1];
            let mut file_path = path.clone();
            file_path.push(name.to_string());
            println!("{}: {size}", file_path.join("/"));
            file_sizes.insert(file_path, size);
        }
    }

    println!();
    println!("Calculating directory sizes");
    let mut path_sizes: HashMap<Vec<String>, u32> = HashMap::new();
    for (path, size) in file_sizes.iter() {
        let mut path = path.clone();
        path.pop(); // Remove file name
        while !path.is_empty() {
            println!("Adding {size} to {}", path.join("/"));
            *path_sizes.entry(path.to_vec()).or_default() += size;
            println!(
                "Total size is now {}",
                path_sizes.get(&path.to_vec()).unwrap()
            );
            path.pop();
        }
    }
    println!();

    // Part 1
    let mut small_path_sizes = path_sizes.clone();
    small_path_sizes.retain(|_, size| size <= &mut 100000);
    let part1_sum = small_path_sizes.values().sum();
    println!("Sum directories with a total size of at most 100000: {part1_sum}");

    // Part2
    const TOTAL_DISK_SPACE: u32 = 70000000;
    let mut to_free = TOTAL_DISK_SPACE - path_sizes.get(&vec!["/".to_string()]).unwrap();
    let mut candidate_path_sizes = path_sizes.clone();
    candidate_path_sizes.retain(|_, size| size >= &mut to_free);
    let mut candidate_path_sizes = candidate_path_sizes
        .iter()
        .collect::<Vec<(&Vec<String>, &u32)>>();
    candidate_path_sizes.sort_by_key(|(_, size)| *size);
    let part2_size = candidate_path_sizes[0].1;
    println!("Size of smallest directory that frees up enough space: {part2_size}");

    Ok((part1_sum, *part2_size))
}

#[cfg(test)]
pub mod test {
    #[test]
    fn solve_example() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let (sum, _) = crate::solve(input).expect("failed to solve");
        assert_eq!(sum, 95437);
    }

    #[test]
    fn solve_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let (sum, _) = crate::solve(input).expect("failed to solve");
        assert_eq!(sum, 1243729);
    }

    #[test]
    fn solve_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let (_, size) = crate::solve(input).expect("failed to solve");
        assert_eq!(size, 4443914);
    }
}
