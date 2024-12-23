use crate::Solution;
use std::collections::{HashMap, HashSet};

pub struct Day23;

impl Solution for Day23 {
    fn part1(&self, input: &str) -> String {
        let graph = parse_input(input);
        let triplets = find_triplets(&graph);
        let t_triplets = triplets
            .iter()
            .filter(|&triplet| triplet.iter().any(|name| name.starts_with('t')))
            .count();
        t_triplets.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let graph = parse_input(input);
        let lan_party = find_largest_clique(&graph);
        let mut password: Vec<_> = lan_party.into_iter().collect();
        password.sort();
        password.join(",")
    }
}

fn parse_input(input: &str) -> HashMap<String, HashSet<String>> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        graph
            .entry(a.to_string())
            .or_default()
            .insert(b.to_string());
        graph
            .entry(b.to_string())
            .or_default()
            .insert(a.to_string());
    }

    graph
}

fn find_triplets(graph: &HashMap<String, HashSet<String>>) -> Vec<Vec<String>> {
    let mut triplets = Vec::new();
    let nodes: Vec<_> = graph.keys().cloned().collect();

    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            // Check if i and j are connected
            if !graph[&nodes[i]].contains(&nodes[j]) {
                continue;
            }

            for k in (j + 1)..nodes.len() {
                // Check if k is connected to both i and j
                if graph[&nodes[i]].contains(&nodes[k]) && graph[&nodes[j]].contains(&nodes[k]) {
                    let mut triplet = vec![nodes[i].clone(), nodes[j].clone(), nodes[k].clone()];
                    triplet.sort();
                    triplets.push(triplet);
                }
            }
        }
    }

    triplets
}

fn find_largest_clique(graph: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    let mut best_clique = HashSet::new();
    let mut current_clique = HashSet::new();
    let nodes: Vec<_> = graph.keys().cloned().collect();

    // Helper function to check if a node can be added to the current clique
    let is_connected_to_all =
        |node: &str, clique: &HashSet<String>, graph: &HashMap<String, HashSet<String>>| {
            clique.iter().all(|member| graph[node].contains(member))
        };

    // Recursive function to find maximum clique
    fn find_clique(
        pos: usize,
        nodes: &[String],
        current: &mut HashSet<String>,
        best: &mut HashSet<String>,
        graph: &HashMap<String, HashSet<String>>,
        is_connected_to_all: &dyn Fn(
            &str,
            &HashSet<String>,
            &HashMap<String, HashSet<String>>,
        ) -> bool,
    ) {
        if current.len() > best.len() {
            *best = current.clone();
        }

        // Try adding remaining nodes
        for i in pos..nodes.len() {
            let node = &nodes[i];
            if is_connected_to_all(node, current, graph) {
                current.insert(node.clone());
                find_clique(i + 1, nodes, current, best, graph, is_connected_to_all);
                current.remove(node);
            }
        }
    }

    find_clique(
        0,
        &nodes,
        &mut current_clique,
        &mut best_clique,
        graph,
        &is_connected_to_all,
    );
    best_clique
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = include_str!("../inputs/sample/23.txt");
        let day = Day23;
        assert_eq!(day.part1(input), "7");
    }

    #[test]
    fn test_part2_sample() {
        let input = include_str!("../inputs/sample/23.txt");
        let day = Day23;
        assert_eq!(day.part2(input), "co,de,ka,ta");
    }
}
