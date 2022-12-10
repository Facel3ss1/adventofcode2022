struct Cpu {
    x: i32,
    cycle_count: i32,
}

fn main() {
    let mut cpu = Cpu {
        x: 1,
        cycle_count: 0,
    };

    let mut signal_strengths_sum = 0;
    let mut cycle_numbers = std::iter::successors(Some(20), |n: &i32| n.checked_add(40))
        .take(6)
        .peekable();

    for line in include_str!("input.txt").lines() {
        let mut line = line.split_whitespace();
        let instruction = line.next().unwrap();
        let operand = line.next().map(|op| op.parse().unwrap()).unwrap_or(0);

        let cycle_count = match instruction {
            "noop" => 1,
            "addx" => 2,
            _ => panic!("Unsupported instruction"),
        };

        for _ in 0..cycle_count {
            cpu.cycle_count += 1;

            if let Some(cycle_number) = cycle_numbers
                .peek()
                .copied()
                .filter(|&cycle_number| cpu.cycle_count == cycle_number)
            {
                signal_strengths_sum += cycle_number * cpu.x;
                cycle_numbers.next();
            }

            let position = (cpu.cycle_count - 1) % 40;
            if (cpu.x - 1..=cpu.x + 1).contains(&position) {
                print!("#");
            } else {
                print!(" ");
            }

            if cpu.cycle_count % 40 == 0 {
                println!();
            }
        }

        cpu.x += operand;
    }

    println!();
    println!("Part 1: {signal_strengths_sum}");
}
