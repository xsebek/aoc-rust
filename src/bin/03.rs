use itertools::Itertools;

advent_of_code::solution!(3);

#[derive(Copy, Clone)]
struct Pos {
    line: usize,
    start: usize,
    stop: usize,
}

fn digit_span_to_num<I>(line: usize, span: I) -> (u32, Pos)
where
    I: Iterator<Item = (usize, char)>,
{
    let (is, ds): (Vec<usize>, String) = span.unzip();
    let d = ds.parse().expect("digits form a number");
    let p = Pos {
        line,
        start: *is.first().expect("Nonempty positions"),
        stop: *is.last().expect("Nonempty positions"),
    };
    (d, p)
}

fn get_numbers_with_position((line, input): (usize, &str)) -> Vec<(u32, Pos)> {
    input
        .chars()
        .enumerate()
        .group_by(|(_i, c)| c.is_ascii_digit())
        .into_iter()
        .filter_map(|(is_digits, g)| {
            if is_digits {
                Some(digit_span_to_num(line, g))
            } else {
                None
            }
        })
        .collect()
}

fn pos_neighborhood(pos: Pos) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let start = if pos.start == 0 { 0 } else { pos.start - 1 };
    for c in start..=(pos.stop + 1) {
        if pos.line != 0 {
            result.push((pos.line - 1, c));
        }
        if c < pos.start || c > pos.stop {
            result.push((pos.line, c))
        }
        result.push((pos.line + 1, c));
    }
    result
}

fn is_symbol(c: &u8) -> bool {
    !c.is_ascii_digit() && *c != b'.'
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let num_pos: Vec<(u32, Pos)> = input
        .lines()
        .enumerate()
        .flat_map(get_numbers_with_position)
        .collect();
    let mut sum = 0;
    for (num, pos) in num_pos {
        for (row, col) in pos_neighborhood(pos) {
            if lines
                .get(row)
                .map_or(false, |l| l.as_bytes().get(col).map_or(false, is_symbol))
            {
                sum += num;
                break;
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let num_pos: Vec<Vec<(u32, Pos)>> = input
        .lines()
        .enumerate()
        .map(get_numbers_with_position)
        .collect();

    let mut sum = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.as_bytes().iter().enumerate() {
            if is_symbol(c) {
                let neighbors: Vec<u32> = num_pos[0.max(row - 1)..=num_pos.len().min(row + 1)]
                    .iter()
                    .flatten()
                    .filter_map(|(n, p)| {
                        if pos_neighborhood(*p).contains(&(row, col)) {
                            Some(n)
                        } else {
                            None
                        }
                    })
                    .cloned()
                    .collect();
                if neighbors.len() > 1 {
                    sum += neighbors.iter().product::<u32>();
                }
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
