use itertools::Itertools;
advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let slided = slide(&input);
    Some(count_load(&slided))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn get(platform: &[Vec<char>], r: usize, c: usize) -> Option<char> {
    Some(*platform.get(r)?.get(c)?)
}

fn set(platform: &mut [Vec<char>], ir: usize, ic: usize, v: char) {
    *platform.get_mut(ir).unwrap().get_mut(ic).unwrap() = v;
}

fn slide_once(platform: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut result = Vec::from(platform);
    for ir in 0..result.len()-1 {
        let row = result.get(ir).unwrap().clone();
        for (ic, &cell) in row.iter().enumerate() {
            let lower_cell = get(&result, ir+1, ic).unwrap_or('-');
            if cell == '.' && lower_cell == 'O' {
                set(&mut result, ir, ic, 'O');
                set(&mut result, ir+1, ic, '.');
            }
        }
    }
    result
}

fn slide(platform: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut platform = Vec::from(platform);
    let mut platform_next = slide_once(&platform);
    //let mut i = 0;
    while platform != platform_next {
        //i += 1;
        //println!("{i}:\n{}\n", to_str(&platform));
        platform = platform_next;
        platform_next = slide_once(&platform);
    }
    platform
}

#[allow(dead_code)]
fn to_str(platform: &[Vec<char>]) -> String {
    platform.iter().map(|row| row.iter().collect::<String>()).join("\n")
}

fn count_load(platform: &[Vec<char>]) -> usize {
    platform.iter().rev().enumerate().map(|(ir, row)| {
        (ir + 1) * row.iter().filter(|&&c| c == 'O').count()
    }).sum()
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
        assert_eq!(result, None);
    }
}
