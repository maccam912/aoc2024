use crate::Solution;
use std::collections::HashSet;

const DEBUG_VISUALIZATION: bool = false; // Set to true to enable visualization

#[derive(Debug, Clone)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Robot {
    fn from_str(s: &str) -> Self {
        let parts: Vec<&str> = s.split(' ').collect();
        let pos_str = &parts[0][2..];
        let vel_str = &parts[1][2..];

        let pos: Vec<i32> = pos_str.split(',').map(|n| n.parse().unwrap()).collect();
        let vel: Vec<i32> = vel_str.split(',').map(|n| n.parse().unwrap()).collect();

        Robot {
            pos: (pos[0], pos[1]),
            vel: (vel[0], vel[1]),
        }
    }

    fn update(&mut self, width: i32, height: i32) {
        self.pos.0 = (self.pos.0 + self.vel.0).rem_euclid(width);
        self.pos.1 = (self.pos.1 + self.vel.1).rem_euclid(height);
    }
}

fn flood_fill(grid: &Vec<Vec<char>>, visited: &mut HashSet<(usize, usize)>, x: usize, y: usize) {
    if visited.contains(&(x, y)) || y >= grid.len() || x >= grid[0].len() || grid[y][x] == '.' {
        return;
    }

    visited.insert((x, y));

    // Check only orthogonal directions (no diagonals)
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    for (dx, dy) in directions {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;

        if new_x >= 0 && new_y >= 0 && new_x < grid[0].len() as i32 && new_y < grid.len() as i32 {
            flood_fill(grid, visited, new_x as usize, new_y as usize);
        }
    }
}

fn analyze_grid(grid: &Vec<Vec<char>>) -> (usize, Vec<usize>, Vec<usize>) {
    let mut visited = HashSet::new();
    let mut islands = 0;

    // Count islands
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if !visited.contains(&(x, y)) && cell != '.' {
                islands += 1;
                flood_fill(grid, &mut visited, x, y);
            }
        }
    }

    // Count robots in each row and column
    let mut row_counts = vec![0; grid.len()];
    let mut col_counts = vec![0; grid[0].len()];

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != '.' {
                row_counts[y] += 1;
                col_counts[x] += 1;
            }
        }
    }

    (islands, row_counts, col_counts)
}

fn display_grid(robots: &[Robot], width: i32, height: i32) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['.'; width as usize]; height as usize];

    // Place robots on grid
    for robot in robots {
        let x = robot.pos.0 as usize;
        let y = robot.pos.1 as usize;
        if x < width as usize && y < height as usize {
            // Count robots at this position
            grid[y][x] = match grid[y][x] {
                '.' => '#',
                '#' => '2',
                n if n.is_ascii_digit() => {
                    let count = n.to_digit(10).unwrap() + 1;
                    char::from_digit(count, 10).unwrap()
                }
                _ => '9', // Cap at 9
            };
        }
    }

    if DEBUG_VISUALIZATION {
        // Display grid
        println!("\nTime step visualization:");
        for row in &grid {
            println!("{}", row.iter().collect::<String>());
        }
        println!();
    }

    grid
}

pub struct Day14;

impl Solution for Day14 {
    fn part1(&self, input: &str) -> String {
        let mut robots: Vec<Robot> = input.lines().map(Robot::from_str).collect();

        // For the sample, use smaller dimensions
        let (width, height) = if robots.len() == 12 {
            (11, 7) // Sample dimensions
        } else {
            (101, 103) // Real dimensions
        };

        // Simulate for 100 seconds
        for _step in 0..100 {
            for robot in robots.iter_mut() {
                robot.update(width, height);
            }
        }

        // Count robots in each quadrant
        let mid_x = width / 2;
        let mid_y = height / 2;

        let mut quadrants = [0; 4];
        for robot in robots.iter() {
            // Skip robots on the middle lines
            if robot.pos.0 == mid_x || robot.pos.1 == mid_y {
                continue;
            }

            let quadrant = match (robot.pos.0 < mid_x, robot.pos.1 < mid_y) {
                (true, true) => 0,   // Top-left
                (false, true) => 1,  // Top-right
                (true, false) => 2,  // Bottom-left
                (false, false) => 3, // Bottom-right
            };
            quadrants[quadrant] += 1;
        }

        // Calculate safety factor
        let safety_factor: i32 = quadrants.iter().product();
        safety_factor.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut robots: Vec<Robot> = input.lines().map(Robot::from_str).collect();

        let width = 101;
        let height = 103;

        let mut min_islands = usize::MAX;
        let mut min_islands_step = 0;
        let mut min_islands_grid = Vec::new();

        // Simulate and look for Christmas tree pattern
        for step in 0..100000 {
            // Increased to 100k steps
            for robot in robots.iter_mut() {
                robot.update(width, height);
            }

            let grid = display_grid(&robots, width, height);
            let (islands, row_counts, col_counts) = analyze_grid(&grid);

            if islands < min_islands {
                min_islands = islands;
                min_islands_step = step + 1;
                min_islands_grid = grid.clone();
                println!(
                    "New minimum islands ({}) found at step {}",
                    islands,
                    step + 1
                );

                // Print row/column statistics
                let max_row = row_counts.iter().max().unwrap();
                let max_col = col_counts.iter().max().unwrap();
                println!("Max robots in any row: {}", max_row);
                println!("Max robots in any column: {}", max_col);

                if islands <= 10 {
                    // Increased threshold to see more potential patterns
                    println!("\nPattern at step {}:", step + 1);
                    for row in &grid {
                        println!("{}", row.iter().collect::<String>());
                    }
                    println!();
                }
            }
        }

        println!(
            "\nFinal minimum pattern (islands: {}, step: {}):",
            min_islands, min_islands_step
        );
        for row in &min_islands_grid {
            println!("{}", row.iter().collect::<String>());
        }
        println!();

        format!(
            "Minimum islands: {} at step {}",
            min_islands, min_islands_step
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(Day14.part1(input), "12");
    }
}
