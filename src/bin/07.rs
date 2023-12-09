use enum_iterator::{all, cardinality, Sequence};
use itertools::{repeat_n, Itertools};
advent_of_code::solution!(7);

fn solve(input: &str, wild_card: bool) -> Option<usize> {
    let hand_bids = parse(input);
    let win_ordered: Vec<_> = hand_bids
        .into_iter()
        .map(|(h, b)| (rank_hand(h, wild_card), b))
        .sorted_by(|(eh1, _), (eh2, _)| Ord::cmp(eh1, eh2))
        .collect();
    Some(
        win_ordered
            .into_iter()
            .map(|(_, b)| b)
            .enumerate()
            .map(|(i, b)| (i + 1) * b)
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, true)
}

type Input = Vec<(Hand, Bid)>;

type Hand = Vec<Card>;
type Bid = usize;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, Sequence)]
enum Card {
    CJ2,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CJ,
    CQ,
    CK,
    CA,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn char_to_card(c: char) -> Card {
    use crate::Card::*;
    match c {
        '2' => C2,
        '3' => C3,
        '4' => C4,
        '5' => C5,
        '6' => C6,
        '7' => C7,
        '8' => C8,
        '9' => C9,
        'T' => CT,
        'J' => CJ,
        'Q' => CQ,
        'K' => CK,
        'A' => CA,
        _ => panic!("Unknown card: {c}"),
    }
}

fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let mut ws = l.split_ascii_whitespace();
            let hand = ws
                .next()
                .expect("bid string")
                .chars()
                .map(char_to_card)
                .collect();
            let bid = ws
                .next()
                .expect("bid string")
                .parse()
                .expect("valid bid amount");
            (hand, bid)
        })
        .collect()
}

fn rank(hand: Hand) -> Rank {
    use crate::Rank::*;
    let cards: Vec<usize> = hand
        .into_iter()
        .sorted()
        .group_by(|c| *c)
        .into_iter()
        .map(|(_c, v)| v.count())
        .sorted()
        .rev()
        .collect();
    match cards.as_slice() {
        // AAAAA - Five of a kind
        [5] => FiveOfAKind,
        // AA8AA - Four of a kind
        [4, 1] => FourOfAKind,
        // 23332 - Full house
        [3, 2] => FullHouse,
        // TTT98 - Three of a kind
        [3, ..] => ThreeOfAKind,
        // 23432 - Two pair
        [2, 2, ..] => TwoPair,
        // A23A4 - One pair
        [2, ..] => OnePair,
        // 23456 - High card
        _ => HighCard,
    }
}

fn rank_hand(hand: Hand, wild_card: bool) -> (Rank, Hand) {
    if wild_card {
        (
            wild_card_options(&hand)
                .into_iter()
                .map(rank)
                .max()
                .expect("a best hand"),
            hand.into_iter()
                .map(|c| if c == Card::CJ { Card::CJ2 } else { c })
                .collect(),
        )
    } else {
        (rank(hand.clone()), hand)
    }
}

fn wild_card_options(hand: &Hand) -> Vec<Hand> {
    transpose(
        hand.iter()
            .map(|c| {
                if *c == Card::CJ {
                    all::<Card>().filter(|c| *c != Card::CJ).collect::<Vec<_>>()
                } else {
                    repeat_n(*c, cardinality::<Card>() - 1).collect::<Vec<_>>()
                }
            })
            .collect(),
    )
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..v.first().map_or(0, |f| f.len()))
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
