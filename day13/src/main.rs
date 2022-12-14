use std::cmp::Ordering;

mod parse;

#[derive(PartialEq, Eq, Clone)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn singleton(packet: Packet) -> Packet {
        Packet::List(vec![packet])
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Number(num), Packet::Number(other_num)) => num.cmp(other_num),
            (Packet::List(list), Packet::List(other_list)) => list
                .iter()
                .zip(other_list.iter())
                .map(|(item, other_item)| item.cmp(other_item))
                .chain(std::iter::once(list.len().cmp(&other_list.len())))
                .find(|ord| ord.is_ne())
                .unwrap_or(Ordering::Equal),
            (Packet::Number(num), other_list) => {
                Packet::singleton(Packet::Number(*num)).cmp(other_list)
            }
            (list, Packet::Number(other_num)) => {
                list.cmp(&Packet::singleton(Packet::Number(*other_num)))
            }
        }
    }
}

fn main() {
    let mut packets: Vec<Packet> = include_str!("input.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    let indices_sum: usize = packets
        .chunks_exact(2)
        .map(|chunk| (&chunk[0], &chunk[1]))
        .map(|(left, right)| left.cmp(right))
        .enumerate()
        .filter_map(|(i, ord)| ord.is_le().then_some(i + 1))
        .sum();

    let divider_packet2 = Packet::singleton(Packet::singleton(Packet::Number(2)));
    let divider_packet6 = Packet::singleton(Packet::singleton(Packet::Number(6)));

    packets.push(divider_packet2.clone());
    packets.push(divider_packet6.clone());

    packets.sort_unstable();

    let decoder_key = packets
        .iter()
        .position(|p| p == &divider_packet2)
        .zip(packets.iter().position(|p| p == &divider_packet6))
        .map(|(p1, p2)| (p1 + 1) * (p2 + 1))
        .unwrap();

    println!("Part 1: {indices_sum}");
    println!("Part 2: {decoder_key}");
}
