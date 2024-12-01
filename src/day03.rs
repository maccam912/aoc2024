use crate::Solution;

pub struct Day03;

impl Solution for Day03 {
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
        let input = read_input(03, true);
        assert_eq!(Day03.part1(&input), "Not implemented");
    }

    #[test]
    fn test_part1_real() {
        let input = read_input(03, false);
        assert_eq!(Day03.part1(&input), "Not implemented");
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input(03, true);
        assert_eq!(Day03.part2(&input), "Not implemented");
    }

    #[test]
    fn test_part2_real() {
        let input = read_input(03, false);
        assert_eq!(Day03.part2(&input), "Not implemented");
    }
}
