use std::collections::{HashMap, HashSet};

#[aoc_generator(day3)]
fn parse(input: &str) -> (Wire, Wire) {
    let mut lines = input.lines();
    (lines.next().unwrap().into(), lines.next().unwrap().into())
}

#[derive(Debug)]
struct Wire {
    points: Vec<(i32, i32)>,
}

impl From<&str> for Wire {
    fn from(line: &str) -> Self {
        use std::iter::successors;
        let points = line.split(',').fold(vec![(0, 0)], |path, next| {
            let (dir, dist) = next.split_at(1);
            let dist = dist.parse().unwrap();
            let next_pos = move_dir(dir, path.last().copied().unwrap());
            path.into_iter()
                .chain(successors(Some(next_pos), |&pos| Some(move_dir(dir, pos))).take(dist))
                .collect()
        });
        Wire { points }
    }
}

fn move_dir(dir: &str, point: (i32, i32)) -> (i32, i32) {
    let (x, y) = point;
    match dir {
        "U" => (x, y - 1),
        "D" => (x, y + 1),
        "L" => (x - 1, y),
        "R" => (x + 1, y),
        _ => unreachable!("invalid direction"),
    }
}

#[aoc(day3, part1)]
fn part1(wires: &(Wire, Wire)) -> u32 {
    let (wire1, wire2) = wires;
    let wire2: HashSet<_> = wire2.points.iter().collect();
    wire1
        .points
        .iter()
        .filter_map(|&point1| {
            if wire2.contains(&point1) {
                Some(dist((0, 0), point1))
            } else {
                None
            }
        })
        .filter(|&dist| dist != 0)
        .min()
        .unwrap()
}

fn dist(point1: (i32, i32), point2: (i32, i32)) -> u32 {
    (point1.0 - point2.0).abs() as u32 + (point1.1 - point2.1).abs() as u32
}

#[aoc(day3, part2)]
fn part2(wires: &(Wire, Wire)) -> usize {
    let (wire1, wire2) = wires;
    let wire2: HashMap<_, _> = wire2
        .points
        .iter()
        .enumerate()
        .map(|(steps, point)| (point, steps))
        .collect();
    wire1
        .points
        .iter()
        .enumerate()
        .filter_map(|(steps1, &point1)| wire2.get(&point1).map(|steps2| steps1 + steps2))
        .filter(|&dist| dist != 0)
        .min()
        .unwrap()
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
