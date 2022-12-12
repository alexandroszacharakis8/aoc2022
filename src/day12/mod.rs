use std::collections::HashMap;

use itertools::{Itertools, iproduct};

pub fn build_graph() -> (HashMap<(usize, usize), Vec<(usize,usize)>>, (usize, usize), (usize, usize)) {
    let mut graph = HashMap::new();

    let input = include_str!("input")
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let (width, height) = (input[0].len(), input.len());
    let (mut start, mut end) = ((0,0), (0,0));
    for (i,j) in iproduct!((0..height), (0..width)) {
        let ii = i as i64;
        let jj = j as i64;
        match input[i][j] {
            'S' => {
                start = (i,j);
                let v: Vec<(usize, usize)> = vec![(ii+1, jj), (ii-1,jj), (ii, jj+1), (ii, jj-1)]
                            .into_iter()
                            .filter(|(x, y)| *x >= 0 && *y >= 0)
                            .map(|(x,y)| (x.abs() as usize, y.abs() as usize))
                            .filter(|(x, y)| *x < height && *y < width)
                            .filter(|(x, y)| {
                                    let c = if input[*x][*y] == 'E' {'z' as i32 - '0' as i32 } else { input[*x][*y] as i32 - '0' as i32 };
                                    c <= 'a' as i32 - '0' as i32 + 1 
                                })
                            .collect_vec();
                graph.insert((i,j), v);
            },
            'E' => {
                end = (i,j);
                graph.insert((i,j), vec![]);
            },
            _ => {
                let v: Vec<(usize, usize)> = vec![(ii+1, jj), (ii-1,jj), (ii, jj+1), (ii, jj-1)]
                            .into_iter()
                            .filter(|(x, y)| *x >= 0 && *y >= 0)
                            .map(|(x,y)| (x.abs() as usize, y.abs() as usize))
                            .filter(|(x, y)| *x < height && *y < width)
                            .filter(|(x, y)| {
                                    let c = if input[*x][*y] == 'E' {'z' as i32 - '0' as i32 } else { input[*x][*y] as i32 - '0' as i32 };
                                    c <= input[i][j] as i32 - '0' as i32 + 1 
                                })
                            .collect_vec();
                graph.insert((i,j), v);
            }
        }
    }
    (graph, start, end)
}

pub fn part1() {
    let (graph, start, end) = build_graph();

    // HashMap with visited nodes and distances
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();

    // queue for unexplored nodes
    let mut q = vec![start];

    // map of explored nodes
    visited.insert(start, 0);

    for i in 0.. {
        // next explored node
        let current = q[i];
        let current_distance = *visited.get(&current).unwrap();
        // neighborhood
        let v = graph.get(&current).unwrap();
        for node in v {
            if *node == end {
                println!("result: {}", current_distance + 1);
                return
            }
            if !visited.contains_key(node) {
                visited.insert(*node, current_distance + 1);
                q.push(*node);
            }
        }
    }
}

pub fn part2() {
    let (graph, _start, end) = build_graph();

    let input = include_str!("input")
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let (width, height) = (input[0].len(), input.len());

    // HashMap with visited nodes and distances
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();

    // queue for unexplored nodes
    // initially put in distance zero all elements of some starting position
    let mut q = iproduct!((0..height), (0..width))
        .filter(|&(i,j)| input[i][j] == 'a' || input[i][j] == 'S')
        .collect_vec();
    // 
    q.iter().for_each(|node| { visited.insert(*node,0); });

    for i in 0.. {
        let current = q[i];
        let current_distance = *visited.get(&current).unwrap();
        // neighborhood
        let v = graph.get(&current).unwrap();
        for node in v {
            if *node == end {
                println!("result: {}", current_distance + 1);
                return
            }
            if !visited.contains_key(node) {
                visited.insert(*node, current_distance + 1);
                q.push(*node);
            }
        }
    }
}