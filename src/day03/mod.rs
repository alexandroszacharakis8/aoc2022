use std::collections::HashSet;

use itertools::Itertools;

pub fn part1() {
    let input = include_str!("input");
    let result: u64 = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let compartments = l.split_at(l.len()/2);
            let mut item_set = HashSet::new();
            for item in compartments.0.chars() {
                item_set.insert(item);
            }
            let duplicate = compartments.1
                .chars()
                .filter(|item| item_set.contains(item))
                .next()
                .unwrap();
            if duplicate as u64 > 96 {
                duplicate as u64 - 96
            }
            else {
                duplicate as u64 - 64 + 26
            }
        })
        .sum();
    println!("result: {:?}", result);
}

pub fn part2() {
    let input = include_str!("input");
    let lines = input
        .lines()
        .filter(|l| !l.is_empty())
        .collect_vec();

    let mut item_set = HashSet::new();
    let mut item_set_doubles = HashSet::new();
    let mut badges = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        if line_num % 3 == 0 {
            item_set.clear();
            item_set_doubles.clear();
            for item in line.chars() {
                item_set.insert(item);
            }
        } 
        else if line_num % 3 == 1 {
            for item in line.chars() {
                if item_set.contains(&item) { item_set_doubles.insert(item); }
            }
        }
        else {
            for item in line.chars() {
                if item_set_doubles.contains(&item) { badges.push(item); break; }
            }
        }
    }
    let result: u64 = badges
        .iter()
        .map(|item| 
            if *item as u64 > 96 {
               *item as u64 - 96
            }
            else {
                *item as u64 - 64 + 26
            }
        )
        .sum();

    println!("result: {:?}", result);
}
