#[allow(dead_code, unused)]
use std::{collections::HashMap, str::FromStr, string::ParseError};

use crate::days::aoc::Solution;

use super::{
    aoc::{get_path, read_file_str, Answer},
    Files,
};

pub struct Day6 {}

impl Day6 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn find_marker(window_size: usize, data: &str) -> usize {
        let result = data
            .as_bytes()
            .windows(window_size)
            .position(|slice| {
                slice
                    .iter()
                    .enumerate()
                    .all(|(i, x)| !slice[i + 1..].contains(x))
            })
            .unwrap()
            + window_size;
        return result;
    }
    pub fn solve1(data: String) -> String {
        let result = Day6::find_marker(4, &data);
        return result.to_string();
    }
    pub fn solve2(data: String) -> String {
        let result = Day6::find_marker(14, &data);
        return result.to_string();
    }
}

impl Solution for Day6 {
    fn solve_example1(&self) -> Answer {
        let instant = self.timer_start();
        let data = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        let solution = Day6::solve1(data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_part1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day6::solve1(data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_example2(&self) -> Answer {
        let instant = self.timer_start();
        let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        let solution = Day6::solve2(data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_part2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day6::solve2(data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn get_day(&self) -> i32 {
        return 6;
    }
}
