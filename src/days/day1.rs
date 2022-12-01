use crate::days::aoc::{read_file_str, Solution};

use super::aoc::{get_path, Answer, Files};

pub struct Day1 {}

impl Day1 {
    pub fn new() -> Self {
        Self {}
    }

    fn get_input_as_groups(data: &String) -> Vec<Vec<i32>> {
        let output: Vec<Vec<i32>> = data
            .split("\n\n")
            .map(|group| {
                group
                    .lines()
                    .map(|line| line.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();
        return output.to_owned();
    }

    fn get_sum_of_top_three(values: &mut Vec<i32>) -> i32 {
        values.sort();
        values.reverse();

        let solution: i32 = values[0..3].iter().sum();
        return solution;
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

        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_part1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let output = Day1::get_input_as_groups(&data);

        let values: Vec<i32> = output.iter().map(|f| f.iter().sum()).collect();

        let solution = values.iter().max().expect("Failed to find max");

        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_example2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        let output = Day1::get_input_as_groups(&data);

        let mut values: Vec<i32> = output.iter().map(|f| f.iter().sum()).collect();
        let solution = Day1::get_sum_of_top_three(&mut values);

        assert_eq!(&solution, &45000);

        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_part2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let output = Day1::get_input_as_groups(&data);

        let mut values: Vec<i32> = output.iter().map(|f| f.iter().sum()).collect();
        let solution = Day1::get_sum_of_top_three(&mut values);

        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn get_day(&self) -> i32 {
        return 1;
    }
}
