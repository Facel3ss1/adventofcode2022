use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn neighbours(
    position: (usize, usize),
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let left = position.1.checked_sub(1).map(|col| (position.0, col));
    let right = (position.1 + 1 < width).then_some((position.0, position.1 + 1));
    let up = position.0.checked_sub(1).map(|row| (row, position.1));
    let down = (position.0 + 1 < height).then_some((position.0 + 1, position.1));

    [left, right, up, down].into_iter().flatten()
}

fn manhattan_distance(position: (usize, usize), end_position: (usize, usize)) -> usize {
    position.0.abs_diff(end_position.0) + position.1.abs_diff(end_position.1)
}

fn a_star(
    grid: &Vec<Vec<u8>>,
    start_position: (usize, usize),
    end_position: (usize, usize),
) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    let mut distances = vec![vec![usize::MAX; width]; height];
    distances[start_position.0][start_position.1] = 0;

    let mut open_set = BinaryHeap::new();
    open_set.push(Reverse((0, start_position)));

    while let Some(Reverse((_, current_position))) = open_set.pop() {
        if current_position == end_position {
            break;
        }

        neighbours(current_position, width, height)
            .filter(|position| {
                grid[current_position.0][current_position.1] + 1 >= grid[position.0][position.1]
            })
            .for_each(|position| {
                let distance = distances[current_position.0][current_position.1].saturating_add(1);
                if distance < distances[position.0][position.1] {
                    distances[position.0][position.1] = distance;

                    if !open_set
                        .iter()
                        .any(|Reverse((_, other))| position == *other)
                    {
                        open_set.push(Reverse((
                            distance.saturating_add(manhattan_distance(position, end_position)),
                            position,
                        )));
                    }
                }
            });
    }

    distances[end_position.0][end_position.1]
}

fn main() {
    let mut grid: Vec<Vec<u8>> = Default::default();
    let mut start_position: Option<(usize, usize)> = None;
    let mut end_position: Option<(usize, usize)> = None;

    for (row, line) in include_str!("input.txt").lines().enumerate() {
        let bytes = line.as_bytes();

        start_position = start_position.or_else(|| {
            bytes
                .iter()
                .enumerate()
                .find(|&(_, &c)| c == b'S')
                .map(|(col, _)| (row, col))
        });

        end_position = end_position.or_else(|| {
            bytes
                .iter()
                .enumerate()
                .find(|&(_, &c)| c == b'E')
                .map(|(col, _)| (row, col))
        });

        grid.push(
            bytes
                .iter()
                .map(|c| match c {
                    b'S' => 0,
                    b'E' => 25,
                    _ => c - b'a',
                })
                .collect(),
        );
    }

    let start_position = start_position.unwrap();
    let end_position = end_position.unwrap();

    let distance = a_star(&grid, start_position, end_position);

    println!("Part 1: {distance}");
}
