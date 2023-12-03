use std::fmt::Display;
use std::ops::Add;
advent_of_code::solution!(2);

#[derive(Copy, Clone)]
struct Reveal {
    red: i32,
    green: i32,
    blue: i32,
}

impl Reveal {
    fn new() -> Reveal {
        Reveal {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    fn from_str(input: &str) -> Option<Reveal> {
        let mut words = input.split(' ');
        let count = words.next()?.parse().ok()?;
        let cube = words.next()?;
        match cube {
            "red" => Some(Reveal {
                red: count,
                ..Reveal::new()
            }),
            "green" => Some(Reveal {
                green: count,
                ..Reveal::new()
            }),
            "blue" => Some(Reveal {
                blue: count,
                ..Reveal::new()
            }),
            _ => None,
        }
    }

    fn contains(self, small: &Reveal) -> bool {
        self.red >= small.red && self.green >= small.green && self.blue >= small.blue
    }

    fn max(self, rhs: Reveal) -> Reveal {
        Reveal {
            red: self.red.max(rhs.red),
            green: self.green.max(rhs.green),
            blue: self.blue.max(rhs.blue),
        }
    }
}

impl Display for Reveal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[R:{:3}, G:{:3}, B:{:3}]",
            self.red, self.green, self.blue
        )
    }
}

impl Add for Reveal {
    type Output = Reveal;

    fn add(self, rhs: Self) -> Self::Output {
        Reveal {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

struct Game {
    reveals: Vec<Vec<Reveal>>,
}

fn parse_game(input: &str) -> Game {
    return Game {
        reveals: input
            .split(": ")
            .last()
            .expect("game header followed by reveals")
            .split("; ")
            .map(|reveal| {
                reveal
                    .split(", ")
                    .map(|r| Reveal::from_str(r).expect("one reveal"))
                    .collect()
            })
            .collect(),
    };
}

fn count_revealed(game: &Game) -> Reveal {
    game.reveals
        .iter()
        .map(|rs| rs.iter().cloned().fold(Reveal::new(), Reveal::add))
        .fold(Reveal::new(), Reveal::max)
}

pub fn part_one(input: &str) -> Option<i32> {
    let games: Vec<Game> = input.lines().map(parse_game).collect();
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    let bag = Reveal {
        red: 12,
        green: 13,
        blue: 14,
    };
    let possible: Vec<(Reveal, i32)> = games
        .iter()
        .map(count_revealed)
        .zip(1..)
        .filter(|(m, _)| {
            //println!("{i:3}:  {} is {} {}", m, if is_in {"in    "} else {"not in"}, bag);
            bag.contains(m)
        })
        .collect();
    Some(possible.iter().map(|(_, i)| i).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let games: Vec<Game> = input.lines().map(parse_game).collect();
    let possible: Vec<Reveal> = games.iter().map(count_revealed).collect();
    Some(
        possible
            .iter()
            .map(|r| (r.red * r.green * r.blue) as u32)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
