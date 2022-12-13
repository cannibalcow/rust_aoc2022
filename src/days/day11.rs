use std::{collections::VecDeque, str::FromStr};

use crate::days::aoc::{read_file_str, Solution};

use super::aoc::{get_path, Answer, Files};

#[derive(Copy, Clone, Debug)]
enum OperationType {
    Add,
    Multiply,
}

enum OperationValue {
    Old,
    Value(u128),
}

struct Monkey {
    holding: VecDeque<u128>,
    operation_type: OperationType,
    operation_value: OperationValue,
    divisible_by: u128,
    divisible_true: usize,
    divisible_false: usize,
    inspects: u128,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split('\n').collect();
        // Starting items
        let holding = lines
            .get(1)
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .replace(',', "")
            .split_whitespace()
            .map(|s| s.parse::<u128>().unwrap())
            .collect::<VecDeque<u128>>();

        // operation
        let (operation_type_str, operation_value_str) = lines
            .get(2)
            .unwrap()
            .strip_prefix("  Operation: new = old ")
            .unwrap()
            .split_once(' ')
            .unwrap();

        let operation_type: OperationType = match operation_type_str {
            "+" => OperationType::Add,
            "*" => OperationType::Multiply,
            _ => panic!("whaaat type is that!?"),
        };

        let operation_value = match operation_value_str {
            "old" => OperationValue::Old,
            val => OperationValue::Value(val.parse::<u128>().unwrap()),
        };

        // divisable
        let divisible_by = lines
            .get(3)
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse::<u128>()
            .unwrap();

        // monky target true
        let divisible_true = lines
            .get(4)
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let divisible_false = lines
            .get(5)
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Ok(Monkey {
            holding,
            operation_type,
            operation_value,
            divisible_by,
            divisible_true,
            divisible_false,
            inspects: 0,
        })
    }
}

fn monkies_from_str(s: &str) -> Vec<Monkey> {
    return s
        .split("\n\n")
        .map(|f| Monkey::from_str(f).unwrap())
        .collect();
}

fn process_rounds(monkeys: &mut Vec<Monkey>, baseline: u128, skip_divide: bool) {
    for monkey_index in 0..monkeys.len() {
        while !monkeys[monkey_index].holding.is_empty() {
            let monkey = &mut monkeys[monkey_index];
            let item = monkey.holding.pop_front();

            if let Some(item) = item {
                let operatation_value = match &monkey.operation_value {
                    OperationValue::Old => item,
                    OperationValue::Value(val) => *val,
                };

                let new_value_after_inspection = match monkey.operation_type {
                    OperationType::Add => (item + operatation_value) % baseline,
                    OperationType::Multiply => (item * operatation_value) % baseline,
                };

                let new_value_after_relief = if skip_divide {
                    &new_value_after_inspection / 3
                } else {
                    new_value_after_inspection
                };

                let throw_target_test = new_value_after_relief % monkey.divisible_by == 0;
                let monkey_throw_true = monkey.divisible_true;
                let monkey_throw_false = monkey.divisible_false;

                monkey.inspects += 1;

                match throw_target_test {
                    true => monkeys[monkey_throw_true]
                        .holding
                        .push_back(new_value_after_relief),
                    false => monkeys[monkey_throw_false]
                        .holding
                        .push_back(new_value_after_relief),
                };
            }
        }
    }
}

pub struct Day11 {}
impl Day11 {
    pub fn new() -> Self {
        Self {}
    }

    fn solve1(data: String) -> String {
        let mut monkies = monkies_from_str(&data);

        let baseline = monkies
            .iter()
            .map(|monkey| monkey.divisible_by)
            .product::<u128>();

        for _ in 0..20 {
            process_rounds(&mut monkies, baseline, true);
        }

        let mut top_monkies = monkies.iter().map(|m| m.inspects).collect::<Vec<u128>>();

        top_monkies.sort();
        top_monkies.reverse();

        let solution: u128 = top_monkies[0] * top_monkies[1];

        solution.to_string()
    }

    fn solve2(data: String) -> String {
        let mut monkies = monkies_from_str(&data);

        let baseline = monkies
            .iter()
            .map(|monkey| monkey.divisible_by)
            .product::<u128>();

        for _ in 0..10_000 {
            process_rounds(&mut monkies, baseline, false);
        }

        let mut top_monkies = monkies.iter().map(|m| m.inspects).collect::<Vec<u128>>();

        top_monkies.sort();
        top_monkies.reverse();

        let solution: u128 = top_monkies[0] * top_monkies[1];

        solution.to_string()
    }
}
impl Default for Day11 {
    fn default() -> Self {
        Self::new()
    }
}

impl Solution for Day11 {
    fn solve_example1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        let solution = Day11::solve1(data);
        Answer::new(&solution, instant.elapsed())
    }

    fn solve_part1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day11::solve1(data);
        Answer::new(&solution, instant.elapsed())
    }

    fn solve_example2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        let solution = Day11::solve2(data);
        Answer::new(&solution, instant.elapsed())
    }

    fn solve_part2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day11::solve2(data);

        Answer::new(&solution, instant.elapsed())
    }

    fn get_day(&self) -> i32 {
        11
    }
}
