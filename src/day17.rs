use crate::Solution;

#[derive(Debug, Default)]
struct Computer {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    ip: usize,
    program: Vec<u8>,
    output: Vec<u8>,
    debug: bool,
}

impl Computer {
    fn new(program: Vec<u8>, reg_a: i64, reg_b: i64, reg_c: i64) -> Self {
        Self {
            reg_a,
            reg_b,
            reg_c,
            ip: 0,
            program,
            output: Vec::new(),
            debug: true,
        }
    }

    fn with_debug(mut self) -> Self {
        self.debug = true;
        self
    }

    fn get_combo_value(&self, operand: u8) -> i64 {
        match operand {
            0..=3 => operand as i64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => panic!("Invalid combo operand 7"),
            _ => unreachable!(),
        }
    }

    fn get_opcode_name(&self, opcode: u8) -> &'static str {
        match opcode {
            0 => "adv (divide reg_a by 2^operand)",
            1 => "bxl (xor reg_b with operand)",
            2 => "bst (set reg_b to operand mod 8)",
            3 => "jnz (jump to operand if reg_a != 0)",
            4 => "bxc (xor reg_b with reg_c)",
            5 => "out (output operand mod 8)",
            6 => "bdv (set reg_b to reg_a / 2^operand)",
            7 => "cdv (set reg_c to reg_a / 2^operand)",
            _ => "invalid opcode",
        }
    }

    fn display_state(&self) {
        // Convert instruction pointer to arrow display
        let mut ip_display = vec![' '; self.program.len()];
        if self.ip + 1 < self.program.len() {
            ip_display[self.ip] = '>';
            ip_display[self.ip + 1] = '>';
        }

        // Display program with instruction pointer
        println!("\nProgram:");
        for i in (0..self.program.len()).step_by(2) {
            if i + 1 < self.program.len() {
                println!("{}{} {} {} \t# {}", 
                    ip_display[i], 
                    ip_display[i+1],
                    self.program[i],
                    self.program[i+1],
                    self.get_opcode_name(self.program[i])
                );
            }
        }

        // Display registers in binary
        println!("\nRegisters:");
        println!("A: {:032b} ({})", self.reg_a as u32, self.reg_a);
        println!("B: {:032b} ({})", self.reg_b as u32, self.reg_b);
        println!("C: {:032b} ({})", self.reg_c as u32, self.reg_c);
        
        // Display output
        println!("\nOutput so far:");
        if self.output.is_empty() {
            println!("(none)");
        } else {
            println!("{:?}", self.output);
        }
        
        println!("\nPress ENTER to continue...");
    }

    fn run(&mut self) {
        use std::io::{self, Read};

        while self.ip + 1 < self.program.len() {
            if self.debug {
                self.display_state();
                
                // Wait for enter key
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer).unwrap();
            }

            let opcode = self.program[self.ip];
            let operand = self.program[self.ip + 1];
            self.ip += 2; 

            match opcode {
                0 => { // adv
                    let power = self.get_combo_value(operand);
                    self.reg_a /= 1 << power;
                }
                1 => { // bxl
                    self.reg_b ^= operand as i64;
                }
                2 => { // bst
                    self.reg_b = self.get_combo_value(operand) % 8;
                }
                3 => { // jnz
                    if self.reg_a != 0 {
                        self.ip = operand as usize;
                    } 
                }
                4 => { // bxc
                    self.reg_b ^= self.reg_c;
                }
                5 => { // out
                    let value = (self.get_combo_value(operand) % 8) as u8;
                    self.output.push(value);
                }
                6 => { // bdv
                    let power = self.get_combo_value(operand);
                    self.reg_b = self.reg_a / (1 << power);
                }
                7 => { // cdv
                    let power = self.get_combo_value(operand);
                    self.reg_c = self.reg_a / (1 << power);
                }
                _ => panic!("Invalid opcode"),
            }
        }
    }
}

pub struct Day17;

impl Solution for Day17 {
    fn part1(&self, input: &str) -> String {
        let mut lines = input.lines();
        
        // Parse register values
        let reg_a = lines.next().unwrap()
            .strip_prefix("Register A: ").unwrap()
            .parse().unwrap();
        let reg_b = lines.next().unwrap()
            .strip_prefix("Register B: ").unwrap()
            .parse().unwrap();
        let reg_c = lines.next().unwrap()
            .strip_prefix("Register C: ").unwrap()
            .parse().unwrap();
        
        // Skip empty line and "Program: " line
        lines.next();
        let program_line = lines.next().unwrap()
            .strip_prefix("Program: ").unwrap();
        
        // Parse program
        let program: Vec<u8> = program_line
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let mut computer = Computer::new(program, reg_a, reg_b, reg_c);
        
        // Enable debug mode if environment variable is set
        if std::env::var("DEBUG").is_ok() {
            computer = computer.with_debug();
        }
        
        computer.run();
        
        computer.output.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn part2(&self, input: &str) -> String {
        let mut lines = input.lines();
        lines.next(); // Skip original A value
        let reg_b = lines.next().unwrap()
            .strip_prefix("Register B: ").unwrap()
            .parse().unwrap();
        let reg_c = lines.next().unwrap()
            .strip_prefix("Register C: ").unwrap()
            .parse().unwrap();
        
        // Skip empty line and get program
        lines.next();
        let program_line = lines.next().unwrap()
            .strip_prefix("Program: ").unwrap();
        let program: Vec<u8> = program_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let target: Vec<u8> = program.clone();
        let mut reg_a = 0i64;
        
        loop {
            let mut computer = Computer::new(program.clone(), reg_a, reg_b, reg_c);
            
            // Enable debug mode if environment variable is set
            if std::env::var("DEBUG").is_ok() {
                computer = computer.with_debug();
            }
            
            computer.run();
            
            if computer.output.len() == target.len() && computer.output == target {
                break;
            }
            reg_a += 1;
            
            // Temporary safety check for development
            if reg_a > 1_000_000_000 {
                return "Exceeded maximum iterations".to_string();
            }
        }
        
        reg_a.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_outputs() {
        // Test case 1: If register C contains 9, the program 2,6 would set register B to 1
        let mut computer = Computer::new(vec![2, 6], 0, 0, 9);
        computer.run();
        assert_eq!(computer.reg_b, 1);

        // Test case 2: If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2
        let mut computer = Computer::new(vec![5, 0, 5, 1, 5, 4], 10, 0, 0);
        computer.run();
        assert_eq!(computer.output, vec![0, 1, 2]);

        // Test case 3: If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0
        let mut computer = Computer::new(vec![0, 1, 5, 4, 3, 0], 2024, 0, 0);
        computer.run();
        assert_eq!(computer.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);

        // Test case 4: If register B contains 29, the program 1,7 would set register B to 26
        let mut computer = Computer::new(vec![1, 7], 0, 29, 0);
        computer.run();
        assert_eq!(computer.reg_b, 26);

        // Test case 5: If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354
        let mut computer = Computer::new(vec![4, 0], 0, 2024, 43690);
        computer.run();
        assert_eq!(computer.reg_b, 44354);
    }

    #[test]
    fn test_debugger_example() {
        let input = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!(Day17.part1(input), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2_example() {
        let input = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!(Day17.part2(input), "117440");
    }
}
