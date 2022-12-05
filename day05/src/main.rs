use std::str::FromStr;

#[derive(Debug, Clone)]
struct Crates {
    stacks: [Vec<char>; 9],
}

impl FromStr for Crates {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.lines().map(str::as_bytes).rev().skip(1);
        let mut stacks: [Vec<char>; 9] = Default::default();

        for line in lines {
            (0..9).for_each(|i| {
                // 0-9 -> 1,5,9...
                let char_index = (i * 4) + 1;
                let crate_char = line[char_index];
                if crate_char != b' ' {
                    stacks[i].push(crate_char as char);
                }
            });
        }

        Ok(Self { stacks })
    }
}

impl Crates {
    fn apply_step_part1(&mut self, step: &Step) {
        for _ in 0..step.quantity {
            let crate_char = self.stacks[step.from].pop().unwrap();
            self.stacks[step.to].push(crate_char);
        }
    }

    fn apply_step_part2(&mut self, step: &Step) {
        let from_stack = &mut self.stacks[step.from];
        let crates = from_stack.split_off(from_stack.len() - step.quantity);
        self.stacks[step.to].extend(crates.into_iter());
    }

    fn top_crates(&self) -> String {
        (0..9).map(|i| self.stacks[i].last().unwrap()).collect()
    }
}

struct Step {
    quantity: usize,
    from: usize,
    to: usize,
}

impl FromStr for Step {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.strip_prefix("move ").unwrap();
        let (quantity, input) = input.split_once(" from ").unwrap();
        let (from, to) = input.split_once(" to ").unwrap();

        let quantity = quantity.parse::<usize>().unwrap();
        let from = from.parse::<usize>().unwrap() - 1;
        let to = to.parse::<usize>().unwrap() - 1;

        Ok(Self { quantity, from, to })
    }
}

fn main() {
    let (crates, steps) = include_str!("input.txt").split_once("\n\n").unwrap();
    let mut crates_part1 = crates.parse::<Crates>().unwrap();
    let mut crates_part2 = crates_part1.clone();

    for step in steps.lines() {
        let step = step.parse::<Step>().unwrap();
        crates_part1.apply_step_part1(&step);
        crates_part2.apply_step_part2(&step);
    }

    println!("Part 1: {}", crates_part1.top_crates());
    println!("Part 2: {}", crates_part2.top_crates());
}
