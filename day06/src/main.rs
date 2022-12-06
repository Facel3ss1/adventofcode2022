use std::collections::VecDeque;

fn find_marker(input: &str, window_size: usize) -> Option<usize> {
    // Invariant: all of the characters must be unique
    let mut window: VecDeque<char> = VecDeque::new();

    for (i, c) in input.char_indices() {
        // Iterate from the back to the front of the queue and try to find a duplicate character
        let found_duplicate = window
            .iter()
            .copied()
            .rev()
            .enumerate()
            .find(|&(_, other)| c == other)
            .map(|(duplicate_index, _)| duplicate_index);

        window.push_front(c);

        // If we find a duplicate, remove it and the characters before it
        if let Some(duplicate_index) = found_duplicate {
            for _ in 0..=duplicate_index {
                window.pop_back();
            }

            continue;
        }

        // If all the window's characters are unique, and the window is the correct size
        if window.len() == window_size {
            return Some(i);
        }
    }

    None
}

fn main() {
    let input = include_str!("input.txt").strip_suffix('\n').unwrap();

    println!("Part 1: {}", find_marker(input, 4).unwrap() + 1);
    println!("Part 2: {}", find_marker(input, 14).unwrap() + 1);
}
