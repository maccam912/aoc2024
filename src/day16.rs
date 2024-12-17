use crate::Solution;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: (usize, usize),
    dir: Direction,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    fn step(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        let (row, col) = pos;
        match self {
            Direction::North if row > 0 => Some((row - 1, col)),
            Direction::South => Some((row + 1, col)),
            Direction::East => Some((row, col + 1)),
            Direction::West if col > 0 => Some((row, col - 1)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    state: State,
    cost: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost).then_with(|| {
            let (r1, c1) = self.state.pos;
            let (r2, c2) = other.state.pos;
            (r1, c1).cmp(&(r2, c2))
        })
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Day16;

impl Solution for Day16 {
    fn part1(&self, input: &str) -> String {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let rows = grid.len();
        let cols = grid[0].len();

        // Find start and end positions
        let mut start_pos = (0, 0);
        let mut end_pos = (0, 0);
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == 'S' {
                    start_pos = (i, j);
                } else if grid[i][j] == 'E' {
                    end_pos = (i, j);
                }
            }
        }

        let start_state = State {
            pos: start_pos,
            dir: Direction::East,
        };

        let mut heap = BinaryHeap::new();
        let mut dist = HashMap::new();

        heap.push(Node {
            state: start_state,
            cost: 0,
        });
        dist.insert(start_state, 0);

        while let Some(Node { state, cost }) = heap.pop() {
            if state.pos == end_pos {
                return cost.to_string();
            }

            if cost > dist[&state] {
                continue;
            }

            // Try moving forward
            if let Some(next_pos) = state.dir.step(state.pos) {
                if next_pos.0 < rows && next_pos.1 < cols && grid[next_pos.0][next_pos.1] != '#' {
                    let next_state = State {
                        pos: next_pos,
                        dir: state.dir,
                    };
                    let next_cost = cost + 1;

                    if !dist.contains_key(&next_state) || next_cost < dist[&next_state] {
                        dist.insert(next_state, next_cost);
                        heap.push(Node {
                            state: next_state,
                            cost: next_cost,
                        });
                    }
                }
            }

            // Try turning left
            let left_dir = state.dir.turn_left();
            let left_state = State {
                pos: state.pos,
                dir: left_dir,
            };
            let left_cost = cost + 1000;

            if !dist.contains_key(&left_state) || left_cost < dist[&left_state] {
                dist.insert(left_state, left_cost);
                heap.push(Node {
                    state: left_state,
                    cost: left_cost,
                });
            }

            // Try turning right
            let right_dir = state.dir.turn_right();
            let right_state = State {
                pos: state.pos,
                dir: right_dir,
            };
            let right_cost = cost + 1000;

            if !dist.contains_key(&right_state) || right_cost < dist[&right_state] {
                dist.insert(right_state, right_cost);
                heap.push(Node {
                    state: right_state,
                    cost: right_cost,
                });
            }
        }

        "No path found".to_string()
    }

    fn part2(&self, input: &str) -> String {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let rows = grid.len();
        let cols = grid[0].len();

        // Find start and end positions
        let mut start_pos = (0, 0);
        let mut end_pos = (0, 0);
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == 'S' {
                    start_pos = (i, j);
                } else if grid[i][j] == 'E' {
                    end_pos = (i, j);
                }
            }
        }

        let start_state = State {
            pos: start_pos,
            dir: Direction::East,
        };

        let mut heap = BinaryHeap::new();
        let mut dist = HashMap::new();
        let mut visited_tiles = HashSet::new();
        let mut min_cost = None;
        let mut parents: HashMap<State, HashSet<State>> = HashMap::new();

        heap.push(Node {
            state: start_state,
            cost: 0,
        });
        dist.insert(start_state, 0);

        while let Some(Node { state, cost }) = heap.pop() {
            if let Some(mc) = min_cost {
                if cost > mc {
                    break; // Found a path worse than the best, we're done
                }
            }

            if state.pos == end_pos {
                if min_cost.is_none() {
                    min_cost = Some(cost);
                }
                // Add all positions in this path to visited_tiles using DFS
                let mut stack = vec![state];
                let mut seen = HashSet::new();
                while let Some(current) = stack.pop() {
                    if seen.insert(current) {
                        visited_tiles.insert(current.pos);
                        if let Some(prev_states) = parents.get(&current) {
                            stack.extend(prev_states);
                        }
                    }
                }
                continue;
            }

            if cost > dist[&state] {
                continue;
            }

            // Try moving forward
            if let Some(next_pos) = state.dir.step(state.pos) {
                if next_pos.0 < rows && next_pos.1 < cols && grid[next_pos.0][next_pos.1] != '#' {
                    let next_state = State {
                        pos: next_pos,
                        dir: state.dir,
                    };
                    let next_cost = cost + 1;

                    if !dist.contains_key(&next_state) || next_cost < dist[&next_state] {
                        dist.insert(next_state, next_cost);
                        parents.insert(next_state, {
                            let mut set = HashSet::new();
                            set.insert(state);
                            set
                        });
                        heap.push(Node {
                            state: next_state,
                            cost: next_cost,
                        });
                    } else if next_cost == dist[&next_state] {
                        // Another path with same cost
                        parents
                            .entry(next_state)
                            .or_insert_with(HashSet::new)
                            .insert(state);
                    }
                }
            }

            // Try turning left
            let left_dir = state.dir.turn_left();
            let left_state = State {
                pos: state.pos,
                dir: left_dir,
            };
            let left_cost = cost + 1000;

            if !dist.contains_key(&left_state) || left_cost < dist[&left_state] {
                dist.insert(left_state, left_cost);
                parents.insert(left_state, {
                    let mut set = HashSet::new();
                    set.insert(state);
                    set
                });
                heap.push(Node {
                    state: left_state,
                    cost: left_cost,
                });
            } else if left_cost == dist[&left_state] {
                // Another path with same cost
                parents
                    .entry(left_state)
                    .or_insert_with(HashSet::new)
                    .insert(state);
            }

            // Try turning right
            let right_dir = state.dir.turn_right();
            let right_state = State {
                pos: state.pos,
                dir: right_dir,
            };
            let right_cost = cost + 1000;

            if !dist.contains_key(&right_state) || right_cost < dist[&right_state] {
                dist.insert(right_state, right_cost);
                parents.insert(right_state, {
                    let mut set = HashSet::new();
                    set.insert(state);
                    set
                });
                heap.push(Node {
                    state: right_state,
                    cost: right_cost,
                });
            } else if right_cost == dist[&right_state] {
                // Another path with same cost
                parents
                    .entry(right_state)
                    .or_insert_with(HashSet::new)
                    .insert(state);
            }
        }

        visited_tiles.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part1_sample() {
        let input = read_input(16, true);
        assert_eq!(Day16.part1(&input), "7036");
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input(16, true);
        assert_eq!(Day16.part2(&input), "45");
    }
}
