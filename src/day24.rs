use crate::Solution;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Logic {
    And,
    Or,
    Xor,
}

struct Gate<'a> {
    logic: Logic,
    a: &'a str,
    b: &'a str,
    out: &'a str,
}

pub struct Day24;

/// Take all wires that start with the given prefix, sort them by name, and
/// decode them to an integer
fn get_value(wires: &HashMap<&str, u8>, prefix: &str) -> u64 {
    // find wires that start with prefix
    let mut wires_to_decode = wires
        .iter()
        .filter(|(name, _)| name.starts_with(prefix))
        .collect::<Vec<_>>();

    // sort wires by name
    wires_to_decode.sort();

    // 00 is the least significant bit
    wires_to_decode.reverse();

    // decode
    let mut result = 0u64;
    for (_, v) in wires_to_decode {
        result <<= 1;
        if *v == 1 {
            result += 1;
        }
    }

    result
}

fn run<'a>(wires: &HashMap<&'a str, u8>, gates: &[Gate<'a>]) -> HashMap<&'a str, u8> {
    let mut wires = wires.clone();
    let mut changed = true;

    while changed {
        changed = false;
        for gate in gates {
            if wires.contains_key(gate.out) {
                continue;
            }

            let Some(&a) = wires.get(gate.a) else {
                continue;
            };
            let Some(&b) = wires.get(gate.b) else {
                continue;
            };

            let v = match gate.logic {
                Logic::And => {
                    if a == 1 && b == 1 {
                        1
                    } else {
                        0
                    }
                }
                Logic::Or => {
                    if a == 1 || b == 1 {
                        1
                    } else {
                        0
                    }
                }
                Logic::Xor => {
                    if a != b {
                        1
                    } else {
                        0
                    }
                }
            };

            wires.insert(gate.out, v);
            changed = true;
        }
    }

    wires
}

impl Solution for Day24 {
    fn part1(&self, input: &str) -> String {
        let (initial_values, gates) = parse_input(input);
        let wire_values = run(&initial_values, &gates);
        get_value(&wire_values, "z").to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (initial_wires, gates) = parse_input(input);

        // Find broken nodes by checking common patterns
        let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
        for g in &gates {
            edges.entry(g.a).or_default().push(g.out);
            edges.entry(g.b).or_default().push(g.out);
        }

        let mut broken_nodes = HashSet::new();
        for g in &gates {
            // z nodes must be XOR (except for the last one)
            if g.out.starts_with("z") && g.out != "z45" && g.logic != Logic::Xor {
                broken_nodes.insert(g.out);
            }
            // z nodes must not be inputs of other nodes
            if g.a.starts_with("z") {
                broken_nodes.insert(g.a);
            }
            if g.b.starts_with("z") {
                broken_nodes.insert(g.b);
            }

            // inputs of XOR nodes (except for z nodes) must be x and y nodes
            if g.logic == Logic::Xor
                && !g.out.starts_with("z")
                && !((g.a.starts_with("x") && g.b.starts_with("y"))
                    || (g.a.starts_with("y") && g.b.starts_with("x")))
            {
                broken_nodes.insert(g.out);
            }

            // XOR nodes (except z nodes) must always be input of exactly two other nodes
            if g.logic == Logic::Xor && !g.out.starts_with("z") && edges[g.out].len() != 2 {
                broken_nodes.insert(g.out);
            }

            // AND nodes must always be input of exactly one other node
            if g.logic == Logic::And
                && !g.out.starts_with("z")
                && edges[g.out].len() != 1
                && !((g.a == "x00" && g.b == "y00") || (g.a == "y00" && g.b == "x00"))
            {
                broken_nodes.insert(g.out);
            }
        }

        // Return the broken nodes sorted
        let mut broken_nodes = broken_nodes.into_iter().collect::<Vec<_>>();
        broken_nodes.sort();
        broken_nodes.join(",")
    }
}

fn parse_input(input: &str) -> (HashMap<&str, u8>, Vec<Gate>) {
    let (wires, gates) = input.split_once("\n\n").unwrap();

    let wires = wires
        .lines()
        .map(|w| {
            let (name, value) = w.split_once(": ").unwrap();
            (name, if value == "1" { 1 } else { 0 })
        })
        .collect();

    let gates = gates
        .lines()
        .map(|l| {
            let (left, out) = l.split_once(" -> ").unwrap();
            let s = left.split_whitespace().collect::<Vec<_>>();
            let logic = match s[1] {
                "AND" => Logic::And,
                "OR" => Logic::Or,
                "XOR" => Logic::Xor,
                _ => panic!("Invalid gate: {}", left),
            };
            Gate {
                logic,
                a: s[0],
                b: s[2],
                out,
            }
        })
        .collect();

    (wires, gates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part1_sample() {
        let input = read_input(24, true);
        assert_eq!(Day24.part1(&input), "2024");
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input(24, true);
        Day24.part2(&input);
    }
}
