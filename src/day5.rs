#[aoc_generator(day5)]
fn parse(input: &str) -> Vec<i32> {
    input.split(',').map(|m| m.parse().unwrap()).collect()
}

#[derive(Clone)]
struct Program {
    memory: Vec<i32>,
    instruction_ptr: usize,
    input: Vec<i32>,
    output: Vec<i32>,
}

impl Program {
    fn new(code: &[i32], input: &[i32]) -> Self {
        let mut input = input.to_vec();
        input.reverse();
        Self {
            memory: code.to_vec(),
            instruction_ptr: 0,
            input,
            output: vec![],
        }
    }

    fn run(&mut self) -> i32 {
        loop {
            let Instruction { operation, params } = &Instruction::parse(self);
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

        *self.output.last().unwrap()
    }

    fn load(&self, param: &Parameter) -> i32 {
        param.load(&self.memory)
    }

    fn store(&mut self, param: &Parameter, val: i32) {
        param.store(&mut self.memory, val)
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
}

#[derive(Debug)]
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
    fn parse(program: &Program) -> Instruction {
        let (operation, param_count) = Self::parse_opcode(program);
        let params = Self::parse_params(program, param_count);

        Instruction { operation, params }
    }

    fn parse_opcode(program: &Program) -> (Operation, usize) {
        let instruction = program.memory[program.instruction_ptr];
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

    fn parse_params(program: &Program, count: usize) -> Vec<Parameter> {
        let instruction = program.memory[program.instruction_ptr];
        let param_memory =
            &program.memory[program.instruction_ptr + 1..=program.instruction_ptr + count];
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

#[derive(Debug)]
enum Parameter {
    Position(usize),
    Immediate(i32),
}

impl Parameter {
    fn load(&self, memory: &[i32]) -> i32 {
        match *self {
            Parameter::Position(pos) => memory[pos],
            Parameter::Immediate(val) => val,
        }
    }

    fn store(&self, memory: &mut [i32], val: i32) {
        match *self {
            Parameter::Position(pos) => memory[pos] = val,
            _ => panic!("invalid output parameter type"),
        }
    }
}

#[aoc(day5, part1)]
fn part1(code: &[i32]) -> i32 {
    Program::new(code, &[1]).run()
}

#[aoc(day5, part2)]
fn part2(code: &[i32]) -> i32 {
    Program::new(code, &[5]).run()
}
