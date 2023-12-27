use std::collections::HashMap;

use itertools::iterate;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<i32> {
    let (workflows, parts) = parse(input);
    let result = parts.iter()
        .filter(|&part| process(&workflows, part))
        .map(|Part { x, m, a, s }| x + m + a + s)
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

enum Cmp {
    LT, GT
}

struct Rule {
    category: char,
    compare: Cmp,
    constant: i32,
    state: String,
}

struct Workflow{
    rules: Vec<Rule>,
    fallback: String,
}

type Workflows = HashMap<String, Workflow>;

struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

fn parse_rule(s: &str) -> Rule {
    let (p, state) = s.split_once(':').expect("goal");
    Rule {
        category: p.chars().nth(0).expect("category"),
        compare: if p.chars().nth(1).expect("cmp") == '<' {Cmp::LT} else {Cmp::GT},
        constant: p[2..].parse().expect("constant"),
        state: state.to_string(),
    }
}

fn parse_workflow(line: &str) -> (String, Workflow) {
    let (name, rest) = line.split_once('{').expect("name");
    let mut rrs = rest[0..rest.len() - 1].split(',').rev();
    let fallback = rrs.next().expect("fallback").to_string();
    (name.to_string(), Workflow {fallback, rules: rrs.rev().map(parse_rule).collect()})
}

fn parse_part(line: &str) -> Part {
    let mut cats = line[1..line.len() - 1].split(',');
    let x = cats.next().and_then(|p| p.strip_prefix("x=")).and_then(|s| str::parse(s).ok()).expect("x");
    let m = cats.next().and_then(|p| p.strip_prefix("m=")).and_then(|s| str::parse(s).ok()).expect("m");
    let a = cats.next().and_then(|p| p.strip_prefix("a=")).and_then(|s| str::parse(s).ok()).expect("a");
    let s = cats.next().and_then(|p| p.strip_prefix("s=")).and_then(|s| str::parse(s).ok()).expect("s");
    Part { x, m, a, s }
}

fn parse(input: &str) -> (Workflows, Vec<Part>) {
    let (input_ws, input_ps) = input.split_once("\n\n").expect("two input parts");
    let parts = input_ps.lines().map(parse_part).collect();
    let workflows = input_ws.lines().map(parse_workflow).collect();
    (workflows, parts)
}

fn match_rule(rule: &Rule, part: &Part) -> bool {
    let v = match rule.category {
        'x' => part.x,
        'm' => part.m,
        'a' => part.a,
        's' => part.s,
        c => panic!("Unknown category {c}")
    };
    match rule.compare {
        Cmp::LT => v < rule.constant,
        Cmp::GT => v > rule.constant,
    }
}

fn step<'a>(workflows: &'a Workflows, state: &str, part: &Part) -> &'a str {
    let w = &workflows[state];
    w.rules.iter().find(|&r| match_rule(r, part)).map_or(&w.fallback, |r| &r.state)
}

fn process(workflows: &Workflows, part: &Part) -> bool {
    let mut state = "in";
    loop {
        state = step(workflows, state, part);
        match state {
            "A" => return true,
            "R" => return false,
            _ => continue,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
