use std::collections::HashSet;

fn solve(input: &str, rope_length: usize) -> usize {
    let lines = input.lines();

    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    visited_positions.insert((0, 0));
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); rope_length];

    for line in lines {
        let (direction, steps) = line.split_once(' ').unwrap();
        let steps = steps.parse().unwrap();

        let head_position = rope.last_mut().unwrap();
        match direction {
            "R" => head_position.0 += steps,
            "L" => head_position.0 -= steps,
            "U" => head_position.1 += steps,
            "D" => head_position.1 -= steps,
            _ => panic!("Unsupported direction"),
        }

        for _ in 0..steps {
            for i in (0..rope.len()).rev().skip(1) {
                let prev_position = rope[i + 1];
                let knot_position = &mut rope[i];

                let vector = (
                    prev_position.0 - knot_position.0,
                    prev_position.1 - knot_position.1,
                );

                if vector.0.abs() > 1 || vector.1.abs() > 1 {
                    knot_position.0 += vector.0.signum();
                    knot_position.1 += vector.1.signum();
                }
            }

            visited_positions.insert(rope[0]);
        }
    }

    visited_positions.len()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve(input, 2));
    println!("Part 2: {}", solve(input, 10));
}
