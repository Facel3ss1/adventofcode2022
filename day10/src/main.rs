struct Cpu {
    x: i32,
    cycle_count: i32,
}

fn main() {
    let mut lines = include_str!("input.txt").lines();
    let mut cpu = Cpu {
        x: 1,
        cycle_count: 0,
    };

    let signal_strengths_sum = std::iter::successors(Some(20), |n: &i32| n.checked_add(40))
        .take(6)
        .map(|cycle_number| {
            let mut signal_strength = None;
            for line in lines.by_ref() {
                let mut line = line.split_whitespace();
                let instruction = line.next().unwrap();
                let operand = line.next().map(|op| op.parse().unwrap()).unwrap_or(0);

                match instruction {
                    "noop" => cpu.cycle_count += 1,
                    "addx" => cpu.cycle_count += 2,
                    _ => panic!("Unsupported instruction"),
                }

                if cpu.cycle_count >= cycle_number {
                    signal_strength = Some(cycle_number * cpu.x);
                }

                cpu.x += operand;

                if signal_strength.is_some() {
                    break;
                }
            }

            signal_strength.unwrap()
        })
        .sum::<i32>();

    println!("Part 1: {signal_strengths_sum}");
}
