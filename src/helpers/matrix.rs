use itertools::Itertools;
use pathfinding::matrix::Matrix;
use pathfinding::matrix::directions::*;

pub type Grid = Matrix<u8>;

pub fn parse(input: &str) -> Grid {
    Grid::from_rows(input.lines().map(|l| l.bytes())).expect("rectangle grid input")
}

#[allow(dead_code)]
pub fn to_str(platform: &Grid) -> String {
    platform
        .iter()
        .map(|row| std::str::from_utf8(row).expect("previously read characters"))
        .join("\n")
}

#[allow(dead_code)]
pub fn d_to_str(dir: (isize, isize)) -> u8 {
    match dir {
        N => b'^',
        S => b'v',
        E => b'>',
        W => b'<',
        _ => b'?',
    }
}
