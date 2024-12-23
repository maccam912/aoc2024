use crate::Solution;

#[derive(Debug, Clone)]
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
                    _ => panic!(
                        "Invalid position for A button: x={}, y={}",
                        self.pos.x, self.pos.y
                    ),
                }
            }
            _ => panic!("Invalid action: {}", action),
        }
        self.actions.push(action);
        println!(
            "KeypadRobot - Action: {}, Now at: ({}, {})",
            action, self.pos.x, self.pos.y
        );
    }

    fn path_to_button(&self, button: char, start_pos: Coordinate) -> (Vec<char>, Coordinate) {
        // button is the target. Using the keypad layout above first get the coordintes of the button
        let target = match button {
            '7' => Coordinate { x: 0, y: 0 },
            '8' => Coordinate { x: 1, y: 0 },
            '9' => Coordinate { x: 2, y: 0 },
            '4' => Coordinate { x: 0, y: 1 },
            '5' => Coordinate { x: 1, y: 1 },
            '6' => Coordinate { x: 2, y: 1 },
            '1' => Coordinate { x: 0, y: 2 },
            '2' => Coordinate { x: 1, y: 2 },
            '3' => Coordinate { x: 2, y: 2 },
            '0' => Coordinate { x: 1, y: 3 },
            'A' => Coordinate { x: 2, y: 3 },
            _ => panic!("Invalid button: {}", button),
        };
        // Then we have our current coordinate. Lets build it in parts - sequence to move some distance right
        let mut path = Vec::new();
        let mut current = start_pos;
        if current.x < target.x {
            path.push('>');
            current.x += 1;
        }
        // Then sequence for up
        while current.y > target.y {
            path.push('^');
            current.y -= 1;
        }
        // Then sequence for down
        while current.y < target.y {
            path.push('v');
            current.y += 1;
        }
        // Finally left
        while current.x > target.x {
            path.push('<');
            current.x -= 1;
        }
        (path, current)
    }

    fn build_sequence(&self, buttons: Vec<char>) -> Vec<char> {
        let mut path = Vec::new();
        let mut current = self.pos.clone();
        for button in buttons {
            let (button_path, button_pos) = self.path_to_button(button, current);
            path.extend(button_path);
            path.push('A');
            current = button_pos;
        }
        path
    }
}

struct DPadRobot {
    pos: Coordinate,
    actions: Vec<char>,
    presses: Vec<char>,
}

impl DPadRobot {
    fn new() -> Self {
        DPadRobot {
            pos: Coordinate { x: 2, y: 0 }, // Start at 2,0 for the A button
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
                // DPad this time is this layout with 0,0 at top left:
                //   ^ A (blank space left of ^)
                // < v >
                match (self.pos.x, self.pos.y) {
                    (1, 0) => self.presses.push('^'),
                    (2, 0) => self.presses.push('A'),
                    (0, 1) => self.presses.push('<'),
                    (1, 1) => self.presses.push('v'),
                    (2, 1) => self.presses.push('>'),
                    _ => panic!(
                        "Invalid position for A button: x={}, y={}",
                        self.pos.x, self.pos.y
                    ),
                }
            }
            _ => panic!("Invalid action: {}", action),
        }
        self.actions.push(action);
        println!(
            "DPadRobot - Action: {}, Now at: ({}, {})",
            action, self.pos.x, self.pos.y
        );
    }

    fn path_to_button(&self, button: char, start_pos: Coordinate) -> (Vec<char>, Coordinate) {
        // button is the target. Using the dpad layout above first get the coordintes of the button
        let target = match button {
            '^' => Coordinate { x: 1, y: 0 },
            'A' => Coordinate { x: 2, y: 0 },
            '<' => Coordinate { x: 0, y: 1 },
            'v' => Coordinate { x: 1, y: 1 },
            '>' => Coordinate { x: 2, y: 1 },
            _ => panic!("Invalid button: {}", button),
        };
        // Then we have our current coordinate. Lets build it in parts - sequence to move some distance right
        let mut path = Vec::new();
        let mut current = start_pos;
        while current.x < target.x {
            path.push('>');
            current.x += 1;
        }
        // Then sequence for up
        while current.y > target.y {
            path.push('^');
            current.y -= 1;
        }
        // Then sequence for down
        while current.y < target.y {
            path.push('v');
            current.y += 1;
        }
        // Finally left
        while current.x > target.x {
            path.push('<');
            current.x -= 1;
        }
        (path, current)
    }

    fn build_sequence(&self, buttons: Vec<char>) -> Vec<char> {
        let mut path = Vec::new();
        let mut current = self.pos.clone();
        for button in buttons {
            let (button_path, button_pos) = self.path_to_button(button, current);
            path.extend(button_path);
            path.push('A');
            println!("Coordinate: ({},{})", button_pos.x, button_pos.y);
            current = button_pos;
        }
        path
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

    #[test]
    fn test_build_sequence() {
        let robot = KeypadRobot::new();
        let buttons = vec!['0', '2', '9', 'A'];
        let sequence = robot.build_sequence(buttons);
        assert_eq!(sequence, "<A^A>^^AvvvA".chars().collect::<Vec<char>>());
    }

    #[test]
    fn test_dpad_robot_actions_and_presses() {
        let mut robot = DPadRobot::new();
        let actions = "v<<A>>^A<A>AvA^<AA>Av<AAA>^A";

        for c in actions.chars() {
            robot.do_action(c);
        }

        assert_eq!(robot.actions, actions.chars().collect::<Vec<char>>());
        assert_eq!(robot.presses, "<A^A>^^AvvvA".chars().collect::<Vec<char>>());
    }

    #[test]
    fn test_build_dpad_sequence_1() {
        let robot = DPadRobot::new();
        let buttons = "AAAA".chars().collect::<Vec<char>>();
        let sequence = robot.build_sequence(buttons);
        assert_eq!(sequence, "AAAA".chars().collect::<Vec<char>>());
    }

    #[test]
    fn test_build_dpad_sequence_2() {
        let robot = DPadRobot::new();
        let buttons = "^^^^A".chars().collect::<Vec<char>>();
        let sequence = robot.build_sequence(buttons);
        assert_eq!(sequence, "<AAAA>A".chars().collect::<Vec<char>>());
    }

    #[test]
    fn test_build_dpad_sequence_3() {
        let robot = DPadRobot::new();
        let buttons = "<A^A>".chars().collect::<Vec<char>>();
        let sequence = robot.build_sequence(buttons);
        assert_eq!(sequence, "v<<A>>^A<A>AvA".chars().collect::<Vec<char>>());
    }

    #[test]
    fn test_build_dpad_sequence_4() {
        let robot = DPadRobot::new();
        let buttons = ">".chars().collect::<Vec<char>>();
        let sequence = robot.build_sequence(buttons);
        assert_eq!(sequence, "vA".chars().collect::<Vec<char>>());
    }

    #[test]
    fn test_part_1_sequence_029a() {
        let robot = KeypadRobot::new();
        let buttons = "029A".chars().collect::<Vec<char>>();
        let sequence = robot.build_sequence(buttons);
        assert_eq!(sequence, "<A^A>^^AvvvA".chars().collect::<Vec<char>>());
        let robot2 = DPadRobot::new();
        let sequence2 = robot2.build_sequence(sequence);
        assert_eq!(
            sequence2,
            "v<<A>>^A<A>AvA^<AA>Av<AAA>^A"
                .chars()
                .collect::<Vec<char>>()
        );
        let robot3 = DPadRobot::new();
        let sequence3 = robot3.build_sequence(sequence2);
        assert_eq!(
            sequence3.len(),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );
    }
}
