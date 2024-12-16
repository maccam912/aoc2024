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
    double_mode: bool,
}

impl Warehouse {
    fn from_str(input: &str, double_mode: bool) -> Self {
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
                    let base_col = if double_mode { col * 2 } else { col };
                    let coord = Coordinate {
                        row: row as i32,
                        col: base_col as i32,
                    };
                    
                    match ch {
                        '#' => { 
                            grid.insert(coord, 1);
                            if double_mode {
                                grid.insert(Coordinate { 
                                    row: row as i32, 
                                    col: (base_col + 1) as i32 
                                }, 1);
                            }
                        },
                        'O' => { 
                            grid.insert(coord, next_crate_id);
                            if double_mode {
                                grid.insert(Coordinate { 
                                    row: row as i32, 
                                    col: (base_col + 1) as i32 
                                }, next_crate_id);
                            }
                            next_crate_id += 1;
                        },
                        '@' => { 
                            robot = Some(Robot { position: coord }); 
                            // In double mode, the space after @ is empty
                        },
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
            double_mode,
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
        let mut to_check = vec![];  // Stack of crate IDs we need to check
        let mut checked = std::collections::HashSet::new();  // Set of crate IDs we've already checked
        
        // Get the initial crate's ID and add it to the to_check list
        let initial_id = match self.get_crate_id(from) {
            Some(id) => id,
            None => return false,
        };
        to_check.push(initial_id);

        while let Some(current_id) = to_check.pop() {
            if checked.contains(&current_id) {
                continue;
            }
            checked.insert(current_id);

            // Find all coordinates containing this crate ID
            let mut crate_coords = vec![];
            for (&coord, &id) in &self.grid {
                if id == current_id {
                    crate_coords.push(coord);
                }
            }

            // Calculate new positions for all parts of this crate
            for coord in crate_coords {
                let new_coord = match direction {
                    '^' => Coordinate { row: coord.row - 1, col: coord.col },
                    'v' => Coordinate { row: coord.row + 1, col: coord.col },
                    '<' => Coordinate { row: coord.row, col: coord.col - 1 },
                    '>' => Coordinate { row: coord.row, col: coord.col + 1 },
                    _ => return false,
                };

                // Check what's in the new position
                if let Some(&id) = self.grid.get(&new_coord) {
                    if id == 1 {
                        // Hit a wall, entire stack cannot move
                        return false;
                    } else if id >= 2 && id != current_id && !checked.contains(&id) {
                        // Found a new crate to check
                        to_check.push(id);
                    }
                }
            }
        }

        // If we got here, it means all crates can move
        true
    }

    fn move_crate(&mut self, from: &Coordinate, direction: char) -> bool {
        // First check if the move is possible
        if !self.can_move_crate(from, direction) {
            return false;
        }

        // Find all crates that need to be moved by checking which ones were validated
        let mut to_move = std::collections::HashSet::new();
        let mut checked = std::collections::HashSet::new();
        let mut to_check = vec![self.get_crate_id(from).unwrap()];

        while let Some(current_id) = to_check.pop() {
            if checked.contains(&current_id) {
                continue;
            }
            checked.insert(current_id);
            to_move.insert(current_id);

            // Find any crates that would be overlapped by moving this one
            let crate_coords: Vec<_> = self.grid.iter()
                .filter(|(_, &id)| id == current_id)
                .map(|(&coord, _)| coord)
                .collect();

            for coord in crate_coords {
                let new_coord = match direction {
                    '^' => Coordinate { row: coord.row - 1, col: coord.col },
                    'v' => Coordinate { row: coord.row + 1, col: coord.col },
                    '<' => Coordinate { row: coord.row, col: coord.col - 1 },
                    '>' => Coordinate { row: coord.row, col: coord.col + 1 },
                    _ => return false,
                };

                if let Some(&id) = self.grid.get(&new_coord) {
                    if id >= 2 && !checked.contains(&id) {
                        to_check.push(id);
                    }
                }
            }
        }

        // Now move all the crates we found
        // First collect all moves we need to make to avoid conflicts
        let mut moves = vec![];
        for &id in &to_move {
            for (&coord, &grid_id) in &self.grid {
                if grid_id == id {
                    let new_coord = match direction {
                        '^' => Coordinate { row: coord.row - 1, col: coord.col },
                        'v' => Coordinate { row: coord.row + 1, col: coord.col },
                        '<' => Coordinate { row: coord.row, col: coord.col - 1 },
                        '>' => Coordinate { row: coord.row, col: coord.col + 1 },
                        _ => return false,
                    };
                    moves.push((coord, new_coord, id));
                }
            }
        }

        // Then apply all moves
        // First remove all old positions
        for (from_coord, _, _) in &moves {
            self.grid.remove(from_coord);
        }
        // Then add all new positions
        for (_, to_coord, id) in moves {
            self.grid.insert(to_coord, id);
        }
        true
    }

    fn calculate_gps(&self) -> i32 {
        let mut crate_positions = std::collections::HashMap::new();
        // First collect all positions for each crate
        for (coord, &v) in self.grid.iter() {
            if v >= 2 {
                let crate_id = v - 2;
                crate_positions
                    .entry(crate_id)
                    .or_insert_with(Vec::new)
                    .push(coord);
            }
        }
        // Then sum using the leftmost position for each crate
        crate_positions
            .into_iter()
            .map(|(_, coords)| {
                let leftmost = coords.into_iter().min_by_key(|c| c.col).unwrap();
                100 * leftmost.row as i32 + leftmost.col as i32
            })
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
        let mut min_row = i32::MAX;
        let mut max_row = i32::MIN;
        let mut min_col = i32::MAX;
        let mut max_col = i32::MIN;

        // Find the bounds of the warehouse
        for coord in self.grid.keys() {
            min_row = min_row.min(coord.row);
            max_row = max_row.max(coord.row);
            min_col = min_col.min(coord.col);
            max_col = max_col.max(coord.col);
        }

        let mut result = String::new();
        for row in min_row..=max_row {
            for col in min_col..=max_col {
                let coord = Coordinate { row, col };
                let robot_here = self.robot.position == coord;
                
                if robot_here {
                    result.push('@');
                } else if let Some(&value) = self.grid.get(&coord) {
                    if value == 1 {
                        result.push('#');
                    } else {
                        result.push('O');
                    }
                } else {
                    result.push('.');
                }
            }
            if row < max_row {
                result.push('\n');
            }
        }
        result
    }
}

#[derive(Debug)]
pub struct Day15;

impl Solution for Day15 {
    fn part1(&self, input: &str) -> String {
        let mut warehouse = Warehouse::from_str(input, false);
        
        // Execute all commands
        for command in warehouse.commands.clone() {
            warehouse.execute_move(command);
        }
        
        warehouse.calculate_gps().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut warehouse = Warehouse::from_str(input, true);
        
        // Execute all commands
        for command in warehouse.commands.clone() {
            warehouse.execute_move(command);
        }
        
        warehouse.calculate_gps().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare_states(actual: &str, expected: &str) -> bool {
        let expected = expected.trim().replace(['[', ']'], "O");
        let actual = actual.trim().replace(['[', ']'], "O");
        actual == expected
    }

    fn assert_states_eq(actual: &str, expected: &str, message: &str) {
        assert!(compare_states(actual, expected), 
            "{}\nExpected state:\n{}\nActual state:\n{}", 
            message, expected, actual);
    }

    #[test]
    fn test_parse_input() {
        let input = "########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########\n\n<^^>>>vv<v>>v<<";
        let warehouse = Warehouse::from_str(input, false);
        
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
        
        let mut warehouse = Warehouse::from_str(input, false);
        
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
        
        let mut warehouse = Warehouse::from_str(input, false);
        
        // Execute all commands
        let commands = warehouse.commands.clone();
        for command in commands {
            warehouse.execute_move(command);
        }
        
        assert_states_eq(&warehouse.warehouse_to_string(), expected_final_state, 
            "Final state mismatch");
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

        let mut warehouse = Warehouse::from_str(input, false);
        
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
        
        let mut warehouse = Warehouse::from_str(input, false);
        
        // Execute all commands
        let commands = warehouse.commands.clone();
        for command in commands {
            warehouse.execute_move(command);
        }
        
        assert_states_eq(&warehouse.warehouse_to_string(), expected_final_state, 
            "Final state mismatch");
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
        
        let mut warehouse = Warehouse::from_str(input, true);
        
        // Execute all commands
        let commands = warehouse.commands.clone();
        for command in commands {
            warehouse.execute_move(command);
        }
        
        assert_states_eq(&warehouse.warehouse_to_string(), expected_final_state, 
            "Final state mismatch");
    }

    #[test]
    fn test_double_mode_example() {
        let input = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########";

        let warehouse = Warehouse::from_str(input, true);
        let expected = "\
####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################
";
        
        assert_states_eq(&warehouse.warehouse_to_string(), expected, 
            "Initial state mismatch in double mode");
    }

    #[test]
    fn test_large_example_wide_boxes_gps() {
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

        let mut warehouse = Warehouse::from_str(input, true);
        
        // Execute all commands
        let commands = warehouse.commands.clone();
        for command in commands {
            warehouse.execute_move(command);
        }

        println!("{}", warehouse.warehouse_to_string());
        
        assert_eq!(warehouse.calculate_gps(), 9021);
    }

    #[test]
    fn test_small_example_wide_boxes() {
        let input = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

        let expected_states = vec![
            // Initial state
            "\
##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############",
            // After move <
            "\
##############
##......##..##
##..........##
##...[][]@..##
##....[]....##
##..........##
##############",
            // After move v
            "\
##############
##......##..##
##..........##
##...[][]...##
##....[].@..##
##..........##
##############",
            // After move v
            "\
##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.......@..##
##############",
            // After move <
            "\
##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##......@...##
##############",
            // After move <
            "\
##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.....@....##
##############",
            // After move ^
            "\
##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############",
            // After move ^
            "\
##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############",
            // After move <
            "\
##############
##......##..##
##...[][]...##
##....[]....##
##....@.....##
##..........##
##############",
            "\
##############
##......##..##
##...[][]...##
##....[]....##
##...@......##
##..........##
##############",
            // After move <
            "\
##############
##......##..##
##...[][]...##
##...@[]....##
##..........##
##..........##
##############",
            // After move ^
            "\
##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############"
        ];

        let mut warehouse = Warehouse::from_str(input, true); // Set double_width to true
        
        // Check initial state
        assert_states_eq(&warehouse.warehouse_to_string(), expected_states[0], 
            "Initial state mismatch");

        // Execute each command and verify the state
        let commands = warehouse.commands.clone();
        for (i, command) in commands.iter().enumerate() {
            // Verify before and after that there are SIX crate positions
            assert_eq!(warehouse.grid.values().filter(|&&v| v >= 2).count(), 6, 
                "Expected 6 crate positions before move {}, but found {}", i + 1, 
                warehouse.grid.values().filter(|&&v| v >= 2).count());
            warehouse.execute_move(*command);
            assert_eq!(warehouse.grid.values().filter(|&&v| v >= 2).count(), 6, 
                "Expected 6 crate positions after move {}, but found {}", i + 1, 
                warehouse.grid.values().filter(|&&v| v >= 2).count());
            assert_states_eq(&warehouse.warehouse_to_string(), expected_states[i + 1], 
                &format!("State mismatch after move {}", i + 1));
        }
    }
}