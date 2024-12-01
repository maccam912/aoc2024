use crate::Solution;

pub struct Day12;

impl Solution for Day12 {
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
        let input = read_input(12, true);
        assert_eq!(Day12.part1(&input), "Not implemented");
    }

    #[test]
    fn test_part1_real() {
        let input = read_input(12, false);
        assert_eq!(Day12.part1(&input), "Not implemented");
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input(12, true);
        assert_eq!(Day12.part2(&input), "Not implemented");
    }

    #[test]
    fn test_part2_real() {
        let input = read_input(12, false);
        assert_eq!(Day12.part2(&input), "Not implemented");
    }
}
