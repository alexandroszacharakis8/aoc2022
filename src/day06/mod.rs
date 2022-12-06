use itertools::Itertools;

pub fn part1() {
    let input = include_str!("input");
    let shifted_by1 = &input[1..];
    let shifted_by2 = &input[2..];
    let shifted_by3 = &input[3..];
    let result = input
        .chars()
        .zip(shifted_by1.chars())
        .zip(shifted_by2.chars())
        .zip(shifted_by3.chars())
        .enumerate()
        .find(|(_, (((a,b),c),d))| {
            let mut v = vec![a,b,c,d]; v.sort();v.dedup(); 
            v.len() == 4
        })
        .unwrap().0 + 4;
    println!("result: {:?}", result);
}

pub fn part2() {
    let input = include_str!("input");
    let input_cloned = input.chars().collect_vec();
    for (i, c) in input.chars().enumerate() {
        let mut v = Vec::new();
        v.push(c);
        for j in 0..14 {
            v.push(input_cloned[i+j]);
        }
        v.sort(); 
        v.dedup();
        if v.len() == 14 {
            println!("result: {:?}", 14+i);
            break;
        }
    }
}