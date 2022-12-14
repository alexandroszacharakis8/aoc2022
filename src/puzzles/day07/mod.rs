use itertools::Itertools;
use std::str::FromStr;

pub fn traverse(lines: &mut Vec<Vec<&str>>, result: &mut Vec<u64>) -> u64 {
    // sum contains the sum of the explored node
    let mut sum = 0;
    while lines.len() > 0 {
        // check out the next "move"
        let line = lines.pop().unwrap();
        match (line[0], line[1]) {
            // change dir: either go back adding the sum of the current dir to parent or go deeper on the filesystem
            ("$", "cd") => 
                if line[2] == ".." { 
                    result.push(sum); return sum 
                } else { 
                    sum += traverse(lines, result) 
                },
            // do nothing on list or on directories (might break in more general cases)
            ("$", "ls") => { },
            ("dir", _) => { },
            // if a single file contribute to the sum
            (_, _) => sum += u64::from_str(line[0]).unwrap(),
        }
    }
    result.push(sum);
    sum
}

pub fn part1() {
    let mut input = include_str!("input.txt")
        .lines()
        .map(|line| line.split(" ").collect_vec()).rev().collect_vec();
    let mut sizes = Vec::new();
    let _ = traverse(&mut input, &mut sizes);
    let result: u64 = sizes.iter().filter(|&&x| x <= 100000).sum::<u64>();
    println!("result: {:?}", result);
}

pub fn part2() {
    let mut input = include_str!("input.txt")
        .lines()
        .map(|line| line.split(" ").collect_vec()).rev().collect_vec();
    let mut sizes = Vec::new();
    let total_size = traverse(&mut input, &mut sizes);
    let result: &u64 = sizes.iter().filter(|&&x| 70000000 - total_size + x >= 30000000).min().unwrap();
    println!("result: {:?}", result);
}