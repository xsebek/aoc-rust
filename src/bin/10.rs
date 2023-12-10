use colored::{Color, Colorize};
use itertools::{iterate, Itertools};
use pathfinding::prelude::*;
use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);
    let path = find_path(&grid)?;
    // print_highlighted(&grid, |_, p| if path.contains(&p) {Some(Color::Red)} else {None});
    Some(path.len() / 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse(input);
    let path = find_path(&grid)?;
    let path_len = path.len();
    let path_set: HashSet<Pos> = HashSet::from_iter(path.clone());
    let inside_path = get_inside_path(&path_set, &grid);

    // print_highlighted(&grid, |_, p| {
    //     if path.contains(&p) {
    //         Some(Color::Red)
    //     } else if *inside_path
    //         .get(p.row)
    //         .and_then(|r| r.get(p.col))
    //         .unwrap_or(&false)
    //     {
    //         Some(Color::Blue)
    //     } else {
    //         None
    //     }
    // });

    Some(inside_path.into_iter().flatten().filter(|x| *x).count() + 1 - path_len)
}

fn parse(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|l| l.chars())).expect("Rectangular grid")
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Pos {
    fn shifted(self, dir: Dir) -> Option<Pos> {
        match dir {
            Dir::N => {
                if self.row == 0 {
                    None
                } else {
                    Some(Pos {
                        row: self.row - 1,
                        ..self
                    })
                }
            }
            Dir::W => {
                if self.col == 0 {
                    None
                } else {
                    Some(Pos {
                        col: self.col - 1,
                        ..self
                    })
                }
            }
            Dir::S => Some(Pos {
                row: self.row + 1,
                ..self
            }),
            Dir::E => Some(Pos {
                col: self.col + 1,
                ..self
            }),
        }
    }

    fn as_tuple(self) -> (usize, usize) {
        (self.row, self.col)
    }
}

fn to_connections(c: char, pos: Pos) -> Vec<Pos> {
    use crate::Dir::*;
    match c {
        'S' => vec![N, E, S, W],
        '-' => vec![W, E],
        '|' => vec![N, S],
        'F' => vec![E, S],
        'L' => vec![E, N],
        '7' => vec![W, S],
        'J' => vec![W, N],
        _ => Vec::new(),
    }
    .into_iter()
    .filter_map(|d| pos.shifted(d))
    .collect()
}

fn pipe_connects(grid: &Matrix<char>, p1: Pos, p2: Pos) -> bool {
    if let Some(c2) = grid.get(p2.as_tuple()) {
        to_connections(*c2, p2).contains(&p1)
    } else {
        false
    }
}

fn step(grid: &Matrix<char>, previous: Pos, pos: Pos) -> Option<Pos> {
    let c = *grid.get(pos.as_tuple())?;
    let connections = to_connections(c, pos);
    connections
        .into_iter()
        .find(|p2| *p2 != previous && pipe_connects(grid, pos, *p2))
}

fn follow_pipe(grid: &Matrix<char>, s: Pos, pos: Pos) -> Option<Vec<Pos>> {
    let maybe_step = |m: &Option<(Pos, Pos)>| -> Option<(Pos, Pos)> {
        m.map(|(prev, p)| step(grid, prev, p).map(|n| (p, n)))
            .flatten()
    };
    let path: Vec<Pos> = iterate(Some((s, pos)), maybe_step)
        .take_while(|m| m.is_some())
        .flatten()
        .take_while_inclusive(|p| p.1 != s)
        .map(|p| p.1)
        .collect();
    if path.last() == Some(&s) {
        Some(std::iter::once(s).chain(path).collect())
    } else {
        None
    }
}

fn find_path(grid: &Matrix<char>) -> Option<Vec<Pos>> {
    let ((row, col), c) = grid.items().find(|(_, c)| **c == 'S')?;
    let s = Pos { row, col };
    let ps: Vec<Vec<Pos>> = to_connections(*c, s)
        .into_iter()
        .filter(|p| pipe_connects(grid, s, *p))
        .flat_map(|p| follow_pipe(grid, s, p))
        .collect();
    if ps.len() == 2 {
        ps.into_iter().next()
    } else {
        None // panic!("Expected one path from S to S: {ps:?}"),
    }
}

fn ray_cast(path: &HashSet<Pos>, row: usize, line: &[char]) -> Vec<bool> {
    use crate::Dir::{E, N, S};
    let mut result = Vec::new();
    let mut inside = false;
    let mut direction = E;
    for (col, c) in line.iter().enumerate() {
        if path.contains(&Pos { row, col }) {
            result.push(true);
            match c {
                '-' => continue,
                '|' => {
                    inside = !inside;
                }
                'F' => {
                    inside = !inside;
                    direction = S;
                }
                'L' => {
                    inside = !inside;
                    direction = N;
                }
                '7' => {
                    if direction == S {
                        inside = !inside
                    };
                }
                'J' => {
                    if direction == N {
                        inside = !inside
                    };
                }
                'S' => {
                    if direction == S {
                        inside = !inside
                    };
                } // S is 7 in my input
                _ => panic!("Forgotten path: {c}"),
            };
        } else {
            result.push(inside);
        }
    }
    result
}

fn get_inside_path(path: &HashSet<Pos>, input: &Matrix<char>) -> Vec<Vec<bool>> {
    input
        .iter()
        .enumerate()
        .map(|(row, line)| ray_cast(path, row, line))
        .collect()
}

fn pretty(c: char) -> char {
    match c {
        'S' => '╋',
        '-' => '─',
        '|' => '│',
        'F' => '┌',
        'L' => '└',
        '7' => '┐',
        'J' => '┘',
        _ => c,
    }
}

#[allow(unused)]
fn print_highlighted<P>(input: &Matrix<char>, predicate: P)
where
    P: Fn(char, Pos) -> Option<Color>,
{
    for (row, line) in input.iter().enumerate() {
        let hs = line
            .iter()
            .enumerate()
            .group_by(|(col, c)| predicate(**c, Pos { row, col: *col }));
        for (h, cs) in hs.into_iter() {
            let s: String = cs.map(|t| pretty(*t.1)).collect();
            if h == Some(Color::Blue) {
                print!("{}", s.blue())
            } else if h == Some(Color::Red) {
                print!("{}", s.red())
            } else {
                print!("{}", s)
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
        let result = part_one(&advent_of_code::template::read_file_indexed(
            "examples",
            DAY,
            Some(1),
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_indexed(
            "examples",
            DAY,
            Some(2),
        ));
        assert_eq!(result, Some(4));
        let result = part_two(&advent_of_code::template::read_file_indexed(
            "examples",
            DAY,
            Some(3),
        ));
        assert_eq!(result, Some(8));
    }
}
