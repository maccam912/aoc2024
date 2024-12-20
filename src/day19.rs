use crate::Solution;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Day19;

impl Day19 {
    fn count_pattern_solutions(
        pattern: &str,
        towels: &[&str],
        cache: &mut HashMap<String, u64>,
    ) -> u64 {
        // Check cache first
        if let Some(&result) = cache.get(pattern) {
            return result;
        }

        // Base case: if pattern is empty, we've found one valid solution
        if pattern.is_empty() {
            return 1;
        }

        let first_char = pattern.chars().next().unwrap();

        // Try each available towel that could match the start of our pattern
        let mut total = 0;
        for &towel in towels
            .iter()
            .filter(|&&t| !t.is_empty() && t.starts_with(first_char))
        {
            if pattern.starts_with(towel) {
                let remaining = &pattern[towel.len()..];
                if remaining.len() < pattern.len() {
                    total += Self::count_pattern_solutions(remaining, towels, cache);
                }
            }
        }

        // Cache the result before returning
        cache.insert(pattern.to_string(), total);
        total
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
            let solutions = Day19::count_pattern_solutions(pattern, &towels, &mut cache);
            let can_make = solutions > 0;
            println!(
                "Pattern '{}': {}",
                pattern,
                if can_make { "✓" } else { "✗" }
            );
            if can_make {
                possible_count += 1;
            }
        }

        possible_count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (towels, patterns) = Day19::parse_input(input);
        let mut cache = HashMap::new();

        let total = patterns
            .iter()
            .map(|pattern| {
                let solutions = Day19::count_pattern_solutions(pattern, &towels, &mut cache);
                println!("Pattern '{}': {} solutions", pattern, solutions);
                solutions
            })
            .sum::<u64>();

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part1_sample() {
        let input =
            "r, wr, b, g, bwu, rb, gb, br\n\nbrwrr\nbggr\ngbbr\nrrbgbr\nubwu\nbwurrg\nbrgr\nbbrgwb";
        assert_eq!(Day19.part1(input), "6");
    }

    #[test]
    fn test_part2_sample() {
        let input =
            "r, wr, b, g, bwu, rb, gb, br\n\nbrwrr\nbggr\ngbbr\nrrbgbr\nubwu\nbwurrg\nbrgr\nbbrgwb";
        assert_eq!(Day19.part2(input), "16");
    }
}
