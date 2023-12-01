advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    part_generic(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    part_generic(input, true)
}

fn part_generic(input: &str, use_names: bool) -> Option<u32> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let digits: Vec<u32> = (0..line.len())
            .filter_map(|i| get_leading_digit(&line[i..], use_names))
            .collect();
        let d1 = digits.first()?;
        let d2 = digits.last()?;
        sum += 10 * d1 + d2;
    }
    Some(sum)
}

fn get_leading_digit(s: &str, use_names: bool) -> Option<u32> {
    let c = s.chars().next()?;
    if let Some(d) = c.to_digit(10) {
        Some(d)
    } else if !use_names {
        None
    } else if s.starts_with("zero") {
        Some(0)
    } else if s.starts_with("one") {
        Some(1)
    } else if s.starts_with("two") {
        Some(2)
    } else if s.starts_with("three") {
        Some(3)
    } else if s.starts_with("four") {
        Some(4)
    } else if s.starts_with("five") {
        Some(5)
    } else if s.starts_with("six") {
        Some(6)
    } else if s.starts_with("seven") {
        Some(7)
    } else if s.starts_with("eight") {
        Some(8)
    } else if s.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_indexed(
            "examples",
            DAY,
            Some(2),
        ));
        assert_eq!(result, Some(285));
    }

    #[test]
    fn test_part_one_on_two() {
        let result = part_one(&advent_of_code::template::read_file_indexed(
            "examples",
            DAY,
            Some(2),
        ));
        assert_eq!(result, None);
    }
}
