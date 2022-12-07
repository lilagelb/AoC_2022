use crate::day::{Answer, Day};

pub struct Day8;
impl Day for Day8 {
    type TypePart1 = u32;
    type TypePart2 = u32;

    fn run(&mut self) -> Answer<Self::TypePart1, Self::TypePart2> {
        let input = self.get_input_for_day_by_line(8);

        Answer::new(None, None)
    }
}
impl Day8 {
    pub fn new() -> Day8 {
        Day8
    }
}
