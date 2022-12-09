use std::collections::HashSet;

fn main() {
    let lines = include_str!("input.txt").lines();

    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut head_position = (0, 0);
    let mut tail_position = (0, 0);
    visited_positions.insert(tail_position);

    for line in lines {
        let (direction, steps) = line.split_once(' ').unwrap();
        let steps = steps.parse().unwrap();

        match direction {
            "R" => head_position.0 += steps,
            "L" => head_position.0 -= steps,
            "U" => head_position.1 += steps,
            "D" => head_position.1 -= steps,
            _ => panic!("Unsupported direction"),
        }

        for _ in 0..steps {
            let vector = (
                head_position.0 - tail_position.0,
                head_position.1 - tail_position.1,
            );

            if vector.0.abs() > 1 || vector.1.abs() > 1 {
                tail_position.0 += vector.0.signum();
                tail_position.1 += vector.1.signum();
            }

            visited_positions.insert(tail_position);
        }
    }

    println!("Part 1: {}", visited_positions.len());
}
