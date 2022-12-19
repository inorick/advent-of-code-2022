extern crate core;

use std::cmp::max;

fn main() {}

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

#[derive(Debug)]
pub struct Blueprint {
    pub nr: u16,
    pub ore_for_ore_robot: u16,
    pub ore_for_clay_robot: u16,
    pub ore_for_obsidian_robot: u16,
    pub clay_for_obsidian_robot: u16,
    pub ore_for_geode_robot: u16,
    pub obsidian_for_geode_robot: u16,
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub ore: u16,
    pub clay: u16,
    pub obsidian: u16,
    pub geodes: u16,
    pub ore_robots: u16,
    pub clay_robots: u16,
    pub obsidian_robots: u16,
    pub geode_robots: u16,
}

pub fn solve(input: String) -> Result<u16, Error> {
    const INITIAL_STATE: GameState = GameState {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geodes: 0,
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,
    };

    let blueprints = parse_blueprints(input);
    let mut quality_levels = vec![];

    for (nr, blueprint) in blueprints.iter().enumerate() {
        println!("{blueprint:?}");
        const MAX_MINUTES: u16 = 24;
        let mut max_geodes = 0;
        let mut candidates = vec![INITIAL_STATE];

        for minute in 1u16..=MAX_MINUTES {
            println!("minute={minute}: states={}", candidates.len());

            let mut next_states = vec![];
            let minutes_left = MAX_MINUTES - minute + 1;
            if minutes_left == 1 && candidates.iter().all(|s| s.geode_robots == 0) {
                break; // No point in building geode robots in the last minute
            }

            while !candidates.is_empty() {
                let state = candidates.pop().expect("failed to get next state");
                if can_build_ore_robot(blueprint, &state) {
                    let mut next_state = state.clone();
                    gather_resources(&mut next_state);
                    build_ore_robot(blueprint, &mut next_state);
                    if can_improve(&next_state, minutes_left - 1, max_geodes) {
                        update_max_geodes(&mut max_geodes, next_state.geodes);
                        next_states.push(next_state);
                    }
                }
                if can_build_clay_robot(blueprint, &state) {
                    let mut next_state = state.clone();
                    gather_resources(&mut next_state);
                    build_clay_robot(blueprint, &mut next_state);
                    if can_improve(&next_state, minutes_left - 1, max_geodes) {
                        update_max_geodes(&mut max_geodes, next_state.geodes);
                        next_states.push(next_state);
                    }
                }
                if can_build_obsidian_robot(blueprint, &state) {
                    let mut next_state = state.clone();
                    gather_resources(&mut next_state);
                    build_obsidian_robot(blueprint, &mut next_state);
                    if can_improve(&next_state, minutes_left - 1, max_geodes) {
                        update_max_geodes(&mut max_geodes, next_state.geodes);
                        next_states.push(next_state);
                    }
                }
                if can_build_geode_robot(blueprint, &state) {
                    let mut next_state = state.clone();
                    gather_resources(&mut next_state);
                    build_geode_robot(blueprint, &mut next_state);
                    if can_improve(&next_state, minutes_left - 1, max_geodes) {
                        update_max_geodes(&mut max_geodes, next_state.geodes);
                        next_states.push(next_state);
                    }
                }
                // Do not build a robot. Just gather
                {
                    let mut next_state = state.clone();
                    gather_resources(&mut next_state);
                    if can_improve(&next_state, minutes_left - 1, max_geodes) {
                        update_max_geodes(&mut max_geodes, next_state.geodes);
                        next_states.push(next_state);
                    }
                }
            }
            candidates = next_states
                .into_iter()
                .filter(|s| can_improve(s, minutes_left - 1, max_geodes))
                .collect();
            candidates.shrink_to_fit();
        }

        let quality_level = ((nr + 1) as u16, max_geodes);
        println!("Quality level: {quality_level:?}");
        quality_levels.push(quality_level.0 * quality_level.1);
        println!();
    }
    println!();

    Ok(quality_levels.iter().sum())
}

fn update_max_geodes(max_geodes: &mut u16, geodes: u16) {
    if geodes > *max_geodes {
        println!("New max geodes: {geodes}");
    }
    *max_geodes = max(*max_geodes, geodes);
}

fn can_improve(state: &GameState, minutes_left: u16, max_geodes: u16) -> bool {
    let future_geodes_by_existing_robots = state.geode_robots * minutes_left;
    let future_geodes_by_future_robots = ((minutes_left.saturating_sub(1)) * minutes_left) / 2;
    state.geodes + future_geodes_by_existing_robots + future_geodes_by_future_robots > max_geodes
}

fn gather_resources(game_state: &mut GameState) {
    game_state.ore += game_state.ore_robots;
    game_state.clay += game_state.clay_robots;
    game_state.obsidian += game_state.obsidian_robots;
    game_state.geodes += game_state.geode_robots;
}

fn can_build_ore_robot(blueprint: &Blueprint, game_state: &GameState) -> bool {
    game_state.ore >= blueprint.ore_for_ore_robot
}

fn build_ore_robot(blueprint: &Blueprint, game_state: &mut GameState) {
    game_state.ore -= blueprint.ore_for_ore_robot;
    game_state.ore_robots += 1;
}

fn can_build_clay_robot(blueprint: &Blueprint, game_state: &GameState) -> bool {
    game_state.ore >= blueprint.ore_for_clay_robot
}

fn build_clay_robot(blueprint: &Blueprint, game_state: &mut GameState) {
    game_state.ore -= blueprint.ore_for_clay_robot;
    game_state.clay_robots += 1;
}

fn can_build_obsidian_robot(blueprint: &Blueprint, game_state: &GameState) -> bool {
    game_state.ore >= blueprint.ore_for_obsidian_robot
        && game_state.clay >= blueprint.clay_for_obsidian_robot
}

fn build_obsidian_robot(blueprint: &Blueprint, game_state: &mut GameState) {
    game_state.ore -= blueprint.ore_for_obsidian_robot;
    game_state.clay -= blueprint.clay_for_obsidian_robot;
    game_state.obsidian_robots += 1;
}

fn can_build_geode_robot(blueprint: &Blueprint, game_state: &GameState) -> bool {
    game_state.ore >= blueprint.ore_for_geode_robot
        && game_state.obsidian >= blueprint.obsidian_for_geode_robot
}

fn build_geode_robot(blueprint: &Blueprint, game_state: &mut GameState) {
    game_state.ore -= blueprint.ore_for_obsidian_robot;
    game_state.obsidian -= blueprint.obsidian_for_geode_robot;
    game_state.geode_robots += 1;
}

fn parse_blueprints(input: String) -> Vec<Blueprint> {
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
                .flat_map(|word| word.parse::<u16>().ok())
                .collect::<Vec<_>>()
        })
        .map(|numbers| Blueprint {
            nr: numbers[0],
            ore_for_ore_robot: numbers[1],
            ore_for_clay_robot: numbers[2],
            ore_for_obsidian_robot: numbers[3],
            clay_for_obsidian_robot: numbers[4],
            ore_for_geode_robot: numbers[5],
            obsidian_for_geode_robot: numbers[6],
        })
        .collect()
}

#[cfg(test)]
pub mod test {
    #[test]
    fn solve_example() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve(input).expect("failed to solve");
        assert_eq!(max_released, 33);
    }

    #[test]
    fn solve_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let solution = crate::solve(input).expect("failed to solve");
        assert_eq!(solution, 1522);
    }
}
