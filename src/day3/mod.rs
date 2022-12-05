use crate::day::{Answer, Day};
use regex::Regex;

pub struct Day3;
impl Day for Day3 {
    type TypePart1 = u32;
    type TypePart2 = u32;

    fn run(&self) -> Answer<Self::TypePart1, Self::TypePart2> {
        let input = self.get_input_for_day_by_line(3);

        let mut part_1 = 0u32;
        for line in &input {
            let compartments = [&line[0..line.len()/2], &line[line.len()/2..]];
            let re = Regex::new(format!(r"([{}])", compartments[0]).as_str())
                .expect("failed to create regex");
            let captures = re.captures(compartments[1]).unwrap();
            part_1 += Self::letter_to_priority(&captures[0]);
        }

        let mut part_2 = 0u32;
        for i in (0..input.len()).step_by(3) {
            let elf1 = &input[i];
            let elf2 = &input[i+1];
            let elf3 = &input[i+2];
            let re = Regex::new(&format!(r"([{}])", elf1)).unwrap();
            let common_between_one_and_two = re.find_iter(elf2);
            for common_letter in common_between_one_and_two {
                if elf3.contains(common_letter.as_str()) {
                    part_2 += Self::letter_to_priority(common_letter.as_str());
                    break;
                }
            }
        }

        Answer::new(Some(part_1), Some(part_2))
    }
}
impl Day3 {
    fn letter_to_priority(letter: &str) -> u32 {
        if letter.to_ascii_lowercase() == letter {
            return letter.as_bytes()[0] as u32 - 96
        }
        else {
            return letter.as_bytes()[0] as u8 as u32 - 38
        }
    }
}