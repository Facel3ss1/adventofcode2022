use std::collections::{HashSet, VecDeque};

fn neighbours(point: (i32, i32, i32)) -> impl Iterator<Item = (i32, i32, i32)> {
    let plus_minus = [1, -1].into_iter();

    plus_minus
        .clone()
        .map(|dx| (dx, 0, 0))
        .chain(plus_minus.clone().map(|dy| (0, dy, 0)))
        .chain(plus_minus.map(|dz| (0, 0, dz)))
        .map(move |(dx, dy, dz)| (point.0 + dx, point.1 + dy, point.2 + dz))
}

fn surface_area(points: &HashSet<(i32, i32, i32)>) -> usize {
    let total_neighbours = points
        .iter()
        .flat_map(|&point| neighbours(point))
        .filter(|neighbour| points.contains(neighbour))
        .count();

    6 * points.len() - total_neighbours
}

fn main() {
    let points: HashSet<(i32, i32, i32)> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut components = line.split(',');
            let x = components.next().unwrap();
            let y = components.next().unwrap();
            let z = components.next().unwrap();

            (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
        })
        .collect();

    let droplet_surface_area = surface_area(&points);
    println!("Part 1: {}", droplet_surface_area);

    let bounds = points.iter().fold((0, 0, 0), |mut acc, &point| {
        if point.0 > acc.0 {
            acc.0 = point.0;
        }
        if point.1 > acc.1 {
            acc.1 = point.1;
        }
        if point.2 > acc.2 {
            acc.2 = point.2;
        }

        acc
    });

    let mut queue: VecDeque<(i32, i32, i32)> = (1..=bounds.0)
        .flat_map(|x| (1..=bounds.1).flat_map(move |y| (1..=bounds.2).map(move |z| (x, y, z))))
        .filter(|&(x, y, z)| {
            x == 1 || y == 1 || z == 1 || x == bounds.0 || y == bounds.1 || z == bounds.2
        })
        .filter(|point| !points.contains(point))
        .collect();
    let mut exterior_points: HashSet<(i32, i32, i32)> = HashSet::new();

    while let Some(point) = queue.pop_front() {
        exterior_points.insert(point);
        queue.extend(
            neighbours(point)
                .filter(|&(x, y, z)| {
                    x >= 1 && y >= 1 && z >= 1 && x <= bounds.0 && y <= bounds.1 && z <= bounds.2
                })
                .filter(|neighbour| !exterior_points.contains(neighbour))
                .filter(|neighbour| !points.contains(neighbour)),
        );
    }

    let air_pockets: HashSet<(i32, i32, i32)> = (1..=bounds.0)
        .flat_map(|x| (1..=bounds.1).flat_map(move |y| (1..=bounds.2).map(move |z| (x, y, z))))
        .filter(|point| !exterior_points.contains(point))
        .filter(|point| !points.contains(point))
        .collect();

    println!(
        "Part 2: {}",
        droplet_surface_area - surface_area(&air_pockets)
    );
}
