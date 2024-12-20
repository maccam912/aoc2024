use crate::Solution;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub struct Day19;

impl Day19 {
    fn can_make_pattern_with_cache(pattern: &str, towels: &[&str], cache: &mut HashMap<String, bool>) -> bool {
        // Check cache first
        if let Some(&result) = cache.get(pattern) {
            return result;
        }

        // Base case: if pattern is empty, we've found a valid solution
        if pattern.is_empty() {
            return true;
        }

        let first_char = pattern.chars().next().unwrap();
        
        // Try each available towel that could match the start of our pattern
        let result = towels.iter()
            .filter(|&&towel| !towel.is_empty() && towel.starts_with(first_char))
            .any(|&towel| {
                if pattern.starts_with(towel) {
                    let remaining = &pattern[towel.len()..];
                    if remaining.len() < pattern.len() && 
                       Self::can_make_pattern_with_cache(remaining, towels, cache) {
                        return true;
                    }
                }
                false
            });

        // Cache the result before returning
        cache.insert(pattern.to_string(), result);
        result
    }

    fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
        let mut parts = input.trim().split("\n\n");
        let towels: Vec<&str> = parts
            .next()
            .unwrap()
            .split(", ")
            .sorted_by_key(|s| std::cmp::Reverse(s.len()))
            .collect();
        let patterns = parts.next().unwrap().lines().collect();
        (towels, patterns)
    }
}

impl Solution for Day19 {
    fn part1(&self, input: &str) -> String {
        let (towels, patterns) = Day19::parse_input(input);
        let mut cache = HashMap::new();
        
        let mut possible_count = 0;
        for pattern in patterns.iter() {
            let can_make = Day19::can_make_pattern_with_cache(pattern, &towels, &mut cache);
            println!("Pattern '{}': {}", pattern, if can_make { "✓" } else { "✗" });
            if can_make {
                possible_count += 1;
            }
        }

        possible_count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        // TODO: Implement part 2 when available
        "Not implemented".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part1_sample() {
        let input = "r, wr, b, g, bwu, rb, gb, br\n\nbrwrr\nbggr\ngbbr\nrrbgbr\nubwu\nbwurrg\nbrgr\nbbrgwb";
        assert_eq!(Day19.part1(input), "6");
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input(19, true);
        assert_eq!(Day19.part2(&input), "Not implemented");
    }
}
