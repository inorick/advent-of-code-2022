extern crate core;

use crate::Error::InvalidInput;
use itertools::Itertools;

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

pub struct Report {
    pub sensor: (i32, i32),
    pub beacon: (i32, i32),
}

fn main() {}

pub fn solve_part1(input: String, line_y: i32) -> Result<u32, Error> {
    let reports = parse_reports(input);

    // Find possible left- and rightmost x coordinates of beacons
    let x_min = reports
        .iter()
        .map(|r| r.sensor.0 - dist(r.beacon, r.sensor) as i32)
        .min()
        .ok_or(InvalidInput)?;
    let x_max = reports
        .iter()
        .map(|r| r.sensor.0 + dist(r.beacon, r.sensor) as i32)
        .max()
        .ok_or(InvalidInput)?;

    // Check line at given y coordinate
    let mut sum = 0;
    for x in x_min..=x_max {
        let p = (x, line_y);

        // Check if p is on the position of a beacon
        if reports.iter().any(|r| r.beacon == p) {
            continue;
        }

        // Check if p is closer than the closest beacon
        for report in &reports {
            let dist_p_to_sensor = dist(p, report.sensor);
            let dist_sensor_to_beacon = dist(report.sensor, report.beacon);
            if dist_p_to_sensor <= dist_sensor_to_beacon {
                // println!("Cannot contain a beacon: ({}, {})", p.0, p.1);
                sum += 1;
                break;
            }
        }
    }
    Ok(sum)
}

pub fn solve_part2(input: String) -> Result<u64, Error> {
    let reports = parse_reports(input);

    // Check line by line for a possible spot for the beacon
    const Y_MAX: i32 = 4000000;
    const X_MIN: i32 = 0;
    const X_MAX: i32 = 4000000;
    const Y_MIN: i32 = 0;
    let mut p = (X_MIN, Y_MIN);
    while p.1 <= Y_MAX {
        let mut is_solution = true;
        for report in &reports {
            let dist_p_to_sensor = dist(p, report.sensor);
            let dist_sensor_to_beacon = dist(report.sensor, report.beacon);
            if dist_p_to_sensor <= dist_sensor_to_beacon {
                let radius = dist(report.sensor, report.beacon);
                let p_y_diff = report.sensor.1.abs_diff(p.1);
                p.0 = report.sensor.0 + (radius - p_y_diff + 1) as i32;
                // println!("Updated p: ({}, {})", p.0, p.1);
                is_solution = false;
                break;
            }
        }
        if is_solution {
            return Ok(p.0 as u64 * X_MAX as u64 + p.1 as u64);
        }
        if p.0 > X_MAX {
            p.0 = X_MIN;
            p.1 += 1;
        }
    }
    Err(InvalidInput)
}

fn parse_reports(input: String) -> Vec<Report> {
    input
        .lines()
        .flat_map(|line| {
            line.split(' ')
                .filter(|line| line.contains('='))
                .map(|assignment| {
                    let mut def = assignment.to_string();
                    def.retain(|c| c == '-' || c.is_numeric());
                    def
                })
                .filter_map(|coord| coord.parse::<i32>().ok())
                .tuples::<(i32, i32)>()
                .tuples::<((i32, i32), (i32, i32))>()
                .map(|(sensor, beacon)| Report { sensor, beacon })
        })
        .collect()
}

pub fn dist(p1: (i32, i32), p2: (i32, i32)) -> u32 {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

#[cfg(test)]
pub mod test {
    #[test]
    fn solve_example() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/example_input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sum = crate::solve_part1(input, 10).expect("failed to solve");
        assert_eq!(sum, 26);
    }

    #[test]
    fn solve_part1() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let sum = crate::solve_part1(input, 2000000).expect("failed to solve");
        assert_eq!(sum, 5878678);
    }

    #[test]
    fn solve_part2() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input.txt");
        let input = std::fs::read_to_string(&path).expect("failed to read file");
        let score = crate::solve_part2(input).expect("failed to solve");
        assert_eq!(score, 11796491041245);
    }
}
