use itertools::Itertools;
use pathfinding::matrix::Matrix;

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
