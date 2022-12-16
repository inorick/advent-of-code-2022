extern crate core;

use crate::Error::InvalidInput;
use std::cmp::{max, min};
use std::collections::HashSet;

fn main() {}

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}
type Name = String;
type Flow = u32;
type Distance = u32;

#[derive(Clone)]
pub struct Node {
    pub name: Name,
    pub flow_rate: Flow,
    pub neighbours: HashSet<Name>, // Directly reachable nodes
    pub connections: HashSet<(Name, Distance)>, // Indirectly reachable nodes with strict. pos. flow
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

const START: &str = "AA";

pub fn solve(input: String, part2: bool) -> Result<u32, Error> {
    let mut nodes = vec![];

    // Parse nodes
    let lines: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.split(' ').collect())
        .collect();
    for line in lines {
        let name = line[1].to_string();
        let mut rate = line[4].to_string();
        rate.retain(|c| c.is_numeric());
        let rate = rate.parse::<Flow>().map_err(|_| InvalidInput)?;
        let neighbours: HashSet<_> = line[9..]
            .iter()
            .map(|s| {
                let mut s = s.to_string();
                s.truncate(2);
                s
            })
            .collect();
        nodes.push(Node {
            name,
            flow_rate: rate,
            neighbours,
            connections: HashSet::new(),
        })
    }
    let nodes = nodes; // Make immutable

    // Create network of nodes with the starting node and all other notes with a flow rate > 0
    let mut network: Vec<_> = nodes
        .iter()
        .filter(|n| n.flow_rate > 0 || n.name == START)
        .cloned()
        .collect();
    for start in &mut network {
        let mut visited = HashSet::new();
        visited.insert(start.name.clone());
        let mut front = HashSet::new();
        start.neighbours.iter().for_each(|n| {
            front.insert(n.clone());
        });
        let mut distance = 1;
        while !front.is_empty() {
            let mut next_front = HashSet::new();
            for front_node in &front {
                let front_node = nodes
                    .iter()
                    .find(|n| &n.name == front_node)
                    .ok_or(InvalidInput)?;
                if front_node.flow_rate > 0 {
                    start
                        .connections
                        .insert((front_node.name.clone(), distance));
                }
                for neighbour in &front_node.neighbours {
                    if !visited.contains(neighbour) && !front.contains(neighbour) {
                        next_front.insert(neighbour.clone());
                    }
                }
                visited.insert(front_node.name.clone());
            }
            front = next_front;
            distance += 1;
        }

        // Print network
        print!("Node {}: ", start.name);
        for (name, distance) in &start.connections {
            print!("({name}, {distance}) ");
        }
        println!();
    }

    // Solve network resulting network
    if part2 {
        visit_with_elephant(
            START.to_string(),
            START.to_string(),
            0,
            0,
            network.clone(),
            0,
            0,
            0,
        )
    } else {
        visit_alone(START.to_string(), network.clone(), 0, 0, 0)
    }
}

pub fn visit_alone(
    name: String,
    mut open_nodes: Vec<Node>,
    mut minute: u32,
    mut flow_rate: u32,
    mut released: u32,
) -> Result<u32, Error> {
    const MAX_MINUTES: u32 = 30;
    let mut max_released = released;

    // Find self
    let own_node = open_nodes
        .iter_mut()
        .find(|n| n.name == name)
        .ok_or(InvalidInput)?;
    let node_rate = own_node.flow_rate;
    let connections = own_node.connections.clone();

    // Open local valve
    if node_rate > 0 {
        open_nodes.retain(|n| n.name != name);
        minute += 1;
        released += flow_rate;
        max_released = max(max_released, released);
        flow_rate += node_rate;
    }

    if minute == MAX_MINUTES {
        return Ok(max_released);
    }
    if minute > MAX_MINUTES {
        panic!("minute={minute}!");
    }

    // Go somewhere else
    for (name, distance) in connections {
        if minute + distance < MAX_MINUTES && open_nodes.iter().any(|n| n.name == name) {
            let released = released + (distance * flow_rate);
            let max_released_by_visit = visit_alone(
                name,
                open_nodes.clone(),
                minute + distance,
                flow_rate,
                released,
            )?;
            max_released = max(max_released, released);
            max_released = max(max_released, max_released_by_visit);
        }
    }

    // Wait until time runs out
    released += (MAX_MINUTES - minute) * flow_rate;
    max_released = max(max_released, released);

    Ok(max_released)
}

pub fn visit_with_elephant(
    my_node_name: String,
    el_node_name: String,
    mut my_tta: u32,
    mut el_tta: u32,
    mut open_nodes: Vec<Node>,
    mut minute: u32,
    mut flow_rate: u32,
    mut released: u32,
) -> Result<u32, Error> {
    if my_node_name != START && my_node_name == el_node_name {
        panic!("Should never visit the same node");
    }

    const MAX_MINUTES: u32 = 26;
    let mut max_released = released;

    // Find my node
    let my_node = open_nodes
        .iter()
        .find(|n| n.name == my_node_name)
        .ok_or(InvalidInput)?;
    let my_node_rate = my_node.flow_rate;
    let my_connections = my_node.connections.clone();

    // Find elephant node
    let el_node = open_nodes
        .iter()
        .find(|n| n.name == el_node_name)
        .ok_or(InvalidInput)?;
    let el_node_rate = el_node.flow_rate;
    let el_connections = el_node.connections.clone();

    // Open local valves
    let i_arrived = my_tta == 0;
    let el_arrived = el_tta == 0;
    let mut opened_a_valve = false;
    if i_arrived && my_node_rate > 0 {
        open_nodes.retain(|n| n.name != my_node_name);
        flow_rate += my_node_rate;
        opened_a_valve = true;
    }
    if el_arrived && el_node_rate > 0 {
        open_nodes.retain(|n| n.name != el_node_name);
        flow_rate += el_node_rate;
        opened_a_valve = true;
    }
    if opened_a_valve {
        minute += 1;
        my_tta = my_tta.saturating_sub(1);
        el_tta = el_tta.saturating_sub(1);
        released += flow_rate;
        max_released = max(max_released, released);
    }

    if minute == MAX_MINUTES {
        return Ok(max_released);
    }
    if minute > MAX_MINUTES {
        panic!("minute={minute}!");
    }

    // Go somewhere else
    if i_arrived && el_arrived {
        for my_conn in &my_connections {
            for el_conn in &el_connections {
                if my_conn.0 == el_conn.0 {
                    continue; // Never go to the same node
                }
                let time_until_next_event = min(my_conn.1, el_conn.1);
                if minute + time_until_next_event < MAX_MINUTES
                    && open_nodes.iter().any(|n| n.name == my_conn.0)
                    && open_nodes.iter().any(|n| n.name == el_conn.0)
                {
                    let released = released + (time_until_next_event * flow_rate);
                    let max_released_by_visit = visit_with_elephant(
                        my_conn.0.clone(),
                        el_conn.0.clone(),
                        my_conn.1 - time_until_next_event,
                        el_conn.1 - time_until_next_event,
                        open_nodes.clone(),
                        minute + time_until_next_event,
                        flow_rate,
                        released,
                    )?;
                    max_released = max(max_released, released);
                    max_released = max(max_released, max_released_by_visit);
                }
            }
        }
    } else if i_arrived {
        for my_conn in &my_connections {
            if my_conn.0 == el_node_name {
                continue; // Never go to the same node
            }
            let time_until_next_event = min(my_conn.1, el_tta);
            if minute + time_until_next_event < MAX_MINUTES
                && open_nodes.iter().any(|n| n.name == my_conn.0)
            {
                let released = released + (time_until_next_event * flow_rate);
                max_released = max(max_released, released);
                let max_released_this_way = visit_with_elephant(
                    my_conn.0.clone(),
                    el_node_name.clone(),
                    my_conn.1 - time_until_next_event,
                    el_tta - time_until_next_event,
                    open_nodes.clone(),
                    minute + time_until_next_event,
                    flow_rate,
                    released,
                )?;
                max_released = max(max_released, max_released_this_way);
            }
        }
    } else if el_arrived {
        for el_conn in &el_connections {
            if el_conn.0 == my_node_name {
                continue; // Never go to the same node
            }
            let time_until_next_event = min(my_tta, el_conn.1);
            if minute + time_until_next_event < MAX_MINUTES
                && open_nodes.iter().any(|n| n.name == el_conn.0)
            {
                let released = released + (time_until_next_event * flow_rate);
                max_released = max(max_released, released);
                let max_released_this_way = visit_with_elephant(
                    my_node_name.clone(),
                    el_conn.0.clone(),
                    my_tta - time_until_next_event,
                    el_conn.1 - time_until_next_event,
                    open_nodes.clone(),
                    minute + time_until_next_event,
                    flow_rate,
                    released,
                )?;
                max_released = max(max_released, max_released_this_way);
            }
        }
    }

    // Wait until time runs out
    released += (MAX_MINUTES - minute) * flow_rate;
    max_released = max(max_released, released);

    Ok(max_released)
}

#[cfg(test)]
pub mod test {
    #[test]
    fn solve_example() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve(input, false).expect("failed to solve");
        assert_eq!(max_released, 1651);
    }

    #[test]
    fn solve_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve(input, false).expect("failed to solve");
        assert_eq!(max_released, 1641);
    }

    #[test]
    fn solve_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let max_released = crate::solve(input, true).expect("failed to solve");
        assert_eq!(max_released, 1641);
    }
}
