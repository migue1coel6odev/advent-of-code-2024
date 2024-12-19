use crate::reverse_computer::ReverseComputer;

pub struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    program: Vec<usize>,
    pointer: usize,
    result: Vec<usize>,
}

impl Computer {
    pub fn new(
        register_a: usize,
        register_b: usize,
        register_c: usize,
        program: Vec<usize>,
    ) -> Self {
        Self {
            register_a,
            register_b,
            register_c,
            program,
            pointer: 0,
            result: vec![],
        }
    }

    pub fn run(&mut self) -> String {
        loop {
            if let Some(next_instruction) = self.get_next_instruction() {
                self.run_instruction(next_instruction);
            } else {
                break;
            }
        }

        self.result
            .iter()
            .flat_map(|f| Computer::separate_number(*f))
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn run_instruction(&mut self, instruction: usize) {
        match instruction {
            0 => {
                if let Some(operand) = self.get_next_instruction() {
                    self.register_a =
                        self.register_a / (2_usize.pow(self.get_value_from_opcode(operand) as u32));
                }
            }
            1 => {
                if let Some(operand) = self.get_next_instruction() {
                    self.register_b = self.register_b ^ operand;
                }
            }
            2 => {
                if let Some(operand) = self.get_next_instruction() {
                    self.register_b = self.get_value_from_opcode(operand) % 8;
                }
            }
            3 => {
                if self.register_a != 0 {
                    if let Some(operand) = self.get_next_instruction() {
                        self.pointer = operand;
                    }
                }
            }
            4 => {
                let _ = self.get_next_instruction();
                self.register_b = self.register_b ^ self.register_c;
            }
            5 => {
                if let Some(operand) = self.get_next_instruction() {
                    self.result.push(self.get_value_from_opcode(operand) % 8);
                }
            }
            6 | 7 => {
                if let Some(operand) = self.get_next_instruction() {
                    self.register_c =
                        self.register_a / (2_usize.pow(self.get_value_from_opcode(operand) as u32));
                }
            }
            _ => (),
        }
    }

    pub fn reverse(&mut self) -> usize {
        let mut reverse_computer = ReverseComputer::new(
            self.register_a,
            self.register_b,
            self.register_c,
            self.program.clone(),
            self.pointer,
            self.result.clone(),
        );
        reverse_computer.run()
    }

    fn separate_number(number: usize) -> Vec<usize> {
        let mut res = vec![];
        let mut current_val = number;

        if number == 0 {
            return vec![0];
        }

        while current_val > 0 {
            res.push(current_val % 10);
            current_val = current_val / 10;
        }
        res.reverse();
        res
    }

    fn get_next_instruction(&mut self) -> Option<usize> {
        if self.pointer < self.program.len() {
            let next_value = self.program[self.pointer];
            self.pointer += 1;
            return Some(next_value);
        }
        None
    }

    fn get_value_from_opcode(&self, opcode: usize) -> usize {
        match opcode {
            0 | 1 | 2 | 3 => opcode,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => 0,
        }
    }
}
