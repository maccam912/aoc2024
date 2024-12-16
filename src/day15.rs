use crate::Solution;
use std::collections::HashMap;
use std::any::Any;

#[derive(Debug)]
pub struct Day15;

impl Solution for Day15 {
    fn part1(&self, _input: &str) -> String {
        todo!("Implement part 1")
    }

    fn part2(&self, _input: &str) -> String {
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

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::North,
            'v' => Direction::South,
            '>' => Direction::East,
            '<' => Direction::West,
            _ => panic!("Invalid direction character: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    row: i32,
    col: i32,
}

impl Coordinate {
    fn next_in_direction(&self, dir: Direction) -> Coordinate {
        match dir {
            Direction::North => Coordinate { row: self.row - 1, col: self.col },
            Direction::South => Coordinate { row: self.row + 1, col: self.col },
            Direction::East => Coordinate { row: self.row, col: self.col + 1 },
            Direction::West => Coordinate { row: self.row, col: self.col - 1 },
        }
    }
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

    fn get_object_at(&self, pos: &Coordinate) -> Option<&GameObject> {
        self.grid.get(pos).map(|id| &self.objects[id])
    }

    fn can_move_chain(&self, start_pos: &Coordinate, dir: Direction) -> bool {
        let next_pos = start_pos.next_in_direction(dir);
        
        match self.get_object_at(&next_pos) {
            None => true, // Empty space
            Some(GameObject::Wall(_)) => false,
            Some(GameObject::SingleCrate(_)) => {
                // Recursively check if the crate can move
                self.can_move_chain(&next_pos, dir)
            }
            Some(GameObject::DoubleCrate(_)) => false, // Not implementing double crates yet
            Some(GameObject::Robot(_)) => false, // Should never happen in valid puzzle
        }
    }

    fn move_objects(&mut self, start_pos: &Coordinate, dir: Direction) -> bool {
        if !self.can_move_chain(start_pos, dir) {
            return false;
        }

        let next_pos = start_pos.next_in_direction(dir);
        
        // If there's a crate at the next position, move it first
        if let Some(obj_id) = self.grid.get(&next_pos).copied() {
            if let GameObject::SingleCrate(_) = &self.objects[&obj_id] {
                // Move the crate first
                self.move_objects(&next_pos, dir);
                
                // Update crate position
                if let GameObject::SingleCrate(ref mut crate_obj) = self.objects.get_mut(&obj_id).unwrap() {
                    let new_pos = next_pos.next_in_direction(dir);
                    self.grid.remove(&next_pos);
                    self.grid.insert(new_pos, obj_id);
                    crate_obj.pos = new_pos;
                }
            }
        }

        // Now move the robot
        if let Some(robot_id) = self.grid.get(start_pos).copied() {
            if let GameObject::Robot(ref mut robot) = self.objects.get_mut(&robot_id).unwrap() {
                self.grid.remove(start_pos);
                self.grid.insert(next_pos, robot_id);
                robot.pos = next_pos;
                self.robot_location = next_pos;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn execute_move(&mut self, dir: Direction) -> bool {
        let start_pos = self.robot_location;
        self.move_objects(&start_pos, dir)
    }

    pub fn parse(input: &str) -> (Self, Vec<Direction>) {
        let lines: Vec<&str> = input.lines().collect();
        
        let height = lines.len() as i32;
        let width = lines[0].len() as i32;
        
        let mut warehouse = Self::new(width, height);
        let mut robot_found = false;
        
        // First part is the map
        for (row, line) in lines.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let pos = Coordinate { row: row as i32, col: col as i32 };
                match c {
                    '#' => {
                        warehouse.add_object(GameObject::Wall(Wall::new(pos.row, pos.col)));
                    }
                    '@' => {
                        if robot_found {
                            panic!("Multiple robots found in map");
                        }
                        let robot = Robot::new(pos.row, pos.col);
                        warehouse.robot_location = pos;
                        warehouse.add_object(GameObject::Robot(robot));
                        robot_found = true;
                    }
                    'O' => {
                        warehouse.add_object(GameObject::SingleCrate(SingleCrate::new(pos.row, pos.col)));
                    }
                    '.' => (), // Empty space
                    _ => panic!("Invalid map character: {}", c),
                }
            }
        }
        
        if !robot_found {
            panic!("No robot found in map");
        }

        (warehouse, Vec::new()) // For now, return empty movement sequence
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_from_char() {
        assert_eq!(Direction::from('^'), Direction::North);
        assert_eq!(Direction::from('v'), Direction::South);
        assert_eq!(Direction::from('>'), Direction::East);
        assert_eq!(Direction::from('<'), Direction::West);
    }

    #[test]
    #[should_panic(expected = "Invalid direction character")]
    fn test_invalid_direction() {
        let _ = Direction::from('x');
    }

    #[test]
    fn test_warehouse_parsing() {
        let input = "
#####
#@O.#
#####".trim();
        let (warehouse, _) = Warehouse::parse(input);
        
        // Check dimensions
        assert_eq!(warehouse.width, 5);
        assert_eq!(warehouse.height, 3);
        
        // Check robot position
        assert_eq!(warehouse.robot_location, Coordinate { row: 1, col: 1 });
        
        // Check crate position
        let crate_pos = Coordinate { row: 1, col: 2 };
        assert!(matches!(warehouse.get_object_at(&crate_pos), Some(GameObject::SingleCrate(_))));
        
        // Check wall positions
        for row in 0..3 {
            for col in 0..5 {
                let pos = Coordinate { row, col };
                if row == 0 || row == 2 || col == 0 || col == 4 {
                    assert!(matches!(warehouse.get_object_at(&pos), Some(GameObject::Wall(_))));
                }
            }
        }
    }

    #[test]
    fn test_robot_movement_empty_space() {
        let input = "
#####
#@..#
#####".trim();
        let (mut warehouse, _) = Warehouse::parse(input);
        
        assert!(warehouse.execute_move(Direction::East));
        assert_eq!(warehouse.robot_location, Coordinate { row: 1, col: 2 });
    }

    #[test]
    fn test_robot_blocked_by_wall() {
        let input = "
####
#@.#
####".trim();
        let (mut warehouse, _) = Warehouse::parse(input);
        
        assert!(!warehouse.execute_move(Direction::North));
        assert_eq!(warehouse.robot_location, Coordinate { row: 1, col: 1 });
    }

    #[test]
    fn test_robot_push_single_crate() {
        let input = "
#####
#@O.#
#####".trim();
        let (mut warehouse, _) = Warehouse::parse(input);
        
        assert!(warehouse.execute_move(Direction::East));
        // Check robot moved
        assert_eq!(warehouse.robot_location, Coordinate { row: 1, col: 2 });
        // Check crate moved
        let crate_pos = Coordinate { row: 1, col: 3 };
        assert!(matches!(warehouse.get_object_at(&crate_pos), Some(GameObject::SingleCrate(_))));
    }

    #[test]
    fn test_robot_push_crate_blocked() {
        let input = "
#####
#@O##
#####".trim();
        let (mut warehouse, _) = Warehouse::parse(input);
        
        assert!(!warehouse.execute_move(Direction::East));
        // Check neither robot nor crate moved
        assert_eq!(warehouse.robot_location, Coordinate { row: 1, col: 1 });
        let crate_pos = Coordinate { row: 1, col: 2 };
        assert!(matches!(warehouse.get_object_at(&crate_pos), Some(GameObject::SingleCrate(_))));
    }
}