use crate::Solution;
use std::collections::HashMap;

pub struct Day24;

#[derive(Debug, Clone)]
enum Gate {
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

impl Solution for Day24 {
    fn part1(&self, input: &str) -> String {
        let (initial_values, gates) = parse_input(input);
        println!("Initial values: {:?}", initial_values);
        println!("Gates: {:?}", gates);
        
        let wire_values = simulate_circuit(&initial_values, &gates);
        println!("Final wire values: {:?}", wire_values);
        
        // Get all z-wires sorted by their numeric suffix in descending order
        let mut z_wires: Vec<_> = wire_values.keys()
            .filter(|k| k.starts_with('z'))
            .collect();
        z_wires.sort_by(|a, b| {
            let a_num = a[1..].parse::<usize>().unwrap_or(0);
            let b_num = b[1..].parse::<usize>().unwrap_or(0);
            b_num.cmp(&a_num)
        });
        
        println!("Z-wires in order: {:?}", z_wires);

        // Combine bits into a decimal number
        let mut result = 0;
        for wire in z_wires.iter() {
            let bit = *wire_values.get(*wire).unwrap_or(&0);
            println!("Wire {} = {}", wire, bit);
            result = (result << 1) | (bit as u64);
            println!("Current result: {}", result);
        }

        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (initial_values, gates) = parse_input(input);
        
        // Helper function to calculate hamming distance
        let calculate_hamming_distance = |gates: &HashMap<String, Gate>| {
            let wire_values = simulate_circuit(&initial_values, gates);
            
            // Helper function to get sorted wires by prefix and convert to decimal
            let get_decimal_value = |prefix: char| {
                let mut wires: Vec<_> = wire_values.keys()
                    .filter(|k| k.starts_with(prefix))
                    .collect();
                wires.sort_by(|a, b| {
                    let a_num = a[1..].parse::<usize>().unwrap_or(0);
                    let b_num = b[1..].parse::<usize>().unwrap_or(0);
                    b_num.cmp(&a_num)
                });
                
                let mut result = 0;
                for wire in wires.iter() {
                    let bit = *wire_values.get(*wire).unwrap_or(&0);
                    result = (result << 1) | (bit as u64);
                }
                result
            };

            let x_value = get_decimal_value('x');
            let y_value = get_decimal_value('y');
            let z_value = get_decimal_value('z');
            let xy_sum = x_value + y_value;

            let max_bits = 32; //std::cmp::max(z_value.ilog2() as usize + 1, xy_sum.ilog2() as usize + 1);
            let z_binary = format!("{:0width$b}", z_value, width = max_bits);
            let xy_binary = format!("{:0width$b}", xy_sum, width = max_bits);

            let hamming_distance = z_binary.chars()
                .zip(xy_binary.chars())
                .filter(|(a, b)| a != b)
                .count();

            (hamming_distance, x_value, y_value, z_value, xy_sum, z_binary, xy_binary)
        };

        // Get original hamming distance
        let (original_distance, x_value, y_value, z_value, xy_sum, z_binary, xy_binary) = 
            calculate_hamming_distance(&gates);

        // Find which bits differ in the original circuit
        let original_diff_bits: Vec<usize> = z_binary.chars()
            .zip(xy_binary.chars())
            .enumerate()
            .filter(|(_, (z, xy))| z != xy)
            .map(|(i, _)| i)
            .collect();

        println!("Original differing bits: {:?}", original_diff_bits);

        // Track all improvements and their affected bits
        let mut improvements = Vec::new();
        
        let gate_entries: Vec<_> = gates.iter().collect();
        let mut count = 0;
        for i in 0..gate_entries.len() {
            for j in (i + 1)..gate_entries.len() {
                count += 1;
                if count % 1000 == 0 {
                    println!("{}/{}\r", count, (gate_entries.len() * gate_entries.len())/2);
                }
                let mut new_gates = gates.clone();
                
                // Swap the output wires
                let (output1, gate1) = gate_entries[i];
                let (output2, gate2) = gate_entries[j];
                
                new_gates.insert(output1.clone(), gate2.clone());
                new_gates.insert(output2.clone(), gate1.clone());
                
                let (new_distance, _, _, _, _, new_z_binary, new_xy_binary) = calculate_hamming_distance(&new_gates);

                if new_distance < original_distance {
                    // Find which bits changed
                    let changed_bits: Vec<usize> = new_z_binary.chars()
                        .zip(new_xy_binary.chars())
                        .enumerate()
                        .filter(|(_, (z, xy))| z != xy)
                        .map(|(i, _)| i)
                        .collect();

                    improvements.push((
                        output1.clone(),
                        output2.clone(),
                        new_distance,
                        changed_bits.clone()
                    ));
                    
                    //println!("Swap {} and {} changes {} bits: {:?}", 
                    //    output1, output2, changed_bits.len(), changed_bits);
                }
            }
        }

        // Sort improvements by number of bits changed
        improvements.sort_by_key(|(_, _, dist, bits)| (*dist, bits.len()));

        // Count how many pairs affect each bit
        let mut bit_coverage = std::collections::HashMap::new();
        for bit in &original_diff_bits {
            bit_coverage.insert(bit, Vec::new());
        }

        for (g1, g2, _, bits) in &improvements {
            for bit in bits {
                if let Some(pairs) = bit_coverage.get_mut(&bit) {
                    pairs.push((g1.clone(), g2.clone()));
                }
            }
        }

        // Sort bits by how many pairs affect them
        let mut bit_counts: Vec<_> = bit_coverage.iter().collect();
        bit_counts.sort_by_key(|(_, pairs)| pairs.len());
        
        format!("Original differing bits ({} bits): {:?}\n\nBit coverage:\n{}\n\nFound {} improvements:",
            original_diff_bits.len(),
            original_diff_bits,
            bit_counts.iter()
                .map(|(&bit, pairs)| format!("Bit {}: {} pairs affect it", 
                    bit, 
                    pairs.len(),
                ))
                .collect::<Vec<_>>()
                .join("\n"),
            improvements.len()
        )
    }
}

fn parse_input(input: &str) -> (HashMap<String, u8>, HashMap<String, Gate>) {
    let mut initial_values = HashMap::new();
    let mut gates = HashMap::new();
    let mut parsing_gates = false;

    for line in input.lines() {
        if line.is_empty() {
            parsing_gates = true;
            continue;
        }

        if !parsing_gates {
            // Parse initial values
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() == 2 {
                initial_values.insert(
                    parts[0].to_string(),
                    parts[1].trim().parse().unwrap_or(0),
                );
            }
        } else {
            // Parse gates
            let parts: Vec<&str> = line.split(" -> ").collect();
            if parts.len() == 2 {
                let gate_parts: Vec<&str> = parts[0].split_whitespace().collect();
                let output_wire = parts[1].trim().to_string();

                if gate_parts.len() == 3 {
                    let input1 = gate_parts[0].to_string();
                    let op = gate_parts[1];
                    let input2 = gate_parts[2].to_string();

                    let gate = match op {
                        "AND" => Gate::And(input1, input2),
                        "OR" => Gate::Or(input1, input2),
                        "XOR" => Gate::Xor(input1, input2),
                        _ => continue,
                    };
                    gates.insert(output_wire, gate);
                }
            }
        }
    }

    (initial_values, gates)
}

fn simulate_circuit(initial_values: &HashMap<String, u8>, gates: &HashMap<String, Gate>) -> HashMap<String, u8> {
    let mut wire_values = initial_values.clone();
    let mut changed = true;
    let mut iteration = 0;

    while changed {
        changed = false;
        iteration += 1;
        // println!("\nIteration {}", iteration);
        
        for (output_wire, gate) in gates {
            if wire_values.contains_key(output_wire) {
                continue;
            }

            match gate {
                Gate::And(w1, w2) => {
                    if let (Some(&v1), Some(&v2)) = (wire_values.get(w1), wire_values.get(w2)) {
                        wire_values.insert(output_wire.clone(), v1 & v2);
                        // println!("AND: {} & {} = {} -> {}", w1, w2, v1 & v2, output_wire);
                        changed = true;
                    }
                }
                Gate::Or(w1, w2) => {
                    if let (Some(&v1), Some(&v2)) = (wire_values.get(w1), wire_values.get(w2)) {
                        wire_values.insert(output_wire.clone(), v1 | v2);
                        // println!("OR: {} | {} = {} -> {}", w1, w2, v1 | v2, output_wire);
                        changed = true;
                    }
                }
                Gate::Xor(w1, w2) => {
                    if let (Some(&v1), Some(&v2)) = (wire_values.get(w1), wire_values.get(w2)) {
                        wire_values.insert(output_wire.clone(), v1 ^ v2);
                        // println!("XOR: {} ^ {} = {} -> {}", w1, w2, v1 ^ v2, output_wire);
                        changed = true;
                    }
                }
            }
        }
    }

    wire_values
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part1_sample() {
        let input = read_input(24, true);
        println!("\nInput:\n{}", input);
        assert_eq!(Day24.part1(&input), "2024");
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input(24, true);
        assert_eq!(Day24.part2(&input), "x: 0, y: 0, z: 0, x+y: 0\nz binary:  0\nx+y binary:0\nDifferent bits: 0");
    }
}
