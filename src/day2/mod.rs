use std::fs;
use crate::day::{Answer, Day};
use crate::day2::RockPaperScissors::{Rock, Paper, Scissors};

pub struct Day2;
impl Day for Day2 {
    type TypePart1 = i32;
    type TypePart2 = i32;

    fn run(&self) -> Answer<Self::TypePart1, Self::TypePart2> {
        let input = self.get_input_for_day_by_line(2);
        let input = input.iter()
            .map(|elem| elem.split(" ").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        let games_1 = input.iter().map(|game| {
                Game {
                    opponent: RockPaperScissors::from(&game[0]),
                    me: RockPaperScissors::from(&game[1])
                }
            })
            .collect::<Vec<Game>>();

        let part_1_score = games_1.iter()
            .map(|game| game.score())
            .sum();

        let games_2 = input.iter().map(|game| {
                let opponent = RockPaperScissors::from(game[0]);
                let me = match game[1] {
                    "X" => match opponent { // lose
                        Rock => Scissors,
                        Scissors => Paper,
                        Paper => Rock,
                    },
                    "Y" => opponent,    // draw
                    "Z" => match opponent {
                        Rock => Paper,
                        Paper => Scissors,
                        Scissors => Rock,
                    },
                    other => panic!("invalid win specifier `{}`", other),
                };
                Game {
                    opponent, me
                }
            })
            .collect::<Vec<Game>>();

        let part_2_score = games_2.iter()
            .map(|game| game.score())
            .sum();

        Answer::new(Some(part_1_score), Some(part_2_score))
    }
}
impl Day2 {
    pub fn new() -> Day2 {
        Day2
    }
}

#[derive(Copy, Clone, PartialEq)]
enum RockPaperScissors {
    Rock, Paper, Scissors
}
impl RockPaperScissors {
    fn from(play: &str) -> RockPaperScissors {
        match play {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("invalid play"),
        }
    }
}

struct Game {
    opponent: RockPaperScissors,
    me: RockPaperScissors,
}
impl Game {
    fn score(&self) -> i32 {
        let mut score = match self.me {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };
        if self.me == self.opponent {
            score += 3;
        }
        else if self.me == Rock && self.opponent == Scissors
            || self.me == Scissors && self.opponent == Paper
            || self.me == Paper && self.opponent == Rock
        {
            score += 6;
        }
        score
    }
}
