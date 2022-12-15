use itertools::Itertools;
use std::str::FromStr;
use std::collections::BTreeMap;

pub fn get_element(line: &str, is_sensor: bool) -> (i128, i128) {
    let i = if is_sensor { 2 } else { 8 }; 
    let x = i128::from_str(line.split_whitespace().collect_vec()[i].split('=').nth(1).unwrap().trim_end_matches(',')).unwrap();
    let y = i128::from_str(line.split_whitespace().collect_vec()[i+1].split('=').nth(1).unwrap().trim_end_matches(':')).unwrap();
    (x, y)
}

pub fn parse_input() -> Vec<((i128, i128), (i128, i128))>{
    // let sensors = Vec::new();
    let lines = include_str!("input.txt")
        .lines()
        .map(|line| (get_element(line, true), get_element(line, false)))
        .collect_vec();
    lines
}

pub fn find_non_beacon(sensor: (i128, i128), beacon: (i128, i128), target: i128) -> (i128, i128) {
    let max_distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

    let distance_target = (target - sensor.1).abs();
    if distance_target > max_distance {
        return (1,0) 
    }
    else {
        let mut leftmost = sensor.0 - (max_distance - distance_target);
        let mut rightmost = sensor.0 + (max_distance - distance_target);
        if beacon == (leftmost, target) {leftmost += 1};
        if beacon == (rightmost, target) {rightmost -= 1};
        return (leftmost, rightmost)
    }
}

pub fn insert_range(tree: &mut BTreeMap<i128, i128>, low: i128, high: i128) {
    if high >= low {
        let (addlow, addhigh): (i128, i128);
        let range_containing_low = tree.iter().filter(|(key,value)| low >= **key && low <= **value).nth(0);
        match range_containing_low {
            None => {addlow = low;}
            Some((smallerlow, _)) => {addlow = *smallerlow;}
        }
        let range_containing_high = tree.iter().filter(|(key,value)| high >= ** key && high <= **value).nth(0);
        match range_containing_high {
            None => {addhigh = high;}
            Some((_, biggery)) => {addhigh = *biggery;}
        }
        let removed = tree.iter().filter(|(key,value)| **key >= addlow && **value <= addhigh).map(|x| (*x.0,*x.1)).collect_vec();
        let _ = removed.iter().for_each(|(k, _)| {tree.remove(&k);});
        tree.insert(addlow, addhigh);
    }
}

pub fn part1() {

    let input = parse_input();
    let mut result:BTreeMap<i128, i128> = BTreeMap::new();

    for (sensor, beacon) in input {
        let target = 2_000_000;
        let (low, high) = find_non_beacon(sensor, beacon, target);

        insert_range(&mut result, low, high); 
    }
    let sum = result
        .iter()
        .map(|r| r.1 - r.0 + 1)
        .sum::<i128>();
    println!("{:?}", sum);
}

pub fn part2() {
    let input = parse_input();
    let mut result:Vec<BTreeMap<i128, i128>> = vec![BTreeMap::new(); 4_000_001];

    for (_, beacon) in input.iter() {
        if beacon.0 <= 4_000_000 && beacon.1 <= 4_000_000 && beacon.0 >= 0 && beacon.1 >= 0 {
            result[beacon.1 as usize].insert(beacon.0, beacon.0);
        }
    }
    for (sensor, beacon) in input.iter() {
        let max_distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        let max_y = (sensor.1 + max_distance).min(4_000_000);
        let min_y = (sensor.1 - max_distance).max(0);
        for target in min_y..=max_y {
            let (mut low, mut high) = find_non_beacon(*sensor, *beacon, target);
            if low < 0 { low = 0; }
            if high > 4_000_000 { high = 4_000_000; }
            insert_range(&mut result[target as usize], low, high); 
        }
    }
    for target in 0..=4_000_000 {
        let points = result[target as usize].iter().map(|r| r.1 - r.0 + 1).sum::<i128>(); 
        if points == 4_000_000 {
            // not considering edge cases
            println!("{:?}", (result[target as usize].iter().last().unwrap().0 - 1)* 4_000_000 + target);
        }
    }
}
