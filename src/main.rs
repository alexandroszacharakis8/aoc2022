mod puzzles;

fn main() {
    let solve_all = true;
    if solve_all {
        solve();
    }
    else {
        puzzles::day14::part1();
        puzzles::day14::part2();
    }
}

fn solve() {
    println!("Day 1:");
    puzzles::day01::part1();
    puzzles::day01::part2();
    println!();
    println!("Day 2:");
    puzzles::day02::part1();
    puzzles::day02::part2();
    println!();
    println!("Day 3:");
    puzzles::day03::part1();
    puzzles::day03::part2();
    println!();
    println!("Day 4:");
    puzzles::day04::part1();
    puzzles::day04::part2();
    println!();
    println!("Day 5:");
    puzzles::day05::part1();
    puzzles::day05::part2();
    println!();
    println!("Day 6:");
    puzzles::day06::part1();
    puzzles::day06::part2();
    println!();
    println!("Day 7:");
    puzzles::day07::part1();
    puzzles::day07::part2();
    println!();
    println!("Day 8:");
    puzzles::day08::part1();
    puzzles::day08::part2();
    println!();
    println!("Day 9:");
    puzzles::day09::part1();
    puzzles::day09::part2();
    println!();
    println!("Day 10:");
    puzzles::day10::part1();
    puzzles::day10::part2();
    println!();
    println!("Day 11:");
    puzzles::day11::part1();
    puzzles::day11::part2();
    println!();
    println!("Day 12:");
    puzzles::day12::part1();
    puzzles::day12::part2();
    println!();
    println!("Day 13:");
    puzzles::day13::part1();
    puzzles::day13::part2();
    println!();
    println!("Day 14:");
    puzzles::day14::part1();
    puzzles::day14::part2();
    println!();
}
