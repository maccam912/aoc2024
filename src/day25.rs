use crate::Solution;

pub struct Day25;

impl Solution for Day25 {
    fn part1(&self, _input: &str) -> String {
        // TODO: Implement solution
        "Not implemented".to_string()
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
        let input = read_input(25, true);
        assert_eq!(Day25.part1(&input), "Not implemented");
    }
    #[test]
    fn test_part2_sample() {
        let input = read_input(25, true);
        assert_eq!(Day25.part2(&input), "Not implemented");
    }
}
