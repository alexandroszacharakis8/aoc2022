use std::str::FromStr;

use itertools::Itertools;

pub fn part1() {
    let target_values = vec![20,60,100,140,180,220];
    let result = include_str!("input.txt")
        .lines()
        .map(|line| line.split_whitespace().collect_vec())
        .fold((0, 1, 0, &target_values[..]), |(cycle, x, sum, list), command| {
            if list.len() == 0 { return (cycle, x, sum, list) }
            match command[0] {
                "noop" => {
                    if cycle + 1 == list[0] {
                        (cycle + 1, x, sum + x * list[0], &list[1..])
                    }
                    else {
                        (cycle + 1, x, sum, list)
                    }
                }
                "addx" => {
                    let v = i64::from_str(command[1]).unwrap();
                    if cycle + 2 >= list[0] {
                        (cycle + 2, x + v, sum + x * list[0], &list[1..])
                    }
                    else {
                        (cycle + 2, x + v, sum, list)
                    }
                },
                _ => panic!()
            }
        });
    println!("result: {:?}", result.2);
}

pub fn part2() {
    let _ = include_str!("input.txt")
        .lines()
        .map(|line| line.split_whitespace().collect_vec())
        .fold((0, 1), |(cycle, x), command| {
            if cycle % 40 == 0 { print!("\n") };
            match command[0] {
                "noop" => {
                    if x - 1 == cycle % 40 || x == cycle % 40 || x + 1 == cycle % 40 {
                        print!("@");
                        (cycle + 1, x)
                    }
                    else {
                        print!(" ");
                        (cycle + 1, x)
                    }
                }
                "addx" => {
                    let v = i64::from_str(command[1]).unwrap();
                    if x - 1 == cycle % 40 || x == cycle % 40 || x + 1 == cycle % 40 {
                        print!("@");
                    }
                    else {
                        print!(" ");
                    }
                    if (cycle + 1) % 40 == 0 { print!("\n") };
                    if x - 1 == (cycle + 1) % 40 || x == (cycle + 1) % 40 || x + 1 == (cycle + 1) % 40 {
                        print!("@");
                    }
                    else {
                        print!(" ");
                    }
                    (cycle + 2, x + v)
                },
                _ => panic!()
            }
        });
        println!();
}