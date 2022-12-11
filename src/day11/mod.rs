use std::fs;
use lazy_static::lazy_static;
use regex::Regex;
use crate::day::{Answer, Day};

pub struct Day11;
impl Day for Day11 {
    type TypePart1 = u64;
    type TypePart2 = u64;

    fn run(&mut self) -> Answer<Self::TypePart1, Self::TypePart2> {
        let input = fs::read_to_string("src/day11/input.txt").unwrap();

        let mut monkeys = create_monkeys(&input, false);

        for _round in 0..20 {
            // Note: this has to be done with indexes so that monkeys can be mutably borrowed at the
            //       bottom of the inner for loop
            for monkey_index in 0..monkeys.len() {
                for _item_index in 0..monkeys[monkey_index].items.len() {
                    let worry_level = monkeys[monkey_index].items.remove(0);
                    let (new_owner, new_worry_level)
                        = monkeys[monkey_index].process_item_part_1(worry_level);

                    monkeys[new_owner].items.push(new_worry_level);
                }
            }
        }
        let mut items_inspected: Vec<u64> = monkeys.iter()
            .map(|monkey| monkey.items_inspected)
            .collect();
        items_inspected.sort();
        let part_1 = items_inspected.iter().rev().take(2).product();


        let mut monkeys = create_monkeys(&input, true);

        for _round in 0..10000 {
            // Note: this has to be done with indexes so that monkeys can be mutably borrowed at the
            //       bottom of the inner for loop
            for monkey_index in 0..monkeys.len() {
                for _item_index in 0..monkeys[monkey_index].items.len() {
                    let worry_level = monkeys[monkey_index].items.remove(0);
                    let (new_owner, new_worry_level)
                        = monkeys[monkey_index].process_item_part_2(worry_level);

                    monkeys[new_owner].items.push(new_worry_level);
                }
            }
        }
        let mut items_inspected: Vec<u64> = monkeys.iter()
            .map(|monkey| monkey.items_inspected)
            .collect();
        items_inspected.sort();
        let part_2 = items_inspected.iter().rev().take(2).product();

        Answer::new(Some(part_1), Some(part_2))
    }
}
impl Day11 {
    pub fn new() -> Day11 {
        Day11
    }
}

fn create_monkeys(text: &str, part_2: bool) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut worry_level_modulus = 1u64;

    for monkey in text.split("\n\n") {
        let monkey = Monkey::from(monkey);
        worry_level_modulus *= monkey.test_denominator;
        monkeys.push(monkey);
    }
    for monkey in &mut monkeys {
        monkey.set_worry_level_modulus(worry_level_modulus);
    }

    monkeys
}

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    worry_level_modulus: u64,
    test_denominator: u64,
    monkey_if_test_true: usize,
    monkey_if_test_false: usize,
    items_inspected: u64,
}
impl Monkey {
    fn from(text: &str) -> Monkey {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Monkey (?P<monkey_number>\d+):\n  Starting items: (?P<starting_items>[0-9 ,]*)\n  Operation: new = old (?P<operation_operand>[+\-*/]) (?P<operation_rhs>\d+|old)\n  Test: divisible by (?P<test_denominator>\d+)\n    If true: throw to monkey (?P<true_monkey>\d+)\n    If false: throw to monkey (?P<false_monkey>\d+)").unwrap();
        }
        let captures = RE.captures(text).unwrap();
        let items = captures.name("starting_items").unwrap().as_str()
            .split(", ")
            .map(|elem| elem.parse().unwrap())
            .collect();
        let operation_operand = captures.name("operation_operand").unwrap().as_str();
        let operation_rhs = captures.name("operation_rhs").unwrap().as_str();
        let test_denominator = captures.name("test_denominator").unwrap().as_str().parse::<u64>().unwrap();
        let monkey_if_test_true: usize = captures.name("true_monkey").unwrap().as_str().parse().unwrap();
        let monkey_if_test_false: usize = captures.name("false_monkey").unwrap().as_str().parse().unwrap();

        let operation: Box<dyn Fn(u64) -> u64> = if operation_rhs == "old" {
            Box::new(|old_level| old_level * old_level)
        } else {
            let rhs = operation_rhs.parse::<u64>().unwrap();
            if operation_operand == "+" {
                Box::new(move |old_level| old_level + rhs)
            } else {
                Box::new(move |old_level| old_level * rhs)
            }
        };

        Monkey {
            items,
            operation,
            worry_level_modulus: 0,
            test_denominator,
            monkey_if_test_true,
            monkey_if_test_false,
            items_inspected: 0,
        }
    }
    fn set_worry_level_modulus(&mut self, modulus: u64) {
        self.worry_level_modulus = modulus;
    }

    fn process_item_part_1(&mut self, item: u64) -> (usize, u64) {
        self.items_inspected += 1;

        let mut worry_level = (self.operation)(item).into();
        worry_level /= 3;

        if worry_level % self.test_denominator == 0 {
            (self.monkey_if_test_true, worry_level)
        } else {
            (self.monkey_if_test_false, worry_level)
        }
    }
    fn process_item_part_2(&mut self, item: u64) -> (usize, u64) {
        self.items_inspected += 1;

        let mut worry_level = (self.operation)(item).into();
        worry_level %= self.worry_level_modulus;

        if worry_level % self.test_denominator == 0 {
            (self.monkey_if_test_true, worry_level)
        } else {
            (self.monkey_if_test_false, worry_level)
        }
    }
}