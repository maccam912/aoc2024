use crate::Solution;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct PrintRules {
    rules: Vec<(u32, u32)>, // (before, after) pairs
    updates: Vec<Vec<u32>>,
}

impl PrintRules {
    fn split_input(input: &str) -> (&str, &str) {
        // Try splitting by double newline, handling both CRLF and LF
        if input.contains("\r\n\r\n") {
            let parts: Vec<&str> = input.split("\r\n\r\n").collect();
            (parts[0], parts[1])
        } else {
            let parts: Vec<&str> = input.split("\n\n").collect();
            (parts[0], parts[1])
        }
    }

    fn parse(input: &str) -> Self {
        let (rules_str, updates_str) = Self::split_input(input);

        let rules: Vec<(u32, u32)> = rules_str
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let mut nums = line.trim().split('|');
                let before = nums.next().unwrap_or("").trim();
                let after = nums.next().unwrap_or("").trim();
                (before.parse().unwrap(), after.parse().unwrap())
            })
            .collect();

        let updates: Vec<Vec<u32>> = updates_str
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                line.trim()
                    .split(',')
                    .map(|n| n.trim().parse().unwrap())
                    .collect()
            })
            .collect();

        PrintRules { rules, updates }
    }

    fn is_valid_order(&self, update: &[u32]) -> bool {
        // For each pair of numbers in the update
        for i in 0..update.len() {
            for j in i + 1..update.len() {
                let a = update[i];
                let b = update[j];

                // Check if there's a rule saying b should come before a
                if self
                    .rules
                    .iter()
                    .any(|&(before, after)| before == b && after == a)
                {
                    return false;
                }
            }
        }
        true
    }

    fn get_middle_number(&self, update: &[u32]) -> u32 {
        update[update.len() / 2]
    }

    fn sort_update(&self, update: &[u32]) -> Vec<u32> {
        let mut sorted = update.to_vec();
        let mut changed = true;
        while changed {
            changed = false;
            for i in 0..sorted.len() {
                for j in i + 1..sorted.len() {
                    let a = sorted[i];
                    let b = sorted[j];
                    // If there's a rule saying b should come before a, swap them
                    if self
                        .rules
                        .iter()
                        .any(|&(before, after)| before == b && after == a)
                    {
                        sorted.swap(i, j);
                        changed = true;
                    }
                }
            }
        }
        sorted
    }

    fn get_invalid_updates(&self) -> Vec<Vec<u32>> {
        self.updates
            .iter()
            .filter(|update| !self.is_valid_order(update))
            .cloned()
            .collect()
    }
}

pub struct Day05;

impl Solution for Day05 {
    fn part1(&self, input: &str) -> String {
        let rules = PrintRules::parse(input);

        let sum: u32 = rules
            .updates
            .iter()
            .filter(|update| rules.is_valid_order(update))
            .map(|update| rules.get_middle_number(update))
            .sum();

        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let rules = PrintRules::parse(input);

        // First find all initially invalid updates
        let invalid_updates: Vec<Vec<u32>> = rules
            .updates
            .iter()
            .filter(|update| !rules.is_valid_order(update))
            .cloned()
            .collect();

        // Now process only those invalid updates
        let sum: u32 = invalid_updates
            .iter()
            .map(|update| {
                let sorted = rules.sort_update(update);
                rules.get_middle_number(&sorted)
            })
            .sum();

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part1_sample() {
        let input = read_input(05, true);
        assert_eq!(Day05.part1(&input), "143");
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input(05, true);
        assert_eq!(Day05.part2(&input), "123");
    }
}
