use crate::Solution;

#[derive(Debug)]
struct ClawMachine {
    button_a: (i64, i64),  // (X, Y) movement for button A
    button_b: (i64, i64),  // (X, Y) movement for button B
    prize: (i64, i64),     // (X, Y) location of prize
}

impl ClawMachine {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        
        let a_line = lines.next().unwrap();
        let b_line = lines.next().unwrap();
        let prize_line = lines.next().unwrap();

        let parse_coords = |s: &str| {
            let parts: Vec<&str> = s.split(", ").collect();
            let x = parts[0].split(|c| c == '+' || c == '=').last().unwrap()
                .trim_start_matches('X')
                .parse().unwrap();
            let y = parts[1].split(|c| c == '+' || c == '=').last().unwrap()
                .trim_start_matches('Y')
                .parse().unwrap();
            (x, y)
        };

        ClawMachine {
            button_a: parse_coords(a_line),
            button_b: parse_coords(b_line),
            prize: parse_coords(prize_line),
        }
    }

    fn can_reach_prize(&self) -> Option<u32> {
        println!("\nAnalyzing machine:");
        println!("Button A moves: X{:+}, Y{:+}", self.button_a.0, self.button_a.1);
        println!("Button B moves: X{:+}, Y{:+}", self.button_b.0, self.button_b.1);
        println!("Prize at: X={}, Y={}", self.prize.0, self.prize.1);
        
        let mut min_tokens = None;
        let mut solutions_found = 0;
        
        for a in 0..=100 {
            for b in 0..=100 {
                let x = a as i64 * self.button_a.0 + b as i64 * self.button_b.0;
                let y = a as i64 * self.button_a.1 + b as i64 * self.button_b.1;
                
                if x == self.prize.0 && y == self.prize.1 {
                    let tokens = (3 * a + b) as u32;
                    solutions_found += 1;
                    println!("Solution found! Press A {} times and B {} times for {} tokens", a, b, tokens);
                    min_tokens = Some(min_tokens.map_or(tokens, |current: u32| current.min(tokens)));
                }
            }
        }
        
        if let Some(min) = min_tokens {
            println!("Found {} solutions. Minimum tokens needed: {}", solutions_found, min);
        } else {
            println!("No solution found - prize cannot be reached!");
        }
        
        min_tokens
    }
}

pub struct Day13;

impl Solution for Day13 {
    fn part1(&self, input: &str) -> String {
        let machines: Vec<ClawMachine> = input.split("\r\n\r\n")
            .flat_map(|s| s.split("\n\n"))
            .map(ClawMachine::parse)
            .collect();

        let total_tokens: u32 = machines.iter()
            .filter_map(|m| m.can_reach_prize())
            .sum();

        total_tokens.to_string()
    }

    fn part2(&self, _input: &str) -> String {
        // TODO: Implement solution
        "Not implemented".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part1_sample() {
        let input = "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400\n\nButton A: X+26, Y+66\nButton B: X+67, Y+21\nPrize: X=12748, Y=12176\n\nButton A: X+17, Y+86\nButton B: X+84, Y+37\nPrize: X=7870, Y=6450\n\nButton A: X+69, Y+23\nButton B: X+27, Y+71\nPrize: X=18641, Y=10279";
        assert_eq!(Day13.part1(&input), "480");
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input(13, true);
        assert_eq!(Day13.part2(&input), "Not implemented");
    }
}
