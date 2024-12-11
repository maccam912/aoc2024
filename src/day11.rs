use crate::Solution;

#[derive(Debug, Clone)]
struct Stone {
    number: u64,
}

impl Stone {
    fn blink(&self) -> Vec<Stone> {
        // Rule 1: If stone is 0, replace with 1
        if self.number == 0 {
            return vec![Stone { number: 1 }];
        }

        // Rule 2: If number has even number of digits, split into two stones
        let digit_count = self.number.to_string().len();
        if digit_count % 2 == 0 {
            let num_str = self.number.to_string();
            let mid = digit_count / 2;
            let left = num_str[..mid].parse::<u64>().unwrap();
            let right = num_str[mid..].parse::<u64>().unwrap();
            return vec![Stone { number: left }, Stone { number: right }];
        }

        // Rule 3: Multiply by 2024
        vec![Stone {
            number: self.number * 2024,
        }]
    }
}

pub struct Day11;

impl Day11 {
    fn solve(&self, input: &str, blinks: usize) -> String {
        let mut stones: Vec<Stone> = input
            .split_whitespace()
            .map(|s| Stone {
                number: s.parse().unwrap(),
            })
            .collect();

        // Simulate blinks
        for _ in 0..blinks {
            stones = stones
                .into_iter()
                .flat_map(|stone| stone.blink())
                .collect();
        }

        stones.len().to_string()
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
