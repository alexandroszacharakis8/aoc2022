use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
pub struct Monkey {
    items: Vec<u64>,
    operation_type: char,
    operation_value: u64,
    test: u64,
    target_success: usize,
    target_failure: usize,
    inspected_items: usize,
}

impl Monkey {
    fn new(items: Vec<u64>, operation_type: char, operation_value: u64, test: u64, target_success: usize, target_failure: usize) -> Self{
        Monkey {
            items,
            operation_type,
            operation_value,
            test,
            target_success,
            target_failure,
            inspected_items: 0,
        }
    }
    fn act_cool(&mut self) -> (usize, u64) {
        self.inspected_items += 1;
        let initial_value = self.items.pop().unwrap();
        let inspected_value = 
            match self.operation_type {
                '+' => initial_value + self.operation_value,
                '*' => initial_value * self.operation_value,
                '^' => initial_value * initial_value,
                _ => panic!()
            } / 3;
        if inspected_value % self.test == 0 { 
            return (self.target_success, inspected_value) 
        } else { 
            return (self.target_failure, inspected_value) 
        }
    }
    fn act_uncool(&mut self, modulo: u64) -> (usize, u64) {
        self.inspected_items += 1;
        let initial_value = self.items.pop().unwrap();
        let inspected_value = 
            match self.operation_type {
                '+' => initial_value + self.operation_value,
                '*' => initial_value * self.operation_value,
                '^' => initial_value * initial_value,
                _ => panic!()
            } % modulo ;
        if inspected_value % self.test == 0 { 
            return (self.target_success, inspected_value) 
        } else { 
            return (self.target_failure, inspected_value) 
        }
    }
}

pub fn parse() -> Vec<Monkey> {
    let input = include_str!("input")
        .lines()
        .filter(|line| !line.is_empty());
    let mut monkeys = Vec::new();
    for chunk in &input.chunks(6) {
        let monkey_info = chunk.collect_vec();
        let mut items = vec![]; 
        for item in monkey_info[1].split_whitespace().dropping(2) {
            items.push(u64::from_str(item.trim_end_matches(',')).unwrap());
        }
        items.reverse();
        let mut operation_type = monkey_info[2].split_whitespace().nth(4).unwrap().chars().nth(0).unwrap();
        let try_operation_value = u64::from_str(monkey_info[2].split_whitespace().nth(5).unwrap());
        let operation_value;
        if try_operation_value.is_err() {
            operation_type = '^';
            operation_value = 0;
        }
        else {
            operation_value  = try_operation_value.unwrap();
        }
        let test = u64::from_str(monkey_info[3].split_whitespace().nth(3).unwrap()).unwrap();
        let target_success = usize::from_str(monkey_info[4].split_whitespace().nth(5).unwrap()).unwrap();
        let target_failure = usize::from_str(monkey_info[5].split_whitespace().nth(5).unwrap()).unwrap();

        let monkey = Monkey::new(
            items,
            operation_type,
            operation_value,
            test,
            target_success,
            target_failure,
        );

        monkeys.push(monkey);
    }
    monkeys
}

pub fn part1() {
    let mut monkeys = parse();
    // round
    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            while !monkeys[idx].items.is_empty() {
                let (target, value) = monkeys[idx].act_cool();
                monkeys[target].items.push(value);
            }
        }
    }
    let mut held = monkeys.iter().map(|monkey| monkey.inspected_items).collect_vec();
    held.sort(); held.reverse();
    let result = held[0] * held[1];
    println!("{:?}", result);
}

pub fn part2() {
    let mut monkeys = parse();
    let modulo = monkeys.iter().map(|monkey| monkey.test).product();
    // round
    for _ in 0..10000 {
        for idx in 0..monkeys.len() {
            while !monkeys[idx].items.is_empty() {
                let (target, value) = monkeys[idx].act_uncool(modulo);
                monkeys[target].items.push(value);
            }
        }
    }
    let mut held = monkeys.iter().map(|monkey| monkey.inspected_items).collect_vec();
    held.sort(); held.reverse();
    let result = held[0] * held[1];
    println!("{:?}", result);
}