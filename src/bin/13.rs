advent_of_code::solution!(13);

fn solve(input: &str, smudge: bool) -> usize {
    let input = parse(input);
    input.iter()
        .map(|g| {
            let lr = find_reflection(g, smudge);
            let td = find_reflection(&transpose(g.as_ref()), smudge);
            100 * lr + td
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, false))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, true))
}

type Grid = Vec<Vec<char>>;
type Input = Vec<Grid>;

fn parse(input: &str) -> Input {
    input.split("\n\n").map(|g| {
        g.lines().map(|l| l.chars().collect()).collect()
    }).collect()
}

fn find_reflection(grid: &Grid, smudge: bool) -> usize {
    (0..grid.len()-1)
        .find(|&i| {
            let mut smudge = smudge;
            let eq = grid[0..=i].iter()
                .rev()
                .zip(grid[i+1..].iter())
                .all(|(l, r)| equal_with_smudge(l, r, &mut smudge));
            eq && !smudge
        })
        .map_or(0, |i| i + 1)
}

fn transpose<T>(v: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Clone,
{
    (0..v.first().map_or(0, |f| f.len()))
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn equal_with_smudge<T: Eq>(v1: &[T], v2: &[T], smudge: &mut bool) -> bool {
    if v1.len() != v2.len() {
        return false;
    }
    for (val1, val2) in v1.iter().zip(v2.iter()) {
        if val1 != val2 {
            if *smudge {
                *smudge = false;
            } else {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
