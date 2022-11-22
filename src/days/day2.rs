use crate::days::aoc::Solution;

use super::aoc::Answer;

pub struct Day2 {}

impl Day2 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Day2 {
    fn solve_example1(&self) -> Answer {
        let instant = self.timer_start();
        return Answer::new("answer example1", instant.elapsed());
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
        return 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
