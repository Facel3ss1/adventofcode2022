fn main() {
    let lines: Vec<Option<u32>> = include_str!("input.txt")
        .lines()
        .map(|l| l.parse().ok())
        .collect();

    let mut calorie_counts: Vec<u32> = lines
        .split(Option::is_none)
        .map(|ns| ns.iter().flatten().sum())
        .collect();

    calorie_counts.sort_unstable();
    calorie_counts.reverse();

    println!("Part 1: {}", calorie_counts[0]);
    println!(
        "Part 2: {}",
        calorie_counts.into_iter().take(3).sum::<u32>()
    );
}
