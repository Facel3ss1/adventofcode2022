use std::collections::HashSet;

fn neighbours(point: (i32, i32, i32)) -> impl Iterator<Item = (i32, i32, i32)> {
    let plus_minus = [1, -1].into_iter();

    plus_minus
        .clone()
        .map(|dx| (dx, 0, 0))
        .chain(plus_minus.clone().map(|dy| (0, dy, 0)))
        .chain(plus_minus.map(|dz| (0, 0, dz)))
        .map(move |(dx, dy, dz)| (point.0 + dx, point.1 + dy, point.2 + dz))
}

fn main() {
    let mut points: HashSet<(i32, i32, i32)> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut components = line.split(',');
            let x = components.next().unwrap();
            let y = components.next().unwrap();
            let z = components.next().unwrap();

            (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
        })
        .collect();

    let total_neighbours = points
        .iter()
        .flat_map(|&point| neighbours(point))
        .filter(|neighbour| points.contains(neighbour))
        .count();

    println!("Part 1: {}", 6 * points.len() - total_neighbours);
}
