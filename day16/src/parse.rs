use std::collections::HashMap;

fn valve_name_to_num(name: &str, valves: &[&str]) -> u64 {
    let valve_pos = valves
        .iter()
        .copied()
        .position(|other_valve| other_valve == name)
        .unwrap();
    1 << valve_pos
}

pub fn parse_graph(input: &str) -> (HashMap<u64, HashMap<u64, u32>>, HashMap<u64, u32>) {
    let mut graph: HashMap<&str, HashMap<&str, u32>> = HashMap::new();
    let mut flow_rates: HashMap<&str, u32> = HashMap::new();

    for line in input.lines() {
        let (valve, neighbours) = line.split_once(';').unwrap();

        let valve = valve.strip_prefix("Valve ").unwrap();
        let (valve_name, flow_rate) = valve.split_once(' ').unwrap();
        let flow_rate = flow_rate
            .strip_prefix("has flow rate=")
            .map(|rate| rate.parse().unwrap())
            .unwrap();

        flow_rates.insert(valve_name, flow_rate);

        let neighbours = neighbours
            .trim_start_matches(|c: char| c.is_lowercase() || c.is_whitespace())
            .split(", ")
            .map(|valve_name| (valve_name, 1))
            .collect();

        graph.insert(valve_name, neighbours);
    }

    let mut valves: Vec<&str> = graph.keys().copied().collect();
    valves.sort_unstable();

    let graph = graph
        .into_iter()
        .map(|(valve_name, neighbours)| {
            let neighbours = neighbours
                .into_iter()
                .map(|(neighbour_name, distance)| {
                    (valve_name_to_num(neighbour_name, &valves), distance)
                })
                .collect();

            (valve_name_to_num(valve_name, &valves), neighbours)
        })
        .collect();
    let flow_rates = flow_rates
        .into_iter()
        .map(|(valve_name, flow_rate)| (valve_name_to_num(valve_name, &valves), flow_rate))
        .collect();

    (graph, flow_rates)
}
