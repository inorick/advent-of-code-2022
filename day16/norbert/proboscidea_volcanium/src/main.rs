extern crate core;

use crate::Error::InvalidInput;
use itertools::Itertools;
use pathfinding::prelude::dijkstra_all;
use std::cmp::{max, min};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

fn main() {}

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StatePart1 {
    pos: String,
    time_remaining: u32,
    open_nodes: Vec<String>,
    will_release: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StatePart2 {
    my_pos: String,
    el_pos: String,
    my_tta: u32,
    el_tta: u32,
    time_remaining: u32,
    open_nodes: Vec<String>,
    will_release: u32,
}

type Names = HashSet<String>;
type Flows = HashMap<String, u32>;
type Neighbours = HashMap<String, HashSet<String>>;
type Graph = HashMap<String, HashMap<String, u32>>;

const START: &str = "AA";

pub fn solve_part1(input: String) -> Result<u32, Error> {
    let (names, flows, neighbours) = parse(input)?;
    let graph = build_graph(&names, &flows, &neighbours);
    let max_released = max_released_part1(30, &flows, &graph);
    Ok(max_released)
}

pub fn solve_part2(input: String) -> Result<u32, Error> {
    let (names, flows, neighbours) = parse(input)?;
    let graph = build_graph(&names, &flows, &neighbours);
    let max_released = max_released_part2(26, &flows, &graph);
    Ok(max_released)
}

fn max_released_part2(time_remaining: u32, flows: &Flows, graph: &Graph) -> u32 {
    let mut max_released = 0;
    fn goto_next(state: StatePart2, max_released: &mut u32, flows: &Flows, graph: &Graph) {
        if state.my_tta == 0 {
            let my_candidates = graph
                .get(&state.my_pos)
                .unwrap()
                .iter()
                .filter(|(dest, _dist)| !state.open_nodes.contains(dest));
            for (my_dest, my_dist) in my_candidates {
                if state.time_remaining < my_dist + 2 {
                    continue; // Not enough time to go to dest, open valve and release some pressure
                }
                let mut new_state = state.clone();
                let time_to_open = my_dist + 1;
                let time_active = new_state.time_remaining - time_to_open;
                let time_to_next_action = min(time_to_open, state.el_tta);
                let flow = flows.get(my_dest).unwrap();
                new_state.my_pos = my_dest.clone();
                new_state.my_tta = time_to_open - time_to_next_action;
                new_state.el_tta -= time_to_next_action;
                new_state.time_remaining -= time_to_next_action; // Time it takes to get to dist and open valve
                new_state.open_nodes.push(my_dest.clone());
                new_state.will_release += time_active * flow;
                *max_released = max(*max_released, new_state.will_release);
                goto_next(new_state, max_released, flows, graph);
            }
        }
        if state.el_tta == 0 {
            let el_candidates = graph
                .get(&state.el_pos)
                .unwrap()
                .iter()
                .filter(|(dest, _dist)| !state.open_nodes.contains(dest));
            for (el_dest, el_dist) in el_candidates {
                if state.time_remaining < el_dist + 2 {
                    continue; // Not enough time to go to dest, open valve and release some pressure
                }
                let mut new_state = state.clone();
                let time_to_open = el_dist + 1;
                let time_active = new_state.time_remaining - time_to_open;
                let time_to_next_action = min(time_to_open, state.my_tta);
                let flow = flows.get(el_dest).unwrap();
                new_state.el_pos = el_dest.clone();
                new_state.el_tta = time_to_open - time_to_next_action;
                new_state.my_tta -= time_to_next_action;
                new_state.time_remaining -= time_to_next_action; // Time it takes to get to dist and open valve
                new_state.open_nodes.push(el_dest.clone());
                new_state.will_release += time_active * flow;
                if potential_release(&new_state, flows) < *max_released {
                    continue;
                }
                *max_released = max(*max_released, new_state.will_release);
                goto_next(new_state, max_released, flows, graph);
            }
        }
    }
    let init_state = StatePart2 {
        my_pos: START.to_string(),
        el_pos: START.to_string(),
        my_tta: 0,
        el_tta: 0,
        open_nodes: vec![],
        time_remaining,
        will_release: 0,
    };
    goto_next(init_state, &mut max_released, flows, graph);
    println!("max_released={max_released}");
    max_released
}

fn max_released_part1(time_remaining: u32, flows: &Flows, graph: &Graph) -> u32 {
    let mut max_released = 0;
    fn goto_next(state: StatePart1, max_released: &mut u32, flows: &Flows, graph: &Graph) {
        let candidates = graph
            .get(&state.pos)
            .unwrap()
            .iter()
            .filter(|(dest, _dist)| !state.open_nodes.contains(dest));
        for (dest, dist) in candidates {
            if state.time_remaining < dist + 2 {
                continue; // Not enough time to go to dest, open valve and release some pressure
            }
            let mut new_state = state.clone();
            let time_to_open = dist + 1;
            let time_active = new_state.time_remaining - time_to_open;
            let flow = flows.get(dest).unwrap();
            new_state.pos = dest.clone();
            new_state.time_remaining -= time_to_open; // Time it takes to get to dist and open valve
            new_state.open_nodes.push(dest.clone());
            new_state.will_release += time_active * flow;
            *max_released = max(*max_released, new_state.will_release);
            goto_next(new_state, max_released, flows, graph);
        }
    }
    let init_state = StatePart1 {
        pos: START.to_string(),
        open_nodes: vec![],
        time_remaining,
        will_release: 0,
    };
    goto_next(init_state, &mut max_released, flows, graph);
    println!("max_released={max_released}");
    max_released
}

fn potential_release(state: &StatePart2, flows: &Flows) -> u32 {
    let mut potential_release = state.will_release;
    if state.time_remaining > 1 {
        let mut possible_flows = flows
            .iter()
            .filter(|(node, _flow)| !state.open_nodes.contains(node))
            .map(|(_node, flow)| flow)
            .sorted()
            .rev();
        let mut time_remaining = state.time_remaining;
        while time_remaining > 1 {
            // Open two valves (me and elephant)
            time_remaining -= 1;
            if let Some(flow) = possible_flows.next() {
                potential_release += time_remaining * flow
            }
            if let Some(flow) = possible_flows.next() {
                potential_release += time_remaining * flow
            }
            // Go to next valve
            time_remaining -= 1;
        }
    }
    potential_release
}

fn build_graph(names: &Names, flows: &Flows, neighbours: &Neighbours) -> Graph {
    let mut graph = Graph::new();
    for start in names {
        let reachables = dijkstra_all(start, |name| {
            neighbours
                .get(name)
                .unwrap()
                .iter()
                .map(|neighbour| (neighbour.clone(), 1))
        });
        for (dest, (_parent, dist)) in &reachables {
            if *flows.get(dest).unwrap() > 0 {
                match graph.entry(start.clone()) {
                    Entry::Vacant(e) => {
                        e.insert(HashMap::from([(dest.clone(), *dist)]));
                    }
                    Entry::Occupied(mut e) => {
                        e.get_mut().insert(dest.clone(), *dist);
                    }
                }
            }
        }
        print!("Node {}: ", start);
        for (dest, dist) in graph.get(start).unwrap() {
            print!("({dest}, {dist}) ");
        }
        println!();
    }
    graph
}

fn parse(input: String) -> Result<(Names, Flows, Neighbours), Error> {
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

#[cfg(test)]
pub mod test {
    #[test]
    fn solve_example_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve_part1(input).expect("failed to solve");
        assert_eq!(max_released, 1651);
    }

    #[test]
    fn solve_example_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve_part2(input).expect("failed to solve");
        assert_eq!(max_released, 1707);
    }

    #[test]
    fn solve_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve_part1(input).expect("failed to solve");
        assert_eq!(max_released, 1641);
    }

    #[test]
    fn solve_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve_part2(input).expect("failed to solve");
        assert_eq!(max_released, 2261);
    }
}
