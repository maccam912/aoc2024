use crate::Solution;

pub struct Day15;

impl Solution for Day15 {
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
        let input = read_input(15, true);
        assert_eq!(Day15.part1(&input), "Not implemented");
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input(15, true);
        assert_eq!(Day15.part2(&input), "Not implemented");
    }
}
