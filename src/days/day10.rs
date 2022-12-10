use std::{collections::HashMap, str::FromStr};

use crate::days::aoc::{read_file_str, Solution};

use super::aoc::{get_path, Answer, Files};

pub struct Day10 {}

#[derive(Debug, Copy, Clone)]
enum InstructionType {
    Noop = 1,
    Addx = 2,
}

struct Instruction {
    instruction_type: InstructionType,
    arg: i32,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(" ") {
            let parts = s.split_once(" ").unwrap();
            return Ok(Instruction::addx(parts.1.parse::<i32>().unwrap()));
        } else {
            return Ok(Instruction::noop());
        }
    }
}

impl Instruction {
    fn noop() -> Self {
        Self {
            instruction_type: InstructionType::Noop,
            arg: 0,
        }
    }

    fn addx(v: i32) -> Self {
        Self {
            instruction_type: InstructionType::Addx,
            arg: v,
        }
    }
}

struct InstructionIterator {
    instruction: Instruction,
    cycle: i32,
}

impl Iterator for InstructionIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cycle < self.instruction.instruction_type as i32 {
            self.cycle = self.cycle + 1;
            return Some(self.cycle);
        }
        None
    }
}

impl IntoIterator for Instruction {
    type Item = i32;
    type IntoIter = InstructionIterator;

    fn into_iter(self) -> Self::IntoIter {
        InstructionIterator {
            instruction: self,
            cycle: 0,
        }
    }
}

struct CPU {
    x: i32,
}

impl CPU {
    fn new() -> Self {
        Self { x: 1 }
    }

    fn run_part1(&mut self, program: Vec<Instruction>) -> Vec<i32> {
        let mut cycle = 0;
        let mut cycle_vals = vec![];

        for instruction in program {
            let value = &instruction.arg.clone();

            for _ in instruction {
                cycle += 1;
                match cycle {
                    20 => cycle_vals.push(self.x * 20),
                    60 => cycle_vals.push(self.x * 60),
                    100 => cycle_vals.push(self.x * 100),
                    140 => cycle_vals.push(self.x * 140),
                    180 => cycle_vals.push(self.x * 180),
                    220 => cycle_vals.push(self.x * 220),
                    _ => (),
                }
            }

            self.x = self.x + value;
        }
        return cycle_vals;
    }
}

struct CRT {
    screen: Vec<Vec<String>>,
}

impl CRT {
    fn new() -> Self {
        let black_square = emojis::get_by_shortcode("christmas_tree").unwrap();
        Self {
            screen: vec![vec![black_square.to_string(); 40]; 6],
        }
    }

    fn run_program(program: Vec<Instruction>) -> HashMap<i32, i32> {
        let mut cycle = 0;
        let mut x = 1;
        let mut xs: HashMap<i32, i32> = HashMap::new();

        for instruction in program {
            let value = instruction.arg.clone();
            for _ in instruction {
                cycle += 1;
                xs.insert(cycle, x);
            }
            x = x + value;
        }

        return xs;
    }

    fn build_image(&mut self, program: Vec<Instruction>) {
        let program_ouput = CRT::run_program(program);
        let santa = emojis::get_by_shortcode("santa").unwrap();

        for cycle in program_ouput {
            let x = cycle.1;
            if [x - 1, x, x + 1].contains(&((cycle.0 - 1) % 40)) {
                self.screen[((cycle.0 - 1) / 40) as usize][((cycle.0 - 1) % 40) as usize] =
                    santa.to_string();
            }
        }
    }

    fn render(&self) {
        for row in &self.screen {
            for line in row {
                print!("{}", &line);
            }
            println!("");
        }
    }
}

impl Day10 {
    pub fn new() -> Self {
        Self {}
    }

    fn get_program(data: String) -> Vec<Instruction> {
        return data
            .lines()
            .map(|f| f.trim())
            .map(|string| Instruction::from_str(string).unwrap())
            .collect();
    }

    fn solve1(data: String) -> String {
        let mut cpu = CPU::new();
        let program = Day10::get_program(data);

        return cpu.run_part1(program).iter().sum::<i32>().to_string();
    }

    fn solve2(data: String) -> String {
        let mut crt = CRT::new();

        let program = Day10::get_program(data);

        crt.build_image(program);
        crt.render();

        return String::from("");
    }
}

impl Solution for Day10 {
    fn solve_example1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example2, self.get_day()));
        let solution = Day10::solve1(data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_part1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day10::solve1(data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_example2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example2, self.get_day()));
        let solution = Day10::solve2(data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_part2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day10::solve2(data);

        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn get_day(&self) -> i32 {
        return 10;
    }
}
