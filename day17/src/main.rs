use std::collections::{HashMap, VecDeque};

const SHAPES: [[[bool; 4]; 4]; 5] = [
    [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
        [true, true, true, true],
    ],
    [
        [false, false, false, false],
        [false, true, false, false],
        [true, true, true, false],
        [false, true, false, false],
    ],
    [
        [false, false, false, false],
        [false, false, true, false],
        [false, false, true, false],
        [true, true, true, false],
    ],
    [
        [true, false, false, false],
        [true, false, false, false],
        [true, false, false, false],
        [true, false, false, false],
    ],
    [
        [false, false, false, false],
        [false, false, false, false],
        [true, true, false, false],
        [true, true, false, false],
    ],
];

enum Direction {
    Left,
    Right,
}

fn check_collision(shape: usize, position: (usize, usize), rows: &VecDeque<[bool; 7]>) -> bool {
    let mut rightmost_col = 0;
    'outer: for col in (0..4).rev() {
        for row in 0..4 {
            if SHAPES[shape][row][col] {
                rightmost_col = col;
                break 'outer;
            }
        }
    }

    let mut did_collide = false;
    'outer: for row in (0..4).rev() {
        for col in 0..=rightmost_col {
            if position
                .0
                .checked_sub(row)
                .and_then(|row_offset| rows[row_offset].get(position.1 + col))
                .copied()
                .filter(|&b| !(b && SHAPES[shape][row][col]))
                .is_none()
            {
                did_collide = true;
                break 'outer;
            }
        }
    }

    did_collide
}

fn highest_row(rows: &VecDeque<[bool; 7]>) -> usize {
    let mut highest_row = 0;
    'outer: for row in (0..rows.len()).rev() {
        if rows[row].iter().any(|&b| b) {
            highest_row = row + 1;
            break 'outer;
        }
    }

    highest_row
}

fn simulate<I>(directions: I, total_shapes: usize) -> usize
where
    I: Iterator<Item = Direction> + Clone,
{
    let mut directions = directions.enumerate().cycle().peekable();

    let mut rows: VecDeque<[bool; 7]> = VecDeque::from([[false; 7]; 7]);
    let mut num_shapes = 0;
    let mut tower_height = 0;

    let mut shape_history: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    for shape in (0..SHAPES.len()).cycle() {
        let mut position: (usize, usize) = (rows.len() - 1, 2);

        let old_highest_row = highest_row(&rows);

        for (_, direction) in directions.by_ref() {
            match direction {
                Direction::Left => {
                    if let Some(new_col) = position.1.checked_sub(1) {
                        if !check_collision(shape, (position.0, new_col), &rows) {
                            position.1 = new_col;
                        }
                    }
                }
                Direction::Right => {
                    if !check_collision(shape, (position.0, position.1 + 1), &rows) {
                        position.1 += 1;
                    }
                }
            }

            if check_collision(shape, (position.0 - 1, position.1), &rows) {
                break;
            }

            position.0 -= 1;
        }

        for row in 0..4 {
            for col in 0..4 {
                if position.1 + col < 7 {
                    rows[position.0 - row][position.1 + col] |= SHAPES[shape][row][col];
                }
            }
        }

        let new_highest_row = highest_row(&rows);
        rows.extend(std::iter::repeat([false; 7]).take((new_highest_row + 7) - rows.len()));
        while rows.len() > 1000 {
            rows.pop_front();
        }

        num_shapes += 1;
        tower_height += new_highest_row - old_highest_row;

        // Give it some time to settle into a repeating pattern
        if num_shapes > 10000 {
            if let Some((prev_num_shapes, prev_tower_height)) = shape_history.insert(
                (shape, directions.peek().unwrap().0),
                (num_shapes, tower_height),
            ) {
                let period = num_shapes - prev_num_shapes;
                let shapes_left = total_shapes - num_shapes;
                let num_periods = shapes_left / period;
                num_shapes += num_periods * period;

                let height_difference = tower_height - prev_tower_height;
                tower_height += num_periods * height_difference;

                shape_history.clear();
            }
        }

        if num_shapes == total_shapes {
            break;
        }
    }

    tower_height
}

fn main() {
    let directions = include_str!("example_input.txt").lines().flat_map(|line| {
        line.chars().map(|c| match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!("Not a direction"),
        })
    });

    println!("Part 1: {}", simulate(directions.clone(), 2022));
    println!("Part 2: {}", simulate(directions, 1_000_000_000_000));
}
