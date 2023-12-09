use itertools::Itertools;
advent_of_code::solution!(6);

fn solve(input: &str, merge_digits: bool) -> Option<usize> {
    let Input { times, distances } = parse(input, merge_digits)?;
    Some(
        times
            .into_iter()
            .zip(distances)
            .map(|(t, d)| -> usize {
                (1..t)
                    .map(|hold| (t - hold) * hold)
                    .filter(|dist| *dist > d)
                    .count()
            })
            .product1()
            .unwrap_or(0),
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, true)
}

struct Input {
    times: Vec<i64>,
    distances: Vec<i64>,
}

fn parse(input: &str, merge: bool) -> Option<Input> {
    let mut lines = input.lines();
    if merge {
        let time = lines
            .next()?
            .strip_prefix("Time:")?
            .split_ascii_whitespace()
            .join("")
            .parse()
            .ok()?;
        let distance = lines
            .next()?
            .strip_prefix("Distance:")?
            .split_ascii_whitespace()
            .join("")
            .parse()
            .ok()?;
        Some(Input {
            times: vec![time],
            distances: vec![distance],
        })
    } else {
        let times = lines
            .next()?
            .strip_prefix("Time:")?
            .split_ascii_whitespace()
            .filter_map(|w| w.parse().ok())
            .collect();
        let distances = lines
            .next()?
            .strip_prefix("Distance:")?
            .split_ascii_whitespace()
            .filter_map(|w| w.parse().ok())
            .collect();
        Some(Input { times, distances })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
