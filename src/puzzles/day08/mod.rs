use itertools::Itertools;

pub fn is_visible(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let (width, height) = (grid[0].len(), grid.len());
    if i == 0 || i == height-1 || j == 0 || j == width-1 {
        return true
    }
    else {
            grid[i][0..j].iter().all(|&x| x < grid[i][j]) ||
            grid[i][j+1..].iter().all(|&x| x < grid[i][j]) ||
            grid[0..i].iter().map(|x| x[j]).all(|x| x < grid[i][j]) ||
            grid[i+1..].iter().map(|x| x[j]).all(|x| x < grid[i][j]) 
    }
}

pub fn scenic_score(grid: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    if !is_visible(grid, i, j) {
        return 0
    }
    let (width, height) = (grid[0].len(), grid.len());
    let mut left = grid[i][0..j].iter().rev().take_while(|&&x| x < grid[i][j]).count();
    if left != j { left +=1 }

    let mut right = grid[i][j+1..].iter().take_while(|&&x| x < grid[i][j]).count();
    if right != width - (j + 1) { right += 1 }

    let mut top = grid[0..i].iter().map(|x| x[j]).rev().take_while(|&x| x < grid[i][j]).count();
    if top != i { top += 1 }
    
    let mut bottom = grid[i+1..].iter().map(|x| x[j]).take_while(|&x| x < grid[i][j]).count();
    if bottom != height - (i + 1) { bottom += 1 }
    left * right * top * bottom
}

pub fn part1() {
    let input = include_str!("input.txt")
    .lines()
    .map(|line| line.chars().collect_vec())
    .collect_vec();
    let mut result = 0;
    for (i,j) in itertools::iproduct!(0..input.len(),(0..input[0].len())) {
        if is_visible(&input, i, j) {
            result += 1;
        }
    }
    println!("result: {:?}", result);
}

pub fn part2() {
    let input = include_str!("input.txt")
    .lines()
    .map(|line| line.chars().collect_vec())
    .collect_vec();
    let mut result = 0;
    for (i,j) in itertools::iproduct!(0..input.len(),(0..input[0].len())) {
        let score = scenic_score(&input, i, j);
        if score > result {
            result = score;
        }
    }
    println!("result: {:?}", result);
}