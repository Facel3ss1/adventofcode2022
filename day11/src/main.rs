mod parse;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    op: Op,
    rhs: Rhs,
    divisible_by: u64,
    throw_if_true: usize,
    throw_if_false: usize,
    num_inspected_items: usize,
}

impl Monkey {
    fn do_turn(&mut self, lcm: Option<u64>) -> Vec<(usize, u64)> {
        let mut thrown_items = Vec::new();

        for mut item in self.items.drain(..) {
            let rhs = match self.rhs {
                Rhs::Old => item,
                Rhs::Const(num) => num,
            };

            match self.op {
                Op::Add => item += rhs,
                Op::Times => item *= rhs,
            }

            if let Some(lcm) = lcm {
                // Addition and multiplication work properly in modular arithmetic
                item %= lcm;
            } else {
                item /= 3;
            }

            let throw_to = if item % self.divisible_by == 0 {
                self.throw_if_true
            } else {
                self.throw_if_false
            };

            self.num_inspected_items += 1;

            thrown_items.push((throw_to, item));
        }

        thrown_items
    }
}

#[derive(Debug, Clone)]
enum Op {
    Add,
    Times,
}

#[derive(Debug, Clone)]
enum Rhs {
    Old,
    Const(u64),
}

fn calc_monkey_business(mut monkeys: Vec<Monkey>) -> usize {
    monkeys.sort_unstable_by_key(|monkey| monkey.num_inspected_items);
    monkeys.reverse();
    monkeys[0].num_inspected_items * monkeys[1].num_inspected_items
}

fn main() {
    let input = include_str!("input.txt").split("\n\n");
    let mut monkeys_part1: Vec<Monkey> = input.map(|monkey| monkey.parse().unwrap()).collect();
    let mut monkeys_part2 = monkeys_part1.clone();

    for _ in 0..20 {
        for i in 0..monkeys_part1.len() {
            let monkey = &mut monkeys_part1[i];
            let thrown_items = monkey.do_turn(None);
            for (throw_to, item) in thrown_items {
                monkeys_part1[throw_to].items.push(item);
            }
        }
    }

    // All the divisors are different prime numbers, so the LCM is just the product of them
    let lcm = monkeys_part2
        .iter()
        .map(|monkey| monkey.divisible_by)
        .product();

    for _ in 0..10_000 {
        for i in 0..monkeys_part2.len() {
            let monkey = &mut monkeys_part2[i];
            let thrown_items = monkey.do_turn(Some(lcm));
            for (throw_to, item) in thrown_items {
                monkeys_part2[throw_to].items.push(item);
            }
        }
    }

    println!("Part 1: {}", calc_monkey_business(monkeys_part1));
    println!("Part 2: {}", calc_monkey_business(monkeys_part2));
}
