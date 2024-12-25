use crate::Solution;

#[derive(Debug, Clone)]
struct Schematic {
    grid: Vec<Vec<char>>,
}

impl Schematic {
    fn from_str(s: &str) -> Self {
        let grid = s.lines().map(|line| line.chars().collect()).collect();
        Self { grid }
    }

    fn get_heights(&self, from_bottom: bool) -> Vec<usize> {
        let width = self.grid[0].len();
        let height = self.grid.len();

        (0..width)
            .map(|x| {
                if from_bottom {
                    // For keys: count '#' from bottom up (subtract 1 to not count first #)
                    let count = (0..height)
                        .rev()
                        .take_while(|&y| self.grid[y][x] == '#')
                        .count();
                    count.saturating_sub(1)
                } else {
                    // For locks: count from top until we hit a '.' (subtract 1 to not count first #)
                    let pos = (0..height)
                        .position(|y| self.grid[y][x] == '.')
                        .unwrap_or(height);
                    pos.saturating_sub(1)
                }
            })
            .collect()
    }
}

fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for section in sections {
        let schematic = Schematic::from_str(section);
        // If top row is filled (#), it's a lock
        if schematic.grid[0].iter().all(|&c| c == '#') {
            locks.push(schematic.get_heights(false));
        } else {
            keys.push(schematic.get_heights(true));
        }
    }

    (locks, keys)
}

fn is_compatible(lock: &[usize], key: &[usize], grid_height: usize) -> bool {
    lock.iter()
        .zip(key.iter())
        .all(|(&l, &k)| l + k <= grid_height - 2) // Must leave room for the first # in each
}

pub struct Day25;

impl Solution for Day25 {
    fn part1(&self, input: &str) -> String {
        let (locks, keys) = parse_input(input);
        let mut valid_pairs = 0;

        // Get grid height from first schematic
        let grid_height = input.lines().take_while(|l| !l.is_empty()).count();

        // Debug print
        println!("Locks:");
        for lock in locks.iter() {
            println!("{:?}", lock);
        }
        println!("\nKeys:");
        for key in keys.iter() {
            println!("{:?}", key);
        }
        println!("\nGrid height: {}", grid_height);

        for (i, lock) in locks.iter().enumerate() {
            for (j, key) in keys.iter().enumerate() {
                if is_compatible(lock, key, grid_height) {
                    println!("\nCompatible pair found:");
                    println!("Lock {}: {:?}", i, lock);
                    println!("Key {}: {:?}", j, key);
                    valid_pairs += 1;
                }
            }
        }

        valid_pairs.to_string()
    }

    fn part2(&self, _input: &str) -> String {
        // Part 2 not required for Day 25
        "Not implemented".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn test_part1_sample() {
        assert_eq!(Day25.part1(SAMPLE), "3");
    }
}
