use std::collections::VecDeque;
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

    pub fn solve1(data: String) -> String {
        let mut stacks = Stacks::from_str(&data).unwrap();
        let procedurs = Procedures::from_str(&data).unwrap();

        for procedure in procedurs.procedures {
            stacks.execute_part1(procedure)
        }

        let mut solution = String::new();

        for s in stacks.get_sorted_keys() {
            let len = &stacks.stacks.get(&s).unwrap().len();
            let s = &stacks.stacks.get(&s).unwrap().get(len - 1).unwrap();
            solution.push_str(s);
        }

        return solution;
    }

    pub fn solve2(data: String) -> String {
        let mut stacks = Stacks::from_str(&data).unwrap();
        let procedurs = Procedures::from_str(&data).unwrap();

        for procedure in procedurs.procedures {
            stacks.execute_part2(procedure)
        }

        let mut solution = String::new();

        for s in stacks.get_sorted_keys() {
            let len = &stacks.stacks.get(&s).unwrap().len();
            let s = &stacks.stacks.get(&s).unwrap().get(len - 1).unwrap();
            solution.push_str(s);
        }

        return solution;
    }
}

#[derive(Debug)]
struct Stacks {
    stacks: HashMap<usize, VecDeque<String>>,
}

#[allow(dead_code)]
impl Stacks {
    fn get_sorted_keys(&self) -> Vec<usize> {
        let _keys = self.stacks.keys();
        let mut keys: Vec<usize> = Vec::new();

        for key in _keys {
            keys.push(key.clone());
        }
        keys.sort();
        return keys;
    }

    fn pretty_print(&self) {
        for key in self.get_sorted_keys() {
            print!("{key}: ");
            print!("{:?}", self.stacks.get(&key).unwrap());
            println!("");
        }
        println!("");
    }
}

trait ExecuteProcedure {
    fn execute_part1(&mut self, procedure: Procedure);
    fn execute_part2(&mut self, procedure: Procedure);
}

impl ExecuteProcedure for Stacks {
    fn execute_part1(&mut self, procedure: Procedure) {
        let from_item = self.stacks.get_mut(&procedure.from).unwrap();
        let drain_length = from_item.len() - procedure.moves;
        let drained_items = from_item.drain(&drain_length..).collect::<VecDeque<_>>();

        self.stacks.entry(procedure.to).and_modify(|target_stack| {
            for value in drained_items.iter().rev() {
                target_stack.push_back(value.to_owned())
            }
        });
    }

    fn execute_part2(&mut self, procedure: Procedure) {
        let from_item = self.stacks.get_mut(&procedure.from).unwrap();
        let drain_length = from_item.len() - procedure.moves;
        let mut drained_items = from_item.drain(&drain_length..).collect::<VecDeque<_>>();

        self.stacks.entry(procedure.to).and_modify(|target_stack| {
            target_stack.append(&mut drained_items);
        });
    }
}

#[derive(Debug)]
struct Procedures {
    procedures: Vec<Procedure>,
}

#[derive(Debug)]
struct Procedure {
    moves: usize,
    from: usize,
    to: usize,
}

#[allow(dead_code)]
impl Procedure {
    fn pretty_print(&self) {
        println!(
            "Move {:?} from {:?} to {:?}",
            &self.moves, &self.from, &self.to
        );
    }
}

impl FromStr for Procedures {
    type Err = ParseError;
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let proc_str = data.split("\n\n").into_iter().nth(1).unwrap();

        let procs = proc_str
            .lines()
            .map(|f| {
                let mut numbers = f
                    .split_ascii_whitespace()
                    .filter_map(|token| token.parse().ok());

                return Procedure {
                    moves: numbers.next().unwrap(),
                    from: numbers.next().unwrap(),
                    to: numbers.next().unwrap(),
                };
            })
            .collect();

        Ok(Procedures { procedures: procs })
    }
}

impl FromStr for Stacks {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stacks_str = s.split("\n\n").into_iter().next();

        let lines = stacks_str.unwrap().split("\n").collect::<Vec<_>>();

        let cols = &lines.last().unwrap().clone();

        let number_of_cols = cols
            .chars()
            .nth(cols.len() - 2)
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap();

        let mut stacks: HashMap<usize, VecDeque<String>> = HashMap::new();

        for idx in 0..number_of_cols {
            stacks.insert(idx + 1, VecDeque::new());
        }

        let mut pos = 5;

        for (index, line) in lines.iter().enumerate() {
            if index == lines.len() - 1 {
                break;
            }

            for idx in 0..number_of_cols {
                stacks.entry(idx + 1).and_modify(|stack| {
                    let col_val = match idx {
                        0 => line.chars().nth(1).unwrap().to_string().to_owned(),
                        _ => line.chars().nth(pos).unwrap().to_string().to_owned(),
                    };
                    if col_val != " " {
                        stack.push_front(col_val);
                    }
                });

                if idx != 0 {
                    pos = pos + 4;
                }
            }
            pos = 5;
        }

        return Ok(Stacks { stacks });
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
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day5::solve1(data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_example2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        let solution = Day5::solve2(data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_part2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day5::solve2(data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn get_day(&self) -> i32 {
        return 5;
    }
}
