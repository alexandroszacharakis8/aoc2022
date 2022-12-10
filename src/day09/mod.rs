use itertools::Itertools;
use std::str::FromStr;
use std::collections::HashSet;

pub fn apply_moves(head: (i64, i64), tail: (i64, i64), direction: &str) -> ((i64, i64), (i64, i64), String) {
    let next_head: (i64, i64);
    let next_tail: (i64, i64);
    match direction {
        // in the up/down/left/right case, tail always follows head
        "U" => {
            next_head = (head.0 + 1, head.1);
            next_tail = if (next_head.0 - tail.0).abs() > 1 { head } else { tail };
        }
        "D" => {
            next_head = (head.0 - 1, head.1);
            next_tail = if (next_head.0 - tail.0).abs() > 1 { head } else { tail };
        }
        "L" => {
            next_head = (head.0, head.1 - 1);
            next_tail = if (next_head.1 - tail.1).abs() > 1 { head } else { tail };
        }
        "R" => {
            next_head = (head.0, head.1 + 1);
            next_tail = if (next_head.1 - tail.1).abs() > 1 { head } else { tail };
        }
        // in diagonal movements we follow closely the rules about distance
        "UR" => {
            next_head = (head.0 + 1, head.1 + 1);
            next_tail = if (next_head.0 - tail.0).abs() > 1 &&  (next_head.1 - tail.1).abs() > 1 { (tail.0 + 1, tail.1 + 1 ) } 
                        else if (next_head.0 - tail.0).abs() > 1 && (next_head.1 - tail.1).abs() == 0 { (tail.0 + 1, tail.1) } 
                        else if (next_head.1 - tail.1).abs() > 1 && (next_head.0 - tail.0).abs() == 0 { (tail.0, tail.1 + 1) } 
                        else if (next_head.0 - tail.0).abs() > 1 && (next_head.1 - tail.1).abs() == 1 { (tail.0 + 1, tail.1 + 1) } 
                        else if (next_head.1 - tail.1).abs() > 1 && (next_head.0 - tail.0).abs() == 1 { (tail.0 + 1, tail.1 + 1) } 
                        else { tail };
        }
        "UL" => {
            next_head = (head.0 + 1, head.1 - 1);
            next_tail = if (next_head.0 - tail.0).abs() > 1 &&  (next_head.1 - tail.1).abs() > 1 { (tail.0 + 1, tail.1 - 1 ) } 
                        else if (next_head.0 - tail.0).abs() > 1 && (next_head.1 - tail.1).abs() == 0 { (tail.0 + 1, tail.1) } 
                        else if (next_head.1 - tail.1).abs() > 1 && (next_head.0 - tail.0).abs() == 0 { (tail.0, tail.1 - 1) } 
                        else if (next_head.0 - tail.0).abs() > 1 && (next_head.1 - tail.1).abs() == 1 { (tail.0 + 1, tail.1 - 1) } 
                        else if (next_head.1 - tail.1).abs() > 1 && (next_head.0 - tail.0).abs() == 1 { (tail.0 + 1, tail.1 - 1) } 
                        else { tail };
        }
        "DR" => {
            next_head = (head.0 - 1, head.1 + 1);
            next_tail = if (next_head.0 - tail.0).abs() > 1 &&  (next_head.1 - tail.1).abs() > 1 { (tail.0 - 1, tail.1 + 1 ) } 
                        else if (next_head.0 - tail.0).abs() > 1 && (next_head.1 - tail.1).abs() == 0 { (tail.0 - 1, tail.1) } 
                        else if (next_head.1 - tail.1).abs() > 1 && (next_head.0 - tail.0).abs() == 0 { (tail.0, tail.1 + 1) } 
                        else if (next_head.0 - tail.0).abs() > 1 && (next_head.1 - tail.1).abs() == 1 { (tail.0 - 1, tail.1 + 1) } 
                        else if (next_head.1 - tail.1).abs() > 1 && (next_head.0 - tail.0).abs() == 1 { (tail.0 - 1, tail.1 + 1) } 
                        else { tail };
        }
        "DL" => {
            next_head = (head.0 - 1, head.1 - 1);
            next_tail = if (next_head.0 - tail.0).abs() > 1 &&  (next_head.1 - tail.1).abs() > 1 { (tail.0 - 1, tail.1 - 1 ) } 
                        else if (next_head.0 - tail.0).abs() > 1 && (next_head.1 - tail.1).abs() == 0 { (tail.0 - 1, tail.1) } 
                        else if (next_head.1 - tail.1).abs() > 1 && (next_head.0 - tail.0).abs() == 0 { (tail.0, tail.1 - 1) } 
                        else if (next_head.0 - tail.0).abs() > 1 && (next_head.1 - tail.1).abs() == 1 { (tail.0 - 1, tail.1 - 1) } 
                        else if (next_head.1 - tail.1).abs() > 1 && (next_head.0 - tail.0).abs() == 1 { (tail.0 - 1, tail.1 - 1) } 
                        else { tail };
        }
        _ => panic!()
    }
    (next_head, next_tail, get_next_direction(tail, next_tail))
}

pub fn get_next_direction(current: (i64, i64), next: (i64, i64)) -> String {
    match (next.0 - current.0, next.1 - current.1) {
        (0,0) => String::from(""),
        (1,0) => String::from("U"),
        (-1,0) => String::from("D"),
        (0,1) => String::from("R"),
        (0,-1) => String::from("L"),
        (1,1) => String::from("UR"),
        (1,-1) => String::from("UL"),
        (-1,1) => String::from("DR"),
        (-1,-1) => String::from("DL"),
        _ => panic!()
    }
}

pub fn part1() {
    let input = include_str!("input")
        .lines()
        .map(|line| line.split_whitespace().collect_vec());

    let mut tail_set = HashSet::new();
    let mut head = (0,0);
    let mut tail = (0,0);

    tail_set.insert(tail);
    for line in input {
        let direction = line[0];
        let steps = usize::from_str(line[1]).unwrap();
        for _ in 0..steps {
            (head, tail, _) = apply_moves(head, tail, direction);
            tail_set.insert(tail);
        }
    }
    let result = tail_set.iter().count();
    println!("result: {:?}", result);
}

pub fn part2() {
    let input = include_str!("input")
        .lines()
        .map(|line| line.split_whitespace().collect_vec());

    // Dictionary to save visited values
    let mut tail_set = HashSet::new();
    tail_set.insert((0,0));

    // initial node
    let mut head = (0,0);
    let mut tail = (0,0);

    // keep in a vector the directions followed by each knot
    let mut previous_directions = vec![];

    // move first knot
    for line in input {
        let direction = line[0];
        let steps = usize::from_str(line[1]).unwrap();


        for _ in 0..steps {
            let next_direction: String;
            (head, tail, next_direction) = apply_moves(head, tail, direction);
            previous_directions.push(next_direction);
        }
    }

    // one by one mover the rest of the knots. We keep the movements of the previous knot.
    for k in 1..9 {
        head = (0,0);
        tail = (0,0);
        let mut next_directions = vec![];
        for direction in previous_directions.iter().filter(|x| x.len() > 0) {
            let next_direction: String;
            (head, tail, next_direction) = apply_moves(head, tail, &direction);
            next_directions.push(next_direction);
            if k == 8 {
                tail_set.insert(tail);
            }
        }
        previous_directions = next_directions;
    }
    let result = tail_set.iter().count();
    println!("result: {:?}", result);

}