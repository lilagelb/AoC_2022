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
        let mut items_inspected_part_1: Vec<u64> = monkeys.iter()
            .map(|monkey| monkey.items_inspected)
            .collect();
        items_inspected_part_1.sort();
        items_inspected_part_1.reverse();
        let part_1 = items_inspected_part_1[0] * items_inspected_part_1[1];


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
        items_inspected.reverse();
        let part_2 = items_inspected[0] * items_inspected[1];

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
    for monkey in text.split("\n\n") {
        monkeys.push(Monkey::from(monkey));
    }
    let mut factors_to_maintain = Vec::new();
    if part_2 {
        factors_to_maintain = monkeys.iter()
            .map(|monkey| monkey.test_denominator)
            .collect();
    }
    else {
        // this essentially disables the modulus arithmetic
        factors_to_maintain = vec![0xffffffffffffffffu64];
    }
    for monkey in &mut monkeys {
        monkey.set_worry_level_factors(&factors_to_maintain);
    }

    monkeys
}

struct Monkey {
    items: Vec<WorryLevel>,
    operation: Box<dyn Fn(&mut WorryLevel)>,
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
            .map(|elem| WorryLevel::from(elem.parse().unwrap()))
            .collect();
        let operation_operand = captures.name("operation_operand").unwrap().as_str();
        let operation_rhs = captures.name("operation_rhs").unwrap().as_str();
        let test_denominator = captures.name("test_denominator").unwrap().as_str().parse::<u64>().unwrap();
        let monkey_if_test_true: usize = captures.name("true_monkey").unwrap().as_str().parse().unwrap();
        let monkey_if_test_false: usize = captures.name("false_monkey").unwrap().as_str().parse().unwrap();

        let operation: Box<dyn Fn(&mut WorryLevel)> = if operation_rhs == "old" {
            Box::new(|old| old.update_square())
        } else {
            let rhs = operation_rhs.parse::<u64>().unwrap();
            if operation_operand == "+" {
                Box::new(move |old_level| {
                    for (factor, remainder) in old_level.factors_and_remainders.iter_mut() {
                        *remainder = (*remainder + rhs) % *factor;
                    }}
                )
            } else {
                Box::new(move |old| old.update_multiply(rhs))
            }
        };

        Monkey {
            items,
            operation,
            test_denominator,
            monkey_if_test_true,
            monkey_if_test_false,
            items_inspected: 0,
        }
    }
    fn set_worry_level_factors(&mut self, factors: &Vec<u64>) {
        for item in &mut self.items {
            item.set_factors(factors)
        }
    }

    fn process_item_part_1(&mut self, worry_level: WorryLevel) -> (usize, WorryLevel) {
        self.items_inspected += 1;

        let mut worry_level = worry_level;
        (self.operation)(&mut worry_level);
        worry_level.update_floor_divide(3);

        if worry_level.factors_and_remainders[0].1 % self.test_denominator == 0 {
            (self.monkey_if_test_true, worry_level)
        } else {
            (self.monkey_if_test_false, worry_level)
        }
    }
    fn process_item_part_2(&mut self, worry_level: WorryLevel) -> (usize, WorryLevel) {
        self.items_inspected += 1;

        let mut worry_level = worry_level;
        (self.operation)(&mut worry_level);

        if worry_level.test_divisibilty(self.test_denominator) {
            (self.monkey_if_test_true, worry_level)
        } else {
            (self.monkey_if_test_false, worry_level)
        }
    }
}

struct WorryLevel {
    factors_and_remainders: Vec<(u64, u64)>,
    initial_value: u64,
}
impl WorryLevel {
    fn from(value: u64) -> WorryLevel {
        let mut worry_level = WorryLevel {
            factors_and_remainders: Vec::new(),
            initial_value: value,
        };
        worry_level
    }
    fn set_factors(&mut self, factors: &Vec<u64>) {
        for factor in factors {
            self.factors_and_remainders.push((*factor, self.initial_value % factor));
        }
    }

    fn update_plus(&mut self, rhs: u64) {
        for (factor, remainder) in self.factors_and_remainders.iter_mut() {
            *remainder = (*remainder + rhs) % *factor;
        }
    }
    fn update_multiply(&mut self, rhs: u64) {
        for (factor, remainder) in self.factors_and_remainders.iter_mut() {
            *remainder = (*remainder * rhs) % *factor;
        }
    }
    fn update_square(&mut self) {
        for (factor, remainder) in self.factors_and_remainders.iter_mut() {
            *remainder = (*remainder * *remainder) % *factor;
        }
    }
    fn update_floor_divide(&mut self, rhs: u64) {
        // only used for part 1, so no modulus necessary
        for (factor, remainder) in self.factors_and_remainders.iter_mut() {
            *remainder = *remainder / rhs;
        }
    }

    fn test_divisibilty(&self, rhs: u64) -> bool {
        for (factor, remainder) in &self.factors_and_remainders {
            if *factor == rhs {
                if *remainder == 0 {
                    return true;
                } else {
                    return false;
                }
            }
        }
        panic!()
    }
}
