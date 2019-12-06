use std::fmt::Debug;

#[derive(Clone)]
pub struct Computer {
    pub memory: Vec<i32>,
    instruction_ptr: usize,
    input: Vec<i32>,
    pub output: Vec<i32>,
}

pub fn run(code: &[i32], input: &[i32]) -> Computer {
    let mut input = input.to_vec();
    input.reverse();
    let mut computer = Computer {
        memory: code.to_vec(),
        instruction_ptr: 0,
        input,
        output: vec![],
    };
    computer.run();
    computer
}

impl Computer {
    fn run(&mut self) {
        // self.dump_mem();
        loop {
            let Instruction { operation, params } = &Instruction::parse(self);
            // self.dump_instruction(operation, params);

            self.instruction_ptr += params.len() + 1;
            match operation {
                Operation::Add => self.compute(params, |a, b| a + b),
                Operation::Mul => self.compute(params, |a, b| a * b),
                Operation::Read => {
                    let val = self.input.pop().expect("no input");
                    self.store(&params[0], val)
                }
                Operation::Write => self.output.push(self.load(&params[0])),
                Operation::JumpIfTrue => {
                    if self.load(&params[0]) != 0 {
                        self.instruction_ptr = self.load(&params[1]) as usize;
                    }
                }
                Operation::JumpIfFalse => {
                    if self.load(&params[0]) == 0 {
                        self.instruction_ptr = self.load(&params[1]) as usize;
                    }
                }
                Operation::LessThan => self.compute(params, |a, b| (a < b) as i32),
                Operation::Equals => self.compute(params, |a, b| (a == b) as i32),
                Operation::Quit => break,
            }
        }
    }

    fn load(&self, param: &Parameter) -> i32 {
        match *param {
            Parameter::Position(pos) => self.memory[pos],
            Parameter::Immediate(val) => val,
        }
    }

    fn store(&mut self, param: &Parameter, val: i32) {
        match *param {
            Parameter::Position(pos) => self.memory[pos] = val,
            _ => panic!("invalid output parameter type"),
        }
    }

    fn compute<F>(&mut self, params: &[Parameter], compute: F)
    where
        F: Fn(i32, i32) -> i32,
    {
        self.store(
            &params[2],
            compute(self.load(&params[0]), self.load(&params[1])),
        );
    }

    #[allow(dead_code)]
    fn dump_mem(&self) {
        println!("memory:");
        let columns = 16;
        self.memory
            .chunks(columns)
            .enumerate()
            .for_each(|(index, line)| {
                print!("#{:0>4}-{:0>4} ", index * columns, (index + 1) * columns - 1);
                line.iter().for_each(|m| print!("{:>7}", m));
                println!();
            });
        println!();
    }

    #[allow(dead_code)]
    fn dump_instruction(&self, operation: &Operation, params: &[Parameter]) {
        println!(
            "#{:0>4}-{:0>4} {:<25} -> {:>11} {}",
            self.instruction_ptr,
            self.instruction_ptr + params.len(),
            format!(
                "{:?}",
                &self.memory[self.instruction_ptr..=self.instruction_ptr + params.len()],
            ),
            format!("{:?}", operation),
            params
                .iter()
                .map(|param| self.dump_param(&param))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    #[allow(dead_code)]
    fn dump_param(&self, param: &Parameter) -> String {
        match param {
            Parameter::Position(pos) => format!("&{}={}", pos, self.memory[*pos]),
            Parameter::Immediate(val) => format!("{}", val),
        }
    }
}

struct Instruction {
    operation: Operation,
    params: Vec<Parameter>,
}

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
    Read,
    Write,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Quit,
}

impl Instruction {
    fn parse(computer: &Computer) -> Instruction {
        let (operation, param_count) = Self::parse_opcode(computer);
        let params = Self::parse_params(computer, param_count);

        Instruction { operation, params }
    }

    fn parse_opcode(computer: &Computer) -> (Operation, usize) {
        let instruction = computer.memory[computer.instruction_ptr];
        let opcode = instruction % 100;
        match opcode {
            1 => (Operation::Add, 3),
            2 => (Operation::Mul, 3),
            3 => (Operation::Read, 1),
            4 => (Operation::Write, 1),
            5 => (Operation::JumpIfTrue, 2),
            6 => (Operation::JumpIfFalse, 2),
            7 => (Operation::LessThan, 3),
            8 => (Operation::Equals, 3),
            99 => (Operation::Quit, 0),
            _ => panic!("invalid opcode"),
        }
    }

    fn parse_params(computer: &Computer, count: usize) -> Vec<Parameter> {
        let instruction = computer.memory[computer.instruction_ptr];
        let param_memory =
            &computer.memory[computer.instruction_ptr + 1..=computer.instruction_ptr + count];
        let kinds = (instruction / 100).to_string();
        let mut kinds = kinds.chars().rev();
        param_memory
            .iter()
            .map(|&val| match kinds.next().unwrap_or('0') {
                '0' => {
                    assert!(val >= 0, "negative positional parameter");
                    Parameter::Position(val as usize)
                }
                '1' => Parameter::Immediate(val),
                _ => panic!("invalid parameter mode"),
            })
            .collect()
    }
}

enum Parameter {
    Position(usize),
    Immediate(i32),
}
