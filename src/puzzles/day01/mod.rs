use std::str::FromStr;

use itertools::Itertools;

pub fn part1() {
    let input = include_str!("input.txt");
    let result = input
        .split("\n\n")
        .map(|el| 
            el
            .lines()
            .map(|line| u64::from_str(line).unwrap()).collect_vec())
            .max_by(|x,y| x.iter().sum::<u64>().cmp(&y.iter().sum::<u64>()))
            .unwrap()
            .iter()
            .sum::<u64>();
    println!("result: {:?}", result);
}

pub fn part2() {
    let input = include_str!("input.txt");
    let result = input
        .split("\n\n")
        .map(|el| 
            el
            .lines()
            .map(|line| u64::from_str(line).unwrap()).collect_vec())
            .sorted_by(|x,y| y.iter().sum::<u64>().cmp(&x.iter().sum::<u64>()))
            .take(3)
            .flatten()
            .sum::<u64>();
    println!("result: {:?}", result);
}
