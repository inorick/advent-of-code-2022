#![allow(dead_code)]
use std::{
    collections::{HashMap, VecDeque},
    fmt,
};

struct Monkey {
    index: usize,
    activity_count: usize,
    starting_items: VecDeque<u128>,
    op: Box<dyn Fn(u128) -> u128>,
    next: Box<dyn Fn(u128) -> u128>,
}
impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Monkey {}: {:?}", self.index, self.starting_items)
    }
}

fn init() -> Vec<Monkey> {
    vec![
        Monkey {
            index: 0,
            activity_count: 0,
            starting_items: VecDeque::from(vec![71, 56, 50, 73]),
            op: Box::new(|x| x * 11),
            next: Box::new(|x| if x % 13 == 0 { 1 } else { 7 }),
        },
        Monkey {
            index: 1,
            activity_count: 0,
            starting_items: VecDeque::from(vec![70, 89, 82]),
            op: Box::new(|x| x + 1),
            next: Box::new(|x| if x % 7 == 0 { 3 } else { 6 }),
        },
        Monkey {
            index: 2,
            activity_count: 0,
            starting_items: VecDeque::from(vec![52, 95]),
            op: Box::new(|x| x * x),
            next: Box::new(|x| if x % 3 == 0 { 5 } else { 4 }),
        },
        Monkey {
            index: 3,
            activity_count: 0,
            starting_items: VecDeque::from(vec![94, 64, 69, 87, 70]),
            op: Box::new(|x| x + 2),
            next: Box::new(|x| if x % 19 == 0 { 2 } else { 6 }),
        },
        Monkey {
            index: 4,
            activity_count: 0,
            starting_items: VecDeque::from(vec![98, 72, 98, 53, 97, 51]),
            op: Box::new(|x| x + 6),
            next: Box::new(|x| if x % 5 == 0 { 0 } else { 5 }),
        },
        Monkey {
            index: 5,
            activity_count: 0,
            starting_items: VecDeque::from(vec![79]),
            op: Box::new(|x| x + 7),
            next: Box::new(|x| if x % 2 == 0 { 7 } else { 0 }),
        },
        Monkey {
            index: 6,
            activity_count: 0,
            starting_items: VecDeque::from(vec![77, 55, 63, 93, 66, 90, 88, 71]),
            op: Box::new(|x| x * 7),
            next: Box::new(|x| if x % 11 == 0 { 2 } else { 4 }),
        },
        Monkey {
            index: 7,
            activity_count: 0,
            starting_items: VecDeque::from(vec![54, 97, 87, 70, 59, 82, 59]),
            op: Box::new(|x| x + 8),
            next: Box::new(|x| if x % 17 == 0 { 1 } else { 3 }),
        },
    ]
}

fn init_tests() -> Vec<Monkey> {
    vec![
        Monkey {
            index: 0,
            activity_count: 0,
            starting_items: VecDeque::from(vec![79, 98]),
            op: Box::new(|x| x * 19),
            next: Box::new(|x| if x % 23 == 0 { 2 } else { 3 }),
        },
        Monkey {
            index: 1,
            activity_count: 0,
            starting_items: VecDeque::from(vec![54, 65, 75, 74]),
            op: Box::new(|x| x + 6),
            next: Box::new(|x| if x % 19 == 0 { 2 } else { 0 }),
        },
        Monkey {
            index: 2,
            activity_count: 0,
            starting_items: VecDeque::from(vec![79, 60, 97]),
            op: Box::new(|x| x * x),
            next: Box::new(|x| if x % 13 == 0 { 1 } else { 3 }),
        },
        Monkey {
            index: 3,
            activity_count: 0,
            starting_items: VecDeque::from(vec![74]),
            op: Box::new(|x| x + 3),
            next: Box::new(|x| u128::from(x % 17 != 0)),
        },
    ]
}

// worry level to be divided by three and rounded down to the nearest integer
pub fn solve() {
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
            let next = (m.next)(calm);
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

