extern crate core;

use crate::Error::InvalidInput;
use itertools::Itertools;
use pathfinding::prelude::{bfs, dijkstra_all};
use std::cmp::max;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

fn main() {}

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct State {
    name: String,
    open_nodes: Vec<String>,
    time_elapsed: u32,
    released: u32,
    flow: u32,
}

const START: &str = "AA";

pub fn solve(input: String) -> Result<u32, Error> {
    let (names, flows, neighbours) = parse(input)?;
    let connections = connections(&names, &flows, neighbours);
    let max_released = max_released(&flows, &connections);
    Ok(max_released)
}

fn max_released(
    flows: &HashMap<String, u32>,
    connections: &HashMap<String, Vec<(String, u32)>>,
) -> u32 {
    const MAX_MINUTES: u32 = 30;
    let mut max_released = 0;
    loop {
        let goal = max_released + 1;
        let result = find_solution(flows, connections, goal, MAX_MINUTES);
        match result {
            None => {
                println!("No solution for pressure goal {goal}");
                break;
            }
            Some(result) => {
                max_released = goal;
                println!("Solution for pressure goal {goal} found:");
                for state in result {
                    println!("{state:?}");
                }
            }
        }
        println!();
    }
    max_released
}

fn find_solution(
    flows: &HashMap<String, u32>,
    connections: &HashMap<String, Vec<(String, u32)>>,
    goal: u32,
    max_minutes: u32,
) -> Option<Vec<State>> {
    let mut max_released = 0;
    bfs(
        &State {
            name: START.to_string(),
            open_nodes: vec![],
            time_elapsed: 0,
            released: 0,
            flow: 0,
        },
        |state| {
            let mut successors = vec![];
            if state.time_elapsed > max_minutes {
                panic!("time_elapsed > MAX_MINUTES");
            }
            if state.time_elapsed == max_minutes {
                return successors;
            }
            let max_future_released = max_future_released(state, flows, max_minutes);
            if state.released + max_future_released < max_released {
                return successors;
            }
            // Open current node
            if !state.open_nodes.contains(&state.name) {
                let mut opened = state.clone();
                opened.open_nodes.push(state.name.clone());
                opened.released += opened.flow;
                max_released = max(max_released, opened.released);
                opened.flow += flows.get(&state.name).expect("failed to get flow");
                opened.time_elapsed += 1;
                successors.push(opened);
            }
            // Visit connected nodes
            for (name, dist) in connections
                .get(&state.name)
                .expect("failed to get connection")
                .iter()
            {
                if state.open_nodes.contains(name) {
                    continue;
                }
                if state.time_elapsed + dist + 1 > max_minutes {
                    continue;
                }
                let mut visit = state.clone();
                visit.name = name.clone();
                visit.time_elapsed += dist;
                visit.released += state.flow * dist;
                successors.push(visit);
            }
            // Do nothing
            let mut stay = state.clone();
            stay.time_elapsed += 1;
            stay.released += state.flow;
            successors.push(stay);
            successors
        },
        |state| state.released >= goal,
    )
}

fn connections(
    names: &HashSet<String>,
    flows: &HashMap<String, u32>,
    neighbours: HashMap<String, HashSet<String>>,
) -> HashMap<String, Vec<(String, u32)>> {
    let mut connections = HashMap::new();
    for start in names {
        let reachables = dijkstra_all(start, |name| {
            neighbours
                .get(name)
                .expect("failed to get neighbour")
                .iter()
                .map(|neighbour| (neighbour.clone(), 1))
                .collect::<Vec<(String, u32)>>()
        });
        for (dest, (_, distance)) in &reachables {
            if *flows.get(dest).expect("failed to get flow") > 0 {
                match connections.entry(start.clone()) {
                    Entry::Vacant(e) => {
                        e.insert(vec![(dest.clone(), *distance)]);
                    }
                    Entry::Occupied(mut e) => {
                        e.get_mut().push((dest.clone(), *distance));
                    }
                }
            }
        }
        print!("Node {}: ", start);
        for (dest, dist) in connections.get(start).expect("failed to get connection") {
            print!("({dest}, {dist}) ");
        }
        println!();
    }
    connections
}

fn parse(
    input: String,
) -> Result<
    (
        HashSet<String>,
        HashMap<String, u32>,
        HashMap<String, HashSet<String>>,
    ),
    Error,
> {
    let lines: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.split(' ').collect())
        .collect();
    let mut names = HashSet::new();
    let mut flows = HashMap::new();
    let mut neighbours = HashMap::new();
    for line in lines {
        let name = line[1].to_string();
        let mut flow = line[4].to_string();
        flow.retain(|c| c.is_numeric());
        let flow = flow.parse::<u32>().map_err(|_| InvalidInput)?;
        let nb: HashSet<_> = line[9..]
            .iter()
            .map(|s| {
                let mut s = s.to_string();
                s.truncate(2);
                s
            })
            .collect();
        names.insert(name.clone());
        flows.insert(name.clone(), flow);
        neighbours.insert(name, nb);
    }
    Ok((names, flows, neighbours))
}

fn max_future_released(state: &State, flows: &HashMap<String, u32>, max_minutes: u32) -> u32 {
    let minutes_remaining = max_minutes - state.time_elapsed;
    let mut potential_flows = flows
        .iter()
        .filter(|(name, _)| state.open_nodes.contains(name))
        .map(|(_, flow)| flow)
        .sorted()
        .rev();
    let mut add_flow = 0;
    let mut add_released = 0;
    for minute in 0..minutes_remaining {
        add_released += state.flow + add_flow;
        if minute % 2 == 0 {
            match potential_flows.next() {
                None => {}
                Some(flow) => {
                    add_flow += flow;
                }
            }
        }
    }
    add_released
}

#[cfg(test)]
pub mod test {
    #[test]
    fn solve_example() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve(input).expect("failed to solve");
        assert_eq!(max_released, 1651);
    }

    #[test]
    fn solve_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve(input).expect("failed to solve");
        assert_eq!(max_released, 1641);
    }
}
