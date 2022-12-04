use crate::day::{Answer, Day};

struct Day5;
impl Day for Day5 {
    type TypePart1 = u32;
    type TypePart2 = u32;

    fn run(&self) -> Answer<Self::TypePart1, Self::TypePart2> {
        let input = self.get_input_for_day_by_line(5);

        Answer::new(None, None)
    }
}