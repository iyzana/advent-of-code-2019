#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(masses: &[u32]) -> u32 {
    masses.iter().copied().map(fuel).sum::<u32>()
}

#[aoc(day1, part2)]
fn part2(masses: &[u32]) -> u32 {
    masses.iter().copied().map(fuel_iter).sum::<u32>()
}

fn fuel(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn fuel_iter(mass: u32) -> u32 {
    use std::iter::successors;

    successors(Some(fuel(mass)), |&mass| Some(fuel(mass)))
        .take_while(|&mass| mass > 0)
        .sum::<u32>()
}

#[allow(dead_code)]
fn fuel_rec(mass: u32) -> u32 {
    if mass == 0 {
        0
    } else {
        let fuel_mass = fuel(mass);
        fuel_mass + fuel_rec(fuel_mass)
    }
}
