use itertools::Itertools;
use std::{str::FromStr, cmp::Ordering};

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Compare {
    L,
    E,
    R,
}

// a normal list
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum List {
    Nil, 
    Simple(u32, Box<List>), 
    Cons(Box<List>, Box<List>), 
}

pub fn parse_list(line: &[char]) -> List {
    if line.len() == 2 { return List::Nil }

    // first element of list
    let mut x = vec![];

    // rest of list
    let mut xs = vec!['['];

    let mut i = 1;
    // case Simple
    if line[1].is_digit(10) {
        while line[i].is_digit(10) { 
            x.push(line[i]);
            i += 1;
        }
        // we end in , or [ or ]
        while line[i] == ',' { i += 1 }
        for char in &line[i..] {
            xs.push(*char);
        }
        let value = u32::from_str(x.into_iter().collect::<String>().as_str()).unwrap();
        return List::Simple(value, Box::new(parse_list(&xs)))
    }
    // case Cons
    else {
        let mut bracket_difference = 1;
        x.push('[');
        i += 1;
        while bracket_difference != 0 { 
            x.push(line[i]);
            if line[i] == '[' { bracket_difference += 1 } else if line[i] == ']' { bracket_difference -= 1 } 
            i += 1;
        }
        while line[i] == ',' { i += 1 }
        // we end in , or [ or ]
        for char in &line[i..] {
            xs.push(*char);
        }
        return List::Cons(Box::new(parse_list(&x)), Box::new(parse_list(&xs)))
    }
}
    
pub fn compare_lists(left: List, right: List) -> Compare {
    match (left.clone(), right.clone()) {
        (List::Simple(l, ls), List::Simple(r, rs)) => {
            if l < r { return Compare::L }
            else if l > r { return Compare::R }
            else {return compare_lists(*ls, *rs) }
        },
        (List::Cons(l, ls), List::Cons(r, rs)) => {
            let comparison = compare_lists(*l, *r);
            if comparison == Compare::E { return compare_lists(*ls, *rs) }
            else { return comparison }
        },
        (List::Nil, List::Nil) => Compare::E,
        (List::Nil, _) => Compare::L,
        (_, List::Nil) => Compare::R,
        (List::Simple(l, ls), List::Cons(_, _)) => {
            let newleft = List::Cons(
                Box::new(
                    List::Simple(
                        l,
                        Box::new(List::Nil))), 
                ls);
            compare_lists(newleft, right)
        }, 
        (List::Cons(_, _), List::Simple(r,rs)) => {
            let newright = List::Cons(
                Box::new(
                    List::Simple(
                        r,
                        Box::new(List::Nil))), 
                rs);
            compare_lists(left, newright)
        } 
    }
}



pub fn part1() {
    let packets = include_str!("input")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_list(&line.chars().collect_vec()))
        .collect_vec();    
    let mut result = 0;
    for i in 0..packets.len()/2 {
        if compare_lists(packets[2*i].clone(), packets[2*i+1].clone()) == Compare::L {
            result += i + 1;
        }
    }
    println!("result: {:?}", result);
}

pub fn part2() {
    let mut packets = include_str!("input")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_list(&line.chars().collect_vec()))
        .collect_vec();    
    let divider2 = parse_list(&['[', '[', '2', ']', ']']);
    let divider6 = parse_list(&['[', '[', '6', ']', ']']);
    packets.push(divider2.clone()); 
    packets.push(divider6.clone()); 
    packets.sort_by(|a, b| {
        let result = compare_lists(a.clone(),b.clone());
        match result {
            Compare::L => return Ordering::Less,
            Compare::R => return Ordering::Greater,
            Compare::E => return Ordering::Equal,
        }
    });
    let result = packets
        .iter()
        .enumerate()
        .filter(|(_, p)| **p == divider2 || **p == divider6)
        .map(|(i, _)| i + 1)
        .fold(1, |acc, x| acc * x);
    println!("result: {:?}", result);
}