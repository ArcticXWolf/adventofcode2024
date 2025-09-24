use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(23);

#[derive(Debug, Default)]
struct Network {
    nodes: Vec<String>,
    connections: HashMap<usize, Vec<usize>>,
}

impl Network {
    fn new(input: &str) -> Self {
        let mut network: Network = Default::default();
        let connection_list = input
            .lines()
            .map(|l| l.split_once('-').unwrap())
            .collect::<Vec<(&str, &str)>>();

        for (c1, c2) in connection_list {
            let c1_string = c1.to_string();
            let c2_string = c2.to_string();

            let c1_idx = if network.nodes.contains(&c1_string) {
                network.nodes.iter().position(|n| *n == c1_string).unwrap()
            } else {
                network.nodes.push(c1_string);
                network.nodes.len() - 1
            };
            let c2_idx = if network.nodes.contains(&c2_string) {
                network.nodes.iter().position(|n| *n == c2_string).unwrap()
            } else {
                network.nodes.push(c2_string);
                network.nodes.len() - 1
            };

            network.connections.entry(c1_idx).or_default().push(c2_idx);
            network.connections.entry(c2_idx).or_default().push(c1_idx);
        }

        for v in network.connections.values_mut() {
            v.sort();
        }

        network
    }

    fn count_three_connected_sets(&self) -> usize {
        let mut count = 0;
        for (idx, name) in self.nodes.iter().enumerate() {
            let connections = self.connections.get(&idx).unwrap();
            for (idx1, idx2) in connections.iter().tuple_combinations() {
                if self.connections.get(idx1).unwrap().contains(idx2) {
                    let name1 = self.nodes.get(*idx1).unwrap();
                    let name2 = self.nodes.get(*idx2).unwrap();
                    if name.starts_with('t') || name1.starts_with('t') || name2.starts_with('t') {
                        count += 1;
                    }
                }
            }
        }
        count / 3
    }

    fn find_connected_set(&self, start_node: usize, current_set: &[usize]) -> Vec<usize> {
        let max_node_current_set = *current_set.iter().max().unwrap_or(&0);
        let connections_to_search: Vec<usize> = self
            .connections
            .get(&start_node)
            .unwrap()
            .iter()
            .filter(|&n| *n > max_node_current_set)
            .filter(|n| {
                let snc = self.connections.get(n).unwrap();
                for cn in current_set {
                    if !snc.contains(cn) {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        if connections_to_search.len() == 0 {
            return current_set.to_vec();
        }

        let mut best_set = vec![];
        for n in connections_to_search.into_iter() {
            let next_set: Vec<usize> = current_set.iter().cloned().chain([n]).collect();
            let connected_set = self.find_connected_set(start_node, &next_set);
            if connected_set.len() > best_set.len() {
                best_set = connected_set;
            }
        }
        best_set
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let network = Network::new(input);
    Some(network.count_three_connected_sets() as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let network = Network::new(input);
    let mut best_set = vec![];
    for id in 0..network.nodes.len() {
        let connected_set = network.find_connected_set(id, &vec![id]);
        if connected_set.len() >= best_set.len() {
            best_set = connected_set;
        }
    }
    Some(
        best_set
            .iter()
            .map(|&n| network.nodes.get(n).unwrap())
            .sorted()
            .join(","),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
