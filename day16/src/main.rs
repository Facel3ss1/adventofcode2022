use std::collections::HashMap;

use crate::parse::parse_graph;

mod parse;

// Remove all the valves (except the starting valve) with a flow rate of zero
fn simplify_graph(graph: &mut HashMap<u64, HashMap<u64, u32>>, flow_rates: &HashMap<u64, u32>) {
    let valves_to_remove: Vec<u64> = graph
        .keys()
        .copied()
        .filter(|&num| num != 1 && flow_rates[&num] == 0)
        .collect();

    for valve_num in valves_to_remove {
        let neighbours: Vec<u64> = graph[&valve_num].keys().copied().collect();

        for neighbour in neighbours.iter().copied() {
            let distance_to_neighbour = graph[&valve_num][&neighbour];

            for other_neighbour in neighbours.iter().copied() {
                if neighbour == other_neighbour {
                    continue;
                }

                let distance_to_other_neighbour = graph[&valve_num][&other_neighbour];

                let neighbour_neighbours = graph.get_mut(&neighbour).unwrap();
                neighbour_neighbours.insert(
                    other_neighbour,
                    distance_to_neighbour + distance_to_other_neighbour,
                );
                neighbour_neighbours.remove(&valve_num);
            }
        }

        graph.remove(&valve_num);
    }
}

// Return the minimum distances between every two nodes in the graph
fn floyd_warshall(graph: &HashMap<u64, HashMap<u64, u32>>) -> HashMap<(u64, u64), u32> {
    let mut shortest_paths = HashMap::new();

    for (&valve_num, neighbours) in graph.iter() {
        for (&neighbour, &distance) in neighbours.iter() {
            shortest_paths.insert((valve_num, neighbour), distance);
        }
    }

    for valve_num in graph.keys().copied() {
        shortest_paths.insert((valve_num, valve_num), 0);
    }

    for valve_k in graph.keys().copied() {
        for valve_i in graph.keys().copied() {
            for valve_j in graph.keys().copied() {
                let new_distance = shortest_paths
                    .entry((valve_i, valve_k))
                    .or_insert(u32::MAX)
                    .saturating_add(*shortest_paths.entry((valve_k, valve_j)).or_insert(u32::MAX));

                if *shortest_paths.entry((valve_i, valve_j)).or_insert(u32::MAX) > new_distance {
                    shortest_paths.insert((valve_i, valve_j), new_distance);
                }
            }
        }
    }

    shortest_paths
}

#[derive(Clone)]
struct State {
    current_valve: u64,
    valves_opened: u64,
    minute: u32,
    pressure_released: u32,
}

fn depth_first_search(
    distances: &HashMap<(u64, u64), u32>,
    flow_rates: &HashMap<u64, u32>,
    total_minutes: u32,
) -> HashMap<u64, u32> {
    let valves: Vec<u64> = flow_rates
        .keys()
        .copied()
        .filter(|&valve| valve != 1)
        .filter(|&valve| distances.contains_key(&(1, valve)))
        .collect();

    let mut stack: Vec<State> = Vec::new();
    let mut max_pressures = HashMap::new();

    stack.extend(
        valves
            .iter()
            .copied()
            .map(|valve| {
                let minute = distances[&(1, valve)] + 1;
                let pressure_released = (total_minutes - minute) * flow_rates[&valve];
                State {
                    current_valve: valve,
                    valves_opened: valve,
                    minute,
                    pressure_released,
                }
            })
            .filter(|state| state.minute <= total_minutes),
    );

    while let Some(current_state) = stack.pop() {
        let pressure_entry = max_pressures
            .entry(current_state.valves_opened)
            .or_insert(0);
        if current_state.pressure_released > *pressure_entry {
            *pressure_entry = current_state.pressure_released
        }

        stack.extend(
            valves
                .iter()
                .copied()
                .filter(|&valve| current_state.valves_opened & valve != valve)
                .filter_map(|next_valve| {
                    let minute = current_state.minute
                        + distances[&(current_state.current_valve, next_valve)]
                        + 1;

                    (minute <= total_minutes).then(|| {
                        let mut next_state = current_state.clone();

                        next_state.current_valve = next_valve;
                        next_state.valves_opened |= next_valve;
                        next_state.minute = minute;
                        next_state.pressure_released +=
                            (total_minutes - minute) * flow_rates[&next_valve];

                        next_state
                    })
                }),
        );
    }

    max_pressures
}

fn main() {
    let (mut graph, flow_rates) = parse_graph(include_str!("input.txt"));
    simplify_graph(&mut graph, &flow_rates);
    let distances = floyd_warshall(&graph);

    let part1 = depth_first_search(&distances, &flow_rates, 30);
    println!("Part 1: {}", part1.into_values().max().unwrap());

    let part2 = depth_first_search(&distances, &flow_rates, 26);
    let part2 = part2
        .iter()
        .map(|(&valves_opened, &pressure_released)| (valves_opened, pressure_released))
        .flat_map(|me| {
            part2.iter().map(
                move |(&elephant_valves_opened, &elephant_pressure_released)| {
                    (me, (elephant_valves_opened, elephant_pressure_released))
                },
            )
        })
        .filter(|&(me, elephant)| me.0 & elephant.0 == 0)
        .map(|(me, elephant)| me.1 + elephant.1)
        .max()
        .unwrap();

    println!("Part 2: {part2}");
}
