use itertools::Itertools;
advent_of_code::solution!(9);

fn solve(input: &str, forward: bool) -> Option<i32> {
    let input = parse(input);
    let interpolated: Vec<Vec<Vec<i32>>> = input.into_iter()
        .map(|l| interpolate(l, forward))
        .collect();
    Some(interpolated.into_iter()
        .map(|vs| {
            if forward {
                *vs.first().expect("first vector").last().expect("interpolated value")
            } else {
                *vs.first().expect("first vector").first().expect("interpolated value")
            }
        }).sum()
    )
}

pub fn part_one(input: &str) -> Option<i32> {
    solve(input, true)
}

pub fn part_two(input: &str) -> Option<i32> {
    solve(input, false)
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|line| {
        line.split_ascii_whitespace().map(|w|
            w.parse().expect("integer separated by spaces")
        ).collect()
    }).collect()
}

fn differentiate(values: &[i32]) -> Vec<i32> {
    values.iter().zip(values.iter().dropping(1)).map(|(l, r)| r - l).collect()
}

fn interpolate(values: Vec<i32>, forward: bool) -> Vec<Vec<i32>> {
    let mut result = vec![values];
    while !result.last().unwrap().iter().all_equal() {
        result.push(differentiate(result.last().unwrap()))
    }
    let last_value = *result.last().unwrap().last().expect("nonempty values");
    result.last_mut().unwrap().push(last_value);
    for i in (0..result.len()-1).rev() {
        if forward {
            let delta = *result.get(i+1).unwrap().last().expect("nonempty deltas");
            let previous = *result.get(i).unwrap().last().expect("nonempty current");
            result.get_mut(i).unwrap().push(previous + delta)
        } else {
            let delta = *result.get(i+1).unwrap().first().expect("nonempty deltas");
            let previous = *result.get(i).unwrap().first().expect("nonempty current");
            result.get_mut(i).unwrap().insert(0, previous - delta)
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
