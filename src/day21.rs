use crate::Solution;

struct Coordinate {
    x: i32,
    y: i32,
}

struct KeypadRobot {
    pos: Coordinate,
    actions: Vec<char>,
    presses: Vec<char>,
}

impl KeypadRobot {
    fn new() -> Self {
        KeypadRobot {
            pos: Coordinate { x: 2, y: 3 }, // Start at 2,3 for the A button
            actions: Vec::new(),
            presses: Vec::new(),
        }
    }

    fn do_action(&mut self, action: char) {
        match action {
            '^' => self.pos.y -= 1,
            'v' => self.pos.y += 1,
            '<' => self.pos.x -= 1,
            '>' => self.pos.x += 1,
            'A' => {
                // This pushes the button you are on. Given the coordinates you're at will determine what to add to presses
                // Keypad is in this layout with 0,0 at top left:
                // 7 8 9
                // 4 5 6
                // 1 2 3
                //   0 A (blank left of zero)
                match (self.pos.x, self.pos.y) {
                    (0, 0) => self.presses.push('7'),
                    (1, 0) => self.presses.push('8'),
                    (2, 0) => self.presses.push('9'),
                    (0, 1) => self.presses.push('4'),
                    (1, 1) => self.presses.push('5'),
                    (2, 1) => self.presses.push('6'),
                    (0, 2) => self.presses.push('1'),
                    (1, 2) => self.presses.push('2'),
                    (2, 2) => self.presses.push('3'),
                    (1, 3) => self.presses.push('0'),
                    (2, 3) => self.presses.push('A'),
                    _ => panic!("Invalid position for A button: x={}, y={}", self.pos.x, self.pos.y),
                }
            }
            _ => panic!("Invalid action: {}", action),
        }
        self.actions.push(action);
    }
}

pub struct Day21;

impl Solution for Day21 {
    fn part1(&self, input: &str) -> String {
        // TODO: Implement solution
        "Not implemented".to_string()
    }

    fn part2(&self, input: &str) -> String {
        // TODO: Implement solution
        "Not implemented".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part1_sample() {
        let input = read_input(21, true);
        assert_eq!(Day21.part1(&input), "Not implemented");
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input(21, true);
        assert_eq!(Day21.part2(&input), "Not implemented");
    }

    #[test]
    fn test_robot_actions_and_presses() {
        let mut robot = KeypadRobot::new();
        let actions = "<A^A>^^AvvvA";
        
        for c in actions.chars() {
            robot.do_action(c);
        }

        assert_eq!(robot.actions, actions.chars().collect::<Vec<char>>());
        assert_eq!(robot.presses, vec!['0', '2', '9', 'A']);
    }
}