use std::collections::HashMap;

fn valve_name_to_num(name: &str) -> u16 {
    let bytes = name.as_bytes();
    let first_byte = (bytes[0] - b'A') as u16;
    let second_byte = (bytes[1] - b'A') as u16;
    first_byte << 8 | second_byte
}

pub fn parse_graph(input: &str) -> (HashMap<u16, HashMap<u16, u32>>, HashMap<u16, u32>) {
    let mut graph = HashMap::new();
    let mut flow_rates = HashMap::new();

    for line in input.lines() {
        let (valve, neighbours) = line.split_once(';').unwrap();

        let valve = valve.strip_prefix("Valve ").unwrap();
        let (valve_name, flow_rate) = valve.split_once(' ').unwrap();
        let valve_num = valve_name_to_num(valve_name);
        let flow_rate = flow_rate
            .strip_prefix("has flow rate=")
            .map(|rate| rate.parse().unwrap())
            .unwrap();

        flow_rates.insert(valve_num, flow_rate);

        let neighbours = neighbours
            .trim_start_matches(|c: char| c.is_lowercase() || c.is_whitespace())
            .split(", ")
            .map(|s| (valve_name_to_num(s), 1))
            .collect();

        graph.insert(valve_num, neighbours);
    }

    (graph, flow_rates)
}
