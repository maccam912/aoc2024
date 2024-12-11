use crate::Solution;
use std::collections::HashMap;

pub struct Day11;

impl Day11 {
    fn solve(&self, input: &str, blinks: usize) -> String {
        // Initialize the frequency map from input
        let mut freq_map: HashMap<u64, u64> = HashMap::new();
        for num in input.split_whitespace() {
            let n = num.parse().unwrap();
            *freq_map.entry(n).or_insert(0) += 1;
        }

        // Simulate blinks
        for _ in 0..blinks {
            let mut new_map: HashMap<u64, u64> = HashMap::new();

            for (num, count) in freq_map.iter() {
                if *num == 0 {
                    // Rule 1: 0 becomes 1
                    *new_map.entry(1).or_insert(0) += count;
                    continue;
                }

                // Rule 2: Check if number has even number of digits
                let num_str = num.to_string();
                let digit_count = num_str.len();
                if digit_count % 2 == 0 {
                    let mid = digit_count / 2;
                    let left = num_str[..mid].parse::<u64>().unwrap();
                    let right = num_str[mid..].parse::<u64>().unwrap();
                    *new_map.entry(left).or_insert(0) += count;
                    *new_map.entry(right).or_insert(0) += count;
                } else {
                    // Rule 3: Multiply by 2024
                    *new_map.entry(num * 2024).or_insert(0) += count;
                }
            }
            freq_map = new_map;
        }

        // Sum all frequencies to get total count
        freq_map.values().sum::<u64>().to_string()
    }
}

impl Solution for Day11 {
    fn part1(&self, input: &str) -> String {
        self.solve(input, 25)
    }

    fn part2(&self, input: &str) -> String {
        self.solve(input, 75)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = "125 17";
        assert_eq!(Day11.part1(input), "55312");
    }
}
