#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

fn main() {}

pub fn solve<const N_STACKS: usize, const INIT_LINES: usize, const PART_2: bool>(
    input: String,
) -> Result<(), Error> {
    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..N_STACKS {
        stacks.push(Vec::new());
    }
    let mut lines = input.split('\n');

    // Read init lines
    for _line in 0..INIT_LINES {
        let line: Vec<char> = lines.next().unwrap().chars().collect();
        for stack_idx in 0..N_STACKS {
            match line[4 * stack_idx + 1] {
                ' ' => {
                    // print!("  ");
                    continue;
                }
                c => {
                    // print!("{c} ");
                    stacks[stack_idx].push(c);
                }
            };
        }
        // println!();
    }
    for stack in &mut stacks {
        stack.reverse();
    }
    lines.next(); // Stack names
    lines.next(); // Empty line

    // Read and perform moves
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let line: Vec<_> = line.split(' ').collect();
        let n = line[1].parse::<usize>().unwrap();
        let from_idx = line[3].parse::<usize>().unwrap();
        let to_idx = line[5].parse::<usize>().unwrap();
        // println!("{n} {from_idx} {to_idx}");
        let mut from = stacks[from_idx - 1].clone();
        let mut to = stacks[to_idx - 1].clone();

        if PART_2 {
            let mut stash: Vec<char> = vec![];
            for _ in 0..n {
                stash.push(from.pop().unwrap());
            }
            for _ in 0..n {
                to.push(stash.pop().unwrap());
            }
        }

        if !PART_2 {
            for _ in 0..n {
                to.push(from.pop().unwrap());
            }
        }

        stacks[from_idx - 1] = from;
        stacks[to_idx - 1] = to;
    }

    // Print top elements
    for mut stack in stacks {
        match stack.pop() {
            None => {
                print!(" ")
            }
            Some(c) => {
                print!("{c}")
            }
        }
    }
    println!();
    Ok(())
}

#[cfg(test)]
pub mod test {
    #[test]
    fn solve_example_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let _sol = crate::solve::<3, 3, false>(input).expect("failed to solve");
    }

    #[test]
    fn solve_example_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let _sol = crate::solve::<3, 3, true>(input).expect("failed to solve");
    }

    #[test]
    fn solve_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let _sol = crate::solve::<9, 8, false>(input).expect("failed to solve");
    }

    #[test]
    fn solve_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let _sol = crate::solve::<9, 8, true>(input).expect("failed to solve");
    }
}
