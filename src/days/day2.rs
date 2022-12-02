use crate::days::aoc::Solution;

use super::{
    aoc::{get_path, read_file_str, Answer},
    Files,
};

pub struct Day2 {}

#[derive(Debug)]
enum HandResult {
    Win,
    Lose,
    Draw,
}

trait Score {
    fn score(&self) -> i32;
}

impl Score for HandResult {
    fn score(&self) -> i32 {
        match self {
            HandResult::Win => 6,
            HandResult::Lose => 0,
            HandResult::Draw => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Score for Hand {
    fn score(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

trait Beats {
    fn beats(&self) -> Self;
    fn loses(&self) -> Self;
}

impl Beats for Hand {
    fn beats(&self) -> Self {
        match *self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    fn loses(&self) -> Self {
        match *self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }
}

impl Hand {
    pub fn parse_round(round: &str) -> (Hand, Hand) {
        let mut split_hand = round.split_whitespace();
        let left_hand = match split_hand.next() {
            Some("A") => Hand::Rock,
            Some("B") => Hand::Paper,
            Some("C") => Hand::Scissors,
            Some(&_) => panic!("Uknown hand type"),
            None => panic!("Unparsable data :("),
        };

        let right_hand = match &split_hand.next() {
            Some("X") => Hand::Rock,
            Some("Y") => Hand::Paper,
            Some("Z") => Hand::Scissors,
            Some(&_) => panic!("Uknown hand type"),
            None => todo!(),
        };

        return (left_hand, right_hand);
    }

    pub fn play(player_hand: &Hand, elf_hand: &Hand) -> (HandResult, i32) {
        let (player_beats, elf_beats) = (&player_hand.beats(), &elf_hand.beats());

        let result = match (player_beats, elf_beats) {
            _ if player_beats == elf_hand => HandResult::Win,
            _ if elf_beats == player_hand => HandResult::Lose,
            _ => HandResult::Draw,
        };

        let score = &result.score() + &player_hand.score();
        return (result, score);
    }

    pub fn parse_round_part2(round: &str) -> (Hand, Hand) {
        let mut split_hand = round.split_whitespace();
        let left_hand = match split_hand.next() {
            Some("A") => Hand::Rock,
            Some("B") => Hand::Paper,
            Some("C") => Hand::Scissors,
            Some(&_) => panic!("Uknown hand type"),
            None => panic!("Unparsable data :("),
        };

        let right_hand = match split_hand.next() {
            Some("X") => left_hand.beats(),
            Some("Y") => left_hand.clone(),
            Some("Z") => left_hand.loses(),
            Some(&_) => panic!("Uknown hand type"),
            None => todo!(),
        };

        return (left_hand, right_hand);
    }
}

impl Day2 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn solve1(data: &String) -> i32 {
        return data
            .lines()
            .map(|round| {
                let hand = Hand::parse_round(round);
                let result = Hand::play(&hand.1, &hand.0);
                return result;
            })
            .map(|result| result.1)
            .into_iter()
            .sum();
    }

    pub fn solve2(data: &String) -> i32 {
        return data
            .lines()
            .map(|round| {
                let hand = Hand::parse_round_part2(round);
                let result = Hand::play(&hand.1, &hand.0);
                return result;
            })
            .map(|result| result.1)
            .into_iter()
            .sum();
    }
}

impl Solution for Day2 {
    fn solve_example1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        let solution = Day2::solve1(&data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_part1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day2::solve1(&data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_example2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        let solution = Day2::solve2(&data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_part2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day2::solve2(&data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn get_day(&self) -> i32 {
        return 2;
    }
}
