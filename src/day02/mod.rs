pub fn score(game: &str) -> u64 {
    let other = game.chars().nth(0).unwrap();
    let selection = game.chars().nth(2).unwrap();
    let mut score = 0;
    match selection {
        'X' =>  score += 1 +
                match other {
                    'C' =>  6,
                    'A' =>  3,
                    'B' =>  0,
                    _ => panic!(),
                },
        'Y' => score += 2 +  
                match other {
                    'A' => 6,
                    'B' => 3,
                    'C' => 0,
                    _ => panic!(),
                },
        'Z' => score += 3 + 
                match other {
                    'B' => 6,
                    'C' => 3,
                    'A' => 0,
                    _ => panic!(),
                },
        _ => panic!() 
    }
    score
}

pub fn transform(game: &str) -> String {
    let other = game.chars().nth(0).unwrap();
    let outcome = game.chars().nth(2).unwrap();
    match other {
        'A' =>  match outcome {
                    'X' =>  String::from("A Z"),
                    'Y' =>  String::from("A X"),
                    'Z' =>  String::from("A Y"),
                    _ => panic!(),
                },
        'B' =>   match outcome {
                    'X' =>  String::from("B X"),
                    'Y' =>  String::from("B Y"),
                    'Z' =>  String::from("B Z"),
                    _ => panic!(),
                },
        'C' =>  match outcome {
                    'X' =>  String::from("C Y"),
                    'Y' =>  String::from("C Z"),
                    'Z' =>  String::from("C X"),
                    _ => panic!(),
                },
        _ => panic!() 
    }
}

pub fn part1() {
    let input = include_str!("input.txt");
    let result = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| score(l))
        .sum::<u64>();
    println!("result: {:?}", result);
}

pub fn part2() {
    let input = include_str!("input.txt");
    let result = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| score(&transform(l)))
        .sum::<u64>();
    println!("result: {:?}", result);
}
