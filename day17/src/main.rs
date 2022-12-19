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

fn check_collision(shape: usize, position: (usize, usize), rows: &[[bool; 7]]) -> bool {
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

fn print_board(shape: usize, position: (usize, usize), rows: &[[bool; 7]]) {
    for (row, cells) in rows.iter().enumerate().rev().take(30) {
        for (col, b) in cells.iter().copied().enumerate() {
            if b || position
                .0
                .checked_sub(row)
                .zip(col.checked_sub(position.1))
                .and_then(|(row, col)| SHAPES[shape].get(row).and_then(|cells| cells.get(col)))
                .copied()
                .filter(|&b| b)
                .is_some()
            {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
    std::io::stdin().read_line(&mut String::new());
}

fn simulate(total_shapes: usize) -> usize {
    let mut directions = include_str!("input.txt")
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '>' => Direction::Right,
                '<' => Direction::Left,
                _ => panic!("Not a direction"),
            })
        })
        .cycle();

    let mut rows: Vec<[bool; 7]> = vec![[false; 7]; 7];
    let mut num_shapes = 0;

    for shape in (0..5).cycle() {
        let mut position: (usize, usize) = (rows.len() - 1, 2);

        for direction in directions.by_ref() {
            // print_board(shape, position, &rows);

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

            // print_board(shape, position, &rows);

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

        let mut highest_row = 0;
        'outer: for row in (0..rows.len()).rev() {
            if rows[row].iter().any(|&b| b) {
                highest_row = row + 1;
                break 'outer;
            }
        }

        rows.extend(std::iter::repeat([false; 7]).take((highest_row + 7) - rows.len()));

        num_shapes += 1;

        if num_shapes == total_shapes {
            break;
        }
    }

    let mut highest_row = 0;
    'outer: for row in (0..rows.len()).rev() {
        if rows[row].iter().any(|&b| b) {
            highest_row = row + 1;
            break 'outer;
        }
    }

    highest_row
}

fn main() {
    println!("Part 1: {}", simulate(2022));
}
