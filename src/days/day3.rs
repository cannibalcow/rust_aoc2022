use crate::days::aoc::Solution;

use super::{
    aoc::{get_path, read_file_str, Answer},
    Files,
};

pub struct Day3 {}

impl Day3 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn solve1(data: String) -> u16 {
        data.split('\n')
            .filter(|&x| !x.is_empty())
            .into_iter()
            .map(|line| line.split_at(line.len() / 2))
            .map(|(left, right)| {
                right
                    .chars()
                    .filter(|chr| left.contains(&chr.to_string()))
                    .map(|v| {
                        if v as u8 >= b'a' {
                            ((v as u16 - b'a' as u16) + 1) as u16
                        } else {
                            ((v as u16 - b'A' as u16) + 27) as u16
                        }
                    })
                    .next()
                    .unwrap() // WHY  THE FUCK !!!
            })
            .sum::<u16>()
    }

    pub fn solve2(data: String) -> u16 {
        data.split('\n')
            .filter(|&x| !x.is_empty())
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|chunk| {
                chunk[0]
                    .chars()
                    .find(|chr| {
                        chunk[1].contains(&chr.to_string()) && chunk[2].contains(&chr.to_string())
                    })
                    .unwrap()
            })
            .map(|v| {
                if v as u8 >= b'a' {
                    ((v as u16 - b'a' as u16) + 1) as u16
                } else {
                    ((v as u16 - b'A' as u16) + 27) as u16
                }
            })
            .sum()
    }
}

impl Default for Day3 {
    fn default() -> Self {
        Self::new()
    }
}

impl Solution for Day3 {
    fn solve_example1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        let solution = Day3::solve1(data);
        Answer::new(&solution.to_string(), instant.elapsed())
    }

    fn solve_part1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day3::solve1(data);
        Answer::new(&solution.to_string(), instant.elapsed())
    }

    fn solve_example2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        let solution = Day3::solve2(data);
        Answer::new(&solution.to_string(), instant.elapsed())
    }

    fn solve_part2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day3::solve2(data);
        Answer::new(&solution.to_string(), instant.elapsed())
    }

    fn get_day(&self) -> i32 {
        3
    }
}
