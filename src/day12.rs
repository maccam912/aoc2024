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

    fn count_holes(&self, region: &HashSet<(usize, usize)>) -> usize {
        // Find bounds of the region
        let min_r = region.iter().map(|&(r, _)| r).min().unwrap();
        let max_r = region.iter().map(|&(r, _)| r).max().unwrap();
        let min_c = region.iter().map(|&(_, c)| c).min().unwrap();
        let max_c = region.iter().map(|&(_, c)| c).max().unwrap();

        let mut holes = 0;
        let mut visited = HashSet::new();

        // For each empty space
        for r in min_r..=max_r {
            for c in min_c..=max_c {
                if region.contains(&(r, c)) || visited.contains(&(r, c)) {
                    continue;
                }

                // Do a flood fill to see if this empty space is completely surrounded
                let mut is_hole = true;
                let mut empty_cells = HashSet::new();
                let mut queue = vec![(r, c)];
                visited.insert((r, c));
                empty_cells.insert((r, c));

                while let Some((cr, cc)) = queue.pop() {
                    for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                        let nr = (cr as i32 + dr) as usize;
                        let nc = (cc as i32 + dc) as usize;
                        
                        // If we hit the boundary, this isn't a hole
                        if nr < min_r || nr > max_r || nc < min_c || nc > max_c {
                            is_hole = false;
                            continue;
                        }

                        // If it's an empty cell we haven't visited, add it to the queue
                        if !region.contains(&(nr, nc)) && !visited.contains(&(nr, nc)) {
                            queue.push((nr, nc));
                            visited.insert((nr, nc));
                            empty_cells.insert((nr, nc));
                        }
                    }
                }

                if is_hole {
                    holes += 1;
                }
            }
        }

        holes
    }

    fn calculate_sides(&self, region: &HashSet<(usize, usize)>) -> usize {
        if region.len() == 1 {
            return 4; // Single cell always has 4 sides
        }

        // Find bounds of the region
        let min_r = region.iter().map(|&(r, _)| r).min().unwrap();
        let max_r = region.iter().map(|&(r, _)| r).max().unwrap();
        let min_c = region.iter().map(|&(_, c)| c).min().unwrap();
        let max_c = region.iter().map(|&(_, c)| c).max().unwrap();

        let mut inside_corners = 0;

        // Look at each potential 2x2 region
        for r in min_r..max_r {
            for c in min_c..max_c {
                // Count how many cells in this 2x2 are part of the region
                let mut count = 0;
                for (dr, dc) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
                    if region.contains(&(r + dr, c + dc)) {
                        count += 1;
                    }
                }

                // If exactly 3 cells are in the region, it's an inside corner
                if count == 3 {
                    inside_corners += 1;
                }
            }
        }

        // Count holes and subtract 2 for each one
        let holes = self.count_holes(region);
        
        // Base 4 sides plus 2 for each inside corner, minus 2 for each hole
        4 + (inside_corners * 2) - (holes * 2)
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

        let sides = self.calculate_sides(region);
        let inside_corners = (sides - 4) / 2; // Reverse calculate the number of inside corners

        println!("\nRegion of type '{}' (size: {}, perimeter: {}, sides: {}, inside corners: {})", 
            plant_type, region.len(), self.calculate_perimeter(region), sides, inside_corners);
        
        // Draw the region with corner markers
        for r in min_r..=max_r {
            for c in min_c..=max_c {
                if !region.contains(&(r, c)) {
                    print!(".");
                    continue;
                }

                // Check if this position is part of a 2x2 that forms an inside corner
                let mut is_inside_corner = false;
                for (dr, dc) in [(0, 0), (-1, -1), (-1, 0), (0, -1)] {
                    // Skip if we'd go out of bounds
                    if (r as i32 + dr) < 0 || (c as i32 + dc) < 0 {
                        continue;
                    }
                    let r2 = (r as i32 + dr) as usize;
                    let c2 = (c as i32 + dc) as usize;
                    
                    // Count cells in this 2x2
                    let mut count = 0;
                    for (dr2, dc2) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
                        if region.contains(&(r2 + dr2, c2 + dc2)) {
                            count += 1;
                        }
                    }

                    // Check if we're looking at a hole by checking surrounding cells
                    let mut surrounding_count = 0;
                    for dr2 in -1..=2 {
                        for dc2 in -1..=2 {
                            if (r2 as i32 + dr2) >= 0 && (c2 as i32 + dc2) >= 0 {
                                let check_r = (r2 as i32 + dr2) as usize;
                                let check_c = (c2 as i32 + dc2) as usize;
                                if region.contains(&(check_r, check_c)) {
                                    surrounding_count += 1;
                                }
                            }
                        }
                    }
                    
                    let is_hole = surrounding_count > 10;

                    // For normal edges: 3 cells = inside corner
                    // For holes: 1 cell = inside corner
                    if (!is_hole && count == 3 && region.contains(&(r, c))) || 
                       (is_hole && count == 1 && region.contains(&(r, c))) {
                        is_inside_corner = true;
                        break;
                    }
                }

                if is_inside_corner {
                    print!("I");
                } else {
                    print!("{}", plant_type);
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

    fn part2(&self, input: &str) -> String {
        // Parse input into grid
        let grid: Vec<Vec<char>> = input.lines()
            .map(|line| line.chars().collect())
            .collect();

        // Find all regions
        let regions = self.find_regions(&grid);

        // Debug: Visualize each region
        println!("\nPart 2 Regions:");
        for (plant_type, region) in regions.iter() {
            self.visualize_region(region, *plant_type);
        }

        // Calculate total price using sides instead of perimeter
        let total_price: usize = regions.iter()
            .map(|(_, region)| {
                let area = region.len();
                let sides = self.calculate_sides(region);
                area * sides
            })
            .sum();

        total_price.to_string()
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

    #[test]
    fn test_part2_sample() {
        let input = "\
AAAA
BBCD
BBCC
EEEC";
        assert_eq!(Day12.part2(input), "80");
    }

    #[test]
    fn test_part2_sample2() {
        let input = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!(Day12.part2(input), "236");
    }

    #[test]
    fn test_part2_sample3() {
        let input = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!(Day12.part2(input), "368");
    }
}
