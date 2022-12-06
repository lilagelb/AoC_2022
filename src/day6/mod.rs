use crate::day::{Answer, Day};
use std::collections::HashSet;
use std::fs;

pub struct Day6;
impl Day for Day6 {
    type TypePart1 = usize;
    type TypePart2 = usize;

    fn run(&self) -> Answer<Self::TypePart1, Self::TypePart2> {
        let input = fs::read_to_string("src/day6/input.txt").unwrap();
        let input = input.as_bytes();

        let mut part_1 = 0;
        let mut part_2 = 0;

        for i in 4..input.len() {
            if Self::are_all_unique(&input[i-4..i]) {
                part_1 = i;
                break;
            }
        }
        for i in 14..input.len() {
            if Self::are_all_unique(&input[i-14..i]) {
                part_2 = i;
                break;
            }
        }

        Answer::new(Some(part_1), Some(part_2))
    }
}
impl Day6 {
    fn are_all_unique(letters: &[u8]) -> bool {
        let mut unique_letters = HashSet::new();
        for letter in letters {
            if !unique_letters.insert(letter) {
                return false;
            }
        }
        return true;
    }
}