#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn wins_against(self) -> Self {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn loses_against(self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn score(self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

fn score(is_part1: bool) -> u32 {
    let lines = include_str!("input.txt").lines();
    let mut score = 0;

    for line in lines {
        let mut letters = line.split_whitespace();

        let opponent = match letters.next().unwrap() {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Unrecognised letter"),
        };

        let me = if is_part1 {
            match letters.next().unwrap() {
                "X" => Shape::Rock,
                "Y" => Shape::Paper,
                "Z" => Shape::Scissors,
                _ => panic!("Unrecognised letter"),
            }
        } else {
            match letters.next().unwrap() {
                "X" => opponent.wins_against(),
                "Y" => opponent,
                "Z" => opponent.loses_against(),
                _ => panic!("Unrecognised letter"),
            }
        };

        score += me.score();
        if me.wins_against() == opponent {
            score += 6;
        } else if me == opponent {
            score += 3;
        }
    }

    score
}

fn main() {
    println!("Part 1: {}", score(true));
    println!("Part 2: {}", score(false));
}
