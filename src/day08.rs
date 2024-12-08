use crate::Solution;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub struct Day08;

impl Day08 {
    fn print_debug_map(
        antennas: &HashMap<char, Vec<Point>>,
        antinodes: &HashSet<Point>,
        max_x: usize,
        max_y: usize,
    ) {
        println!("\nMap with antinodes (#) and original antennas:");
        for y in 0..=max_y {
            for x in 0..=max_x {
                let point = Point::new(x as i32, y as i32);
                if antinodes.contains(&point) {
                    print!("#");
                } else {
                    let mut found = false;
                    for (ch, positions) in antennas {
                        if positions.contains(&point) {
                            print!("{}", ch);
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        print!(".");
                    }
                }
            }
            println!();
        }
        println!();
    }

    fn solve(&self, input: &str, is_part2: bool, debug: bool) -> String {
        let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;

        // Parse input and collect antennas by frequency
        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != '.' {
                    antennas
                        .entry(ch)
                        .or_default()
                        .push(Point::new(x as i32, y as i32));
                }
                max_x = max_x.max(x);
            }
            max_y = max_y.max(y);
        }

        let mut antinodes = HashSet::new();

        if is_part2 {
            // Add antinodes at antenna positions (if not the only one of its frequency)
            for (_freq, positions) in antennas.iter() {
                if positions.len() > 1 {
                    for pos in positions {
                        antinodes.insert(*pos);
                    }
                }
            }
        }

        // For each frequency group
        for (_freq, positions) in antennas.iter() {
            // Skip if there's only one antenna of this frequency
            if positions.len() <= 1 {
                continue;
            }

            // Check all pairs of antennas with the same frequency
            for i in 0..positions.len() {
                for j in (i + 1)..positions.len() {
                    let a1 = positions[i];
                    let a2 = positions[j];

                    // Calculate the difference vector
                    let dx = a2.x - a1.x;
                    let dy = a2.y - a1.y;

                    if is_part2 {
                        // Part 2: Try different values of k
                        for k in -100..=100 {
                            let antinode = Point::new(a1.x + k * dx, a1.y + k * dy);

                            if antinode.x >= 0
                                && antinode.x <= max_x as i32
                                && antinode.y >= 0
                                && antinode.y <= max_y as i32
                            {
                                antinodes.insert(antinode);
                            }
                        }
                    } else {
                        // Part 1: Only check the two reflection points
                        let antinode1 = Point::new(a1.x - dx, a1.y - dy);

                        let antinode2 = Point::new(a2.x + dx, a2.y + dy);

                        if antinode1.x >= 0
                            && antinode1.x <= max_x as i32
                            && antinode1.y >= 0
                            && antinode1.y <= max_y as i32
                            && antinode1 != a1
                            && antinode1 != a2
                        {
                            antinodes.insert(antinode1);
                        }

                        if antinode2.x >= 0
                            && antinode2.x <= max_x as i32
                            && antinode2.y >= 0
                            && antinode2.y <= max_y as i32
                            && antinode2 != a1
                            && antinode2 != a2
                        {
                            antinodes.insert(antinode2);
                        }
                    }
                }
            }
        }

        if debug {
            Self::print_debug_map(&antennas, &antinodes, max_x, max_y);
        }

        antinodes.len().to_string()
    }
}

impl Solution for Day08 {
    fn part1(&self, input: &str) -> String {
        self.solve(input, false, false)
    }

    fn part2(&self, input: &str) -> String {
        self.solve(input, true, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part1_sample() {
        let input = read_input(8, true);
        let day = Day08;
        assert_eq!(day.solve(&input, false, true), "14");
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input(8, true);
        let day = Day08;
        assert_eq!(day.solve(&input, true, true), "34");
    }
}
