use std::{collections::HashMap, ops::RangeInclusive};

use itertools::{self, Itertools};
use pathfinding::directed::bfs::bfs_reach;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<i32> {
    let (workflows, parts) = parse(input);
    let result = parts
        .iter()
        .filter(|&part| process(&workflows, part))
        .map(|Part { x, m, a, s }| x + m + a + s)
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (workflows, parts) = parse(input);
    let all_ok = all_good_parts(&workflows).into_iter().collect_vec();
    let diff = parts
        .iter()
        .filter(|&part| {
            let process = process(&workflows, part);
            let any = all_ok.iter().any(|pr| pr.contains(part));
            process != any
        })
        .collect_vec();
    assert!(diff.is_empty());
    Some(all_ok.iter().map(PartRange::len).sum())
}

enum Cmp {
    LT,
    GT,
}

struct Rule {
    category: char,
    compare: Cmp,
    constant: i32,
    state: String,
}

struct Workflow {
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
    let mut pcs = p.chars();
    Rule {
        category: pcs.next().expect("category"),
        compare: if pcs.next().expect("cmp") == '<' {
            Cmp::LT
        } else {
            Cmp::GT
        },
        constant: pcs.as_str().parse().expect("constant"),
        state: state.to_string(),
    }
}

fn parse_workflow(line: &str) -> (String, Workflow) {
    let (name, rest) = line.split_once('{').expect("name");
    let mut rrs = rest[0..rest.len() - 1].split(',').rev();
    let fallback = rrs.next().expect("fallback").to_string();
    (
        name.to_string(),
        Workflow {
            fallback,
            rules: rrs.rev().map(parse_rule).collect(),
        },
    )
}

fn parse_part(line: &str) -> Part {
    let mut cats = line[1..line.len() - 1].split(',');
    let x = cats
        .next()
        .and_then(|p| p.strip_prefix("x="))
        .and_then(|s| str::parse(s).ok())
        .expect("x");
    let m = cats
        .next()
        .and_then(|p| p.strip_prefix("m="))
        .and_then(|s| str::parse(s).ok())
        .expect("m");
    let a = cats
        .next()
        .and_then(|p| p.strip_prefix("a="))
        .and_then(|s| str::parse(s).ok())
        .expect("a");
    let s = cats
        .next()
        .and_then(|p| p.strip_prefix("s="))
        .and_then(|s| str::parse(s).ok())
        .expect("s");
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
        c => panic!("Unknown category {c}"),
    };
    match rule.compare {
        Cmp::LT => v < rule.constant,
        Cmp::GT => v > rule.constant,
    }
}

fn step<'a>(workflows: &'a Workflows, state: &str, part: &Part) -> &'a str {
    let w = &workflows[state];
    w.rules
        .iter()
        .find(|&r| match_rule(r, part))
        .map_or(&w.fallback, |r| &r.state)
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

#[derive(Eq, PartialEq, Hash, Clone)]
struct PartRange {
    x: RangeInclusive<i64>,
    m: RangeInclusive<i64>,
    a: RangeInclusive<i64>,
    s: RangeInclusive<i64>,
}

impl PartRange {
    fn with(self, select: char, value: RangeInclusive<i64>) -> PartRange {
        match select {
            'x' => PartRange { x: value, ..self },
            'm' => PartRange { m: value, ..self },
            'a' => PartRange { a: value, ..self },
            's' => PartRange { s: value, ..self },
            c => panic!("Unknown category {c}"),
        }
    }

    fn select(&self, select: char) -> &RangeInclusive<i64> {
        match select {
            'x' => &self.x,
            'm' => &self.m,
            'a' => &self.a,
            's' => &self.s,
            c => panic!("Unknown category {c}"),
        }
    }

    fn contains(&self, p: &Part) -> bool {
        self.x.contains(&(p.x as i64))
            && self.m.contains(&(p.m as i64))
            && self.a.contains(&(p.a as i64))
            && self.s.contains(&(p.s as i64))
    }

    fn len(&self) -> i64 {
        let l = |r: &RangeInclusive<i64>| 0.max(r.end() + 1 - r.start());
        l(&self.x) * l(&self.m) * l(&self.a) * l(&self.s)
    }
}

fn split_parts<'a>(
    workflows: &'a Workflows,
    (state, part): &(&str, PartRange),
) -> Vec<(&'a str, PartRange)> {
    if *state == "A" || *state == "R" {
        return Vec::new();
    }
    let mut result: Vec<(&'a str, PartRange)> = Vec::new();

    let mut part_rest = part.clone();

    let w = &workflows[*state];

    for rule in w.rules.iter() {
        let v = part_rest.select(rule.category);
        let rv = rule.constant as i64;

        let (selected, rest) = match rule.compare {
            Cmp::LT => (*v.start()..=rv - 1, rv..=*v.end()),
            Cmp::GT => (rv + 1..=*v.end(), *v.start()..=rv),
        };

        if !selected.is_empty() {
            result.push((&rule.state, part_rest.clone().with(rule.category, selected)))
        }

        if rest.is_empty() {
            break;
        } else {
            part_rest = part_rest.with(rule.category, rest);
        }
    }
    if part_rest.len() != 0 {
        result.push((&w.fallback, part_rest))
    }
    result
}

fn all_good_parts(workflows: &Workflows) -> impl IntoIterator<Item = PartRange> + '_ {
    let start = PartRange {
        x: 1..=4000,
        m: 1..=4000,
        a: 1..=4000,
        s: 1..=4000,
    };
    bfs_reach(("in", start), |s| split_parts(workflows, s))
        .filter(|(s, _)| *s == "A")
        .map(|(_, p)| p)
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
        assert_eq!(result, Some(167409079868000));
    }
}
