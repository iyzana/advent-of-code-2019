use std::collections::HashSet;

#[aoc_generator(day3)]
fn parse(input: &str) -> (Wire, Wire) {
    let mut lines = input.lines();
    (
        Wire::from(lines.next().unwrap()),
        Wire::from(lines.next().unwrap()),
    )
}

#[derive(Debug)]
struct Wire {
    points: Vec<(i32, i32)>,
}

impl From<&str> for Wire {
    fn from(line: &str) -> Self {
        let mut points = vec![(0, 0)];
        let mut current = points[0];
        for (dir, dist) in line.split(',').map(|op| op.split_at(1)) {
            let dir = dir.chars().next().unwrap();
            let dist = dist.parse().unwrap();
            for _ in 0..dist {
                current = move_dir(dir, current);
                points.push(current);
            }
        }
        Self { points }
    }
}

fn move_dir(dir: char, point: (i32, i32)) -> (i32, i32) {
    let (x, y) = point;
    match dir {
        'U' => (x, y - 1),
        'D' => (x, y + 1),
        'L' => (x - 1, y),
        'R' => (x + 1, y),
        _ => unreachable!("invalid direction"),
    }
}

#[aoc(day3, part1)]
fn part1(wires: &(Wire, Wire)) -> u32 {
    min_val(wires, |point| point.0.abs() as u32 + point.1.abs() as u32)
}

fn min_val<F>(wires: &(Wire, Wire), dist_fn: F) -> u32
where
    F: Fn(&(i32, i32)) -> u32,
{
    let wire_1 = &wires.0.points;
    let wire_2: HashSet<_> = wires.1.points.iter().collect();
    wire_1
        .iter()
        .filter(|&point| wire_2.contains(point))
        .map(dist_fn)
        .filter(|&dist| dist != 0)
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
fn part2(wires: &(Wire, Wire)) -> u32 {
    min_val(wires, |point| {
        get_steps(&wires.0, *point) + get_steps(&wires.1, *point)
    })
}

fn get_steps(wire: &Wire, point: (i32, i32)) -> u32 {
    wire.points.iter().position(|&p| p == point).unwrap() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generator_test() {
        assert_eq!(
            Wire::from("R2,D2").points,
            vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        );
    }

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(&parse(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            )),
            159
        );
        assert_eq!(
            part1(&parse(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            135
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(&parse(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            )),
            610
        );
        assert_eq!(
            part2(&parse(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            410
        );
    }
}
