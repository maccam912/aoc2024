use crate::Solution;

pub struct Day04;

impl Solution for Day04 {
    fn part1(&self, input: &str) -> String {
        // TODO: Implement solution
        "Not implemented".to_string()
    }

    fn part2(&self, input: &str) -> String {
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
        let input = read_input(04, true);
        assert_eq!(Day04.part1(&input), "Not implemented");
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input(04, true);
        assert_eq!(Day04.part2(&input), "Not implemented");
    }
}
