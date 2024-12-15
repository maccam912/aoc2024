use crate::Solution;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Box,
    Robot,
}

impl Cell {
    fn to_char(self) -> char {
        match self {
            Cell::Empty => '.',
            Cell::Wall => '#',
            Cell::Box => 'O',
            Cell::Robot => '@',
        }
    }
}

#[derive(Debug, Clone)]
struct Warehouse {
    grid: Vec<Vec<Cell>>,
    robot_pos: (usize, usize),
}

impl Warehouse {
    fn parse(input: &str) -> (Self, String) {
        let mut lines = input.lines().collect::<Vec<_>>();
        let mut moves = String::new();
        let mut grid_lines = Vec::new();
        
        // Separate grid from moves
        let mut parsing_grid = true;
        for line in lines {
            if line.trim().is_empty() {
                parsing_grid = false;
                continue;
            }
            if parsing_grid {
                grid_lines.push(line);
            } else {
                moves.push_str(line.trim());
            }
        }

        let mut grid = Vec::new();
        let mut robot_pos = (0, 0);
        
        for (row, line) in grid_lines.iter().enumerate() {
            let mut grid_row = Vec::new();
            for (col, c) in line.chars().enumerate() {
                let cell = match c {
                    '#' => Cell::Wall,
                    'O' => Cell::Box,
                    '@' => {
                        robot_pos = (row, col);
                        Cell::Robot
                    }
                    _ => Cell::Empty,
                };
                grid_row.push(cell);
            }
            grid.push(grid_row);
        }

        (Warehouse { grid, robot_pos }, moves)
    }

    fn can_move_to(&self, row: isize, col: isize) -> bool {
        if row < 0 || col < 0 || 
           row >= self.grid.len() as isize || 
           col >= self.grid[0].len() as isize {
            return false;
        }
        matches!(self.grid[row as usize][col as usize], Cell::Empty)
    }

    fn try_push(&mut self, start_row: usize, start_col: usize, delta_row: isize, delta_col: isize) -> bool {
        let new_row = start_row as isize + delta_row;
        let new_col = start_col as isize + delta_col;

        // Check bounds
        if new_row < 0 || new_col < 0 || 
           new_row >= self.grid.len() as isize || 
           new_col >= self.grid[0].len() as isize {
            return false;
        }

        let new_row = new_row as usize;
        let new_col = new_col as usize;

        // First check if we can push all boxes in this direction
        let mut check_row = new_row;
        let mut check_col = new_col;
        let mut boxes_to_move = vec![(start_row, start_col)];
        
        loop {
            match self.grid[check_row][check_col] {
                Cell::Empty => {
                    // Found empty space at end of chain, we can move all boxes
                    // Move boxes from back to front
                    for (box_row, box_col) in boxes_to_move.iter().rev() {
                        let next_row = *box_row as isize + delta_row;
                        let next_col = *box_col as isize + delta_col;
                        self.grid[*box_row][*box_col] = Cell::Empty;
                        self.grid[next_row as usize][next_col as usize] = Cell::Box;
                    }
                    return true;
                }
                Cell::Wall => {
                    // Hit a wall, can't push
                    return false;
                }
                Cell::Box => {
                    // Found another box, add it to chain and keep checking
                    boxes_to_move.push((check_row, check_col));
                    check_row = (check_row as isize + delta_row) as usize;
                    check_col = (check_col as isize + delta_col) as usize;
                    
                    // Check bounds for next position
                    if check_row >= self.grid.len() || check_col >= self.grid[0].len() {
                        return false;
                    }
                }
                Cell::Robot => {
                    // Shouldn't happen in normal gameplay
                    return false;
                }
            }
        }
    }

    fn try_move(&mut self, direction: char) {
        let (row, col) = self.robot_pos;
        let (delta_row, delta_col) = match direction {
            '^' => (-1isize, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => return,
        };

        let new_row = row as isize + delta_row;
        let new_col = col as isize + delta_col;
        
        // Check bounds
        if new_row < 0 || new_col < 0 || 
           new_row >= self.grid.len() as isize || 
           new_col >= self.grid[0].len() as isize {
            return;
        }

        let new_row = new_row as usize;
        let new_col = new_col as usize;

        match self.grid[new_row][new_col] {
            Cell::Empty => {
                // Simple move
                self.grid[row][col] = Cell::Empty;
                self.grid[new_row][new_col] = Cell::Robot;
                self.robot_pos = (new_row, new_col);
            }
            Cell::Wall => {
                // Can't move into wall
                return;
            }
            Cell::Box => {
                // Try to push the box
                if self.try_push(new_row, new_col, delta_row, delta_col) {
                    // Box was pushed, move robot
                    self.grid[row][col] = Cell::Empty;
                    self.grid[new_row][new_col] = Cell::Robot;
                    self.robot_pos = (new_row, new_col);
                }
            }
            Cell::Robot => {
                // Shouldn't happen
                return;
            }
        }
    }

    fn calculate_gps_sum(&self) -> usize {
        let mut sum = 0;
        for (row, grid_row) in self.grid.iter().enumerate().skip(1) {  // Skip first wall row
            for (col, cell) in grid_row.iter().enumerate().skip(1) {   // Skip first wall column
                if *cell == Cell::Box {
                    // Row and col are already 1-based since we skipped the walls
                    sum += 100 * row + col;
                }
            }
        }
        sum
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        for row in &self.grid {
            for cell in row {
                result.push(cell.to_char());
            }
            result.push('\n');
        }
        result
    }

    fn apply_moves(&mut self, moves: &str, steps: Option<usize>) -> String {
        let mut result = String::new();
        result.push_str("Initial state:\n");
        result.push_str(&self.to_string());
        result.push('\n');

        for (i, m) in moves.chars().enumerate() {
            if let Some(max_steps) = steps {
                if i >= max_steps {
                    break;
                }
            }
            self.try_move(m);
            result.push_str(&format!("Move {}:\n", m));
            result.push_str(&self.to_string());
            result.push('\n');
        }
        result.trim_end().to_string()
    }
}

pub struct Day15;

impl Solution for Day15 {
    fn part1(&self, input: &str) -> String {
        let (mut warehouse, moves) = Warehouse::parse(input);
        warehouse.apply_moves(&moves, None);
        warehouse.calculate_gps_sum().to_string()
    }

    fn part2(&self, _input: &str) -> String {
        "Not implemented".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    const SAMPLE1: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const SAMPLE2: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const EXPECTED_MOVES: &str = "\
Initial state:
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move <:
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move ^:
########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move ^:
########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move >:
########
#..@OO.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move >:
########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move >:
########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move v:
########
#....OO#
##..@..#
#...O..#
#.#.O..#
#...O..#
#...O..#
########

Move v:
########
#....OO#
##..@..#
#...O..#
#.#.O..#
#...O..#
#...O..#
########

Move <:
########
#....OO#
##.@...#
#...O..#
#.#.O..#
#...O..#
#...O..#
########

Move v:
########
#....OO#
##.....#
#..@O..#
#.#.O..#
#...O..#
#...O..#
########

Move >:
########
#....OO#
##.....#
#...@O.#
#.#.O..#
#...O..#
#...O..#
########

Move >:
########
#....OO#
##.....#
#....@O#
#.#.O..#
#...O..#
#...O..#
########

Move v:
########
#....OO#
##.....#
#.....O#
#.#.O@.#
#...O..#
#...O..#
########

Move <:
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########

Move <:
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########";

    #[test]
    fn test_part1_small_sample() {
        assert_eq!(Day15.part1(SAMPLE1), "2028");
    }

    #[test]
    fn test_part1_large_sample() {
        assert_eq!(Day15.part1(SAMPLE2), "10092");
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input(15, true);
        assert_eq!(Day15.part2(&input), "Not implemented");
    }

    #[test]
    fn test_warehouse_moves() {
        let (mut warehouse, moves) = Warehouse::parse(SAMPLE1);
        let result = warehouse.apply_moves(&moves, None);
        assert_eq!(result, EXPECTED_MOVES);
    }
}
