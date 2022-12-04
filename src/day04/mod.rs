use std::str::FromStr;

use itertools::Itertools;

pub fn part1() {
    let input = include_str!("input");
    let result = input
        .lines()
        .map(|line| line
                .split(',')
                .map(|pair| pair.split('-').map(|el| u64::from_str(el).unwrap()).collect_vec())
                .collect_vec()
        )
        .filter(|pair| 
            (pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1]) ||  
            (pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1])
        )
        .count();
    println!("result: {:?}", result);
}

pub fn part2() {
    let input = include_str!("input");
    let result = input
        .lines()
        .map(|line| line
                .split(',')
                .map(|pair| pair.split('-').map(|el| u64::from_str(el).unwrap()).collect_vec())
                .collect_vec()
        )
        .filter(|pair| 
            (pair[1][0] <= pair[0][1] && pair[1][0] >= pair[0][0]) ||  
            (pair[0][0] <= pair[1][1] && pair[0][0] >= pair[1][0]) 
        )
        .count();
    println!("result: {:?}", result);
}
