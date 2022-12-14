use std::cmp::Ordering;

mod parse;

#[derive(PartialEq, Eq)]
enum Expr {
    Number(u32),
    List(Vec<Expr>),
}

impl Expr {
    fn singleton(number: u32) -> Expr {
        Expr::List(vec![Expr::Number(number)])
    }
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Expr {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Expr::Number(num), Expr::Number(other_num)) => num.cmp(other_num),
            (Expr::List(list), Expr::List(other_list)) => list
                .iter()
                .zip(other_list.iter())
                .map(|(item, other_item)| item.cmp(other_item))
                .chain(std::iter::once(list.len().cmp(&other_list.len())))
                .find(|ord| ord.is_ne())
                .unwrap_or(Ordering::Equal),
            (Expr::Number(num), other_list) => Expr::singleton(*num).cmp(other_list),
            (list, Expr::Number(other_num)) => list.cmp(&Expr::singleton(*other_num)),
        }
    }
}

fn main() {
    let pairs = include_str!("input.txt").split("\n\n");

    let indices_sum: usize = pairs
        .map(|pair| -> (Expr, Expr) {
            let (left, right) = pair.split_once('\n').unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .map(|(left, right)| left.cmp(&right))
        .enumerate()
        .filter_map(|(i, ord)| ord.is_le().then_some(i + 1))
        .sum();

    println!("Part 1: {indices_sum}");
}
