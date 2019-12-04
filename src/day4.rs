use std::ops::{Range, RangeInclusive};

#[aoc_generator(day4)]
fn parse(input: &str) -> RangeInclusive<u32> {
    let start = input[..6].parse().unwrap();
    let end = input[7..].parse().unwrap();
    start..=end
}

#[aoc(day4, part1)]
fn part1(range: &RangeInclusive<u32>) -> usize {
    combinations1(range, 0, 6, false, 0)
}

// this is more complex than brute forcing, but ~5000 times faster
fn combinations1(
    range: &RangeInclusive<u32>,
    current: u32,
    remaining_digits: u32,
    had_double: bool,
    prev_digit: u32,
) -> usize {
    let candidates = prev_digit..=9;
    if remaining_digits == 1 {
        return if had_double { candidates.count() } else { 1 };
    }
    candidates
        .map(|digit| {
            let digit_range = get_range(current, remaining_digits - 1, digit);
            if !range.contains(&digit_range.start) && !range.contains(&digit_range.end) {
                return 0;
            }
            let had_double = had_double || digit == prev_digit;
            combinations1(
                range,
                digit_range.start,
                remaining_digits - 1,
                had_double,
                digit,
            )
        })
        .sum()
}

fn get_range(current_min: u32, order: u32, digit: u32) -> Range<u32> {
    let new_min = current_min + digit * 10_u32.pow(order);
    let new_max = current_min + (digit + 1) * 10_u32.pow(order);
    new_min..new_max
}

#[aoc(day4, part2)]
fn part2(range: &RangeInclusive<u32>) -> usize {
    combinations2(range, 0, 6, 0, false, 0)
}

fn combinations2(
    range: &RangeInclusive<u32>,
    current: u32,
    remaining_digits: u32,
    adjacent_count: u32,
    had_double: bool,
    prev_digit: u32,
) -> usize {
    let candidates = prev_digit..=9;
    if remaining_digits == 1 {
        return if had_double {
            candidates.count()
        } else if adjacent_count == 2 {
            candidates.count() - 1 // must not make it a triple
        } else if adjacent_count == 1 {
            1 // have to complete double
        } else {
            0 // would have no double
        };
    }
    candidates
        .map(|digit| {
            let digit_range = get_range(current, remaining_digits - 1, digit);
            if !range.contains(&digit_range.start) && !range.contains(&digit_range.end) {
                return 0;
            }
            let (adjacent_count, had_double) = if digit == prev_digit {
                (adjacent_count + 1, had_double)
            } else {
                (1, had_double || adjacent_count == 2)
            };
            combinations2(
                range,
                digit_range.start,
                remaining_digits - 1,
                adjacent_count,
                had_double,
                digit,
            )
        })
        .sum()
}
