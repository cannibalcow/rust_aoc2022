#[allow(dead_code, unused)]
use std::{collections::HashMap, str::FromStr, string::ParseError};

use crate::days::aoc::Solution;

use super::{
    aoc::{get_path, read_file_str, Answer},
    Files,
};

pub struct Day5 {}

impl Day5 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn solve1(data: String) -> i32 {
        let stacks = Stacks::from_str(&data);
        1
    }

    pub fn solve2(data: String) -> i32 {
        1
    }
}

#[derive(Debug)]
struct Stacks {
    stacks: HashMap<usize, Stack>,
}

impl FromStr for Stacks {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stacks_str = s.split("\n\n").into_iter().next();

        let lines = stacks_str.unwrap().split("\n").collect::<Vec<_>>();

        println!("hest: {:?}", &lines);

        let cols = &lines.last().unwrap().clone();

        let number_of_cols = cols
            .chars()
            .nth(cols.len() - 2)
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap();

        let mut stacks: HashMap<usize, Stack> = HashMap::new();

        for idx in 0..number_of_cols {
            stacks.insert(idx, Stack::new());
        }

        let mut pos = 5;

        for (index, line) in lines.iter().enumerate() {
            if index == lines.len() - 1 {
                println!("Skippiing: {:?}", &line);
                break;
            }
            for idx in 0..number_of_cols {
                stacks.entry(idx).and_modify(|stack| {
                    println!("=====");
                    print!("line: {:?} ", &line);

                    let col_val = match idx {
                        0 => {
                            println!(
                                "Val: {:?}",
                                line.chars().nth(1).unwrap().to_string().to_owned()
                            );

                            line.chars().nth(1).unwrap().to_string().to_owned()
                        }
                        _ => {
                            println!(
                                "Val: {:?}",
                                line.chars().nth(pos).unwrap().to_string().to_owned()
                            );

                            line.chars().nth(pos).unwrap().to_string().to_owned()
                        }
                    };
                    if col_val != " " {
                        stack.crates.push(col_val);
                    }
                });
                println!("col: {:?} pos: {:?}", idx, &pos);

                if idx != 0 {
                    pos = pos + 4;
                }
            }
            pos = 5;
        }

        println!("Data: {:?}", &s);

        println!("Number of columns: {:?}", &number_of_cols);

        println!("{:?}", &stacks);

        return Ok(Stacks {
            stacks: HashMap::new(),
        });
    }
}

#[derive(Debug)]
struct Stack {
    crates: Vec<String>,
}

impl Stack {
    fn new() -> Self {
        Self { crates: Vec::new() }
    }
}

struct Procedure {
    moves: i32,
    from: i32,
    to: i32,
}

impl FromStr for Procedure {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl Solution for Day5 {
    fn solve_example1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));

        let solution = Day5::solve1(data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_part1(&self) -> Answer {
        let instant = self.timer_start();
        //       let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        //        let solution = Day5::solve1(data);
        let solution = 1;
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_example2(&self) -> Answer {
        let instant = self.timer_start();
        //let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        //let solution = Day5::solve2(data);
        let solution = 1;
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_part2(&self) -> Answer {
        let instant = self.timer_start();
        //let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        //let solution = Day5::solve2(data);
        let solution = 1;
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn get_day(&self) -> i32 {
        return 5;
    }
}
