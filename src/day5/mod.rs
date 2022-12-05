use crate::day::{Answer, Day};
use regex::Regex;
use std::fs;

pub struct Day5;
impl Day for Day5 {
    type TypePart1 = String;
    type TypePart2 = String;

    fn run(&self) -> Answer<Self::TypePart1, Self::TypePart2> {
        let input = fs::read_to_string("src/day5/input.txt").unwrap()
            .split("\n\n")
            .map(|elem| elem.to_string())
            .collect::<Vec<String>>();
        let stacks_initial = input[0].split("\n");
        let operations = input[1].trim_end_matches("\n").split("\n");

        let mut part_1_stacks: Vec<Vec<&str>> = Vec::new();
        let stack_crate_re = Regex::new(r"\[[A-Z]\]").unwrap();
        for line in stacks_initial {
            for crate_match in stack_crate_re.find_iter(line) {
                let stack_number = crate_match.start()/4;
                if part_1_stacks.len() <= stack_number {
                    part_1_stacks.resize(stack_number + 1, Vec::new());
                }
                part_1_stacks[stack_number].insert(0, crate_match.as_str().get(1..2).unwrap())
            }
        }
        let mut part_2_stacks = part_1_stacks.clone();

        let operation_re = Regex::new(
            r"move (?P<number_to_move>\d+) from (?P<from_stack>\d+) to (?P<to_stack>\d+)"
        ).unwrap();
        for operation in operations {
            let operation_captures = operation_re.captures(operation).unwrap();
            let number_to_move = operation_captures.name("number_to_move").unwrap()
                .as_str().parse::<usize>().unwrap();
            // note the minus ones to make the stacks zero-indexed
            let from_stack = operation_captures.name("from_stack").unwrap()
                .as_str().parse::<usize>().unwrap() - 1;
            let to_stack = operation_captures.name("to_stack").unwrap()
                .as_str().parse::<usize>().unwrap() - 1;

            let mut part_2_popped_stack = Vec::new();
            for _ in 0..number_to_move {
                let popped_part_1 = part_1_stacks[from_stack].pop().unwrap();
                let popped_part_2 = part_2_stacks[from_stack].pop().unwrap();
                part_1_stacks[to_stack].push(popped_part_1);
                part_2_popped_stack.insert(0, popped_part_2);
            }
            part_2_stacks[to_stack].append(&mut part_2_popped_stack);
        }

        let mut part_1 = String::new();
        let mut part_2 = String::new();
        for (part_1_stack, part_2_stack) in part_1_stacks.iter().zip(part_2_stacks) {
            part_1 += part_1_stack.last().unwrap();
            part_2 += part_2_stack.last().unwrap();
        }

        Answer::new(Some(part_1), Some(part_2))
    }
}