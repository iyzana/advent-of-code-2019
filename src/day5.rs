#[aoc_generator(day5)]
fn parse(input: &str) -> Program {
    let data = input.split(',').map(|d| d.parse().unwrap()).collect();
    Program {
        data,
        instruction_ptr: 0,
        input: vec![],
        output: vec![],
    }
}

#[derive(Clone)]
struct Program {
    data: Vec<i32>,
    instruction_ptr: usize,
    input: Vec<i32>,
    output: Vec<i32>,
}

impl Program {
    fn run(&mut self) -> i32 {
        while self.parse_op(self.instruction_ptr).execute(self) {}
        *self.output.last().unwrap()
    }

    fn parse_op(&self, pos: usize) -> Instruction {
        let instruction = self.data[pos];
        let opcode = instruction % 100;
        let param_types = (instruction / 100).to_string();

        match opcode {
            1 => Instruction::Add(
                self.parse_param(&param_types, pos, 0),
                self.parse_param(&param_types, pos, 1),
                self.parse_param(&param_types, pos, 2),
            ),
            2 => Instruction::Mul(
                self.parse_param(&param_types, pos, 0),
                self.parse_param(&param_types, pos, 1),
                self.parse_param(&param_types, pos, 2),
            ),
            3 => Instruction::Read(self.parse_param(&param_types, pos, 0)),
            4 => Instruction::Write(self.parse_param(&param_types, pos, 0)),
            5 => Instruction::JumpIfTrue(
                self.parse_param(&param_types, pos, 0),
                self.parse_param(&param_types, pos, 1),
            ),
            6 => Instruction::JumpIfFalse(
                self.parse_param(&param_types, pos, 0),
                self.parse_param(&param_types, pos, 1),
            ),
            7 => Instruction::LessThan(
                self.parse_param(&param_types, pos, 0),
                self.parse_param(&param_types, pos, 1),
                self.parse_param(&param_types, pos, 2),
            ),
            8 => Instruction::Equals(
                self.parse_param(&param_types, pos, 0),
                self.parse_param(&param_types, pos, 1),
                self.parse_param(&param_types, pos, 2),
            ),
            99 => Instruction::Quit,
            _ => unreachable!("invalid opcode"),
        }
    }

    fn parse_param(&self, types: &str, pos: usize, index: usize) -> Parameter {
        let val = self.data[pos + index + 1];
        match types.chars().nth_back(index).unwrap_or('0') {
            '0' if val >= 0 => Parameter::Position(val as usize),
            '0' if val < 0 => panic!("negative positional paramter"),
            '1' => Parameter::Immediate(val),
            _ => unreachable!("invalid parameter mode"),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Mul(Parameter, Parameter, Parameter),
    Read(Parameter),
    Write(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    Quit,
}

impl Instruction {
    fn execute(&self, program: &mut Program) -> bool {
        let Program {
            data,
            instruction_ptr,
            ..
        } = program;
        *instruction_ptr += self.arg_count() + 1;
        match self {
            Instruction::Add(arg1, arg2, out) => {
                let val = arg1.get(data) + arg2.get(data);
                out.write(data, val);
            }
            Instruction::Mul(arg1, arg2, out) => {
                let val = arg1.get(data) * arg2.get(data);
                out.write(data, val);
            }
            Instruction::Read(out) => out.write(data, program.input.pop().expect("no input")),
            Instruction::Write(arg1) => program.output.push(arg1.get(data)),
            Instruction::JumpIfTrue(cmp, target) => {
                if cmp.get(data) != 0 {
                    assert!(target.get(data) >= 0);
                    *instruction_ptr = target.get(data) as usize;
                }
            }
            Instruction::JumpIfFalse(cmp, target) => {
                if cmp.get(data) == 0 {
                    assert!(target.get(data) >= 0);
                    *instruction_ptr = target.get(data) as usize;
                }
            }
            Instruction::LessThan(arg1, arg2, out) => {
                let val = (arg1.get(data) < arg2.get(data)) as i32;
                out.write(data, val);
            }
            Instruction::Equals(arg1, arg2, out) => {
                let val = (arg1.get(data) == arg2.get(data)) as i32;
                out.write(data, val);
            }
            Instruction::Quit => return false,
        }
        true
    }

    fn arg_count(&self) -> usize {
        match self {
            Instruction::Add(_, _, _) => 3,
            Instruction::Mul(_, _, _) => 3,
            Instruction::Read(_) => 1,
            Instruction::Write(_) => 1,
            Instruction::JumpIfTrue(_, _) => 2,
            Instruction::JumpIfFalse(_, _) => 2,
            Instruction::LessThan(_, _, _) => 3,
            Instruction::Equals(_, _, _) => 3,
            Instruction::Quit => 0,
        }
    }
}

#[derive(Debug)]
enum Parameter {
    Position(usize),
    Immediate(i32),
}

impl Parameter {
    fn get(&self, data: &[i32]) -> i32 {
        match *self {
            Parameter::Position(pos) => data[pos],
            Parameter::Immediate(val) => val,
        }
    }

    fn write(&self, data: &mut [i32], val: i32) {
        match *self {
            Parameter::Position(pos) => data[pos] = val,
            _ => unreachable!("invalid output parameter type"),
        }
    }
}

#[aoc(day5, part1)]
fn part1(program: &Program) -> i32 {
    let mut program = Program {
        input: vec![1],
        ..program.clone()
    };
    program.run()
}

#[aoc(day5, part2)]
fn part2(program: &Program) -> i32 {
    let mut program = Program {
        input: vec![5],
        ..program.clone()
    };
    program.run()
}
