use itertools::Itertools;
use pathfinding::matrix::Matrix;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let input = Matrix::from_rows(input.lines().map(|l| l.chars())).ok()?;
    let slided = slide(input);
    Some(count_load(&slided))
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = Matrix::from_rows(input.lines().map(|l| l.chars())).ok()?;
    let (cycle_start, rs) = rotations(input);
    let cycle_len = rs.len() - cycle_start;
    let in_cycle = (1_000_000_000 - cycle_start - 1) % cycle_len;

    rs.get(cycle_start + in_cycle).map(count_load)
}

fn slide_once(platform: Matrix<char>) -> Matrix<char> {
    let mut result = platform;
    for ir in 0..result.rows - 1 {
        let row = Vec::from(result.iter().nth(ir).unwrap());
        for (ic, &cell) in row.iter().enumerate() {
            // let lower_cell = get(&result, ir+1, ic).unwrap_or('-');
            let lower_cell = *result.get((ir + 1, ic)).unwrap_or(&'-');
            if cell == '.' && lower_cell == 'O' {
                // set(&mut result, ir, ic, 'O');
                // set(&mut result, ir+1, ic, '.');
                *result.get_mut((ir, ic)).unwrap() = 'O';
                *result.get_mut((ir + 1, ic)).unwrap() = '.'
            }
        }
    }
    result
}

fn slide(platform: Matrix<char>) -> Matrix<char> {
    let mut platform = platform;
    let mut platform_next = slide_once(platform.clone());
    //let mut i = 0;
    while platform != platform_next {
        //i += 1;
        //println!("{i}:\n{}\n", to_str(&platform));
        platform = platform_next;
        platform_next = slide_once(platform.clone());
    }
    platform
}

#[allow(dead_code)]
fn to_str(platform: &Matrix<char>) -> String {
    platform
        .iter()
        .map(|row| row.iter().collect::<String>())
        .join("\n")
}

fn count_load(platform: &Matrix<char>) -> usize {
    platform
        .iter()
        .rev()
        .enumerate()
        .map(|(ir, row)| (ir + 1) * row.iter().filter(|&&c| c == 'O').count())
        .sum()
}

fn rotation(platform: Matrix<char>) -> Matrix<char> {
    let mut platform = slide(platform); // N
    platform = slide(platform.rotated_cw(1)); // W
    platform = slide(platform.rotated_cw(1)); // S
    platform = slide(platform.rotated_cw(1)); // E
    platform.rotated_cw(1) // N
}

fn rotations(platform: Matrix<char>) -> (usize, Vec<Matrix<char>>) {
    let mut rotation_list = Vec::new();
    let mut rotated = rotation(platform);
    //let mut i = 0;

    loop {
        //i += 1;
        //println!("{i}:\n{}\nLoad: {}\n", to_str(&rotated), count_load(&rotated));
        rotation_list.push(rotated.clone());
        rotated = rotation(rotated);
        if let Some(p) = rotation_list.iter().rposition(|i| i == &rotated) {
            return (p, rotation_list);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
