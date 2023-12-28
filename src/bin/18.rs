use std::str::FromStr;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<i64> {
    let input = parse(input);
    let polygon = digging(&input);
    let area = area2d(&polygon);
    Some(area)
}

pub fn part_two(input: &str) -> Option<i64> {
    let input = parse(input);
    let fixed = input.iter().map(fix_bug).collect();
    let polygon = digging(&fixed);
    let area = area2d(&polygon);
    Some(area)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Dir {
    R,
    D,
    L,
    U,
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Dir::L),
            "R" => Ok(Dir::R),
            "U" => Ok(Dir::U),
            "D" => Ok(Dir::D),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Dig {
    direction: Dir,
    distance: i64,
    color: String,
}

type Input = Vec<Dig>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !str::is_empty(line))
        .map(|line| {
            let mut words = line.split_ascii_whitespace();
            let direction = words.next().expect("direction").parse().expect("L/R/U/D");
            let distance = words.next().expect("distance").parse().expect("number");
            let color = words.next().expect("color").to_string();
            Dig {
                direction,
                distance,
                color,
            }
        })
        .collect()
}

type Point = (i64, i64);
type Line = Vec<Point>;

fn dig(dig: &Dig, (x, y): Point) -> Point {
    match dig.direction {
        Dir::L => (x - dig.distance, y),
        Dir::R => (x + dig.distance, y),
        Dir::U => (x, y - dig.distance),
        Dir::D => (x, y + dig.distance),
    }
}

fn digging(input: &Input) -> Line {
    let mut ps = vec![(0, 0)];
    for d in input.iter() {
        ps.push(dig(d, *ps.last().unwrap()));
    }
    ps
}

fn length(line: &Line) -> i64 {
    line.iter()
        .zip(line.iter().skip(1))
        .map(|((x1, y1), (x2, y2))| i64::abs(x2 - x1) + i64::abs(y2 - y1))
        .sum()
}

fn shoelace(polygon: &Line) -> i64 {
    let s: i64 = polygon
        .iter()
        .zip(polygon.iter().skip(1).chain(polygon.first()))
        .map(|((x1, y1), (x2, y2))| x1 * y2 - x2 * y1)
        .sum();
    i64::abs(s / 2)
}

fn area2d(polygon: &Line) -> i64 {
    let circumfence = length(polygon);
    let area = shoelace(polygon);
    area + circumfence / 2 + 1
}

fn dir_from_digit(d: u8) -> Dir {
    match d {
        0 => Dir::R,
        1 => Dir::D,
        2 => Dir::L,
        3 => Dir::U,
        _ => panic!("Unknown direction: {d}"),
    }
}

fn fix_bug(dig: &Dig) -> Dig {
    let dist = i64::from_str_radix(&dig.color[2..7], 16).expect("5 distance digits");
    let d = dig.color[7..8].parse::<u8>().expect("1 direction digit");
    Dig {
        direction: dir_from_digit(d),
        distance: dist,
        color: String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shoelace() {
        let rect = vec![(0, 0), (0, 6), (2, 6), (2, 0)];
        let result = shoelace(&rect);
        assert_eq!(result, 12)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
