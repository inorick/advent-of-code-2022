use std::{
    collections::HashSet,
    fs::{self},
};

fn get_input(path: &str) -> String {
    fs::read_to_string(path).expect("read file failed")
}

pub fn solve(path: &str) {
    let s = &get_input(path);
    println!("{}", detect_marker(s, 4));
    println!("{}", detect_message(s, 14));
}

fn all_different(cs: &Vec<char>) -> bool {
    let mut s = HashSet::new();
    for c in cs.iter() {
        s.insert(c);
    }
    s.len() == cs.len()
}

fn detect_message(s: &str, marker_len: usize) -> u32 {
    let pos = detect(s, marker_len);
    pos - 1 - (marker_len as u32)
}

fn detect_marker(s: &str, marker_len: usize) -> u32 {
    let pos = detect(s, marker_len);
    pos + marker_len as u32 + 1
}

fn detect(s: &str, marker_len: usize) -> u32 {
    let mut pos = 0;
    let mut stack = vec![];
    for c in s.chars() {
        pos += 1;
        stack.push(c);
        if stack.len() == marker_len && all_different(&stack) {
            return pos;
        }
        if stack.len() > marker_len - 1 {
            stack.remove(0);
        }
    }
    pos
}

#[test]
fn test_all_different() {
    assert!(!all_different(&"bvwb".chars().collect()));
    assert!(all_different(&"bvwg".chars().collect()));
}

#[test]
fn test_detect_marker() {
    assert_eq!(detect_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
    assert_eq!(detect_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
    assert_eq!(detect_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
    assert_eq!(detect_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
}

#[test]
fn test_detect_message() {
    assert_eq!(detect_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
    assert_eq!(detect_message("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
    assert_eq!(detect_message("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
    assert_eq!(detect_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
    assert_eq!(detect_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
}
