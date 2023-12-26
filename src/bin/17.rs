use pathfinding::matrix::directions::DIRECTIONS_4;
use pathfinding::prelude::dijkstra;
use advent_of_code::helpers::matrix::{self, Grid};
advent_of_code::solution!(17);

fn solve(input: &str, min: usize, max: usize) -> Option<u32> {
    let input = parse(input);
    let hot = hottest(&input, ((0,0), None), (input.rows-1, input.columns-1), min, max);
    // let path_map = Matrix::from_fn(input.rows, input.columns, |p| {
    //     hot.as_ref().unwrap().0.iter().find(|&&v| v.0 == p).map_or(b'.', |&v| v.1.map_or(b'S', d_to_str))
    // });
    // println!("{}\n", to_str(&path_map));
    hot.map(|r| r.1)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 0, 3)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 4, 10)
}

fn parse(input: &str) -> Grid {
    matrix::parse(input).map(|c| c - b'0')
}

type Pos = (usize, usize);
type Dir = (isize, isize);

type Ix = (Pos, Option<Dir>);

fn hottest(grid: &Grid, start: Ix, end: Pos, min: usize, max: usize) -> Option<(Vec<Ix>, u32)> {
    dijkstra(&start, |&p| reach(grid, min, max, p), |&ix| ix.0 == end)
}

fn reach(grid: &Grid, min: usize, max: usize, start: Ix) -> Vec<(Ix, u32)> {
    //let dirs = match start {
    //    Ix::Start(_) => Vec::from(DIRECTIONS_4),
    //    Ix::Moving(d, _) => turn90(d),
    //};
    let dirs = start.1.map_or(Vec::from(DIRECTIONS_4), turn90);
    let mut result = Vec::new();
    for dir in dirs {
        let mut cost: u32 = 0;
        for (i, pos) in grid.in_direction(start.0, dir).take(max).enumerate() {
            let loss = *grid.get(pos).expect("position in direction");
            cost += loss as u32;
            if i + 1 >= min {
                result.push(((pos, Some(dir)), cost))
            }
        }
    }
    result
}

fn turn90(dir: Dir) -> Vec<Dir> {
    vec![(-dir.1, dir.0), (dir.1, -dir.0)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two_worst_case() {
        let result = part_two(&advent_of_code::template::read_file_indexed("examples", DAY, Some(2)));
        assert_eq!(result, Some(71));
    }
}
