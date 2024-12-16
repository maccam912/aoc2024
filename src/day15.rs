use crate::Solution;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    row: i32,
    col: i32,
}

#[derive(Debug)]
struct Robot {
    position: Coordinate,
}

#[derive(Debug)]
struct Wall;

#[derive(Debug)]
struct Crate;

#[derive(Debug)]
struct DoubleCrate;

#[derive(Debug)]
struct Warehouse {
    grid: HashMap<Coordinate, i32>,
    robot: Robot,
    commands: Vec<char>,
}

impl Warehouse {
    fn from_str(input: &str) -> Self {
        let mut grid = HashMap::new();
        let mut robot = None;
        let mut commands = Vec::new();
        let mut next_crate_id = 2; // Start crate IDs at 2
        
        // Parse the grid
        let lines: Vec<&str> = input.lines().collect();
        let mut parsing_grid = true;
        
        for (row, line) in lines.iter().enumerate() {
            if line.is_empty() {
                parsing_grid = false;
                continue;
            }
            
            if parsing_grid {
                for (col, ch) in line.chars().enumerate() {
                    let coord = Coordinate {
                        row: row as i32,
                        col: col as i32,
                    };
                    
                    match ch {
                        '#' => { grid.insert(coord, 1); }, // Wall
                        'O' => { 
                            grid.insert(coord, next_crate_id);
                            next_crate_id += 1;
                        },
                        '@' => { robot = Some(Robot { position: coord }); },
                        _ => (), // Empty space or other characters
                    }
                }
            } else {
                // Parse commands
                commands.extend(line.chars().filter(|&c| "^v<>".contains(c)));
            }
        }
        
        Warehouse {
            grid,
            robot: robot.expect("Robot position not found in input"),
            commands,
        }
    }

    fn is_wall(&self, coord: &Coordinate) -> bool {
        self.grid.get(coord).map_or(false, |&v| v == 1)
    }

    fn is_crate(&self, coord: &Coordinate) -> bool {
        self.grid.get(coord).map_or(false, |&v| v >= 2)
    }

    fn get_crate_id(&self, coord: &Coordinate) -> Option<i32> {
        self.grid.get(coord).copied().filter(|&v| v >= 2)
    }

    fn can_move_crate(&self, from: &Coordinate, direction: char) -> bool {
        let to = match direction {
            '^' => Coordinate { row: from.row - 1, col: from.col },
            'v' => Coordinate { row: from.row + 1, col: from.col },
            '<' => Coordinate { row: from.row, col: from.col - 1 },
            '>' => Coordinate { row: from.row, col: from.col + 1 },
            _ => return false,
        };

        // Check if destination is blocked by wall
        if self.is_wall(&to) {
            return false;
        }

        // Get the crate ID at the current position
        let current_id = match self.get_crate_id(from) {
            Some(id) => id,
            None => return false,
        };

        // Check if this is part of a double crate
        let is_double = {
            let left = Coordinate { row: from.row, col: from.col - 1 };
            let right = Coordinate { row: from.row, col: from.col + 1 };
            self.get_crate_id(&left) == Some(current_id) || 
            self.get_crate_id(&right) == Some(current_id)
        };

        // If there's a crate in the way, recursively check if it can be pushed
        if self.is_crate(&to) {
            return self.can_move_crate(&to, direction);
        }

        if !is_double {
            return true;
        }

        // For double crates
        match direction {
            '<' | '>' => {
                // For horizontal movement, both parts must be able to move
                let other = if self.get_crate_id(&Coordinate { row: from.row, col: from.col - 1 }) == Some(current_id) {
                    Coordinate { row: to.row, col: to.col - 1 }
                } else {
                    Coordinate { row: to.row, col: to.col + 1 }
                };
                // Check if destination is blocked by wall
                if self.is_wall(&other) {
                    return false;
                }
                // If there's a crate in the way of either part, it must be able to move
                if self.is_crate(&other) {
                    return self.can_move_crate(&other, direction);
                }
                true
            },
            '^' | 'v' => {
                // For vertical movement, check both sides
                let left = Coordinate { row: from.row, col: from.col - 1 };
                let right = Coordinate { row: from.row, col: from.col + 1 };
                
                if self.get_crate_id(&left) == Some(current_id) {
                    let other_to = Coordinate { row: to.row, col: to.col - 1 };
                    if self.is_wall(&other_to) {
                        return false;
                    }
                    if self.is_crate(&other_to) {
                        return self.can_move_crate(&other_to, direction);
                    }
                } else if self.get_crate_id(&right) == Some(current_id) {
                    let other_to = Coordinate { row: to.row, col: to.col + 1 };
                    if self.is_wall(&other_to) {
                        return false;
                    }
                    if self.is_crate(&other_to) {
                        return self.can_move_crate(&other_to, direction);
                    }
                }
                true
            },
            _ => false,
        }
    }

    fn move_crate(&mut self, from: &Coordinate, direction: char) -> bool {
        if !self.can_move_crate(from, direction) {
            return false;
        }

        let to = match direction {
            '^' => Coordinate { row: from.row - 1, col: from.col },
            'v' => Coordinate { row: from.row + 1, col: from.col },
            '<' => Coordinate { row: from.row, col: from.col - 1 },
            '>' => Coordinate { row: from.row, col: from.col + 1 },
            _ => return false,
        };

        // If there's a crate in the destination, move it first (recursively)
        if self.is_crate(&to) {
            self.move_crate(&to, direction);
        }

        let current_id = self.get_crate_id(from).unwrap();
        
        // Handle double crates
        let left = Coordinate { row: from.row, col: from.col - 1 };
        let right = Coordinate { row: from.row, col: from.col + 1 };
        
        if self.get_crate_id(&left) == Some(current_id) {
            // If there's a crate in the destination of the left part, move it first
            let to_left = Coordinate { row: to.row, col: to.col - 1 };
            if self.is_crate(&to_left) {
                self.move_crate(&to_left, direction);
            }
            // Move left part
            let from_left = left;
            self.grid.remove(&from_left);
            self.grid.insert(to_left, current_id);
        } else if self.get_crate_id(&right) == Some(current_id) {
            // If there's a crate in the destination of the right part, move it first
            let to_right = Coordinate { row: to.row, col: to.col + 1 };
            if self.is_crate(&to_right) {
                self.move_crate(&to_right, direction);
            }
            // Move right part
            let from_right = right;
            self.grid.remove(&from_right);
            self.grid.insert(to_right, current_id);
        }

        // Move the main crate
        self.grid.remove(from);
        self.grid.insert(to, current_id);
        true
    }

    fn calculate_gps(&self) -> i32 {
        self.grid
            .iter()
            .filter(|(_, &v)| v >= 2) // Only consider crates (value >= 2)
            .map(|(coord, _)| 100 * (coord.row as i32) + coord.col as i32)
            .sum()
    }

    fn execute_move(&mut self, command: char) {
        let robot_pos = self.robot.position;
        let new_pos = match command {
            '^' => Coordinate { row: robot_pos.row - 1, col: robot_pos.col },
            'v' => Coordinate { row: robot_pos.row + 1, col: robot_pos.col },
            '<' => Coordinate { row: robot_pos.row, col: robot_pos.col - 1 },
            '>' => Coordinate { row: robot_pos.row, col: robot_pos.col + 1 },
            _ => return,
        };

        // If there's a wall in the way, don't move
        if self.is_wall(&new_pos) {
            return;
        }

        // If there's a crate in the way, try to move it
        if self.is_crate(&new_pos) {
            if !self.move_crate(&new_pos, command) {
                return;
            }
        }

        // Update robot position
        self.robot.position = new_pos;
    }

    fn warehouse_to_string(&self) -> String {
        let mut max_row = 0;
        let mut max_col = 0;
        
        // Find the bounds
        for coord in self.grid.keys() {
            max_row = max_row.max(coord.row);
            max_col = max_col.max(coord.col);
        }
        
        let mut result = String::new();
        for row in 0..=max_row {
            for col in 0..=max_col {
                let coord = Coordinate { row, col };
                let ch = if self.is_wall(&coord) {
                    '#'
                } else if self.is_crate(&coord) {
                    'O'
                } else if coord == self.robot.position {
                    '@'
                } else {
                    '.'
                };
                result.push(ch);
            }
            result.push('\n');
        }
        result
    }
}

#[derive(Debug)]
pub struct Day15;

impl Solution for Day15 {
    fn part1(&self, input: &str) -> String {
        let mut warehouse = Warehouse::from_str(input);
        
        // Execute all commands
        for command in warehouse.commands.clone() {
            warehouse.execute_move(command);
        }
        
        warehouse.calculate_gps().to_string()
    }

    fn part2(&self, input: &str) -> String {
        "Not implemented".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########\n\n<^^>>>vv<v>>v<<";
        let warehouse = Warehouse::from_str(input);
        
        // Basic checks
        assert!(warehouse.grid.values().any(|&v| v == 1)); // Has walls
        assert!(warehouse.grid.values().any(|&v| v >= 2)); // Has crates
        assert!(!warehouse.commands.is_empty()); // Has commands

        // Specific position checks
        let wall_positions = [
            Coordinate { row: 0, col: 0 },
            Coordinate { row: 0, col: 1 },
            Coordinate { row: 1, col: 0 },
        ];
        for pos in wall_positions {
            assert_eq!(warehouse.grid.get(&pos), Some(&1), "Expected wall at {:?}", pos);
        }

        // Empty space check
        assert_eq!(warehouse.grid.get(&Coordinate { row: 1, col: 1 }), None, "Expected empty space at (1,1)");

        // Crate check
        assert!(warehouse.grid.get(&Coordinate { row: 1, col: 3 }).map_or(false, |&id| id >= 2), 
            "Expected crate at (1,3)");
    }

    #[test]
    fn test_small_example_gps() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        
        let mut warehouse = Warehouse::from_str(input);
        
        // Execute all commands
        let commands = warehouse.commands.clone();
        for command in commands {
            warehouse.execute_move(command);
        }
        
        assert_eq!(warehouse.calculate_gps(), 2028, "Final GPS sum mismatch");
    }

    #[test]
    fn test_small_example_layout() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

        let expected_final_state = "\
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########
";
        
        let mut warehouse = Warehouse::from_str(input);
        
        // Execute all commands
        let commands = warehouse.commands.clone();
        for command in commands {
            warehouse.execute_move(command);
        }
        
        assert_eq!(warehouse.warehouse_to_string(), expected_final_state, 
            "\nExpected final state:\n{}\nActual final state:\n{}", 
            expected_final_state, warehouse.warehouse_to_string());
    }

    #[test]
    fn test_large_example_gps() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        let mut warehouse = Warehouse::from_str(input);
        
        // Execute all commands
        let commands = warehouse.commands.clone();
        for command in commands {
            warehouse.execute_move(command);
        }
        
        assert_eq!(warehouse.calculate_gps(), 10092, "Final GPS sum mismatch");
    }

    #[test]
    fn test_large_example_layout() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        let expected_final_state = "\
##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########
";

        let mut warehouse = Warehouse::from_str(input);
        
        // Execute all commands
        let commands = warehouse.commands.clone();
        for command in commands {
            warehouse.execute_move(command);
        }
        
        assert_eq!(warehouse.warehouse_to_string(), expected_final_state, 
            "\nExpected final state:\n{}\nActual final state:\n{}", 
            expected_final_state, warehouse.warehouse_to_string());
    }

    #[test]
    fn test_small_example_wide_boxes() {
        let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

        let mut warehouse = Warehouse::from_str(input);
        
        // Execute all commands
        let commands = warehouse.commands.clone();
        for command in commands {
            warehouse.execute_move(command);
        }

        let expected_final_state = "\
##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############
";
        
        assert_eq!(warehouse.warehouse_to_string(), expected_final_state, 
            "\nExpected final state:\n{}\nActual final state:\n{}", 
            expected_final_state, warehouse.warehouse_to_string());
    }

    #[test]
    fn test_large_example_wide_boxes() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        let mut warehouse = Warehouse::from_str(input);
        
        // Execute all commands
        let commands = warehouse.commands.clone();
        for command in commands {
            warehouse.execute_move(command);
        }

        let expected_final_state = "\
####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################
";
        
        assert_eq!(warehouse.calculate_gps(), 9021, "Final GPS sum mismatch");
        assert_eq!(warehouse.warehouse_to_string(), expected_final_state, 
            "\nExpected final state:\n{}\nActual final state:\n{}", 
            expected_final_state, warehouse.warehouse_to_string());
    }
}