use std::collections::{HashMap, HashSet};
use std::ops::Add;
use regex::Regex;
use crate::day::{Answer, Day};

pub struct Day15;
impl Day for Day15 {
    type TypePart1 = i64;
    type TypePart2 = i64;

    fn run(&mut self) -> Answer<Self::TypePart1, Self::TypePart2> {
        let input = self.get_input_for_day_by_line(15);
        let row_to_check = 2000000;

        let sensor_parse_re = Regex::new(r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)").unwrap();

        let mut sensors = Vec::new();
        let mut beacons_on_row_to_check = HashSet::new();
        for line in input {
            let captures = sensor_parse_re.captures(&line).unwrap();
            let position = Position::new(
                captures.name("sensor_x").unwrap().as_str().parse().unwrap(),
                captures.name("sensor_y").unwrap().as_str().parse().unwrap(),
            );
            let beacon = Position::new(
                captures.name("beacon_x").unwrap().as_str().parse().unwrap(),
                captures.name("beacon_y").unwrap().as_str().parse().unwrap(),
            );
            sensors.push(Sensor::new(position, beacon));
            if beacon.y == row_to_check {
                beacons_on_row_to_check.insert(beacon);
            }
        }

        let mut combined_range = (i64::MAX, i64::MIN);
        let mut rejected_points: HashSet<Position> = HashSet::new();
        let mut part_2 = 0i64;
        for sensor in &sensors {
            if let Some(sensor_range) = sensor.get_range_at_y(row_to_check) {
                if combined_range.0 > sensor_range.0 {
                    combined_range.0 = sensor_range.0;
                }
                if combined_range.1 < sensor_range.1 {
                    combined_range.1 = sensor_range.1;
                }
            }
            'next_point: for point in sensor.get_outskirt_in_range(Position::new(0, 0), Position::new(4000000, 4000000)) {
                if rejected_points.contains(&point) {
                    continue;
                }
                for sensor in &sensors {
                    if point.distance_to(&sensor.position) <= sensor.nearest_beacon_distance as u64 {
                        rejected_points.insert(point);
                        continue 'next_point;
                    }
                }
                part_2 = 4000000 * point.x + point.y;
                break;
            }
        }

        let part_1 = combined_range.1 - combined_range.0
            + 1
            - beacons_on_row_to_check.len() as i64;

        Answer::new(Some(part_1), Some(part_2))
    }
}
impl Day15 {
    pub fn new() -> Day15 {
        Day15
    }
}

struct Sensor {
    position: Position,
    nearest_beacon_distance: i64,
}
impl Sensor {
    fn new(position: Position, nearest_beacon: Position) -> Sensor {
        Sensor {
            position,
            nearest_beacon_distance: position.distance_to(&nearest_beacon) as i64,
        }
    }

    fn get_range_at_y(&self, y: i64) -> Option<(i64, i64)> {
        if y.abs_diff(self.position.y) > self.nearest_beacon_distance as u64 {
            return None;
        }
        let half_range = self.nearest_beacon_distance - y.abs_diff(self.position.y) as i64;
        Some((self.position.x - half_range, self.position.x + half_range))
    }

    fn get_outskirt_in_range(&self, start: Position, end: Position) -> Vec<Position> {
        let mut points = Vec::new();
        for i in 0..self.nearest_beacon_distance {
            let potential_points = vec![
                self.position + Position::new(i, self.nearest_beacon_distance + 1 - i),
                self.position + Position::new(-i, i - self.nearest_beacon_distance - 1),
                self.position + Position::new(self.nearest_beacon_distance + 1 - i, i),
                self.position + Position::new(i - self.nearest_beacon_distance - 1, -i),
            ];
            for point in potential_points {
                if start.x <= point.x && point.x <= end.x
                    && start.y <= point.y && point.y <= end.y
                {
                    points.push(point);
                }
            }
        }
        points
    }
}


#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}
impl Position {
    fn new(x: i64, y: i64) -> Position {
        Position {
            x, y
        }
    }

    fn distance_to(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}
impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}