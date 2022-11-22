use crate::days::aoc::Solution;

use super::aoc::{get_path, read_file_i32, Answer, Files};

pub struct Day1 {}

impl Day1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Day1 {
    fn solve_example1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_i32(&get_path(Files::Example1, self.get_day()));

        let result = &data.into_iter().reduce(|a, b| a + b).unwrap();

        return Answer::new(&result.to_string(), instant.elapsed());
    }

    fn solve_part1(&self) -> Answer {
        let instant = self.timer_start();
        return Answer::new("answer part1", instant.elapsed());
    }

    fn solve_example2(&self) -> Answer {
        let instant = self.timer_start();
        return Answer::new("answer example2", instant.elapsed());
    }

    fn solve_part2(&self) -> Answer {
        let instant = self.timer_start();
        return Answer::new("answer part2", instant.elapsed());
    }

    fn get_day(&self) -> i32 {
        return 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
