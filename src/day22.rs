use crate::Solution;

pub struct Day22;

impl Solution for Day22 {
    fn part1(&self, input: &str) -> String {
        let initial_secrets: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();

        let sum: u64 = initial_secrets
            .iter()
            .map(|&secret| generate_nth_secret(secret, 2000))
            .sum();

        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let initial_secrets: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();

        // Generate all price sequences for each buyer
        let all_prices: Vec<Vec<i32>> = initial_secrets
            .iter()
            .map(|&secret| generate_price_sequence(secret))
            .collect();

        // Try all possible sequences of 4 changes (-9 to 9 for each position)
        let mut max_bananas = 0;
        let mut best_sequence = vec![0; 4];

        // Calculate total combinations for progress tracking
        let total_combinations = 19 * 19 * 19 * 19; // (-9 to 9 = 19 possibilities per position)
        let mut combinations_tried = 0;
        let progress_interval = total_combinations / 100; // Update every 1%

        for a in -9..=9 {
            for b in -9..=9 {
                for c in -9..=9 {
                    for d in -9..=9 {
                        let sequence = vec![a, b, c, d];
                        let total = calculate_total_bananas(&all_prices, &sequence);
                        if total > max_bananas {
                            max_bananas = total;
                            best_sequence = sequence.clone();
                            println!(
                                "New best sequence {:?} gives {} bananas",
                                best_sequence, max_bananas
                            );
                        }

                        combinations_tried += 1;
                        if combinations_tried % progress_interval == 0 {
                            println!(
                                "Progress: {:.1}% ({}/{} combinations)",
                                (combinations_tried as f64 / total_combinations as f64) * 100.0,
                                combinations_tried,
                                total_combinations
                            );
                        }
                    }
                }
            }
        }

        println!(
            "Final best sequence {:?} gives {} bananas",
            best_sequence, max_bananas
        );
        max_bananas.to_string()
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

fn generate_price_sequence(initial: u64) -> Vec<i32> {
    let mut prices = Vec::with_capacity(2001);
    let mut secret = initial;

    // Get initial price (ones digit)
    prices.push((secret % 10) as i32);

    // Generate 2000 more prices
    for _ in 0..2000 {
        // Generate next secret
        let result = secret * 64;
        secret ^= result;
        secret %= 16777216;

        let result = secret / 32;
        secret ^= result;
        secret %= 16777216;

        let result = secret * 2048;
        secret ^= result;
        secret %= 16777216;

        // Get ones digit as price
        prices.push((secret % 10) as i32);
    }
    prices
}

fn calculate_total_bananas(all_prices: &[Vec<i32>], sequence: &[i32]) -> i32 {
    let mut total = 0;

    // For each buyer
    for prices in all_prices {
        let mut found = false;

        // Look for the sequence in price changes
        for i in 0..prices.len().saturating_sub(4) {
            let mut matches = true;
            for j in 0..4 {
                let change = prices[i + j + 1] - prices[i + j];
                if change != sequence[j] {
                    matches = false;
                    break;
                }
            }
            if matches {
                total += prices[i + 4]; // Add price at the time sequence is found
                found = true;
                break;
            }
        }
    }
    total
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
        let input = "1\n2\n3\n2024";
        assert_eq!(Day22.part2(input), "23");
    }
}
