use crate::days::aoc::Solution;
use lazy_static::lazy_static;
use regex::Regex;

use super::{
    aoc::{get_path, read_file_str, Answer},
    Files,
};

pub struct Day4 {}

#[derive(Debug)]
struct Assignment {
    first: (i32, i32),
    second: (i32, i32),
}

impl Assignment {
    fn new(first: (i32, i32), second: (i32, i32)) -> Self {
        Self { first, second }
    }

    pub fn from(line: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)").unwrap();
        }

        return match RE.captures(line.trim()) {
            Some(cap) => Assignment::new(
                (
                    cap[1].parse::<i32>().unwrap(),
                    cap[2].parse::<i32>().unwrap(),
                ),
                (
                    cap[3].parse::<i32>().unwrap(),
                    cap[4].parse::<i32>().unwrap(),
                ),
            ),
            None => unreachable!("Can't parse"),
        };
    }

    pub fn fully_overlaps(&self) -> bool {
        let first_overlaps = self.first.0 <= self.second.0 && self.first.1 >= self.second.1;
        let second_overlaps = self.second.0 <= self.first.0 && self.second.1 >= self.first.1;
        first_overlaps || second_overlaps
    }

    pub fn overlap_at_all(&self) -> bool {
        self.first.0 <= self.second.1 && self.second.0 <= self.first.1
    }
}

impl Day4 {
    pub fn new() -> Self {
        Self {}
    }

    fn get_assignments(data: String) -> Vec<Assignment> {
        return data
            .lines()
            .map(Assignment::from)
            .collect::<Vec<Assignment>>();
    }

    pub fn solve1(data: String) -> i32 {
        return Day4::get_assignments(data)
            .iter()
            .filter(|&assignment| assignment.fully_overlaps())
            .count() as i32;
    }

    pub fn solve2(data: String) -> i32 {
        return Day4::get_assignments(data)
            .iter()
            .filter(|&ass| ass.overlap_at_all())
            .count() as i32;
    }
}

impl Default for Day4 {
    fn default() -> Self {
        Self::new()
    }
}

impl Solution for Day4 {
    fn solve_example1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        let solution = Day4::solve1(data).to_string();
        Answer::new(&solution, instant.elapsed())
    }

    fn solve_part1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day4::solve1(data).to_string();
        Answer::new(&solution, instant.elapsed())
    }

    fn solve_example2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        let solution = Day4::solve2(data).to_string();
        Answer::new(&solution, instant.elapsed())
    }

    fn solve_part2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day4::solve2(data).to_string();
        Answer::new(&solution, instant.elapsed())
    }

    fn get_day(&self) -> i32 {
        4
    }
}
