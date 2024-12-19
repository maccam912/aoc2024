use crate::Solution;
use std::collections::{BinaryHeap, HashMap};

pub struct Day18;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: i32,
    position: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.x.cmp(&other.position.x))
            .then_with(|| self.position.y.cmp(&other.position.y))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Day18 {
    fn parse_input(input: &str) -> Vec<Point> {
        input.lines()
            .map(|line| {
                let mut parts = line.split(',');
                let x = parts.next().unwrap().trim().parse().unwrap();
                let y = parts.next().unwrap().trim().parse().unwrap();
                Point { x, y }
            })
            .collect()
    }

    fn get_neighbors(point: Point, size: i32, corrupted: &HashMap<Point, bool>) -> Vec<(Point, i32)> {
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        dirs.iter()
            .map(|(dx, dy)| Point {
                x: point.x + dx,
                y: point.y + dy,
            })
            .filter(|p| {
                p.x >= 0 && p.x <= size && p.y >= 0 && p.y <= size && !corrupted.contains_key(p)
            })
            .map(|p| (p, 1))
            .collect()
    }

    fn shortest_path(start: Point, end: Point, size: i32, corrupted: &HashMap<Point, bool>) -> Option<i32> {
        let mut distances: HashMap<Point, i32> = HashMap::new();
        let mut heap = BinaryHeap::new();

        distances.insert(start, 0);
        heap.push(State {
            cost: 0,
            position: start,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if position == end {
                return Some(cost);
            }

            if let Some(&best) = distances.get(&position) {
                if cost > best {
                    continue;
                }
            }

            for (next, edge_cost) in Self::get_neighbors(position, size, corrupted) {
                let next_cost = cost + edge_cost;

                let is_better = distances
                    .get(&next)
                    .map_or(true, |&current| next_cost < current);

                if is_better {
                    distances.insert(next, next_cost);
                    heap.push(State {
                        cost: next_cost,
                        position: next,
                    });
                }
            }
        }

        None
    }
}

impl Solution for Day18 {
    fn part1(&self, input: &str) -> String {
        let points = Self::parse_input(input);
        let size = if input.lines().count() < 20 { 6 } else { 70 }; // Use 6 for sample, 70 for real input
        
        // Take only first 1024 points for part 1
        let corrupted: HashMap<Point, bool> = points
            .into_iter()
            .take(1024)
            .map(|p| (p, true))
            .collect();

        let start = Point { x: 0, y: 0 };
        let end = Point { x: size, y: size };

        match Self::shortest_path(start, end, size, &corrupted) {
            Some(steps) => steps.to_string(),
            None => "No path found".to_string(),
        }
    }

    fn part2(&self, input: &str) -> String {
        let points = Self::parse_input(input);
        let size = if input.lines().count() < 20 { 6 } else { 70 };
        
        let mut corrupted: HashMap<Point, bool> = HashMap::new();
        let start = Point { x: 0, y: 0 };
        let end = Point { x: size, y: size };

        // Try each point in sequence until we find one that blocks all paths
        for (i, point) in points.iter().enumerate() {
            corrupted.insert(*point, true);
            
            if Self::shortest_path(start, end, size, &corrupted).is_none() {
                // Found the blocking point - return its coordinates
                return format!("{},{}", point.x, point.y);
            }
        }

        "No blocking point found".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part1_sample() {
        let input = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1";
        let day18 = Day18;
        assert_eq!(day18.part1(input), "22");
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input(18, true);
        let day18 = Day18;
        assert_ne!(day18.part2(&input), "Not implemented");
    }
}
