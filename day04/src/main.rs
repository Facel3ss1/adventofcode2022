use std::ops::RangeInclusive;

fn parse_range(input: &str) -> RangeInclusive<u32> {
    let (start, end) = input
        .split_once('-')
        .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
        .unwrap();

    start..=end
}

fn range_contains(first: RangeInclusive<u32>, second: RangeInclusive<u32>) -> bool {
    let first_contains_second = first.start() <= second.start() && first.end() >= second.end();
    let second_contains_first = second.start() <= first.start() && second.end() >= first.end();

    first_contains_second || second_contains_first
}

fn main() {
    let lines = include_str!("input.txt").lines();

    let count = lines
        .map(|line| {
            line.split_once(',')
                .map(|(first, second)| (parse_range(first), parse_range(second)))
                .unwrap()
        })
        .filter(|(first, second)| range_contains(first.clone(), second.clone()))
        .count();

    println!("{count}");
}
