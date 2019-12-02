#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<usize> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
fn part1(program: &[usize]) -> usize {
    run(program, 12, 2)
}

#[aoc(day2, part2)]
fn part2(program: &[usize]) -> usize {
    for noun in 0..=99 {
        for verb in 0..=99 {
            if run(program, noun, verb) == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!("no solution")
}

fn run(program: &[usize], input1: usize, input2: usize) -> usize {
    let mut program = program.to_vec();
    program[1] = input1;
    program[2] = input2;
    let result = execute(program);
    result[0]
}

fn execute(mut program: Vec<usize>) -> Vec<usize> {
    use std::ops::{Add, Mul};
    for ptr in (0..).step_by(4) {
        let op = match program[ptr] {
            1 => Add::add,
            2 => Mul::mul,
            99 => break,
            _ => unreachable!("invalid opcode"),
        };
        let arg1_ptr = program[ptr + 1];
        let arg2_ptr = program[ptr + 2];
        let store_ptr = program[ptr + 3];
        program[store_ptr] = op(program[arg1_ptr], program[arg2_ptr]);
    }
    program
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(execute(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(execute(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(execute(vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(
            execute(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
