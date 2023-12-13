use itertools::Itertools;
advent_of_code::solution!(11);

fn solve(factor: usize, input: &str) -> Option<usize> {
    let input = parse(input);
    let expanded_universe: Vec<_> = expand_universe(factor, input.as_slice());
    let mut sum = 0;
    for pair in expanded_universe.into_iter().tuple_combinations::<(_, _)>() {
        if pair.0 > pair.1 {
            continue;
        }
        sum += manhattan(pair);
    }
    Some(sum)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(2, input)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(1_000_000, input)
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn expand_universe(factor: usize, universe: &[Vec<char>]) -> Vec<(usize, usize)> {
    let empty_row = find_all_empty(universe);
    let empty_col = find_all_empty(&transpose(universe));
    let factor = factor - 1;

    universe
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            let row = &row;
            line.iter()
                .enumerate()
                .filter_map(|(col, c)| {
                    if *c == '.' {
                        return None;
                    }
                    let e_rows = empty_row.iter().take_while(|r| **r < *row).count();
                    let e_cols = empty_col.iter().take_while(|c| **c < col).count();
                    Some((factor * e_rows + *row, factor * e_cols + col))
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn find_all_empty(vvs: &[Vec<char>]) -> Vec<usize> {
    vvs.iter()
        .enumerate()
        .filter(|(_r, l)| l.iter().all_equal())
        .map(|p| p.0)
        .collect()
}

fn transpose<T>(v: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..v.first().map_or(0, |f| f.len()))
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn manhattan(((x1, y1), (x2, y2)): ((usize, usize), (usize, usize))) -> usize {
    usize::abs_diff(x2, x1) + usize::abs_diff(y2, y1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        assert_eq!(solve(10, input), Some(1030));
        assert_eq!(solve(100, input), Some(8410));
    }
}
