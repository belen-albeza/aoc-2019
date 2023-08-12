use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

type Vec2 = (i64, i64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Chunk {
    Right(u64),
    Left(u64),
    Up(u64),
    Down(u64),
}

impl From<&str> for Chunk {
    fn from(value: &str) -> Self {
        let n = value.chars().skip(1).collect::<String>().parse().unwrap();

        match value.chars().nth(0) {
            Some('R') => Self::Right(n),
            Some('L') => Self::Left(n),
            Some('U') => Self::Up(n),
            Some('D') => Self::Down(n),
            _ => unreachable!("unrecognized direction"),
        }
    }
}

impl Chunk {
    pub fn delta(&self) -> Vec2 {
        match *self {
            Self::Right(x) => (x as i64, 0),
            Self::Left(x) => (-(x as i64), 0),
            Self::Up(y) => (0, -(y as i64)),
            Self::Down(y) => (0, y as i64),
        }
    }
}

type Segment = (Vec2, Vec2);

fn to_segments(chunks: &[Chunk]) -> Vec<Segment> {
    let points: Vec<Vec2> = chunks.into_iter().fold(vec![(0, 0)], |mut acc, chunk| {
        let last = acc.last().unwrap().to_owned();
        let current = (last.0 + chunk.delta().0, last.1 + chunk.delta().1);
        acc.push(current);
        acc
    });
    points.windows(2).map(|pair| (pair[0], pair[1])).collect()
}

fn steps(s: Segment) -> u64 {
    ((s.1 .0 - s.0 .0).abs() + (s.1 .1 - s.0 .1).abs()) as u64
}

fn is_vertical(s: Segment) -> bool {
    let (start, end) = s;
    start.0 == end.0
}

fn is_horizontal(s: Segment) -> bool {
    !is_vertical(s)
}

fn distance(p: Vec2) -> u64 {
    (p.0.abs() + p.1.abs()) as u64
}

fn intersection(s1: Segment, s2: Segment) -> Option<Vec2> {
    if (is_vertical(s1) && is_vertical(s2)) || (is_horizontal(s1) && is_horizontal(s2)) {
        return None;
    }

    let h = if is_horizontal(s1) { s1 } else { s2 };
    let v = if is_vertical(s1) { s1 } else { s2 };

    let x = v.0 .0;
    let y = h.0 .1;

    let min_x = std::cmp::min(h.0 .0, h.1 .0);
    let max_x = std::cmp::max(h.0 .0, h.1 .0);
    let min_y = std::cmp::min(v.0 .1, v.1 .1);
    let max_y = std::cmp::max(v.0 .1, v.1 .1);

    if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
        Some((x, y))
    } else {
        None
    }
}

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> (Vec<Chunk>, Vec<Chunk>) {
    let wires: Vec<Vec<Chunk>> = input
        .lines()
        .map(|line| line.split(",").map(|chunk| chunk.into()).collect())
        .collect();

    (wires[0].clone(), wires[1].clone())
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &(Vec<Chunk>, Vec<Chunk>)) -> u64 {
    let wire1_segments = to_segments(&input.0);
    let wire2_segments = to_segments(&input.1);

    let intersections: Vec<Vec2> = wire1_segments
        .into_iter()
        .flat_map(|s1| {
            wire2_segments
                .iter()
                .filter_map(move |s2| intersection(s1, *s2))
        })
        .filter(|p| *p != (0, 0))
        .collect();

    intersections
        .into_iter()
        .map(|p| distance(p))
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &(Vec<Chunk>, Vec<Chunk>)) -> u64 {
    let wire1_segments = to_segments(&input.0);
    let wire2_segments = to_segments(&input.1);

    let mut intersections: Vec<(Vec2, u64)> = vec![];

    let mut s1_steps = 0u64;
    for s1 in wire1_segments {
        s1_steps += steps(s1);
        let mut s2_steps = 0u64;
        for s2 in &wire2_segments {
            s2_steps += steps(*s2);
            if let Some(position) = intersection(s1, *s2) {
                if position != (0, 0) {
                    let s1_offset = steps((position, s1.1));
                    let s2_offset = steps((position, s2.1));
                    intersections.push((position, s1_steps - s1_offset + s2_steps - s2_offset));
                }
            }
        }
    }

    intersections
        .into_iter()
        .map(|(_, steps)| steps)
        .min()
        .unwrap() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("R8,U5,L5,D3\nU7,R6,D4,L4"),
            (
                vec![
                    Chunk::Right(8),
                    Chunk::Up(5),
                    Chunk::Left(5),
                    Chunk::Down(3)
                ],
                vec![
                    Chunk::Up(7),
                    Chunk::Right(6),
                    Chunk::Down(4),
                    Chunk::Left(4)
                ],
            )
        )
    }

    #[test]
    fn test_to_segments() {
        let chunks = vec![
            Chunk::Right(8),
            Chunk::Up(5),
            Chunk::Left(5),
            Chunk::Down(3),
        ];
        assert_eq!(
            to_segments(&chunks),
            vec![
                ((0, 0), (8, 0)),
                ((8, 0), (8, -5)),
                ((8, -5), (3, -5)),
                ((3, -5), (3, -2)),
            ]
        );
    }

    #[test]
    fn test_intersection() {
        assert_eq!(
            intersection(((0, 0), (3, 0)), ((1, -4), (1, 2))),
            Some((1, 0))
        );
    }

    #[test]
    fn test_solve_part1_basic() {
        let wires = parse_input("R8,U5,L5,D3\nU7,R6,D4,L4");
        assert_eq!(solve_part1(&wires), 6);
    }

    #[test]
    fn test_solve_part1_extra() {
        let wires =
            parse_input("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(solve_part1(&wires), 159);

        let wires2 = parse_input(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        );
        assert_eq!(solve_part1(&wires2), 135);
    }

    #[test]
    fn test_solve_part2_basic() {
        let wires = parse_input("R8,U5,L5,D3\nU7,R6,D4,L4");
        assert_eq!(solve_part2(&wires), 30);
    }

    #[test]
    fn test_solve_part2_extra() {
        let wires =
            parse_input("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(solve_part2(&wires), 610);

        let wires2 = parse_input(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        );
        assert_eq!(solve_part2(&wires2), 410);
    }
}
