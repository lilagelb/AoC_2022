use crate::day::{Answer, Day};
use regex::Regex;

pub struct Day4;
impl Day for Day4 {
    type TypePart1 = u32;
    type TypePart2 = u32;

    fn run(&self) -> Answer<Self::TypePart1, Self::TypePart2> {
        let input = self.get_input_for_day_by_line(4);

        let pair_extraction_re = Regex::new(
            r"(?P<elf_1_start>\d+)-(?P<elf_1_end>\d+),(?P<elf_2_start>\d+)-(?P<elf_2_end>\d+)"
        ).unwrap();

        let mut part_1 = 0u32;
        let mut part_2 = 0u32;
        for pair in input {
            let captures = pair_extraction_re.captures(&pair).unwrap();
            let elf_1_start = captures.name("elf_1_start").unwrap().as_str().parse::<u32>().unwrap();
            let elf_1_end = captures.name("elf_1_end").unwrap().as_str().parse::<u32>().unwrap();
            let elf_2_start = captures.name("elf_2_start").unwrap().as_str().parse::<u32>().unwrap();
            let elf_2_end = captures.name("elf_2_end").unwrap().as_str().parse::<u32>().unwrap();

            // one is completely inside the other
            if elf_1_start <= elf_2_start && elf_2_end <= elf_1_end
                || elf_2_start <= elf_1_start && elf_1_end <= elf_2_end
            {
                part_1 += 1;
                part_2 += 1;
            }
            // the regions overlap
            else if elf_1_start <= elf_2_start && elf_2_start <= elf_1_end
                || elf_2_start <= elf_1_start && elf_1_start <= elf_2_end
            {
                part_2 += 1;
            }
        }
        Answer::new(Some(part_1), Some(part_2))
    }
}
impl Day4 {
    pub fn new() -> Day4 {
        Day4
    }
}