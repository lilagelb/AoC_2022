use std::fmt::{Display, Formatter};

pub trait Day {
    type TypePart1: Display;
    type TypePart2: Display;

    fn run(&self) -> Answer<Self::TypePart1, Self::TypePart2>;
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