use itertools::Itertools;
advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split(',')
            .map(|e| e.trim())
            .map(Hash::runs)
            .map(|h| h.0)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .split(',')
            .map(|e| e.trim())
            .fold(HashMap::new(), HashMap::step)
            .boxes
            .iter()
            .enumerate()
            .map(focusing_power)
            .sum(),
    )
}

struct Hash(u32);

impl Hash {
    fn run(self, value: u8) -> Self {
        Hash((self.0 + value as u32) * 17 % 256)
    }

    fn runs(values: &str) -> Self {
        values.bytes().fold(Hash(0), Hash::run)
    }
}

type Lens = u8;

struct HashMap {
    boxes: [Vec<(String, Lens)>; 256],
}

impl HashMap {
    fn new() -> Self {
        HashMap {
            boxes: std::array::from_fn(|_| Vec::new()),
        }
    }

    fn step(mut self, instruction: &str) -> Self {
        if let Some((label, lens)) = instruction.split_once('=') {
            self.update_lens(label, lens);
        } else if let Some(label) = instruction.strip_suffix('-') {
            self.remove_lens(label);
        } else {
            panic!("Unparsed instruction: '{instruction}'")
        }
        // println!("\nAfter '{instruction}':");
        // self.print();
        self
    }

    fn remove_lens(&mut self, label: &str) {
        let box_num = Hash::runs(label).0 as usize;
        let pos = self.boxes[box_num]
            .iter()
            .position(|(lab, _l)| lab == label);
        if let Some(pos) = pos {
            self.boxes[box_num].remove(pos);
        }
    }

    fn update_lens(&mut self, label: &str, lens: &str) {
        let box_num = Hash::runs(label).0 as usize;
        let lens = lens.parse().expect("lens number");
        let pos = self.boxes[box_num]
            .iter()
            .position(|(lab, _l)| lab == label);
        if let Some(pos) = pos {
            self.boxes[box_num][pos].1 = lens;
        } else {
            self.boxes[box_num].push((label.to_string(), lens))
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for (i, b) in self.boxes.iter().enumerate() {
            if !b.is_empty() {
                let lenses = b
                    .iter()
                    .map(|(lab, lens)| format!("[{lab} {lens}]"))
                    .join(" ");
                println!("Box {i}: {lenses}");
            }
        }
    }
}

fn focusing_power((box_index, lenses): (usize, &Vec<(String, Lens)>)) -> usize {
    lenses
        .iter()
        .enumerate()
        .map(|(lens_index, (_lab, lens))| (box_index + 1) * (lens_index + 1) * (*lens as usize))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(Hash::runs("HASH").0, 52);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
