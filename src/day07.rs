use crate::Solution;

pub struct Day07;

impl Solution for Day07 {
    fn part1(&self, input: &str) -> String {
        let sum: i64 = input
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() != 2 {
                    return None;
                }

                let test_value: i64 = parts[0].trim().parse().ok()?;
                let numbers: Vec<i64> = parts[1]
                    .split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .collect();

                if can_make_value(&numbers, test_value) {
                    Some(test_value)
                } else {
                    None
                }
            })
            .sum();

        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let sum: i64 = input
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() != 2 {
                    return None;
                }

                let test_value: i64 = parts[0].trim().parse().ok()?;
                let numbers: Vec<i64> = parts[1]
                    .split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .collect();

                if can_make_value_part2(&numbers, test_value) {
                    Some(test_value)
                } else {
                    None
                }
            })
            .sum();

        sum.to_string()
    }
}

fn can_make_value(numbers: &[i64], target: i64) -> bool {
    if numbers.len() == 1 {
        return numbers[0] == target;
    }

    let operators = vec!['+', '*'];
    try_combinations(numbers, &mut vec![], &operators, target)
}

fn can_make_value_part2(numbers: &[i64], target: i64) -> bool {
    if numbers.len() == 1 {
        return numbers[0] == target;
    }

    let operators = vec!['+', '*', 'c'];
    try_combinations(numbers, &mut vec![], &operators, target)
}

fn try_combinations(
    numbers: &[i64],
    current_ops: &mut Vec<char>,
    operators: &[char],
    target: i64,
) -> bool {
    if current_ops.len() == numbers.len() - 1 {
        let mut result = numbers[0];
        let mut i = 0;
        while i < current_ops.len() {
            if current_ops[i] == 'c' {
                // Handle concatenation
                let mut concat_str = result.to_string();
                concat_str.push_str(&numbers[i + 1].to_string());
                result = concat_str.parse().unwrap();
            } else {
                match current_ops[i] {
                    '+' => result += numbers[i + 1],
                    '*' => result *= numbers[i + 1],
                    _ => unreachable!(),
                }
            }
            i += 1;
        }
        return result == target;
    }

    for &op in operators.iter() {
        current_ops.push(op);
        if try_combinations(numbers, current_ops, operators, target) {
            return true;
        }
        current_ops.pop();
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(Day07.part1(input), "3749");
    }

    #[test]
    fn test_part2_sample() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(Day07.part2(input), "11387");
    }
}
