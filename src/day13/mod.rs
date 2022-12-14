use std::cmp::Ordering;
use std::fs;
use crate::day::{Answer, Day};

pub struct Day13;
impl Day for Day13 {
    type TypePart1 = usize;
    type TypePart2 = usize;

    fn run(&mut self) -> Answer<Self::TypePart1, Self::TypePart2> {
        let input: Vec<Vec<Packet>> = fs::read_to_string("src/day13/input.txt").unwrap()
            .trim()
            .split("\n\n")
            .map(|pair| {
                pair.split("\n")
                    .map(|packet| self.parse_packet(packet.as_bytes(), &mut 0))
                    .collect()
            })
            .collect();

        let mut part_1 = 0usize;
        let mut part_2_packets = Vec::new();
        for (index, pair) in input.iter().enumerate() {
            if pair[0] < pair[1] {
                part_1 += index + 1;
            }
            part_2_packets.push(&pair[0]);
            part_2_packets.push(&pair[1]);
        }
        let divider_packet_1 = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
        let divider_packet_2 = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);
        part_2_packets.push(&divider_packet_1);
        part_2_packets.push(&divider_packet_2);

        part_2_packets.sort();
        let mut part_2 = 1;
        for (index, packet) in part_2_packets.iter().enumerate() {
            if packet == &&divider_packet_1 {
                part_2 *= index + 1;
            } else if packet == &&divider_packet_2 {
                part_2 *= index + 1;
            }
        }

        Answer::new(Some(part_1), Some(part_2))
    }
}
impl Day13 {
    pub fn new() -> Day13 {
        Day13
    }

    fn parse_packet(&self, packet: &[u8], index: &mut usize) -> Packet {
        let mut packet_vec = Vec::new();
        *index += 1; // consume opening bracket
        loop {
            match packet[*index] as char {
                '[' => {
                    packet_vec.push(self.parse_packet(packet, index));
                },
                ']' => {
                    *index += 1;
                    return Packet::List(packet_vec)
                },
                ',' => {
                    *index += 1;
                },
                number => {
                    *index += 1;
                    let mut complete_number = String::from(number);
                    while packet[*index].is_ascii_digit() {
                        complete_number += &packet[*index].to_string();
                        *index += 1;
                    }
                    packet_vec.push(Packet::Integer(complete_number.parse().unwrap()));
                }
            }
        }
    }
}

#[derive(Eq, PartialEq, Ord)]
enum Packet {
    Integer(i32),
    List(Vec<Packet>),
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut self_int_value = -1;
        let mut other_int_value = -1;
        if let Packet::Integer(value) = self {
            self_int_value = *value;
        }
        if let Packet::Integer(value) = other {
            other_int_value = *value;
        }
        if self_int_value == -1 && other_int_value != -1 {
            // one int, one not cases
            return self.partial_cmp(&Packet::List(vec![Packet::Integer(other_int_value)]));
        }  else if self_int_value != -1 && other_int_value == -1 {
            // one int, one not cases
            return Packet::List(vec![Packet::Integer(self_int_value)]).partial_cmp(&other);
        } else if self_int_value != -1 && other_int_value != -1 {
            // two ints
            return self_int_value.partial_cmp(&other_int_value);
        } else {
            // two lists
            if let Packet::List(self_list) = self {
            if let Packet::List(other_list) = other {

            for i in 0..self_list.len() {
                let other_value = match other_list.get(i) {
                    Some(value) => value,
                    None => return Some(Ordering::Greater),
                };
                match self_list[i].partial_cmp(&other_value) {
                    Some(Ordering::Equal) => {},
                    other_ordering => return other_ordering,
                }
            }
            if self_list.len() < other_list.len() {
                return Some(Ordering::Less)
            } else {
                return Some(Ordering::Equal)
            }

            }}
        }
        panic!()
    }
}
