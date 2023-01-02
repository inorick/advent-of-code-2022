extern crate core;

use crate::Error::Solve;
use good_lp::solvers::coin_cbc::CoinCbcSolution;
use good_lp::{
    default_solver, variable, variables, Constraint, Expression, ResolutionError, Solution,
    SolverModel, Variable,
};
use num_bigint::BigInt;
use std::cmp::min;
use std::iter::{zip, Sum};
use std::ops::Mul;

fn main() {}

#[derive(Debug)]
pub enum Error {
    InvalidInput,
    Solve(ResolutionError),
}

#[derive(Debug)]
pub struct Blueprint {
    pub nr: u32,
    pub ore_per_ore_robot: u32,
    pub ore_per_clay_robot: u32,
    pub ore_per_obsidian_robot: u32,
    pub clay_per_obsidian_robot: u32,
    pub ore_per_geode_robot: u32,
    pub obsidian_per_geode_robot: u32,
}

#[derive(Clone, Debug)]
pub struct Robots {
    pub ore: Vec<Variable>,
    pub clay: Vec<Variable>,
    pub obsidian: Vec<Variable>,
    pub geodes: Vec<Variable>,
}

#[derive(Clone, Debug)]
pub struct Resources {
    pub ore: Expression,
    pub clay: Expression,
    pub obsidian: Expression,
    pub geodes: Expression,
}

pub fn solve_part1(input: String) -> Result<u32, Error> {
    const MAX_MINUTES: usize = 24;
    let blueprints = parse(input);
    let max_geodes = max_geodes(&blueprints, MAX_MINUTES)?;
    Ok(max_geodes.iter().map(|(nr, geodes)| nr * geodes).sum())
}

pub fn solve_part2(input: String) -> Result<u32, Error> {
    const MAX_MINUTES: usize = 32;
    let blueprints = parse(input);
    let blueprints = blueprints.get(..min(blueprints.len(), 3)).unwrap();
    let max_geodes = max_geodes(blueprints, MAX_MINUTES)?;
    Ok(max_geodes.iter().map(|(_, geodes)| geodes).product())
}

pub fn solve_part2_all_blueprints(input: String) -> Result<BigInt, Error> {
    const MAX_MINUTES: usize = 32;
    let blueprints = parse(input);
    let max_geodes = max_geodes(&blueprints, MAX_MINUTES)?;
    Ok(max_geodes
        .iter()
        .map(|(_, geodes)| BigInt::from(*geodes)) // Avoid overflow
        .product())
}

fn max_geodes(blueprints: &[Blueprint], max_minutes: usize) -> Result<Vec<(u32, u32)>, Error> {
    let mut result = vec![];
    for (nr, blueprint) in blueprints.iter().enumerate() {
        // Variables
        let mut vars = variables!();
        let robots = Robots {
            ore: vars.add_vector(variable().binary(), max_minutes),
            clay: vars.add_vector(variable().binary(), max_minutes),
            obsidian: vars.add_vector(variable().binary(), max_minutes),
            geodes: vars.add_vector(variable().binary(), max_minutes),
        };

        // Objective
        let objective = gathered(max_minutes, &robots).geodes;
        let mut problem = vars.maximise(&objective).using(default_solver);

        // Constraints
        let one_robot_per_minute_max = one_robot_per_minute_max(&robots, max_minutes);
        let enough_resources_to_build = enough_resources_to_build(blueprint, &robots, max_minutes);
        for cons in one_robot_per_minute_max {
            problem = problem.with(cons);
        }
        for cons in enough_resources_to_build {
            problem = problem.with(cons);
        }

        // Solve
        let sol = problem.solve().map_err(Solve)?;
        let max_geodes = sol.eval(&objective);
        let nr = (nr + 1) as u32;
        let max_geodes = max_geodes as u32;
        result.push((nr, max_geodes));

        // Debug output
        for minute in 0..max_minutes {
            println!("minute={minute}");
            print_robots(minute, &sol, &robots);
            print_build(minute, &sol, &robots);
            print_spent(minute, &sol, blueprint, &robots);
            print_gathered(minute, &sol, &robots);
        }
        println!("nr={nr}");
        println!("max_geodes={max_geodes}");
        println!();
    }
    Ok(result)
}

fn one_robot_per_minute_max(robots: &Robots, max_minutes: usize) -> Vec<Constraint> {
    let mut constraints = vec![];
    for minute in 0..max_minutes {
        constraints.push(
            (robots.ore[minute]
                + robots.clay[minute]
                + robots.obsidian[minute]
                + robots.geodes[minute])
                .leq(1),
        );
    }
    constraints
}

fn enough_resources_to_build(
    blueprint: &Blueprint,
    robots: &Robots,
    max_minutes: usize,
) -> Vec<Constraint> {
    let mut constraints = vec![];
    for minute in 0..max_minutes {
        let have = gathered(minute, robots);
        let need = spent(minute + 1, blueprint, robots);
        constraints.push(have.ore.geq(need.ore));
        constraints.push(have.clay.geq(need.clay));
        constraints.push(have.obsidian.geq(need.obsidian));
    }
    constraints
}

fn gathered(minute: usize, robots: &Robots) -> Resources {
    Resources {
        ore: Expression::sum(
            zip((1..=minute).rev(), robots.ore.iter().take(minute))
                .map(|(k, var)| var.mul(k as u32)),
        ) + minute as u32,
        clay: Expression::sum(
            zip((1..=minute).rev(), robots.clay.iter().take(minute))
                .map(|(k, var)| var.mul(k as u32)),
        ),
        obsidian: Expression::sum(
            zip((1..=minute).rev(), robots.obsidian.iter().take(minute))
                .map(|(k, var)| var.mul(k as u32)),
        ),
        geodes: Expression::sum(
            zip((1..=minute).rev(), robots.geodes.iter().take(minute))
                .map(|(k, var)| var.mul(k as u32)),
        ),
    }
}

fn spent(minute: usize, blueprint: &Blueprint, robots: &Robots) -> Resources {
    let ore_robots = Expression::sum(robots.ore.iter().take(minute + 1));
    let clay_robots = Expression::sum(robots.clay.iter().take(minute + 1));
    let obsidian_robots = Expression::sum(robots.obsidian.iter().take(minute + 1));
    let geode_robots = Expression::sum(robots.geodes.iter().take(minute + 1));
    let ore = ore_robots * blueprint.ore_per_ore_robot
        + clay_robots * blueprint.ore_per_clay_robot
        + obsidian_robots.clone() * blueprint.ore_per_obsidian_robot
        + geode_robots.clone() * blueprint.ore_per_geode_robot;
    let clay = obsidian_robots * blueprint.clay_per_obsidian_robot;
    let obsidian = geode_robots * blueprint.obsidian_per_geode_robot;
    Resources {
        ore,
        clay,
        obsidian,
        geodes: 0.into(),
    }
}

fn print_gathered(minute: usize, sol: &CoinCbcSolution, robots: &Robots) {
    let ore: f64 = zip((1..=minute).rev(), robots.ore.iter().take(minute))
        .map(|(k, var)| sol.value(*var).mul(k as f64))
        .sum::<f64>()
        + minute as f64;
    let clay: f64 = zip((1..=minute).rev(), robots.clay.iter().take(minute))
        .map(|(k, var)| sol.value(*var).mul(k as f64))
        .sum();
    let obsidian: f64 = zip((1..=minute).rev(), robots.obsidian.iter().take(minute))
        .map(|(k, var)| sol.value(*var).mul(k as f64))
        .sum();
    let geodes: f64 = zip((1..=minute).rev(), robots.geodes.iter().take(minute))
        .map(|(k, var)| sol.value(*var).mul(k as f64))
        .sum();
    println!("Gathered: ore={ore:>2}, clay={clay:>2}, obsidian={obsidian:>2}, geodes={geodes:>2}");
}

fn print_spent(minute: usize, sol: &CoinCbcSolution, blueprint: &Blueprint, robots: &Robots) {
    let ore_robots: f64 = robots
        .ore
        .iter()
        .take(minute + 1)
        .map(|var| sol.value(*var))
        .sum();
    let clay_robots: f64 = robots
        .clay
        .iter()
        .take(minute + 1)
        .map(|var| sol.value(*var))
        .sum();
    let obsidian_robots: f64 = robots
        .obsidian
        .iter()
        .take(minute + 1)
        .map(|var| sol.value(*var))
        .sum();
    let geode_robots: f64 = robots
        .geodes
        .iter()
        .take(minute + 1)
        .map(|var| sol.value(*var))
        .sum();
    let ore = ore_robots * blueprint.ore_per_ore_robot as f64
        + clay_robots * blueprint.ore_per_clay_robot as f64
        + obsidian_robots * blueprint.ore_per_obsidian_robot as f64
        + geode_robots * blueprint.ore_per_geode_robot as f64;
    let clay = obsidian_robots * blueprint.clay_per_obsidian_robot as f64;
    let obsidian = geode_robots * blueprint.obsidian_per_geode_robot as f64;
    println!("Spent:    ore={ore:>2}, clay={clay:>2}, obsidian={obsidian:>2}");
}

fn print_build(minute: usize, sol: &CoinCbcSolution, robots: &Robots) {
    let ore = sol.value(robots.ore[minute]);
    let clay = sol.value(robots.clay[minute]);
    let obsidian = sol.value(robots.obsidian[minute]);
    let geodes = sol.value(robots.geodes[minute]);
    println!("Build:    ore={ore:>2}, clay={clay:>2}, obsidian={obsidian:>2}, geodes={geodes:>2}");
}

fn print_robots(minute: usize, sol: &CoinCbcSolution, robots: &Robots) {
    let ore: f64 = robots
        .ore
        .iter()
        .take(minute)
        .map(|var| sol.value(*var))
        .sum::<f64>()
        + 1_f64;
    let clay: f64 = robots
        .clay
        .iter()
        .take(minute)
        .map(|var| sol.value(*var))
        .sum::<f64>();
    let obsidian: f64 = robots
        .obsidian
        .iter()
        .take(minute)
        .map(|var| sol.value(*var))
        .sum::<f64>();
    let geodes: f64 = robots
        .geodes
        .iter()
        .take(minute)
        .map(|var| sol.value(*var))
        .sum::<f64>();
    println!("Robots:   ore={ore:>2}, clay={clay:>2}, obsidian={obsidian:>2}, geodes={geodes:>2}");
}

fn parse(input: String) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|word| {
                    word.to_string()
                        .chars()
                        .filter(|c| c.is_numeric())
                        .collect::<String>()
                })
                .filter(|word| !word.is_empty())
                .flat_map(|word| word.parse::<u32>().ok())
                .collect::<Vec<_>>()
        })
        .map(|numbers| Blueprint {
            nr: numbers[0],
            ore_per_ore_robot: numbers[1],
            ore_per_clay_robot: numbers[2],
            ore_per_obsidian_robot: numbers[3],
            clay_per_obsidian_robot: numbers[4],
            ore_per_geode_robot: numbers[5],
            obsidian_per_geode_robot: numbers[6],
        })
        .collect()
}

#[cfg(test)]
pub mod test {
    use num_bigint::BigInt;
    use std::str::FromStr;

    #[test]
    fn solve_example_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve_part1(input).expect("failed to solve");
        assert_eq!(max_released, 33);
    }

    #[test]
    fn solve_example_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve_part2(input).expect("failed to solve");
        assert_eq!(max_released, 56 * 62);
    }

    #[test]
    fn solve_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let solution = crate::solve_part1(input).expect("failed to solve");
        assert_eq!(solution, 1528);
    }

    #[test]
    fn solve_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve_part2(input).expect("failed to solve");
        assert_eq!(max_released, 16926);
    }

    #[test]
    fn solve_part2_all_blueprints() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve_part2_all_blueprints(input).expect("failed to solve");
        assert_eq!(
            max_released,
            BigInt::from_str("321526786821710237554812382609844207616000").unwrap()
        );
    }
}
