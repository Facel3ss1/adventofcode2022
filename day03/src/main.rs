use std::collections::HashSet;

fn main() {
    let lines = include_str!("input.txt").lines();

    let mut sum = 0;
    for line in lines {
        let chars = line.bytes();
        let first_half: HashSet<u8> = chars.clone().take(line.len() / 2).collect();
        let second_half: HashSet<u8> = chars.skip(line.len() / 2).collect();

        let common_char = *first_half.intersection(&second_half).next().unwrap();

        if common_char.is_ascii_lowercase() {
            sum += (common_char - 96) as u32;
        } else {
            sum += (common_char - 38) as u32;
        }
    }

    println!("Part 1: {sum}");
}
