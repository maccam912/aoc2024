use crate::Solution;

pub struct Day22;

impl Solution for Day22 {
    fn part1(&self, input: &str) -> String {
        let initial_secrets: Vec<u64> = input
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();

        let sum: u64 = initial_secrets
            .iter()
            .map(|&secret| generate_nth_secret(secret, 2000))
            .sum();

        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        // Part 2 not implemented yet
        "Not implemented".to_string()
    }
}

fn generate_nth_secret(mut secret: u64, n: usize) -> u64 {
    for _ in 0..n {
        // Step 1: Multiply by 64, mix, and prune
        let result = secret * 64;
        secret ^= result;
        secret %= 16777216;

        // Step 2: Divide by 32, mix, and prune
        let result = secret / 32;
        secret ^= result;
        secret %= 16777216;

        // Step 3: Multiply by 2048, mix, and prune
        let result = secret * 2048;
        secret ^= result;
        secret %= 16777216;
    }
    secret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part1_sample() {
        let input = "1\n10\n100\n2024";
        assert_eq!(Day22.part1(input), "37327623");
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input(22, true);
        assert_eq!(Day22.part2(&input), "Not implemented");
    }
}
