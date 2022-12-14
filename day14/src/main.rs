use std::collections::HashSet;

fn parse_point(input: &str) -> (i32, i32) {
    input
        .split_once(',')
        .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
        .unwrap()
}

fn main() {
    let mut grid: HashSet<(i32, i32)> = HashSet::new();

    for line in include_str!("input.txt").lines() {
        let mut points = line.split(" -> ");
        let mut prev_point = parse_point(points.next().unwrap());

        for current_point in points {
            let current_point = parse_point(current_point);

            if current_point.1 == prev_point.1 {
                for col in prev_point.0..=current_point.0 {
                    grid.insert((col, current_point.1));
                }

                for col in current_point.0..=prev_point.0 {
                    grid.insert((col, current_point.1));
                }
            } else if current_point.0 == prev_point.0 {
                for row in prev_point.1..=current_point.1 {
                    grid.insert((current_point.0, row));
                }

                for row in current_point.1..=prev_point.1 {
                    grid.insert((current_point.0, row));
                }
            }

            prev_point = current_point;
        }
    }

    let height = grid.iter().map(|pos| pos.1).max().unwrap();

    for i in 0..750 {
        grid.insert((i, height + 2));
    }

    let mut num_sand_part2 = 0;
    let mut num_sand_part1 = 0;
    'outer: loop {
        let mut sand = (500, 0);

        loop {
            if sand.1 == height && num_sand_part1 == 0 {
                num_sand_part1 = num_sand_part2;
            } else if !grid.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
            } else if !grid.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand.1 += 1;
                sand.0 -= 1;
            } else if !grid.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand.1 += 1;
                sand.0 += 1;
            } else if !(sand.0 == 500 && sand.1 == 0) {
                grid.insert(sand);
                num_sand_part2 += 1;
                break;
            } else {
                num_sand_part2 += 1;
                break 'outer;
            }
        }
    }

    println!("Part 1: {num_sand_part1}");
    println!("Part 2: {num_sand_part2}");
}
