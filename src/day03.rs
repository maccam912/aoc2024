use crate::Solution;
use std::fs;

pub struct Day03;

impl Solution for Day03 {
    fn part1(&self, input: &str) -> String {
        process_part1(input)
    }

    fn part2(&self, input: &str) -> String {
        process_part2(input)
    }
}

fn process_part1(input: &str) -> String {
    let mut total = 0;
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        if i + 3 < chars.len() && chars[i..i + 4].iter().collect::<String>() == "mul(" {
            // Found potential mul instruction, look for closing parenthesis
            let start = i + 4;
            let mut end = start;
            let mut found_valid = false;
            let mut depth = 1;

            while end < chars.len() && depth > 0 {
                if chars[end] == '(' {
                    depth += 1;
                } else if chars[end] == ')' {
                    depth -= 1;
                    if depth == 0 {
                        // We found a matching closing parenthesis
                        let inside = &chars[start..end].iter().collect::<String>();
                        if let Some((x, y)) = parse_mul_args(inside) {
                            total += x * y;
                            found_valid = true;
                        }
                    }
                }
                end += 1;
            }

            if found_valid {
                i = end;
                continue;
            }
        }
        i += 1;
    }

    total.to_string()
}

fn process_part2(input: &str) -> String {
    let mut total = 0;
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();
    let mut mul_enabled = true;

    while i < chars.len() {
        if i + 4 < chars.len() {
            let next_four: String = chars[i..i + 4].iter().collect();
            let next_five: String = if i + 5 < chars.len() {
                chars[i..i + 5].iter().collect()
            } else {
                String::new()
            };

            if next_four == "mul(" {
                // Found potential mul instruction
                let start = i + 4;
                let mut end = start;
                let mut found_valid = false;
                let mut depth = 1;

                while end < chars.len() && depth > 0 {
                    if chars[end] == '(' {
                        depth += 1;
                    } else if chars[end] == ')' {
                        depth -= 1;
                        if depth == 0 && mul_enabled {
                            // We found a matching closing parenthesis
                            let inside = &chars[start..end].iter().collect::<String>();
                            if let Some((x, y)) = parse_mul_args(inside) {
                                total += x * y;
                                found_valid = true;
                            }
                        }
                    }
                    end += 1;
                }

                if found_valid {
                    i = end;
                    continue;
                }
            } else if next_five == "don't" {
                mul_enabled = false;
            } else if next_three_chars(&chars, i) == "do(" {
                mul_enabled = true;
            }
        }
        i += 1;
    }

    total.to_string()
}

fn next_three_chars(chars: &[char], start: usize) -> String {
    if start + 3 <= chars.len() {
        chars[start..start + 3].iter().collect()
    } else {
        String::new()
    }
}

fn parse_mul_args(args: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = args.split(',').collect();
    if parts.len() != 2 {
        return None;
    }

    let x = parts[0].trim().parse::<i32>().ok()?;
    let y = parts[1].trim().parse::<i32>().ok()?;

    // Validate that numbers are 1-3 digits
    if x > 999 || y > 999 || x < 1 || y < 1 {
        return None;
    }

    Some((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = fs::read_to_string("inputs/sample/03.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "161");
    }

    #[test]
    fn test_part2_sample() {
        let input = fs::read_to_string("inputs/sample/03_2.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "48");
    }
}
