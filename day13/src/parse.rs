use std::str::FromStr;

use crate::Expr;

impl FromStr for Expr {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse_expr(input).0)
    }
}

fn parse_expr(input: &str) -> (Expr, &str) {
    match input.chars().next().unwrap() {
        '[' => parse_list(&input[1..]),
        _ => parse_number(input),
    }
}

fn parse_list(mut input: &str) -> (Expr, &str) {
    let mut list = Vec::new();

    loop {
        match input.chars().next().unwrap() {
            ']' => break,
            ',' => input = &input[1..],
            _ => {
                let (expr, rest) = parse_expr(input);
                list.push(expr);
                input = rest;
            }
        }
    }

    (Expr::List(list), &input[1..])
}

fn parse_number(input: &str) -> (Expr, &str) {
    let (number, rest) = input
        .find(|c| c == ',' || c == ']')
        .map(|i| input.split_at(i))
        .unwrap_or((input, ""));
    (Expr::Number(number.parse().unwrap()), rest)
}
