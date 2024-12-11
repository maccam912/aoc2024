use crate::Solution;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone)]
struct Grid {
    heights: Vec<Vec<u32>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn from_string(input: &str) -> Self {
        let heights: Vec<Vec<u32>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let rows = heights.len();
        let cols = heights[0].len();
        Grid {
            heights,
            rows,
            cols,
        }
    }

    fn get_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let current_height = self.heights[row][col];
        let target_height = current_height + 1;

        // Check all four directions
        if row > 0 && self.heights[row - 1][col] == target_height {
            neighbors.push((row - 1, col));
        }
        if row + 1 < self.rows && self.heights[row + 1][col] == target_height {
            neighbors.push((row + 1, col));
        }
        if col > 0 && self.heights[row][col - 1] == target_height {
            neighbors.push((row, col - 1));
        }
        if col + 1 < self.cols && self.heights[row][col + 1] == target_height {
            neighbors.push((row, col + 1));
        }

        neighbors
    }

    fn find_trailhead_score(&self, start_row: usize, start_col: usize) -> usize {
        if self.heights[start_row][start_col] != 0 {
            return 0;
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut reachable_nines = HashSet::new();

        queue.push_back((start_row, start_col));
        visited.insert((start_row, start_col));

        while let Some((row, col)) = queue.pop_front() {
            if self.heights[row][col] == 9 {
                reachable_nines.insert((row, col));
                continue;
            }

            for (next_row, next_col) in self.get_neighbors(row, col) {
                if !visited.contains(&(next_row, next_col)) {
                    visited.insert((next_row, next_col));
                    queue.push_back((next_row, next_col));
                }
            }
        }

        reachable_nines.len()
    }

    fn find_distinct_paths(&self, start_row: usize, start_col: usize) -> usize {
        if self.heights[start_row][start_col] != 0 {
            return 0;
        }

        let mut paths = HashSet::new();
        let mut current_path = Vec::new();
        current_path.push((start_row, start_col));

        fn dfs(
            grid: &Grid,
            row: usize,
            col: usize,
            current_path: &mut Vec<(usize, usize)>,
            paths: &mut HashSet<Vec<(usize, usize)>>,
        ) {
            if grid.heights[row][col] == 9 {
                paths.insert(current_path.clone());
                return;
            }

            for (next_row, next_col) in grid.get_neighbors(row, col) {
                if !current_path.contains(&(next_row, next_col)) {
                    current_path.push((next_row, next_col));
                    dfs(grid, next_row, next_col, current_path, paths);
                    current_path.pop();
                }
            }
        }

        dfs(self, start_row, start_col, &mut current_path, &mut paths);
        paths.len()
    }
}

pub struct Day10;

impl Solution for Day10 {
    fn part1(&self, input: &str) -> String {
        let grid = Grid::from_string(input);
        let mut total_score = 0;

        for row in 0..grid.rows {
            for col in 0..grid.cols {
                if grid.heights[row][col] == 0 {
                    total_score += grid.find_trailhead_score(row, col);
                }
            }
        }

        total_score.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let grid = Grid::from_string(input);
        let mut total_rating = 0;

        for row in 0..grid.rows {
            for col in 0..grid.cols {
                if grid.heights[row][col] == 0 {
                    total_rating += grid.find_distinct_paths(row, col);
                }
            }
        }

        total_rating.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part1_sample() {
        let input =
            "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";
        assert_eq!(Day10.part1(input), "36");
    }

    #[test]
    fn test_part2_sample() {
        let input =
            "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";
        assert_eq!(Day10.part2(input), "81");
    }
}
