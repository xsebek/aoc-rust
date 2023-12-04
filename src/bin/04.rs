use num::pow;
advent_of_code::solution!(4);

fn parse_card(input: &str) -> (Vec<usize>, Vec<usize>) {
    let (win, my) = input.split_once('|').expect("Two parts of card");
    let to_ints = |s: &str| -> Vec<usize> {s.split(' ').filter_map(|i| i.parse().ok()).collect()};
    (to_ints(win), to_ints(my))
}

fn count_card((win, my): (Vec<usize>, Vec<usize>)) -> usize {
    my.iter().filter(|m| win.contains(m)).count()
}

fn score_card(card: (Vec<usize>, Vec<usize>)) -> usize {
    let wins = count_card(card);
    if wins > 0 { pow(2, wins - 1) } else { 0 }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.lines().map(parse_card).map(score_card).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let wins: Vec<usize> = input.lines().map(parse_card).map(count_card).collect();
    let mut counts: Vec<usize> = vec![1; wins.len()];
    for (i, w) in wins.iter().cloned().enumerate() {
        let c = *counts.get(i).expect("current count");
        for j in i+1..i+1+w {
            *(counts.get_mut(j).expect("modified count")) += c
        }
    }
    Some(counts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_indexed("examples", DAY, Some(2)));
        assert_eq!(result, Some(30));
    }
}
