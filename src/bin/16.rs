use std::iter::{Chain, repeat};
use itertools::{chain, Itertools};
use pathfinding::prelude::bfs_reach;
use advent_of_code::helpers::matrix;
use pathfinding::matrix::directions::*;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<usize> {
    let input = matrix::parse(input);
    // println!("{}\n", matrix::to_str(&input));
    let energized_points = energized(&input, (E, (0,0)));
    // for &p in &energized_points {
    //     input[p] = b'#';
    // }
    // println!("{}", matrix::to_str(&input));
    Some(energized_points.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = matrix::parse(input);
    let start = chain!(
        repeat(W).zip((0..input.rows).map(|r| (r, input.columns-1))),
        repeat(E).zip((0..input.rows).map(|r| (r, 0))),
        repeat(S).zip((0..input.columns).map(|c| (0, c))),
        repeat(N).zip((0..input.columns).map(|c| (input.rows-1, c)))
    ).collect::<Vec<LightIx>>();
    start.into_iter().map(|ix| energized(&input, ix).len()).max()
}

type Dir = (isize, isize);
type Point = (usize, usize);
type LightIx = (Dir, Point);

fn energized(grid: &matrix::Grid, start: LightIx) -> Vec<Point> {
    let bfs = bfs_reach(start, |&ix| reach(grid, ix));
    bfs.map(|(_d, p)| p).unique().collect()
}

fn reach(grid: &matrix::Grid, (dir, pos): LightIx) -> Vec<LightIx> {
    let move_in = |d: Dir| grid.move_in_direction(pos, d)
        .map(|p| (d, p));
    grid.get(pos).map_or(Vec::new(), |&c| {
        match (dir, c) {
            // TODO: use vector transpose
            (N, b'/') => move_in(E).into_iter().collect(),
            (E, b'/') => move_in(N).into_iter().collect(),
            (S, b'/') => move_in(W).into_iter().collect(),
            (W, b'/') => move_in(S).into_iter().collect(),
            (N, b'\\') => move_in(W).into_iter().collect(),
            (E, b'\\') => move_in(S).into_iter().collect(),
            (S, b'\\') => move_in(E).into_iter().collect(),
            (W, b'\\') => move_in(N).into_iter().collect(),
            (N | S, b'-') => [move_in(W), move_in(E)].into_iter().flatten().collect(),
            (E | W, b'|') => [move_in(N), move_in(S)].into_iter().flatten().collect(),
            // . or - or |
            (_, _) => move_in(dir).into_iter().collect(),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
