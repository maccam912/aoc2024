use crate::Solution;
use std::collections::HashMap;
use std::any::Any;

#[derive(Debug)]
pub struct Day15;

impl Solution for Day15 {
    fn part1(&self, input: &str) -> String {
        todo!("Implement part 1")
    }

    fn part2(&self, input: &str) -> String {
        todo!("Implement part 2")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    row: i32,
    col: i32,
}

#[derive(Debug, Clone)]
pub struct Robot {
    pos: Coordinate,
}

#[derive(Debug, Clone)]
pub struct SingleCrate {
    pos: Coordinate,
}

#[derive(Debug, Clone)]
pub struct DoubleCrate {
    pos: Coordinate, // tracks left side of the crate
}

#[derive(Debug, Clone)]
pub struct Wall {
    pos: Coordinate,
}

impl Robot {
    pub fn new(row: i32, col: i32) -> Self {
        Self {
            pos: Coordinate { row, col },
        }
    }
}

impl SingleCrate {
    pub fn new(row: i32, col: i32) -> Self {
        Self {
            pos: Coordinate { row, col },
        }
    }
}

impl DoubleCrate {
    pub fn new(row: i32, col: i32) -> Self {
        Self {
            pos: Coordinate { row, col },
        }
    }
}

impl Wall {
    pub fn new(row: i32, col: i32) -> Self {
        Self {
            pos: Coordinate { row, col },
        }
    }
}

pub trait HasPosition {
    fn pos(&self) -> &Coordinate;
}

impl HasPosition for Robot {
    fn pos(&self) -> &Coordinate {
        &self.pos
    }
}

impl HasPosition for SingleCrate {
    fn pos(&self) -> &Coordinate {
        &self.pos
    }
}

impl HasPosition for DoubleCrate {
    fn pos(&self) -> &Coordinate {
        &self.pos
    }
}

impl HasPosition for Wall {
    fn pos(&self) -> &Coordinate {
        &self.pos
    }
}

#[derive(Debug, Clone)]
enum GameObject {
    Robot(Robot),
    SingleCrate(SingleCrate),
    DoubleCrate(DoubleCrate),
    Wall(Wall),
}

impl GameObject {
    fn pos(&self) -> &Coordinate {
        match self {
            GameObject::Robot(r) => r.pos(),
            GameObject::SingleCrate(c) => c.pos(),
            GameObject::DoubleCrate(d) => d.pos(),
            GameObject::Wall(w) => w.pos(),
        }
    }
}

struct Warehouse {
    objects: HashMap<isize, GameObject>, // Owns the objects, uses an ID to keep track of them
    grid: HashMap<Coordinate, isize>, // Maps coordinates to object IDs
    width: i32,
    height: i32,
    robot_location: Coordinate,
}

impl Warehouse {
    fn new(width: i32, height: i32) -> Self {
        Self {
            objects: HashMap::new(),
            grid: HashMap::new(),
            width,
            height,
            robot_location: Coordinate { row: 0, col: 0 },
        }
    }

    fn add_object(&mut self, obj: GameObject) -> isize {
        let id = self.objects.len() as isize;
        self.objects.insert(id, obj.clone());
        self.grid.insert(obj.pos().clone(), id);
        // If GameObject is a DoubleCrate, also add the right side
        if let GameObject::DoubleCrate(dc) = obj {
            self.grid.insert(Coordinate { row: dc.pos().row, col: dc.pos().col + 1 }, id);
        }
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "";

    #[test]
    fn test_part1_sample() {
        let day = Day15;
        assert_eq!(day.part1(SAMPLE), "0");
    }

    #[test]
    fn test_part2_sample() {
        let day = Day15;
        assert_eq!(day.part2(SAMPLE), "0");
    }
}