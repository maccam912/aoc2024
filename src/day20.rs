use crate::Solution;
use std::collections::{HashSet, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use rayon::prelude::*;

pub struct Day20;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: i32,
    col: i32,
}

impl Pos {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    fn neighbors(&self) -> Vec<Pos> {
        vec![
            Pos::new(self.row - 1, self.col),
            Pos::new(self.row + 1, self.col),
            Pos::new(self.row, self.col - 1),
            Pos::new(self.row, self.col + 1),
        ]
    }
}

impl Day20 {
    fn parse_input(input: &str) -> (Vec<Vec<char>>, Pos, Pos) {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut start = Pos::new(0, 0);
        let mut end = Pos::new(0, 0);

        for (row, line) in grid.iter().enumerate() {
            for (col, &ch) in line.iter().enumerate() {
                if ch == 'S' {
                    start = Pos::new(row as i32, col as i32);
                } else if ch == 'E' {
                    end = Pos::new(row as i32, col as i32);
                }
            }
        }

        (grid, start, end)
    }

    fn find_normal_path(grid: &[Vec<char>], start: Pos, end: Pos) -> (i32, HashSet<Pos>) {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut path = HashSet::new();

        queue.push_back((start, 0, vec![start]));
        visited.insert(start);

        while let Some((pos, steps, current_path)) = queue.pop_front() {
            if pos == end {
                path.extend(current_path);
                return (steps, path);
            }

            for next_pos in pos.neighbors() {
                if next_pos.row < 0
                    || next_pos.row >= grid.len() as i32
                    || next_pos.col < 0
                    || next_pos.col >= grid[0].len() as i32
                {
                    continue;
                }

                let cell = grid[next_pos.row as usize][next_pos.col as usize];
                if cell != '#' && visited.insert(next_pos) {
                    let mut new_path = current_path.clone();
                    new_path.push(next_pos);
                    queue.push_back((next_pos, steps + 1, new_path));
                }
            }
        }

        panic!("No path found!");
    }

    fn find_shortcuts(grid: &[Vec<char>], path: &HashSet<Pos>) -> Vec<i32> {
        let mut shortcuts = Vec::new();
        let mut wall_tiles = HashSet::new();

        // Find all wall tiles adjacent to path tiles
        for &pos in path {
            for neighbor in pos.neighbors() {
                if neighbor.row >= 0
                    && neighbor.row < grid.len() as i32
                    && neighbor.col >= 0
                    && neighbor.col < grid[0].len() as i32
                {
                    let cell = grid[neighbor.row as usize][neighbor.col as usize];
                    if cell == '#' {
                        wall_tiles.insert(neighbor);
                    }
                }
            }
        }

        // For each wall tile, check all pairs of path neighbors
        for wall in wall_tiles {
            let mut path_neighbors = Vec::new();
            for neighbor in wall.neighbors() {
                if neighbor.row >= 0
                    && neighbor.row < grid.len() as i32
                    && neighbor.col >= 0
                    && neighbor.col < grid[0].len() as i32
                {
                    if path.contains(&neighbor) {
                        path_neighbors.push(neighbor);
                    }
                }
            }

            // Check all pairs of neighbors
            for i in 0..path_neighbors.len() {
                for j in i + 1..path_neighbors.len() {
                    let start = path_neighbors[i];
                    let end = path_neighbors[j];

                    // Calculate time saved: normal path between neighbors - 2
                    let normal_time = Self::find_normal_path(grid, start, end).0;
                    let shortcut_time = 2; // Always takes 2 steps through the wall
                    let saved = normal_time - shortcut_time;
                    if saved > 0 {
                        shortcuts.push(saved);
                    }
                }
            }
        }

        shortcuts
    }

    fn find_long_shortcuts(grid: &[Vec<char>], path: &HashSet<Pos>, max_shortcut_length: i32) -> Vec<i32> {
        let path_points: Vec<_> = path.iter().collect();
        let total_pairs = (path_points.len() * (path_points.len() - 1)) / 2;
        let pairs_checked = Arc::new(AtomicUsize::new(0));
        let progress_interval = total_pairs / 100; // Show progress every 1%
        println!("Total point pairs to check: {}", total_pairs);

        // Create all pairs of indices
        let pairs: Vec<_> = (0..path_points.len())
            .flat_map(|i| ((i + 1)..path_points.len()).map(move |j| (i, j)))
            .collect();

        // Process pairs in parallel
        let shortcuts: Vec<_> = pairs.par_iter()
            .filter_map(|&(i, j)| {
                let pairs_checked_ref = Arc::clone(&pairs_checked);
                let current = pairs_checked_ref.fetch_add(1, Ordering::Relaxed);
                if progress_interval > 0 && current % progress_interval == 0 {
                    println!("Progress: {:.1}% ({}/{})", 
                        (current as f64 / total_pairs as f64) * 100.0,
                        current,
                        total_pairs
                    );
                }

                let start = *path_points[i];
                let end = *path_points[j];

                // Calculate manhattan distance
                let manhattan_dist = (end.row - start.row).abs() + (end.col - start.col).abs();
                
                // Only consider points that are within max_shortcut_length manhattan distance
                if manhattan_dist <= max_shortcut_length {
                    // Calculate normal path length between these points
                    let normal_time = Self::find_normal_path(grid, start, end).0;
                    
                    // If normal path is longer than manhattan distance, we found a shortcut
                    if normal_time > manhattan_dist {
                        let saved = normal_time - manhattan_dist;
                        Some(saved)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        println!("Finished checking all {} pairs", total_pairs);
        println!("Found {} shortcuts", shortcuts.len());
        shortcuts
    }
}

impl Solution for Day20 {
    fn part1(&self, input: &str) -> String {
        let (grid, start, end) = Self::parse_input(input);
        let (normal_time, path) = Self::find_normal_path(&grid, start, end);
        println!("Normal path length: {}", normal_time);

        let time_savings = Self::find_shortcuts(&grid, &path);

        // Count occurrences of each time saving
        let mut counts: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for &saving in &time_savings {
            *counts.entry(saving).or_insert(0) += 1;
        }

        // Print in ascending order
        let mut savings: Vec<_> = counts.iter().collect();
        savings.sort_by_key(|&(k, _)| k);

        for (saving, count) in savings {
            if *count == 1 {
                println!("There is one cheat that saves {} picoseconds.", saving);
            } else {
                println!(
                    "There are {} cheats that save {} picoseconds.",
                    count, saving
                );
            }
        }

        time_savings
            .iter()
            .filter(|&&x| x >= 100)
            .count()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (grid, start, end) = Self::parse_input(input);
        let (normal_time, path) = Self::find_normal_path(&grid, start, end);
        println!("Normal path length: {}", normal_time);

        let time_savings = Self::find_long_shortcuts(&grid, &path, 20);

        // Count occurrences of each time saving
        let mut counts: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for &saving in &time_savings {
            *counts.entry(saving).or_insert(0) += 1;
        }

        // Print in ascending order
        let mut savings: Vec<_> = counts.iter().collect();
        savings.sort_by_key(|&(k, _)| k);

        for (saving, count) in savings {
            println!(
                "There are {} cheats that save {} picoseconds.",
                count, saving
            );
        }

        time_savings
            .iter()
            .filter(|&&x| x >= 100)
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_part1_sample() {
        let day = Day20;
        let result = day.part1(SAMPLE);
        assert_eq!(result, "0");
    }

    #[test]
    fn test_part2_sample() {
        let day = Day20;
        let (grid, start, end) = Day20::parse_input(SAMPLE);
        let (normal_time, path) = Day20::find_normal_path(&grid, start, end);
        let time_savings = Day20::find_long_shortcuts(&grid, &path, 20);
        
        // Count occurrences of each time saving
        let mut counts: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for &saving in &time_savings {
            *counts.entry(saving).or_insert(0) += 1;
        }
        
        // Check specific values from the example
        assert_eq!(*counts.get(&76).unwrap_or(&0), 3); // 3 cheats that save 76 picoseconds
        assert_eq!(*counts.get(&74).unwrap_or(&0), 4); // 4 cheats that save 74 picoseconds
        assert_eq!(*counts.get(&72).unwrap_or(&0), 22); // 22 cheats that save 72 picoseconds
    }
}
