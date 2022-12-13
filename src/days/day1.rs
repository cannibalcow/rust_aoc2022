use crate::days::aoc::{read_file_str, Solution};

use super::aoc::{get_path, Answer, Files};

pub struct Day1 {}

impl Day1 {
    pub fn new() -> Self {
        Self {}
    }

    fn get_input_as_groups(data: &str) -> Vec<Vec<i32>> {
        let output: Vec<Vec<i32>> = data
            .split("\n\n")
            .map(|group| {
                group
                    .lines()
                    .map(|line| line.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();
        output
    }

    fn get_sum_of_top_three(values: &mut [i32]) -> i32 {
        values.sort();
        values.reverse();
        values[0..3].iter().sum()
    }
}

impl Default for Day1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Solution for Day1 {
    fn solve_example1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        let output = Day1::get_input_as_groups(&data);

        let values: Vec<i32> = output.iter().map(|f| f.iter().sum()).collect();

        let solution = values.iter().max().expect("Failed to find max");

        assert_eq!(solution, &24000);

        Answer::new(&solution.to_string(), instant.elapsed())
    }

    fn solve_part1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let output = Day1::get_input_as_groups(&data);

        let values: Vec<i32> = output.iter().map(|f| f.iter().sum()).collect();

        let solution = values.iter().max().expect("Failed to find max");

        Answer::new(&solution.to_string(), instant.elapsed())
    }

    fn solve_example2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        let output = Day1::get_input_as_groups(&data);

        let mut values: Vec<i32> = output.iter().map(|f| f.iter().sum()).collect();
        let solution = Day1::get_sum_of_top_three(&mut values);

        assert_eq!(&solution, &45000);

        Answer::new(&solution.to_string(), instant.elapsed())
    }

    fn solve_part2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let output = Day1::get_input_as_groups(&data);

        let mut values: Vec<i32> = output.iter().map(|f| f.iter().sum()).collect();
        let solution = Day1::get_sum_of_top_three(&mut values);

        Answer::new(&solution.to_string(), instant.elapsed())
    }

    fn get_day(&self) -> i32 {
        1
    }
}
