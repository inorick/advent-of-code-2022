use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path).expect("could not read file input.txt");

    let mut sum: u32 = 0;
    let mut sums: Vec<u32> = vec![];
    io::BufReader::new(file).lines().for_each(|line| {
        if let Ok(line) = line {
            if line.is_empty() {
                sums.push(sum);
                sum = 0;
            } else {
                let n: u32 = line.parse().expect("parse error");
                sum += n;
            }
        }
    });

    sums.sort_by(|a, b| b.cmp(a));
    sums.iter().take(3).for_each(|n| println!("{n}"));
}
