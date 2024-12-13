use crate::Solution;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day12;

impl Day12 {
    fn find_regions(&self, grid: &Vec<Vec<char>>) -> Vec<(char, HashSet<(usize, usize)>)> {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut visited = HashSet::new();
        let mut regions = Vec::new();

        // Helper function for BFS
        fn bfs(grid: &Vec<Vec<char>>, start: (usize, usize), visited: &mut HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
            let rows = grid.len();
            let cols = grid[0].len();
            let plant_type = grid[start.0][start.1];
            let mut region = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(start);
            visited.insert(start);
            region.insert(start);

            while let Some((r, c)) = queue.pop_front() {
                // Check all 4 directions
                for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let new_r = r as i32 + dr;
                    let new_c = c as i32 + dc;
                    
                    if new_r >= 0 && new_r < rows as i32 && new_c >= 0 && new_c < cols as i32 {
                        let new_r = new_r as usize;
                        let new_c = new_c as usize;
                        if !visited.contains(&(new_r, new_c)) && grid[new_r][new_c] == plant_type {
                            queue.push_back((new_r, new_c));
                            visited.insert((new_r, new_c));
                            region.insert((new_r, new_c));
                        }
                    }
                }
            }
            region
        }

        // Find all regions
        for r in 0..rows {
            for c in 0..cols {
                if !visited.contains(&(r, c)) {
                    let region = bfs(grid, (r, c), &mut visited);
                    regions.push((grid[r][c], region));
                }
            }
        }

        regions
    }

    fn calculate_perimeter(&self, region: &HashSet<(usize, usize)>) -> usize {
        let mut perimeter = 0;
        for &(r, c) in region {
            // Check all 4 sides
            for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_r = r as i32 + dr;
                let new_c = c as i32 + dc;
                // If adjacent cell is not in region (either out of bounds or different type), add to perimeter
                if !region.contains(&(new_r as usize, new_c as usize)) {
                    perimeter += 1;
                }
            }
        }
        perimeter
    }

    fn visualize_region(&self, region: &HashSet<(usize, usize)>, plant_type: char) {
        if region.is_empty() {
            return;
        }

        // Find bounds of the region
        let min_r = region.iter().map(|&(r, _)| r).min().unwrap();
        let max_r = region.iter().map(|&(r, _)| r).max().unwrap();
        let min_c = region.iter().map(|&(_, c)| c).min().unwrap();
        let max_c = region.iter().map(|&(_, c)| c).max().unwrap();

        println!("\nRegion of type '{}' (size: {}, perimeter: {}):", 
            plant_type, region.len(), self.calculate_perimeter(region));
        
        // Draw the region
        for r in min_r..=max_r {
            for c in min_c..=max_c {
                if region.contains(&(r, c)) {
                    print!("{}", plant_type);
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

impl Solution for Day12 {
    fn part1(&self, input: &str) -> String {
        // Parse input into grid
        let grid: Vec<Vec<char>> = input.lines()
            .map(|line| line.chars().collect())
            .collect();

        // Find all regions
        let regions = self.find_regions(&grid);

        // Debug: Visualize each region
        for (plant_type, region) in regions.iter() {
            self.visualize_region(region, *plant_type);
        }

        // Calculate total price
        let total_price: usize = regions.iter()
            .map(|(_, region)| {
                let area = region.len();
                let perimeter = self.calculate_perimeter(region);
                area * perimeter
            })
            .sum();

        total_price.to_string()
    }

    fn part2(&self, _input: &str) -> String {
        "Not implemented".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = "\
AAAA
BBCD
BBCC
EEEC";
        assert_eq!(Day12.part1(input), "140");
    }

    #[test]
    fn test_part1_sample2() {
        let input = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(Day12.part1(input), "772");
    }

    #[test]
    fn test_part1_sample3() {
        let input = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(Day12.part1(input), "1930");
    }
}
