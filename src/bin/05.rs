use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i64> {
    let Input { seeds, maps } = parse(input)?;
    seeds
        .iter()
        .map(|s| {
            maps.iter().fold(*s, |acc, map| {
                convert::source_to_destination(&map.maps, acc)
            })
        })
        .min()
}

pub fn part_two(input: &str) -> Option<i64> {
    let Input { seeds, maps } = parse(input)?;
    let seed_pairs = seeds.into_iter().chunks(2);
    let seed_ranges: Vec<Range> = seed_pairs
        .into_iter()
        .map(|mut v| {
            let start = v.next().unwrap();
            let stop = start + v.next().unwrap();
            Range { start, stop }
        })
        .collect();

    maps.iter()
        .fold(seed_ranges, |acc, map| {
            acc.iter()
                .flat_map(|a| convert::source_range_to_destination(&map.maps, *a))
                .collect()
        })
        .into_iter()
        .map(|r| r.start)
        .min()
}

struct Input {
    seeds: Vec<i64>,
    maps: Vec<CategoryMap>,
}

struct CategoryMap {
    #[allow(dead_code)]
    from: String,
    #[allow(dead_code)]
    to: String,
    maps: Vec<convert::Map>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Range {
    start: i64,
    stop: i64,
}

impl Range {
    fn is_empty(self) -> bool {
        self.start >= self.stop
    }
}

mod convert {
    use crate::Range;

    #[derive(Eq, PartialEq, Copy, Clone, Debug)]
    pub struct Map {
        pub destination_start: i64,
        pub source_start: i64,
        pub range_len: i64,
    }

    pub fn parse_map(input: &str) -> Option<Map> {
        let iss = input.split(' ').flat_map(|s| str::parse(s).ok());
        match iss.collect::<Vec<i64>>()[..] {
            [destination_start, source_start, range_len] => Some(Map {
                destination_start,
                source_start,
                range_len,
            }),
            _ => None,
        }
    }

    impl Map {
        pub fn source(self) -> Range {
            Range {
                start: self.source_start,
                stop: self.source_start + self.range_len,
            }
        }

        pub fn destination(self) -> Range {
            Range {
                start: self.destination_start,
                stop: self.destination_start + self.range_len,
            }
        }

        pub fn to_destination(self, range: Range) -> Range {
            assert!(!range.is_empty() && !self.source().is_empty() && !self.destination().is_empty());
            assert!(self.source().start <= range.start && range.stop <= self.source().stop);
            let start = self.destination_start + (range.start - self.source_start);
            let stop = start + (range.stop - range.start);
            Range { start, stop }
        }

        pub fn source_to_destination(self, source: i64) -> Option<i64> {
            if source >= self.source_start && source <= self.source_start + self.range_len {
                Some(self.destination_start + source - self.source_start)
            } else {
                None
            }
        }
    }

    pub fn source_to_destination(maps: &[Map], source: i64) -> i64 {
        maps.iter()
            .flat_map(|m| (*m).source_to_destination(source))
            .next()
            .unwrap_or(source)
    }

    pub fn source_range_to_destination(maps: &[Map], range: Range) -> Vec<Range> {
        let (rs, ds) = maps
            .iter()
            .fold((vec![range], Vec::new()), |(rs, ds), map| {
                let mut dss = ds.clone();
                let mut rss = Vec::new();
                for r in rs {
                    let (rs, o_d) = map_range(map, r);
                    if let Some(d) = o_d {
                        dss.push(d)
                    }
                    rss.extend(rs)
                }
                (rss, dss)
            });
        ds.into_iter()
            .chain(rs)
            .filter(|d| !d.is_empty())
            .collect()
    }

    fn map_range(map: &Map, r: Range) -> (Vec<Range>, Option<Range>) {
        let s = map.source();
        let to_dest = |res| Some(map.to_destination(res));
        if r.is_empty() || s.is_empty() || s.stop <= r.start || r.stop <= s.start {
            // AB CD or CD AB
            (vec![r], None)
        } else if s.start <= r.start && r.stop <= s.stop {
            // A|CD|B
            //  ^^^^
            (Vec::new(), to_dest(r))
        } else if s.start <= r.start && s.stop <= r.stop {
            // A|CB||BD|
            //  ^^^^
            (vec![Range {start: s.stop, stop: r.stop}],
             to_dest(Range {start: r.start, stop: s.stop}))
        } else if r.start <= s.start && s.stop <= r.stop {
            // |CA||AB||BD|
            //     ^^^^
            (
                vec![
                    Range {
                        start: r.start,
                        stop: s.start,
                    },
                    Range {
                        start: s.stop,
                        stop: r.stop,
                    },
                ],
                to_dest(s),
            )
        } else if r.start <= s.start && r.stop <= s.stop {
            // |CA||AD|B
            //     ^^^^
            (
                vec![Range {
                    start: r.start,
                    stop: s.start,
                }],
                to_dest(Range {
                    start: s.start,
                    stop: r.stop,
                }),
            )
        } else {
            panic!("Unhandled interval intersection: R {r:?}, S {s:?}");
        }
    }
}

fn parse(input: &str) -> Option<Input> {
    let mut lines = input.lines();
    let seeds = lines
        .next()?
        .split_once(':')?
        .1
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    lines.next();
    let mut maps = Vec::new();
    for (empty, mut cat) in &lines.group_by(|l| l.trim().is_empty()) {
        if empty {
            continue;
        }
        let name = cat.next()?.strip_suffix(" map:")?.split_once("-to-")?;
        let c_maps = cat.filter_map(convert::parse_map).collect();
        maps.push(CategoryMap {
            from: name.0.to_string(),
            to: name.1.to_string(),
            maps: c_maps,
        })
    }
    Some(Input { seeds, maps })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let maps = vec![
            convert::Map {
                destination_start: 50,
                source_start: 98,
                range_len: 2,
            },
            convert::Map {
                destination_start: 52,
                source_start: 50,
                range_len: 48,
            },
        ];
        let result = (48..=51)
            .map(|i| convert::source_to_destination(&maps, i))
            .collect::<Vec<i64>>();

        assert_eq!(result, vec![48, 49, 52, 53]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
