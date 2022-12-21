extern crate core;

use crate::Error::InvalidInput;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {}

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

#[derive(Debug)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug)]
pub enum Monkey {
    Number(u64),
    Operation(Operation, String, String),
}

pub fn solve_part1(input: String) -> Result<u64, Error> {
    let monkeys = parse_input(input)?;
    let monkeys = resolve_operations(monkeys)?;
    let root = monkeys.get("root").ok_or(InvalidInput)?;
    let root = match root {
        Monkey::Number(n) => Ok(*n),
        _ => Err(InvalidInput),
    }?;
    Ok(root)
}

pub fn solve_part2(input: String) -> Result<(), Error> {
    let mut monkeys = parse_input(input)?;

    // Print Sage program
    let comma = ", ".to_string();
    #[allow(unstable_name_collisions)]
    let names: String = monkeys.keys().intersperse(&comma).cloned().collect();
    println!("{names} = var('{names}')");

    let root = monkeys.remove("root").ok_or(InvalidInput)?;
    let _human = monkeys.remove("humn").ok_or(InvalidInput)?;
    let monkeys = resolve_operations(monkeys)?;

    println!("sol = solve([");
    for (name, monkey) in &monkeys {
        print_equation(name, monkey);
    }
    match root {
        Monkey::Operation(_, arg1, arg2) => {
            println!("{arg1} == {arg2}");
        }
        _ => Err(InvalidInput)?,
    }
    print!("], ");
    print!("{names}");
    println!(", algorithm='sympy')");
    println!("print(sol[0][humn])");
    println!();
    Ok(())
}

fn print_equation(name: &str, monkey: &Monkey) {
    match monkey {
        Monkey::Number(n) => {
            println!("{name} == {n},");
        }
        Monkey::Operation(op, arg1, arg2) => {
            let op = match op {
                Operation::Addition => "+",
                Operation::Subtraction => "-",
                Operation::Multiplication => "*",
                Operation::Division => "/",
            };
            println!("{name} == {arg1} {op} {arg2},");
        }
    }
}

fn resolve_operations(
    mut monkeys: HashMap<String, Monkey>,
) -> Result<HashMap<String, Monkey>, Error> {
    loop {
        let mut new_numbers = HashMap::new();
        for (name, monkey) in &monkeys {
            if let Monkey::Operation(op, arg1, arg2) = monkey {
                let arg1 = monkeys.get(arg1);
                let arg2 = monkeys.get(arg2);
                if let Some(Monkey::Number(arg1)) = arg1 {
                    if let Some(Monkey::Number(arg2)) = arg2 {
                        let new_number = match op {
                            Operation::Addition => arg1 + arg2,
                            Operation::Subtraction => arg1 - arg2,
                            Operation::Multiplication => arg1 * arg2,
                            Operation::Division => arg1 / arg2,
                        };
                        new_numbers.insert(name.clone(), new_number);
                    }
                }
            };
        }
        if new_numbers.is_empty() {
            break;
        }
        for (name, new_number) in new_numbers {
            monkeys.insert(name, Monkey::Number(new_number));
        }
    }
    Ok(monkeys)
}

fn parse_input(input: String) -> Result<HashMap<String, Monkey>, Error> {
    let mut monkeys = HashMap::new();
    for line in input.lines() {
        let words: Vec<_> = line.split(' ').collect();
        match words.len() {
            2 => {
                let name = words[0][..4].to_string();
                let number = words[1].parse::<u64>().map_err(|_| InvalidInput)?;
                monkeys.insert(name, Monkey::Number(number));
            }
            4 => {
                let name = words[0][..4].to_string();
                let arg1 = words[1].to_string();
                let arg2 = words[3].to_string();
                let operation = match words[2] {
                    "+" => Ok(Operation::Addition),
                    "-" => Ok(Operation::Subtraction),
                    "*" => Ok(Operation::Multiplication),
                    "/" => Ok(Operation::Division),
                    _ => Err(InvalidInput),
                }?;
                monkeys.insert(name, Monkey::Operation(operation, arg1, arg2));
            }
            _ => Err(InvalidInput)?,
        }
    }
    Ok(monkeys)
}

#[cfg(test)]
pub mod test {
    #[test]
    fn solve_example() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve_part1(input).expect("failed to solve");
        assert_eq!(sol, 152);
    }

    #[test]
    fn solve_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sol = crate::solve_part1(input).expect("failed to solve");
        assert_eq!(sol, 118565889858886);
    }

    #[test]
    fn solve_example_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        crate::solve_part2(input).expect("failed to solve");
    }

    #[test]
    fn solve_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        crate::solve_part2(input).expect("failed to solve");
    }
}
