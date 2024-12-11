use crate::Solution;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_forward(&self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0, pos.1 + 1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
        }
    }
}

pub struct Day06;

impl Day06 {
    fn parse_map(input: &str) -> (Vec<Vec<char>>, (usize, usize, Direction)) {
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut start_pos = None;

        for (i, row) in map.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == '^' {
                    start_pos = Some((i, j, Direction::Up));
                }
            }
        }

        let start = start_pos.expect("No starting position found");
        (map, start)
    }

    fn simulate_guard(&self, input: &str) -> usize {
        let (map, (start_row, start_col, start_dir)) = Self::parse_map(input);
        let rows = map.len() as i32;
        let cols = map[0].len() as i32;
        let mut visited = HashSet::new();

        let mut pos = (start_row as i32, start_col as i32);
        let mut dir = start_dir;
        visited.insert(pos);

        loop {
            let next_pos = dir.move_forward(pos);

            // Check if guard is out of bounds
            if next_pos.0 < 0 || next_pos.0 >= rows || next_pos.1 < 0 || next_pos.1 >= cols {
                break;
            }

            // Check if there's an obstacle ahead
            if map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                dir = dir.turn_right();
            } else {
                pos = next_pos;
                visited.insert(pos);
            }
        }

        visited.len()
    }

    fn simulate_with_obstruction(
        &self,
        map: &[Vec<char>],
        start: (usize, usize, Direction),
    ) -> bool {
        let rows = map.len() as i32;
        let cols = map[0].len() as i32;
        let mut visited = HashMap::new();

        let mut pos = (start.0 as i32, start.1 as i32);
        let mut dir = start.2;
        visited.insert((pos, dir), 0);
        let mut steps = 0;

        loop {
            let next_pos = dir.move_forward(pos);
            steps += 1;

            // Check if guard is out of bounds
            if next_pos.0 < 0 || next_pos.0 >= rows || next_pos.1 < 0 || next_pos.1 >= cols {
                return false;
            }

            // Check if there's an obstacle ahead
            if map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                dir = dir.turn_right();
            } else {
                pos = next_pos;
            }

            // Check if we've been in this position and direction before (loop detected)
            if let Some(prev_steps) = visited.get(&(pos, dir)) {
                // Only count it as a loop if we've taken some steps
                return steps - prev_steps > 1;
            }
            visited.insert((pos, dir), steps);
        }
    }

    fn find_loop_positions(&self, input: &str) -> usize {
        let (map, start) = Self::parse_map(input);
        let mut count = 0;

        // Try placing an obstruction at each empty position
        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j] == '.' && (i != start.0 || j != start.1) {
                    // Place obstruction
                    let mut map_clone = map.clone();
                    map_clone[i][j] = '#';

                    // Simulate guard movement
                    if self.simulate_with_obstruction(&map_clone, start) {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

impl Solution for Day06 {
    fn part1(&self, input: &str) -> String {
        self.simulate_guard(input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        self.find_loop_positions(input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(Day06.part1(input), "41");
    }

    #[test]
    fn test_part2_sample() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(Day06.part2(input), "6");
    }
}
