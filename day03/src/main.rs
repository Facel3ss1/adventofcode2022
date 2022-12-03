use itertools::Itertools;
use std::collections::HashSet;

fn common_char<'a, G>(group: G) -> u8
where
    G: Iterator<Item = &'a [u8]>,
{
    group
        .map(|items| items.iter().copied().collect::<HashSet<u8>>())
        .reduce(|acc, items| &acc & &items)
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
}

fn sum_groups<'a, I, G>(groups: I) -> u32
where
    G: Iterator<Item = &'a [u8]>,
    I: Iterator<Item = G>,
{
    groups
        .map(|group| {
            let common_char = common_char(group);

            if common_char.is_ascii_lowercase() {
                // a-z -> 1-26
                (common_char - 96) as u32
            } else {
                // A-Z -> 27-52
                (common_char - 38) as u32
            }
        })
        .sum()
}

fn main() {
    let lines = include_str!("input.txt").lines().map(str::as_bytes);

    let part1_groups = lines.clone().map(|line| {
        let (first_half, second_half) = line.split_at(line.len() / 2);
        [first_half, second_half].into_iter()
    });

    let part2_groups = lines.chunks(3);

    println!("Part 1: {}", sum_groups(part1_groups));
    println!("Part 2: {}", sum_groups(part2_groups.into_iter()));
}
