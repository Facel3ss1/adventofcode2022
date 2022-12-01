fn main() {
    let lines = include_str!("input.txt")
        .lines()
        .map(|l| l.parse::<u32>().ok());

    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    let mut calorie_total = 0;

    for line in lines {
        match line {
            Some(calorie_count) => calorie_total += calorie_count,
            None => {
                if calorie_total > first {
                    third = second;
                    second = first;
                    first = calorie_total;
                } else if calorie_total > second {
                    third = second;
                    second = calorie_total;
                } else if calorie_total > third {
                    third = calorie_total;
                }

                calorie_total = 0;
            }
        }
    }

    println!("Part 1: {}", first);
    println!("Part 2: {}", first + second + third);
}
