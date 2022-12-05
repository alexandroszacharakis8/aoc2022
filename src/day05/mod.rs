use std::str::FromStr;

use itertools::Itertools;

// const S_EXAMPLE: usize = 3;
const S_INPUT: usize = 9;

pub fn get_stacks(stacks: Vec<&str>) -> Vec<Vec<char>> {
    let mut res = Vec::new();
    for _ in 0..S_INPUT {
        let stack = Vec::new();
        res.push(stack);
    }
    for line in stacks {
        for (i, el) in line.chars().enumerate() {
            if el.is_ascii_alphabetic() {
                res[i/4].push(el);
            }
        }
    }
    for stack in res.iter_mut() {
        stack.reverse()
    }
    res
}

pub fn get_moves(moves: Vec<&str>) -> Vec<(usize,usize,usize)> {
    moves.iter().map(|m|  m.split(" ").filter_map(|m| usize::from_str(m).ok()).collect_tuple().unwrap()).collect_vec()
}

pub fn apply_move_p1((times, from, to): &(usize, usize, usize), stacks: &mut Vec<Vec<char>>, ) {
    for _ in 0..*times {
        let elem = stacks[*from-1].pop().unwrap();
        stacks[*to-1].push(elem)
    }
}

pub fn apply_move_p2((times, from, to): &(usize, usize, usize), stacks: &mut Vec<Vec<char>>) {
    let mut taken = vec![];
    for _ in 0..*times {
        let elem = stacks[*from-1].pop().unwrap();
        taken.push(elem)
    }
    for _ in 0..*times {
        let elem = taken.pop().unwrap();
        stacks[*to-1].push(elem);
    }
}

pub fn part1() {
    let (stacks, moves) = include_str!("input").split("\n\n").map(|el| el.lines().collect_vec()).collect_tuple().unwrap();
    let (mut stacks, moves) = (get_stacks(stacks), get_moves(moves));
    for mov in moves {
        apply_move_p1(&mov, &mut stacks);
    }
    let mut res: Vec<char> = Vec::new();
    for mut stack in stacks {
        res.push(stack.pop().unwrap())
    }
    println!("result: {:?}", res.into_iter().collect::<String>());
}

pub fn part2() {
    let (stacks, moves) = include_str!("input").split("\n\n").map(|el| el.lines().collect_vec()).collect_tuple().unwrap();
    let (mut stacks, moves) = (get_stacks(stacks), get_moves(moves));
    for mov in moves {
        apply_move_p2(&mov, &mut stacks);
    }
    let mut res: Vec<char> = Vec::new();
    for mut stack in stacks {
        res.push(stack.pop().unwrap())
    }
    println!("result: {:?}", res.into_iter().collect::<String>());
}
