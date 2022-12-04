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

fn range_overlaps(first: RangeInclusive<u32>, second: RangeInclusive<u32>) -> bool {
    let first_overlaps_second = second.contains(first.start()) || second.contains(first.end());
    let second_overlaps_first = first.contains(second.start()) || first.contains(second.end());

    first_overlaps_second || second_overlaps_first
}

fn main() {
    let ranges = include_str!("input.txt").lines().map(|line| {
        line.split_once(',')
            .map(|(first, second)| (parse_range(first), parse_range(second)))
            .unwrap()
    });

    let part1_count = ranges
        .clone()
        .filter(|(first, second)| range_contains(first.clone(), second.clone()))
        .count();

    let part2_count = ranges
        .filter(|(first, second)| range_overlaps(first.clone(), second.clone()))
        .count();

    println!("Part 1: {part1_count}");
    println!("Part 2: {part2_count}");
}
