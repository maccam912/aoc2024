use crate::Solution;

pub struct Day02;

impl Solution for Day02 {
    fn part1(&self, input: &str) -> String {
        input
            .lines()
            .filter(|line| is_safe_sequence(line, false))
            .count()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        input
            .lines()
            .filter(|line| is_safe_sequence(line, true))
            .count()
            .to_string()
    }
}

fn is_safe_sequence(line: &str, allow_remove_one: bool) -> bool {
    let numbers: Vec<i32> = line
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if numbers.len() < 2 {
        return false;
    }

    // For part 2, try removing each number once and check if sequence becomes valid
    if allow_remove_one {
        for skip_idx in 0..numbers.len() {
            let mut test_sequence = Vec::new();
            for (i, &num) in numbers.iter().enumerate() {
                if i != skip_idx {
                    test_sequence.push(num);
                }
            }
            if is_valid_sequence(&test_sequence) {
                return true;
            }
        }
        // If no single removal makes it valid, check if it's valid as is
        is_valid_sequence(&numbers)
    } else {
        is_valid_sequence(&numbers)
    }
}

fn is_valid_sequence(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return false;
    }

    // Check if sequence is increasing or decreasing
    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i - 1];

        // Check if difference is within bounds (1 to 3)
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        // Check if sequence maintains its direction
        if diff > 0 {
            decreasing = false;
        } else {
            increasing = false;
        }

        // If neither increasing nor decreasing, it's not valid
        if !increasing && !decreasing {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(Day02.part1(input), "2");
    }

    #[test]
    fn test_part2_sample() {
        let input = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(Day02.part2(input), "4");
    }
}
