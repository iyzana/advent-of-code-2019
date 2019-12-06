use crate::computer;

#[aoc_generator(day5)]
fn parse(input: &str) -> Vec<i32> {
    input.split(',').map(|m| m.parse().unwrap()).collect()
}

#[aoc(day5, part1)]
fn part1(code: &[i32]) -> i32 {
    *computer::run(code, &[1]).output.last().unwrap()
}

#[aoc(day5, part2)]
fn part2(code: &[i32]) -> i32 {
    *computer::run(code, &[5]).output.last().unwrap()
}
