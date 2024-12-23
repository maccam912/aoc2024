use std::collections::{HashMap, HashSet};
use crate::Solution;

pub struct Day23;

impl Solution for Day23 {
    fn part1(&self, input: &str) -> String {
        let graph = parse_input(input);
        let triplets = find_triplets(&graph);
        let t_triplets = triplets.iter()
            .filter(|&triplet| triplet.iter().any(|name| name.starts_with('t')))
            .count();
        t_triplets.to_string()
    }

    fn part2(&self, _input: &str) -> String {
        "Not implemented".to_string()
    }
}

fn parse_input(input: &str) -> HashMap<String, HashSet<String>> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        graph.entry(a.to_string())
            .or_default()
            .insert(b.to_string());
        graph.entry(b.to_string())
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
                if graph[&nodes[i]].contains(&nodes[k]) && 
                   graph[&nodes[j]].contains(&nodes[k]) {
                    let mut triplet = vec![nodes[i].clone(), nodes[j].clone(), nodes[k].clone()];
                    triplet.sort();
                    triplets.push(triplet);
                }
            }
        }
    }
    
    triplets
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
        assert_eq!(day.part2(input), "Not implemented");
    }
}
