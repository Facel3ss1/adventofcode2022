const GRID_SIZE: usize = 99;

fn main() {
    let mut grid: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];

    for (row, line) in include_str!("input.txt").lines().enumerate() {
        for (col, byte) in line.as_bytes().iter().copied().enumerate() {
            grid[row][col] = byte - b'0';
        }
    }

    let mut visible: [[bool; GRID_SIZE]; GRID_SIZE] = [[false; GRID_SIZE]; GRID_SIZE];

    for current_num in (0..=9).rev() {
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if grid[row][col] >= current_num {
                    visible[row][col] = true;
                    break;
                }
            }

            for col in (0..GRID_SIZE).rev() {
                if grid[row][col] >= current_num {
                    visible[row][col] = true;
                    break;
                }
            }
        }

        for col in 0..GRID_SIZE {
            for row in 0..GRID_SIZE {
                if grid[row][col] >= current_num {
                    visible[row][col] = true;
                    break;
                }
            }

            for row in (0..GRID_SIZE).rev() {
                if grid[row][col] >= current_num {
                    visible[row][col] = true;
                    break;
                }
            }
        }
    }

    let mut sum = 0;
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            if visible[row][col] {
                sum += 1;
                print!("{}", grid[row][col]);
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!("Part 1: {sum}");
}
