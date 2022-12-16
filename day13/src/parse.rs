use std::str::FromStr;

use crate::Packet;

impl FromStr for Packet {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse_packet(input).0)
    }
}

fn parse_packet(input: &str) -> (Packet, &str) {
    match input.chars().next().unwrap() {
        '[' => parse_list(input),
        _ => parse_number(input),
    }
}

fn parse_list(mut input: &str) -> (Packet, &str) {
    let mut list = Vec::new();

    input = input.strip_prefix('[').unwrap();
    loop {
        match input.chars().next().unwrap() {
            ']' => break,
            ',' => input = &input[1..],
            _ => {
                let (packet, rest) = parse_packet(input);
                list.push(packet);
                input = rest;
            }
        }
    }

    (Packet::List(list), &input[1..])
}

fn parse_number(input: &str) -> (Packet, &str) {
    let (number, rest) = input
        .find(|c: char| !c.is_ascii_digit())
        .map(|i| input.split_at(i))
        .unwrap_or((input, ""));
    (Packet::Number(number.parse().unwrap()), rest)
}
