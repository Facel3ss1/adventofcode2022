use std::str::FromStr;

#[derive(Debug)]
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
    fn apply_step(&mut self, step: Step) {
        for _ in 0..step.quantity {
            let crate_char = self.stacks[step.from].pop().unwrap();
            self.stacks[step.to].push(crate_char);
        }
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
    let mut crates = crates.parse::<Crates>().unwrap();

    for step in steps.lines() {
        let step = step.parse::<Step>().unwrap();
        crates.apply_step(step);
    }

    println!("{}", crates.top_crates());
}
