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

    fn does_beat(self, other: Shape) -> bool {
        self.wins_against() == other
    }

    fn score(self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

struct Round {
    me: Shape,
    opponent: Shape,
}

impl Round {
    fn parse_part1(line: &str) -> Self {
        let mut letters = line.split_whitespace();

        let opponent = match letters.next().unwrap() {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Unrecognised letter"),
        };

        let me = match letters.next().unwrap() {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => panic!("Unrecognised letter"),
        };

        Self { me, opponent }
    }

    fn parse_part2(line: &str) -> Self {
        let mut letters = line.split_whitespace();

        let opponent = match letters.next().unwrap() {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Unrecognised letter"),
        };

        let me = match letters.next().unwrap() {
            "X" => opponent.wins_against(),
            "Y" => opponent,
            "Z" => opponent.loses_against(),
            _ => panic!("Unrecognised letter"),
        };

        Self { me, opponent }
    }
}

fn score(parser: impl Fn(&str) -> Round) -> u32 {
    let lines = include_str!("input.txt").lines();
    let mut score = 0;

    for line in lines {
        let round = parser(line);
        score += round.me.score();

        if round.me.does_beat(round.opponent) {
            score += 6;
        } else if round.me == round.opponent {
            score += 3;
        }
    }

    score
}

fn main() {
    println!("Part 1: {}", score(Round::parse_part1));
    println!("Part 2: {}", score(Round::parse_part2));
}
