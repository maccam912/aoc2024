use crate::Solution;
use std::collections::{HashSet, VecDeque};

static DEBUG: bool = false;

pub struct Day12;

impl Day12 {
    fn find_regions(&self, grid: &[Vec<char>]) -> Vec<(char, HashSet<(usize, usize)>)> {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut visited = HashSet::new();
        let mut regions = Vec::new();

        // Helper function for BFS
        fn bfs(
            grid: &[Vec<char>],
            start: (usize, usize),
            visited: &mut HashSet<(usize, usize)>,
        ) -> HashSet<(usize, usize)> {
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
                    if DEBUG {
                        println!("\nFound region of type '{}' at ({}, {})", grid[r][c], r, c);
                        println!("Region coordinates: {:?}", region);
                    }
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
        if DEBUG {
            println!("Region perimeter: {}", perimeter);
        }
        perimeter
    }

    fn count_holes(&self, region: &HashSet<(usize, usize)>) -> usize {
        if region.is_empty() {
            return 0;
        }

        // Find bounds of the region
        let min_r = region.iter().map(|&(r, _)| r).min().unwrap();
        let max_r = region.iter().map(|&(r, _)| r).max().unwrap();
        let min_c = region.iter().map(|&(_, c)| c).min().unwrap();
        let max_c = region.iter().map(|&(_, c)| c).max().unwrap();

        if DEBUG {
            println!("\nChecking for holes in region:");
            for r in min_r..=max_r {
                for c in min_c..=max_c {
                    print!("{}", if region.contains(&(r, c)) { "█" } else { "." });
                }
                println!();
            }
        }

        // Create a set of all empty spaces
        let empty_spaces: HashSet<(usize, usize)> = (min_r..=max_r)
            .flat_map(|r| (min_c..=max_c).map(move |c| (r, c)))
            .filter(|pos| !region.contains(pos))
            .collect();

        // Start flood fill from the edges
        let mut queue = VecDeque::new();

        // Add all empty spaces on the edges to the queue
        for r in min_r..=max_r {
            if !region.contains(&(r, min_c)) {
                queue.push_back((r, min_c));
            }
            if !region.contains(&(r, max_c)) {
                queue.push_back((r, max_c));
            }
        }
        for c in min_c..=max_c {
            if !region.contains(&(min_r, c)) {
                queue.push_back((min_r, c));
            }
            if !region.contains(&(max_r, c)) {
                queue.push_back((max_r, c));
            }
        }

        // Mark all reachable empty spaces
        let mut reachable = HashSet::new();
        while let Some((r, c)) = queue.pop_front() {
            if !reachable.insert((r, c)) {
                continue;
            }

            // Check all 8 directions (including diagonals)
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }

                    let new_r = r as i32 + dr;
                    let new_c = c as i32 + dc;

                    if new_r >= min_r as i32
                        && new_r <= max_r as i32
                        && new_c >= min_c as i32
                        && new_c <= max_c as i32
                    {
                        let new_pos = (new_r as usize, new_c as usize);
                        if empty_spaces.contains(&new_pos) && !reachable.contains(&new_pos) {
                            queue.push_back(new_pos);
                        }
                    }
                }
            }
        }

        // Get unreachable spaces (potential holes)
        let unreachable: HashSet<_> = empty_spaces.difference(&reachable).cloned().collect();

        // Count connected groups of unreachable spaces - each group is one hole
        let mut hole_count = 0;
        let mut unprocessed = unreachable.clone();

        while !unprocessed.is_empty() {
            // Start a new hole group
            let start = *unprocessed.iter().next().unwrap();
            let mut hole_group = VecDeque::new();
            hole_group.push_back(start);
            unprocessed.remove(&start);

            // Flood fill to find all connected spaces in this hole
            while let Some((r, c)) = hole_group.pop_front() {
                // Check all 8 directions
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }

                        let new_r = r as i32 + dr;
                        let new_c = c as i32 + dc;

                        if new_r >= min_r as i32
                            && new_r <= max_r as i32
                            && new_c >= min_c as i32
                            && new_c <= max_c as i32
                        {
                            let new_pos = (new_r as usize, new_c as usize);
                            if unprocessed.contains(&new_pos) {
                                hole_group.push_back(new_pos);
                                unprocessed.remove(&new_pos);
                            }
                        }
                    }
                }
            }

            hole_count += 1;
        }

        if DEBUG {
            println!("Total holes found: {}", hole_count);
            // Print the region with holes marked
            let mut hole_groups = vec![vec!['.'; max_c - min_c + 1]; max_r - min_r + 1];
            for r in min_r..=max_r {
                for c in min_c..=max_c {
                    let pos = (r, c);
                    if region.contains(&pos) {
                        hole_groups[r - min_r][c - min_c] = '█';
                    } else if unreachable.contains(&pos) {
                        hole_groups[r - min_r][c - min_c] = 'O';
                    }
                }
            }
            for row in hole_groups {
                println!("{}", row.iter().collect::<String>());
            }
        }

        hole_count
    }

    fn calculate_sides(&self, region: &HashSet<(usize, usize)>) -> usize {
        // If this is a single cell region, just return 4 sides
        if region.len() == 1 {
            if DEBUG {
                println!("Single cell region - 4 sides");
            }
            return 4;
        }

        // Find bounds of the region
        let min_r = region.iter().map(|&(r, _)| r).min().unwrap();
        let max_r = region.iter().map(|&(r, _)| r).max().unwrap();
        let min_c = region.iter().map(|&(_, c)| c).min().unwrap();
        let max_c = region.iter().map(|&(_, c)| c).max().unwrap();

        let mut inside_corners = 0;

        if DEBUG {
            println!("\nChecking for inside corners in region:");
            for r in min_r..=max_r {
                for c in min_c..=max_c {
                    print!("{}", if region.contains(&(r, c)) { "█" } else { "." });
                }
                println!();
            }
        }

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
                    if DEBUG {
                        println!("Found inside corner at ({}, {})", r, c);
                        println!("2x2 region:");
                        for dr in 0..2 {
                            for dc in 0..2 {
                                print!(
                                    "{}",
                                    if region.contains(&(r + dr, c + dc)) {
                                        "O"
                                    } else {
                                        "."
                                    }
                                );
                            }
                            println!();
                        }
                        println!();
                    }
                    inside_corners += 1;
                }
            }
        }

        // Count holes and subtract 4 for each one
        let holes = self.count_holes(region);

        // Base 4 sides plus 2 for each inside corner, minus 4 for each hole
        let result = 4 + (inside_corners * 2) - (holes * 4);

        if DEBUG {
            println!("Final calculation:");
            println!("  Base sides: 4");
            println!(
                "  Inside corners: {} (+{})",
                inside_corners,
                inside_corners * 2
            );
            println!("  Holes: {} (-{})", holes, holes * 4);
            println!("  Total sides: {}", result);
        }

        result
    }

    fn part1(&self, input: &str) -> String {
        // Parse input into grid
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        // Find all regions
        let regions = self.find_regions(&grid);

        // Calculate total price
        let total_price: usize = regions
            .iter()
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
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        // Find all regions
        let regions = self.find_regions(&grid);

        // Calculate total price using sides instead of perimeter
        let total_price: usize = regions
            .iter()
            .map(|(_, region)| {
                let area = region.len();
                let sides = self.calculate_sides(region);
                area * sides
            })
            .sum();

        total_price.to_string()
    }
}

impl Solution for Day12 {
    fn part1(&self, input: &str) -> String {
        self.part1(input)
    }

    fn part2(&self, input: &str) -> String {
        self.part2(input)
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

    #[test]
    fn test_part2_sample4() {
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
        assert_eq!(Day12.part2(input), "1206");
    }

    #[test]
    fn test_part2_sample5() {
        let input = "\
XOO
OXO
OOO";
        // O is one region, 10 sides
        // each X is 4 sides
        // should be 18 sides
        assert_eq!(Day12.part2(input), "78");
    }
    #[test]
    fn test_part2_sample6() {
        let input = "\
OOOO
OXXO
OXXO
OOOO";
        // O is one region, 10 sides
        // each X is 4 sides
        // should be 18 sides
        assert_eq!(Day12.part2(input), "112");
    }
    #[test]
    fn test_part2_sample7() {
        let input = "\
XOOO
OXOO
OOXO
OOOO";
        // O is one region, 10 sides
        // each X is 4 sides
        // should be 18 sides
        assert_eq!(Day12.part2(input), "194");
    }
}
