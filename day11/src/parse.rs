use std::str::FromStr;

use crate::{Monkey, Op, Rhs};

impl FromStr for Monkey {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines().skip(1).map(|line| line.trim_start());

        let items = lines.next().unwrap();
        let operation = lines.next().unwrap();
        let divisible_by = lines.next().unwrap();
        let throw_if_true = lines.next().unwrap();
        let throw_if_false = lines.next().unwrap();

        let items = items
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();
        let (op, rhs) = operation
            .strip_prefix("Operation: new = old ")
            .unwrap()
            .split_once(' ')
            .map(|(op, rhs)| (op.parse().unwrap(), rhs.parse().unwrap()))
            .unwrap();
        let divisible_by = divisible_by
            .strip_prefix("Test: divisible by ")
            .map(|num| num.parse().unwrap())
            .unwrap();
        let throw_if_true = throw_if_true
            .strip_prefix("If true: throw to monkey ")
            .map(|num| num.parse().unwrap())
            .unwrap();
        let throw_if_false = throw_if_false
            .strip_prefix("If false: throw to monkey ")
            .map(|num| num.parse().unwrap())
            .unwrap();

        Ok(Monkey {
            items,
            op,
            rhs,
            divisible_by,
            throw_if_true,
            throw_if_false,
            num_inspected_items: 0,
        })
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = match s {
            "+" => Self::Add,
            "*" => Self::Times,
            _ => panic!("Unsupported operation"),
        };

        Ok(op)
    }
}

impl FromStr for Rhs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rhs = match s {
            "old" => Self::Old,
            _ => Self::Const(s.parse().unwrap()),
        };

        Ok(rhs)
    }
}
