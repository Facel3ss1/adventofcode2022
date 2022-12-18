use std::collections::HashMap;

use crate::parse::parse_graph;

mod parse;

// Remove all the valves (except the starting valve) with a flow rate of zero
fn simplify_graph(graph: &mut HashMap<u16, HashMap<u16, u32>>, flow_rates: &HashMap<u16, u32>) {
    let valves_to_remove: Vec<u16> = graph
        .keys()
        .filter(|&num| *num != 0 && flow_rates[num] == 0)
        .copied()
        .collect();

    for valve_num in valves_to_remove {
        let neighbours: Vec<u16> = graph[&valve_num].keys().copied().collect();

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
fn floyd_warshall(graph: &HashMap<u16, HashMap<u16, u32>>) -> HashMap<(u16, u16), u32> {
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
    valves_opened: Vec<u16>,
    minute: u32,
    pressure_released: u32,
}

fn main() {
    let (mut graph, flow_rates) = parse_graph(include_str!("input.txt"));
    simplify_graph(&mut graph, &flow_rates);
    let distances = floyd_warshall(&graph);
    let valves: Vec<u16> = graph.keys().copied().filter(|&valve| valve != 0).collect();

    let mut stack: Vec<State> = Vec::new();
    let mut max_pressure = 0;

    stack.extend(
        valves
            .iter()
            .copied()
            .map(|valve| {
                let minute = distances[&(0, valve)] + 1;
                let pressure_released = (30 - minute) * flow_rates[&valve];
                State {
                    valves_opened: vec![valve],
                    minute,
                    pressure_released,
                }
            })
            .filter(|state| state.minute <= 30),
    );

    while let Some(current_state) = stack.pop() {
        if current_state.pressure_released > max_pressure {
            max_pressure = current_state.pressure_released
        }

        stack.extend(
            valves
                .iter()
                .copied()
                .filter(|&valve| {
                    !current_state
                        .valves_opened
                        .iter()
                        .any(|&opened_valve| opened_valve == valve)
                })
                .filter_map(|next_valve| {
                    let current_valve = current_state.valves_opened.last().copied().unwrap();
                    let minute = current_state.minute + distances[&(current_valve, next_valve)] + 1;

                    (minute <= 30).then(|| {
                        let mut next_state = current_state.clone();

                        next_state.minute = minute;
                        next_state.pressure_released += (30 - minute) * flow_rates[&next_valve];
                        next_state.valves_opened.push(next_valve);

                        next_state
                    })
                }),
        );
    }

    println!("Part 1: {}", max_pressure);
}
