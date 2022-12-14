fn parse_point(input: &str) -> (usize, usize) {
    input
        .split_once(',')
        .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
        .unwrap()
}

fn main() {
    let mut grid: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];

    for line in include_str!("input.txt").lines() {
        let mut points = line.split(" -> ");
        let mut prev_point = parse_point(points.next().unwrap());

        for current_point in points {
            let current_point = parse_point(current_point);

            if current_point.1 == prev_point.1 {
                for col in prev_point.0..=current_point.0 {
                    grid[current_point.1][col] = true;
                }

                for col in current_point.0..=prev_point.0 {
                    grid[current_point.1][col] = true;
                }
            } else if current_point.0 == prev_point.0 {
                for row in prev_point.1..=current_point.1 {
                    grid[row][current_point.0] = true;
                }

                for row in current_point.1..=prev_point.1 {
                    grid[row][current_point.0] = true;
                }
            }

            prev_point = current_point;
        }
    }

    let mut num_sand = 0;
    'outer: loop {
        let mut sand = (500, 0);

        loop {
            if sand.1 == grid.len() - 1 {
                break 'outer;
            } else if !grid[sand.1 + 1][sand.0] {
                sand.1 += 1;
            } else if !sand
                .0
                .checked_sub(1)
                .map(|col| grid[sand.1 + 1][col])
                .unwrap_or(true)
            {
                sand.1 += 1;
                sand.0 -= 1;
            } else if !(sand.0 + 1 < grid[0].len())
                .then(|| grid[sand.1 + 1][sand.0 + 1])
                .unwrap_or(true)
            {
                sand.1 += 1;
                sand.0 += 1;
            } else {
                grid[sand.1][sand.0] = true;
                num_sand += 1;
                break;
            }
        }
    }

    println!("Part 1: {num_sand}");
}
