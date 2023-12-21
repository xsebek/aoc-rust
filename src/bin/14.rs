use itertools::{Itertools, repeat_n};
use pathfinding::matrix::Matrix;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let mut input = parse(input);
    input.rotate_ccw(1); // N to W
    let slided = slide_west(&input);
    let result = count_west(&slided);
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut input = parse(input);
    input.rotate_ccw(1); // N to W
    let (cycle_start, rs) = rotations(input);

    let cycle_len = rs.len() - cycle_start;
    let in_cycle = (1_000_000_000 - cycle_start - 1) % cycle_len;

    rs.get(cycle_start + in_cycle).map(count_west)
}

type Platform = Matrix<u8>;

fn parse(input: &str) -> Platform {
    Platform::from_rows(input.lines().map(|l| l.bytes())).expect("rectangle grid input")
}

fn slide_west(platform: &Platform) -> Platform {
    Platform::from_rows(platform.iter().map(|row| {
        let mut result = Vec::with_capacity(row.len());
        let mut it = row.iter();
        'outer: while let Some(&c) = it.next() {
            if c == b'.' {
                let mut count_dot = 1;
                for &c2 in it.by_ref() {
                    if c2 == b'O' {
                        result.push(c2);
                    }
                    else if c2 == b'.' {
                        count_dot += 1;
                    }
                    else {
                        result.extend(repeat_n(b'.', count_dot));
                        result.push(c2);
                        continue 'outer
                    }
                }
                result.extend(repeat_n(b'.', count_dot));
            }
            else {
                result.push(c)
            }
        }
        result
    })).expect("same dimensions of platform")
}

fn count_west(platform: &Platform) -> usize {
    platform.iter()
        .flat_map(|row| row.iter()
            .rev()
            .enumerate()
            .map(|(i, &c)| if c == b'O' {i + 1} else {0}))
        .sum()
}

#[allow(dead_code)]
fn to_str(platform: &Platform) -> String {
    platform
        .iter()
        .map(|row| std::str::from_utf8(row).expect("previously ok characters"))
        .join("\n")
}

fn rotation(platform: &Platform) -> Platform {
    let mut platform = slide_west(platform); // W
    platform.rotate_cw(1);
    platform = slide_west(&platform); // S
    platform.rotate_cw(1);
    platform = slide_west(&platform); // E
    platform.rotate_cw(1);
    platform = slide_west(&platform); // N
    platform.rotate_cw(1);
    platform // W
}

fn rotations(mut platform: Platform) -> (usize, Vec<Platform>) {
    let mut rotation_list = Vec::new();

    // let mut i = 0;
    loop {
        // i += 1;
        platform = rotation(&platform);
        //println!("{i}:\n{}\nLoad: {}\n", to_str(&platform.rotated_cw(1)), count_west(&platform));
        if let Some(p) = rotation_list.iter().rposition(|i| i == &platform) {
            return (p, rotation_list);
        }
        rotation_list.push(platform.clone());
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
