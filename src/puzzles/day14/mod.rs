use itertools::Itertools;
use std::{str::FromStr, collections::HashSet};

pub fn parse_input() -> HashSet<(i32,i32)>{
    let mut rocks = HashSet::new();
    let lines = include_str!("input.txt")
        .lines()
        .map(|line| {
            line
                .split("->")
                .map(|pair| pair.split(",").collect_vec())
                .map(|pair| (i32::from_str(pair[0].trim()).unwrap(), i32::from_str(pair[1].trim()).unwrap()))
                .collect_vec()
        });
    lines.for_each(|line| {
            for i in 0..line.len()-1 {
                let (x1,y1,x2,y2) = (line[i].0, line[i].1, line[i+1].0, line[i+1].1);
                if x1 == x2 { 
                    for y in y1..=y2 { rocks.insert((x1, y)); }
                    for y in y2..=y1 { rocks.insert((x1, y)); }
                }
                if y1 == y2 { 
                    for x in x1..=x2 { rocks.insert((x, y1)); }
                    for x in x2..=x1 { rocks.insert((x, y1)); }
                }
            }
    });
    rocks
}

pub fn part1() {
    let mut rocks = parse_input();
    let maxy = *rocks.iter().map(|(_,y)| y).max().unwrap();
    let mut path = vec![(500,0)];
    for result in 0.. {
        let (mut x, mut y) = path.pop().unwrap();
        while y < maxy {
            let down = rocks.contains(&(x, y+1));
            let left = rocks.contains(&(x-1, y+1));
            let right = rocks.contains(&(x+1, y+1));
            if down && left && right { 
                rocks.insert((x,y)); 
                break
            }
            else if down && left { path.push((x,y)); x += 1; y+=1 }
            else if down { path.push((x,y)); x -= 1; y+=1 }
            else { path.push((x,y)); y += 1}
        }
        if y >= maxy { 
            println!("{:?}", result);
            return 
        }
    }
    
}

pub fn part2() {
    let mut rocks = parse_input();
    let maxy = *rocks.iter().map(|(_,y)| y).max().unwrap();
    let mut path = vec![(500,0)];
    for result in 0.. {
        if rocks.contains(&(500,0)) {
            println!("{:?}", result);
            return 
        }
        let (mut x, mut y) = path.pop().unwrap();
        loop {
            let down = rocks.contains(&(x, y+1));
            let left = rocks.contains(&(x-1, y+1));
            let right = rocks.contains(&(x+1, y+1));
            if down && left && right || y+1 == maxy+2{ 
                rocks.insert((x,y)); 
                break
            }
            else if down && left { path.push((x,y)); x += 1; y+=1 }
            else if down { path.push((x,y)); x -= 1; y+=1 }
            else { path.push((x,y)); y += 1}
        }
    }
}