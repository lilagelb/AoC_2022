use std::collections::HashSet;
use std::ops::{AddAssign, Sub};
use regex::Regex;
use crate::day::{Answer, Day};

pub struct Day9;
impl Day for Day9 {
    type TypePart1 = usize;
    type TypePart2 = usize;

    fn run(&mut self) -> Answer<Self::TypePart1, Self::TypePart2> {
        let input = self.get_input_for_day_by_line(9);

        let direction_parse_re = Regex::new(r"^(?P<direction>\w) (?P<distance>\d+)").unwrap();

        let mut rope_part_1 = Rope::new(2);
        let mut rope_part_2 = Rope::new(10);

        let mut places_visited_part_1 = HashSet::new();
        let mut places_visited_part_2 = HashSet::new();

        for line in input {
            let captures = direction_parse_re.captures(&line).unwrap();
            let direction = captures.name("direction").unwrap().as_str();
            let distance: i32 = captures.name("distance").unwrap().as_str().parse().unwrap();

            for _ in 0..distance {
                rope_part_1.move_head(direction);
                places_visited_part_1.insert(rope_part_1.knots[1]);
                rope_part_2.move_head(direction);
                places_visited_part_2.insert(rope_part_2.knots[9]);
            }
        }

        let part_1 = places_visited_part_1.len();
        let part_2 = places_visited_part_2.len();

        Answer::new(Some(part_1), Some(part_2))
    }
}
impl Day9 {
    pub fn new() -> Day9 {
        Day9
    }
}

struct Rope {
    knots: Vec<Vec2>,
    num_knots: usize,
}
impl Rope {
    fn new(num_knots: usize) -> Rope {
        let mut knots = Vec::new();
        knots.resize(num_knots, Vec2::new([0, 0]));

        Rope { knots, num_knots }
    }

    fn move_head(&mut self, direction: &str) {
        match direction {
            "U" => self.knots[0] += Vec2::new([0, 1]),
            "D" => self.knots[0] += Vec2::new([0, -1]),
            "R" => self.knots[0] += Vec2::new([1, 0]),
            "L" => self.knots[0] += Vec2::new([-1, 0]),
            _ => {},
        }
        for knot in 1..self.num_knots {
            self.update_knot(knot);
        }
    }

    fn update_knot(&mut self, knot_number: usize) {
        let difference = self.knots[knot_number - 1] - self.knots[knot_number];

        if difference.x.abs() <= 1 && difference.y.abs() <= 1 {
            // the knot is next to the the one ahead of it
            return;
        }
        // the knot should move in the direction of the one ahead of it by one space in x and/or y
        self.knots[knot_number] += difference.to_ones();

    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
}
impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Vec2 {
    fn new(vector: [i32; 2]) -> Vec2 {
        Vec2 { x: vector[0], y: vector[1] }
    }

    fn to_ones(&self) -> Vec2 {
        Vec2 {
            x: self.x / if self.x != 0 { self.x.abs() } else { 1 },
            y: self.y / if self.y != 0 { self.y.abs() } else { 1 },
        }
    }
}