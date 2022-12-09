const GRID_SIZE: usize = 99;

fn main() {
    let mut grid: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];

    for (row, line) in include_str!("input.txt").lines().enumerate() {
        for (col, byte) in line.as_bytes().iter().copied().enumerate() {
            grid[row][col] = byte - b'0';
        }
    }

    // 0000UDLR - Which bits are set says which directions the tree is visible from
    let mut visibilities: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];

    for current_num in (0..=9).rev() {
        for row in 0..GRID_SIZE {
            // Right
            for col in 0..GRID_SIZE {
                if grid[row][col] >= current_num {
                    visibilities[row][col] |= 0b0001;
                    break;
                }
            }

            // Left
            for col in (0..GRID_SIZE).rev() {
                if grid[row][col] >= current_num {
                    visibilities[row][col] |= 0b0010;
                    break;
                }
            }
        }

        for col in 0..GRID_SIZE {
            // Down
            for row in 0..GRID_SIZE {
                if grid[row][col] >= current_num {
                    visibilities[row][col] |= 0b0100;
                    break;
                }
            }

            // Up
            for row in (0..GRID_SIZE).rev() {
                if grid[row][col] >= current_num {
                    visibilities[row][col] |= 0b1000;
                    break;
                }
            }
        }
    }

    let mut max_scenic_score = 1;
    for tree_row in 1..GRID_SIZE - 1 {
        for tree_col in 1..GRID_SIZE - 1 {
            let visibility = visibilities[tree_row][tree_col];
            if visibility == 0 {
                continue;
            }

            let tree_height = grid[tree_row][tree_col];
            let mut scenic_score = 1;

            // Right
            let mut viewing_distance = 1;
            if visibility & 0b0001 > 0 {
                viewing_distance = tree_col;
            } else {
                for col in (0..tree_col).rev() {
                    if grid[tree_row][col] >= tree_height {
                        break;
                    }
                    viewing_distance += 1;
                }
            }
            scenic_score *= viewing_distance;

            // Left
            let mut viewing_distance = 1;
            if visibility & 0b0010 > 0 {
                viewing_distance = (GRID_SIZE - tree_col) - 1;
            } else {
                for col in tree_col + 1..GRID_SIZE {
                    if grid[tree_row][col] >= tree_height {
                        break;
                    }
                    viewing_distance += 1;
                }
            }
            scenic_score *= viewing_distance;

            // Down
            let mut viewing_distance = 1;
            if visibility & 0b0100 > 0 {
                viewing_distance = tree_row;
            } else {
                for row in (0..tree_row).rev() {
                    if grid[row][tree_col] >= tree_height {
                        break;
                    }
                    viewing_distance += 1;
                }
            }
            scenic_score *= viewing_distance;

            // Up
            let mut viewing_distance = 1;
            if visibility & 0b1000 > 0 {
                viewing_distance = (GRID_SIZE - tree_row) - 1;
            } else {
                for row in tree_row + 1..GRID_SIZE {
                    if grid[row][tree_col] >= tree_height {
                        break;
                    }
                    viewing_distance += 1;
                }
            }
            scenic_score *= viewing_distance;

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    let mut num_visible = 0;
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            if visibilities[row][col] > 0 {
                num_visible += 1;
                print!("{}", grid[row][col]);
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();

    println!("Part 1: {num_visible}");
    println!("Part 2: {max_scenic_score}");
}
