use regex::Regex;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
        let d1 = digits.first().expect("first char");
        let d2 = digits.last().expect("last char");
        let str: String = vec![d1, d2].into_iter().collect();
        sum += str::parse::<u32>(&*str).expect("two digits are a number");
    }
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    let re_digit = Regex::new(r"^(\d|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    for line in input.lines() {
        let mut digits = Vec::<char>::new();
        for i in 0..line.len() {
            let m = re_digit.captures(&line[i..]);
            if m.is_none() {
                continue;
            }
            match m.unwrap().get(0).unwrap().as_str() {
                "zero" => digits.push('0'),
                "one" => digits.push('1'),
                "two" => digits.push('2'),
                "three" => digits.push('3'),
                "four" => digits.push('4'),
                "five" => digits.push('5'),
                "six" => digits.push('6'),
                "seven" => digits.push('7'),
                "eight" => digits.push('8'),
                "nine" => digits.push('9'),
                c => digits.push(c.chars().next().unwrap()),
            }
        }
        let d1 = digits.first().expect("first char");
        let d2 = digits.last().expect("last char");
        let str: String = vec![d1, d2].into_iter().collect();
        sum += str::parse::<u32>(&*str).expect("two digits are a number");
    }
    return Some(sum);
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
}
