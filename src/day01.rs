use crate::Solution;
use std::collections::HashMap;

pub struct Day01;

impl Day01 {
    fn parse_input(&self, input: &str) -> (Vec<i64>, Vec<i64>) {
        let mut left = Vec::new();
        let mut right = Vec::new();

        for line in input.lines() {
            let mut parts = line.split_whitespace();
            let l: i64 = parts.next().unwrap().parse().unwrap();
            let r: i64 = parts.next().unwrap().parse().unwrap();
            left.push(l);
            right.push(r);
        }

        (left, right)
    }
}

impl Solution for Day01 {
    fn part1(&self, input: &str) -> String {
        let (mut left, mut right) = self.parse_input(input);

        // Sort both lists independently
        left.sort_unstable();
        right.sort_unstable();

        // Calculate total distance between corresponding pairs
        let total_distance: i64 = left
            .iter()
            .zip(right.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();

        total_distance.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (left, right) = self.parse_input(input);

        // Count occurrences in right list
        let right_counts: HashMap<i64, i64> = right.iter().fold(HashMap::new(), |mut map, &num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });

        // Calculate similarity score
        let similarity_score: i64 = left
            .iter()
            .map(|&num| num * right_counts.get(&num).copied().unwrap_or(0))
            .sum();

        similarity_score.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part1_sample() {
        let input = read_input(1, true);
        assert_eq!(Day01.part1(&input), "11");
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input(1, true);
        assert_eq!(Day01.part2(&input), "31");
    }
}
