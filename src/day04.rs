use crate::Solution;

pub struct Day04;

impl Solution for Day04 {
    fn part1(&self, input: &str) -> String {
        process_part1(input)
    }

    fn part2(&self, input: &str) -> String {
        process_part2(input).to_string()
    }
}

fn check_word_at_position(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    row_delta: i32,
    col_delta: i32,
) -> bool {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let word = ['X', 'M', 'A', 'S'];

    for i in 0..4 {
        let new_row = row as i32 + i as i32 * row_delta;
        let new_col = col as i32 + i as i32 * col_delta;

        if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
            return false;
        }

        if grid[new_row as usize][new_col as usize] != word[i as usize] {
            return false;
        }
    }

    true
}

fn check_x_mas_at_position(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    // First check if center is 'A'
    if grid[row][col] != 'A' {
        return false;
    }

    // Get the characters in each diagonal
    let top_left = grid[row - 1][col - 1];
    let top_right = grid[row - 1][col + 1];
    let bottom_left = grid[row + 1][col - 1];
    let bottom_right = grid[row + 1][col + 1];

    // Check diagonal 1 (top-left to bottom-right)
    let diag1_m_count = [top_left, bottom_right]
        .iter()
        .filter(|&&c| c == 'M')
        .count();
    let diag1_s_count = [top_left, bottom_right]
        .iter()
        .filter(|&&c| c == 'S')
        .count();

    // Check diagonal 2 (top-right to bottom-left)
    let diag2_m_count = [top_right, bottom_left]
        .iter()
        .filter(|&&c| c == 'M')
        .count();
    let diag2_s_count = [top_right, bottom_left]
        .iter()
        .filter(|&&c| c == 'S')
        .count();

    // For a valid pattern, each diagonal must have exactly one M and one S
    if diag1_m_count == 1 && diag1_s_count == 1 && diag2_m_count == 1 && diag2_s_count == 1 {
        true
    } else {
        false
    }
}

fn process_part1(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    if rows == 0 {
        return "0".to_string();
    }
    let cols = grid[0].len();

    let mut count = 0;

    // Define all possible directions to check
    let directions = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down-right
        (-1, 1),  // up-right
        (1, -1),  // down-left
        (-1, -1), // up-left
    ];

    // Check all possible starting positions
    for row in 0..rows {
        for col in 0..cols {
            // Try each direction from this position
            for &(row_delta, col_delta) in &directions {
                if check_word_at_position(&grid, row, col, row_delta, col_delta) {
                    count += 1;
                }
            }
        }
    }

    count.to_string()
}

fn process_part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // We need at least a 3x3 grid and need to stay 1 away from edges
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if check_x_mas_at_position(&grid, row, col) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = "MMMSXXMASM\n\
                    MSAMXMSMSA\n\
                    AMXSXMAAMM\n\
                    MSAMASMSMX\n\
                    XMASAMXAMM\n\
                    XXAMMXXAMA\n\
                    SMSMSASXSS\n\
                    SAXAMASAAA\n\
                    MAMMMXMMMM\n\
                    MXMXAXMASX";

        assert_eq!(process_part1(input), "18");
    }

    #[test]
    fn test_part2_sample() {
        let input = ".M.S......\n\
                    ..A..MSMS.\n\
                    .M.S.MAA..\n\
                    ..A.ASMSM.\n\
                    .M.S.M....\n\
                    ..........\n\
                    S.S.S.S.S.\n\
                    .A.A.A.A..\n\
                    M.M.M.M.M.\n\
                    ..........";

        assert_eq!(process_part2(input), 9);
    }
}
