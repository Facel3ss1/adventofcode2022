mod parse;

#[derive(Debug)]
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
    fn do_turn(&mut self) -> impl Iterator<Item = (usize, u64)> + '_ {
        self.items.drain(..).map(|mut item| {
            let rhs = match self.rhs {
                Rhs::Old => item,
                Rhs::Const(num) => num,
            };

            match self.op {
                Op::Add => item += rhs,
                Op::Times => item *= rhs,
            }

            item /= 3;

            let throw_to = if item % self.divisible_by == 0 {
                self.throw_if_true
            } else {
                self.throw_if_false
            };

            self.num_inspected_items += 1;

            (throw_to, item)
        })
    }
}

#[derive(Debug)]
enum Op {
    Add,
    Times,
}

#[derive(Debug)]
enum Rhs {
    Old,
    Const(u64),
}

fn main() {
    let input = include_str!("input.txt").split("\n\n");
    let mut monkeys: Vec<Monkey> = input.map(|monkey| monkey.parse().unwrap()).collect();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let thrown_items: Vec<(usize, u64)> = monkey.do_turn().collect();
            for (throw_to, item) in thrown_items {
                monkeys[throw_to].items.push(item);
            }
        }
    }

    monkeys.sort_unstable_by_key(|monkey| monkey.num_inspected_items);
    monkeys.reverse();
    let monkey_business = monkeys[0].num_inspected_items * monkeys[1].num_inspected_items;

    println!("Part 1: {monkey_business}");
}
