// Based heavily on https://topaz.github.io/paste/#XQAAAQCRDgAAAAAAAAA6nMjJFHMADebh9lMSAXn5c0lZw0XzLjIVxATQJaSMlgO28y8f4vw2/ZRr/BoX9x1wncFquy1SYbjPUHtpSp1bPYZU63+UbJgg05wGIRzCsLgHYhjpS2Q3uXCzIYUZbr40EePh1kqfhS97EJrBw9DxiN0iZMXbpFk0tttccIts4MOYg4ue+/egqBLWRwvjg/mhfrgyG1B8No8vdWwCJHmuplIZPDfmqD5WQ7wM748Az/S5pHYDKCYwQf7CryRjbJQ3OM0I0VD7XiGt57blkJfGXziGT5dK+z81wslPdxvuXsMsxTG/iUHpUSjf/0qJ5QvPgImAr+jbdrdFB5/kvnxJZH+wxvRTk3N5J7d7LpEh+ltZC659X0U4XueBzNn99BReogKTh5jDRvcSc46s/2iCj9plDZhEZxUybHVj5C/qAKRvEj+py+8qMlCxmnZobDAJEIeiaHIRImVAPLD9irLI4cCtkuRiAfqNfvvBzQoGdz5qz9DUGZGZwivSB/PJicf+cUgu8z2kd1oc9QKYmgLsRwkVL0EUl0Wf3aKo12JOchHNm9BYbp9CL96o4r2VItmV1zlL3us1ET/heC1l4RvjirAEdA/tZ/QXnbHWYkJd2PRz0sjLLj6dan8+gGieE/4IyN0fZEPQXPxzqF6kfFcMop4qH6jCwVyBFshCqt5jy3XALT2l2j09d95fZmC5eLRbjN6EK2HcXog/OH2rJKAGrC3ROVHJOa+n6T7Q/Z+NHHCHRp/EPyHdpukp/qn48CISR+7xfYWsuby3R7S5+ZBLhGAOPyc3aZ8YCwYS3h1D1lrt1Tn2QPskOWDNhRcCnXPFVEjMWnwqcwmLnuLsvI/+U3i5UwU4UDoKFZGizjQwo7qncQWwSVDqQNtH0Pp3z5fX0zFn+SWtCa1nqdvcTrUns6FlPUzdJfGZsnIQ1il4PtafAZ/WagA0Raso5ZYK5Xx8yuvgL4W0zejtEu50bBJonHdRZl1l8HR4h9IPpRF6lM5HgyVDWehCewFgmPLT8nOJgnjFbtVgQqWHdlVTNrDV0TO3hV+38U6jqz2IMNOstbYRR1aEW1Et9ucHRzCZ0IjYxy5Kbmb8uJu55DyKq1twdVgU2xrJUAvWcgLbxHijqrPQz8Li8eQ/mpeOizLNrO63GbVdoLvNwSpg8Bj+uJ7WcjlRYZv4kqjce/qTaxWjSX4so27KwwlD9HA8ISGc0mf8aZzeNiUyqfswVlby7P2Y3VPPcMT+ZSMmpIw2ADmaFijGBeNxr7PxgBEtivewRt6HoQ21CMDxQxE+vUyJspunPWYFmYoS+NPEYptKQfzsv7ah0WEon2Q+Vl8Z3N++kG5OXOQ1hJLGljLL7KVt4XAAWmf+h+jr8l4axn9MriqHM6YThKGnX8hxqwFZmP+DZoLfK+yin2fUyfBeglK5ACddPuDJCGKn4R+BV/xBDQXEjqMsTQ1Qp2MtWy5bezK24X9o3dr4M5pP4Mh0OZuZO1Z1IaDWVwLJj6R6B0a7JmhIcigxXEPzKqVUmWcPVv3mSAs=

// This solution uses a clever approach to solve the robot keypad sequence problem
// The core idea is to model the keypads as graphs and find shortest paths between buttons,
// while handling multiple layers of robot control through recursive path finding

use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::iter;

// Position on a keypad, using zero-based row and column indices
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Pos(usize, usize);

// Find all possible shortest paths between two positions on a keypad while avoiding a gap
// Uses a breadth-first search (BFS) to ensure we find all shortest paths
// The gap parameter represents a position that cannot be crossed (empty space on keypad)
fn paths(a: Pos, b: Pos, gap: Pos) -> Vec<String> {
    let mut q = VecDeque::from([(a, String::new())]);
    let mut res = vec![];
    while let Some((Pos(i, j), mut path)) = q.pop_front() {
        // Found target position - add the final 'A' press and save this path
        if Pos(i, j) == b {
            path.push('A');
            res.push(path);
            continue;
        }

        // For each direction (left/up/down/right):
        // 1. Check if moving in this direction gets us closer to target
        // 2. Ensure we don't cross the gap position
        // 3. Generate the sequence of moves needed to reach the target position
        // 4. Add the new position and path to the queue

        // Move left if target is to the left and we won't cross the gap
        if b.1 < j && !(gap.0 == i && gap.1 < j && gap.1 >= b.1) {
            let mut new_path = path.clone();
            new_path.extend(iter::repeat('<').take(j - b.1));
            q.push_back((Pos(i, b.1), new_path));
        }
        // Move up if target is above and won't cross gap
        if b.0 < i && !(gap.1 == j && gap.0 < i && gap.0 >= b.0) {
            let mut new_path = path.clone();
            new_path.extend(iter::repeat('^').take(i - b.0));
            q.push_back((Pos(b.0, j), new_path));
        }
        // Move down if target is below and won't cross gap
        if b.0 > i && !(gap.1 == j && gap.0 > i && gap.0 <= b.0) {
            let mut new_path = path.clone();
            new_path.extend(iter::repeat('v').take(b.0 - i));
            q.push_back((Pos(b.0, j), new_path));
        }
        // Move right if target is to the right and won't cross gap
        if b.1 > j && !(gap.0 == i && gap.1 > j && gap.1 <= b.1) {
            let mut new_path = path.clone();
            new_path.extend(iter::repeat('>').take(b.1 - j));
            q.push_back((Pos(i, b.1), new_path));
        }
    }
    res
}

// Represents a keypad with button positions and a gap position
struct Keypad {
    keymap: HashMap<char, Pos>, // Maps button symbols to their positions
    gap: Pos,                   // Position of the empty space
}

impl Keypad {
    // Create the numeric keypad layout (the door keypad)
    // 789
    // 456
    // 123
    //  0A
    fn numeric() -> Self {
        let keymap = HashMap::from([
            ('7', Pos(0, 0)),
            ('8', Pos(0, 1)),
            ('9', Pos(0, 2)),
            ('4', Pos(1, 0)),
            ('5', Pos(1, 1)),
            ('6', Pos(1, 2)),
            ('1', Pos(2, 0)),
            ('2', Pos(2, 1)),
            ('3', Pos(2, 2)),
            ('0', Pos(3, 1)),
            ('A', Pos(3, 2)),
        ]);
        let gap = Pos(3, 0);
        Keypad { keymap, gap }
    }

    // Create the directional keypad layout (the robot control pad)
    //  ^A
    // <v>
    fn directional() -> Self {
        let keymap = HashMap::from([
            ('^', Pos(0, 1)),
            ('A', Pos(0, 2)),
            ('<', Pos(1, 0)),
            ('v', Pos(1, 1)),
            ('>', Pos(1, 2)),
        ]);
        let gap = Pos(0, 0);
        Keypad { keymap, gap }
    }

    // Find all possible paths between two buttons on this keypad
    fn paths(&self, a: char, b: char) -> Vec<String> {
        paths(self.keymap[&a], self.keymap[&b], self.gap)
    }
}

// Find the shortest sequence length to type a code through multiple layers of robot control
// Uses dynamic programming with memoization to avoid recomputing paths
// Parameters:
// - np: numeric keypad (final door keypad)
// - dp: directional keypad (robot control pad)
// - code: sequence to type
// - depth: current robot layer (0 = door keypad, 1+ = control keypads)
// - max_depth: total number of robot layers
// - cache: memoization cache to store computed results
fn shortest_len(
    np: &Keypad,
    dp: &Keypad,
    code: String,
    depth: usize,
    max_depth: usize,
    cache: &mut HashMap<(usize, String), usize>,
) -> usize {
    // Check if we've already computed this result
    if let Some(&cached) = cache.get(&(depth, code.clone())) {
        return cached;
    }

    // Choose which keypad to use based on depth
    let kp = if depth == 0 { np } else { dp };

    // For each pair of consecutive characters in the sequence:
    // 1. Find all possible paths between them on the current keypad
    // 2. If at max depth, take the shortest path length
    // 3. Otherwise, recursively find shortest sequence for each path
    let res = iter::once('A')
        .chain(code.chars())
        .tuple_windows()
        .map(|(a, b)| {
            let paths = kp.paths(a, b);
            if depth == max_depth {
                paths.iter().map(String::len).min().unwrap()
            } else {
                paths
                    .into_iter()
                    .map(|path| shortest_len(np, dp, path, depth + 1, max_depth, cache))
                    .min()
                    .unwrap()
            }
        })
        .sum::<usize>();

    // Cache and return the result
    cache.insert((depth, code), res);
    res
}

// Part 1: Find complexity sum for 2 layers of robots
pub fn part1(input: &str) -> usize {
    let np = Keypad::numeric();
    let dp = Keypad::directional();
    let mut cache = HashMap::new();
    input
        .lines()
        .map(|code| {
            shortest_len(&np, &dp, code.to_string(), 0, 2, &mut cache)
                * code[0..3].parse::<usize>().unwrap()
        })
        .sum()
}

// Part 2: Same as part 1 but with 25 layers of robots
pub fn part2(input: &str) -> usize {
    let np = Keypad::numeric();
    let dp = Keypad::directional();
    let mut cache = HashMap::new();
    input
        .lines()
        .map(|code| {
            shortest_len(&np, &dp, code.to_string(), 0, 25, &mut cache)
                * code[0..3].parse::<usize>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_lengths_and_complexity() {
        let np = Keypad::numeric();
        let dp = Keypad::directional();
        let mut cache = HashMap::new();

        // Test cases from the problem statement
        let test_cases = [
            ("029A", 68, 29),
            ("980A", 60, 980),
            ("179A", 68, 179),
            ("456A", 64, 456),
            ("379A", 64, 379),
        ];

        for (code, expected_len, numeric_part) in test_cases {
            let len = shortest_len(&np, &dp, code.to_string(), 0, 2, &mut cache);
            assert_eq!(
                len, expected_len,
                "Sequence length mismatch for code {}",
                code
            );

            let complexity = len * numeric_part;
            println!(
                "Code: {}, Length: {}, Numeric: {}, Complexity: {}",
                code, len, numeric_part, complexity
            );
        }

        // Test total complexity for part 1
        let input = "029A\n980A\n179A\n456A\n379A";
        assert_eq!(part1(input), 126384);

        // Test part 2 with same input but 25 robots
        let mut cache = HashMap::new();
        for (code, _, numeric_part) in test_cases {
            let len = shortest_len(&np, &dp, code.to_string(), 0, 25, &mut cache);
            println!(
                "Part 2 - Code: {}, Length: {}, Numeric: {}, Complexity: {}",
                code,
                len,
                numeric_part,
                len * numeric_part
            );
        }
    }
}

use crate::Solution;

pub struct Day21;

impl Solution for Day21 {
    fn part1(&self, input: &str) -> String {
        part1(input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        part2(input).to_string()
    }
}
