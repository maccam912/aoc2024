use crate::Solution;

pub struct Day02;

impl Solution for Day02 {
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
        let input = read_input(02, true);
        assert_eq!(Day02.part1(&input), "Not implemented");
    }

    #[test]
    fn test_part1_real() {
        let input = read_input(02, false);
        assert_eq!(Day02.part1(&input), "Not implemented");
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input(02, true);
        assert_eq!(Day02.part2(&input), "Not implemented");
    }

    #[test]
    fn test_part2_real() {
        let input = read_input(02, false);
        assert_eq!(Day02.part2(&input), "Not implemented");
    }
}
