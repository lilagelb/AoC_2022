use crate::day::{Answer, Day};
use std::fs;

pub struct Day1;
impl Day for Day1 {
    type TypePart1 = i32;
    type TypePart2 = i32;

    fn run(&self) -> Answer<Self::TypePart1, Self::TypePart2> {
        let input = fs::read_to_string("src/day1/input.txt")
            .expect("error reading day 1 input")
            .split("\n\n")
            .map(|elf| {
                elf.split("\n")
                    .map(|elem| elem.parse::<i32>().unwrap_or(0))
                    .collect()
            })
            .collect::<Vec<Vec<i32>>>();

        let mut carried_calories = input.iter()
            .map(|elem| elem.iter().sum::<i32>())
            .collect::<Vec<i32>>();
        carried_calories.sort();
        carried_calories.reverse();

        Answer::new(Some(carried_calories[0]), Some(carried_calories[0..3].iter().sum::<i32>()))
    }
}
impl Day1 {
    pub fn new() -> Day1 {
        Day1
    }
}