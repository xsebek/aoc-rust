use num::integer::lcm;
use std::collections::HashMap;
advent_of_code::solution!(8);

fn count_steps<F>(network: &Network, steps: &str, state: &str, predicate: F) -> usize
where
    F: Fn(&str) -> bool,
{
    let mut count = 0;
    let mut state = state;
    while !predicate(state) {
        let dir = steps.as_bytes()[count % steps.len()];
        count += 1;
        state = step(network, state, dir);
    }
    count
}

pub fn part_one(input: &str) -> Option<usize> {
    let Input { steps, network } = parse(input);
    Some(count_steps(&network, &steps, "AAA", |s| s == "ZZZ"))
}

pub fn part_two(input: &str) -> Option<usize> {
    let Input { steps, network } = parse(input);
    let starts: Vec<&String> = network.keys().filter(|k| k.ends_with('A')).collect();
    let ends: Vec<usize> = starts
        .iter()
        .map(|s| count_steps(&network, &steps, s, |e| e.ends_with('Z')))
        .collect();
    Some(ends.into_iter().reduce(lcm).unwrap_or(0))
}

struct Input {
    steps: String,
    network: Network,
}

type Network = HashMap<String, (String, String)>;

fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let steps = lines
        .next()
        .expect("first line contains L/R steps")
        .to_string();
    lines.next();
    let network = HashMap::from_iter(
        lines
            .map(|line| {
                let (n, lr) = line.split_once(" = ")?;
                let (l, r) = lr.strip_prefix('(')?.strip_suffix(')')?.split_once(", ")?;
                Some((n.to_string(), (l.to_string(), r.to_string())))
            })
            .map(|v| v.expect("all network lines should be valid")),
    );
    Input { steps, network }
}

fn step<'a>(network: &'a Network, state: &str, direction: u8) -> &'a str {
    match direction {
        b'L' => network.get(state).unwrap().0.as_str(),
        b'R' => network.get(state).unwrap().1.as_str(),
        _ => panic!("Unknown direction '{direction}'"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
        let result = part_one(&advent_of_code::template::read_file_indexed(
            "examples",
            DAY,
            Some(1),
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_indexed(
            "examples",
            DAY,
            Some(2),
        ));
        assert_eq!(result, Some(6));
    }
}
