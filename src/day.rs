use std::fmt::{Display, Formatter};
use std::fs;

pub trait Day {
    type TypePart1: Display;
    type TypePart2: Display;

    fn run(&mut self) -> Answer<Self::TypePart1, Self::TypePart2>;

    fn get_input_for_day_by_line(&self, day: u32) -> Vec<String> {
        fs::read_to_string(format!("src/day{}/input.txt", day))
            .expect(&format!("failed to get input for day {}", day))
            .trim_end()
            .split("\n")
            .map(|elem| elem.to_string())
            .collect()
    }
}

pub struct Answer<TPart1: Display, TPart2: Display> {
    part_1: Option<TPart1>,
    part_2: Option<TPart2>,
}
impl<TPart1: Display, TPart2: Display> Display for Answer<TPart1, TPart2> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "part 1: ")?;
        match &self.part_1 {
            Some(answer) => write!(f, "{}\n", answer)?,
            None => write!(f, "not given\n")?,
        };
        write!(f, "part 2: ")?;
        match &self.part_2 {
            Some(answer) => write!(f, "{}\n", answer)?,
            None => write!(f, "not given\n")?,
        };
        Ok(())
    }
}
impl<TPart1: Display, TPart2: Display> Answer<TPart1, TPart2> {
    pub fn new(part_1: Option<TPart1>, part_2: Option<TPart2>) -> Answer<TPart1, TPart2> {
        Answer {
            part_1, part_2
        }
    }
}