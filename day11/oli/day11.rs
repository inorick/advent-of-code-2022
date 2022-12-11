#![allow(dead_code)]
use std::{
    collections::{HashMap, VecDeque},
    fmt,
};

struct Monkey {
    index: usize,
    activity_count: usize,
    test: (u128, u128, u128),
    starting_items: VecDeque<u128>,
    op: Box<dyn Fn(u128) -> u128>,
}
impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Monkey {}: {:?}", self.index, self.starting_items)
    }
}
impl Monkey {
    fn new(
        index: usize,
        v: Vec<u128>,
        op: Box<dyn Fn(u128) -> u128>,
        test: (u128, u128, u128),
    ) -> Self {
        Self {
            index,
            activity_count: 0,
            test,
            starting_items: VecDeque::from(v),
            op,
        }
    }
    fn next(&self, x: u128) -> u128 {
        if x % self.test.0 == 0 {
            self.test.1
        } else {
            self.test.2
        }
    }
}

fn init() -> Vec<Monkey> {
    vec![
        Monkey::new(0, vec![71, 56, 50, 73], Box::new(|x| x * 11), (13, 1, 7)),
        Monkey::new(1, vec![70, 89, 82], Box::new(|x| x + 1), (7, 3, 6)),
        Monkey::new(2, vec![52, 95], Box::new(|x| x * x), (3, 5, 4)),
        Monkey::new(3, vec![94, 64, 69, 87, 70], Box::new(|x| x + 2), (19, 2, 6)),
        Monkey::new(
            4,
            vec![98, 72, 98, 53, 97, 51],
            Box::new(|x| x + 6),
            (5, 0, 5),
        ),
        Monkey::new(5, vec![79], Box::new(|x| x + 7), (2, 7, 0)),
        Monkey::new(
            6,
            vec![77, 55, 63, 93, 66, 90, 88, 71],
            Box::new(|x| x * 7),
            (11, 2, 4),
        ),
        Monkey::new(
            7,
            vec![54, 97, 87, 70, 59, 82, 59],
            Box::new(|x| x + 8),
            (17, 1, 3),
        ),
    ]
}

fn init_tests() -> Vec<Monkey> {
    vec![
        Monkey::new(0, vec![79, 98], Box::new(|x| x * 19), (23, 2, 3)),
        Monkey::new(1, vec![54, 65, 75, 74], Box::new(|x| x + 6), (19, 2, 0)),
        Monkey::new(2, vec![79, 60, 97], Box::new(|x| x * x), (13, 1, 3)),
        Monkey::new(3, vec![74], Box::new(|x| x + 3), (17, 0, 1)),
    ]
}

// worry level to be divided by three and rounded down to the nearest integer
pub fn solve() {
    {
        let monkeys = init_tests();
        let monkeys = play_rounds(20, monkeys, Box::new(calm));
        for m in &monkeys {
            println!("Monkey {}: {}", m.index, m.activity_count);
        }
        println!("monkey_business: {}", monkey_business(&monkeys));
    }
    {
        let monkeys = init();
        let monkeys = play_rounds(20, monkeys, Box::new(calm));
        for m in &monkeys {
            println!("Monkey {}: {}", m.index, m.activity_count);
        }
        println!("monkey_business: {}", monkey_business(&monkeys));
    }
    {
        let monkeys = init();
        let monkeys = play_rounds(10000, monkeys, Box::new(reduce2));
        for m in &monkeys {
            println!("Monkey {}: {}", m.index, m.activity_count);
        }
        println!("monkey_business: {}", monkey_business(&monkeys));
    }
}

fn reduce1(worry: u128) -> u128 {
    worry % (13 * 17 * 19 * 23)
}
fn reduce2(worry: u128) -> u128 {
    worry % (13 * 7 * 3 * 19 * 5 * 2 * 11 * 17)
}
fn calm(worry: u128) -> u128 {
    let x: f32 = worry as f32 / 3.0;
    x.floor() as u128
}

fn round(mut monkeys: Vec<Monkey>, f: &dyn Fn(u128) -> u128) -> Vec<Monkey> {
    let mut to_add: HashMap<usize, Vec<u128>> = HashMap::new();
    monkeys.iter_mut().for_each(|mut m| {
        // add items previously passed by other monkey
        if let Some(value) = to_add.get(&m.index) {
            for v in value {
                m.starting_items.push_back(*v);
            }
            to_add.remove(&m.index);
        }
        while let Some(i) = m.starting_items.pop_front() {
            m.activity_count += 1;
            let worry = (m.op)(i);
            let calm = f(worry);
            let next = m.next(calm);
            to_add
                .entry(next as usize)
                .and_modify(|v| v.push(calm))
                .or_insert_with(|| vec![calm]);
        }
    });
    for (k, v) in to_add {
        for item in v {
            monkeys[k].starting_items.push_back(item);
        }
    }
    monkeys
}

fn print_monkeys(monkeys: &Vec<Monkey>) {
    for m in monkeys {
        println!("{m:?}");
    }
}

fn play_rounds(count: usize, monkeys: Vec<Monkey>, f: Box<dyn Fn(u128) -> u128>) -> Vec<Monkey> {
    let mut res = monkeys;
    for _ in 0..count {
        res = round(res, &f);
    }
    print_monkeys(&res);
    res
}

fn monkey_business(monkeys: &[Monkey]) -> usize {
    let mut activities: Vec<usize> = monkeys.iter().map(|m| m.activity_count).collect();
    println!("monkey_business numbers: {activities:?}");
    activities.sort();
    activities[activities.len() - 1] * activities[activities.len() - 2]
}

#[test]
fn test_abc() {
    // let monkeys = init_tests();
    let monkeys = init();
    let monkeys = play_rounds(10000, monkeys, Box::new(calm));
    for m in &monkeys {
        println!("Monkey {}: {}", m.index, m.activity_count);
    }
    println!("monkey_business: {}", monkey_business(&monkeys));
}

