use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

use colored::Colorize;

use crate::days::aoc::{read_file_str, slow_down, step, Solution};

use super::aoc::{get_path, Answer, Files};

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn next_pos(&self, direction: Direction) -> Option<Pos> {
        match direction {
            Direction::Up => Some(Pos::new(self.x, self.y + 1)),
            Direction::Down => {
                if self.y == 0 {
                    None
                } else {
                    Some(Pos::new(self.x, self.y - 1))
                }
            }
            Direction::Left => {
                if self.x == 0 {
                    None
                } else {
                    Some(Pos::new(self.x - 1, self.y))
                }
            }
            Direction::Right => Some(Pos::new(self.x + 1, self.y)),
        }
    }
}

#[allow(unused)]
impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y);
        Ok(())
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct HeightMap {
    start: Pos,
    end: Pos,
    steps: i32,
    data: HashMap<Pos, char>,
    visited: HashSet<Pos>,
    path: HashSet<Pos>,
    current: Pos,
    prev: Pos,
}

impl HeightMap {
    fn get_pos_value(&self, pos: &Pos) -> Option<i32> {
        self.data.get(pos).map(|chr| *chr as i32)
    }

    fn get_available_paths(&self, check_pos: &Pos) -> Option<Pos> {
        let all_directions = vec![
            check_pos.next_pos(Direction::Up),
            check_pos.next_pos(Direction::Down),
            check_pos.next_pos(Direction::Left),
            check_pos.next_pos(Direction::Right),
        ];

        let current_value: i32 = if check_pos.eq(&self.start) {
            96i32 // start_value
        } else {
            self.get_pos_value(&check_pos).unwrap()
        };

        println!("I'm at {}", &check_pos);
        println!("{:?}", &all_directions);
        //println!("{:?}", &self.visited);

        let mut result: Vec<(Pos, i32)> = all_directions
            .iter()
            .filter(|item| item.is_some())
            .map(|item| item.unwrap())
            .filter(|pos| !self.visited.contains(pos))
            .inspect(|f| println!("{:?}", &f))
            .filter(|pos| self.get_pos_value(pos).is_some()) //
            .map(|pos| (pos, self.get_pos_value(&pos).unwrap()))
            .filter(|(_, value)| value == &(current_value + 1) || value == &(current_value))
            .collect();

        result.reverse();

        result.sort_by_key(|f| f.0.x);

        // Take route with max value
        let chosen = result.iter().max_by_key(|f| f.1.abs()).map(|f| f.0);

        chosen

        //result
    }

    fn find(&mut self) {
        println!("Begin: {}", self.start);
        while self.current != self.end {
            println!("{}", &self);
            // Get neighbours distances.
            let next_pos: Option<Pos> = self.get_available_paths(&self.current);

            if next_pos.is_none() {
                println!("Cur: {}", &self.current);
                println!("End: {}", &self.end);
                println!("Endv: {}", self.get_pos_value(&self.end).unwrap());
                //step();
                // BROKEN!! Måste läsa på om path finding
                if self.current != self.end {
                    self.visited.insert(self.current);
                    self.current = *self.path.iter().nth(self.path.len() - 1).unwrap();
                    continue;
                }

                println!("DONE!");
                return;
            }

            slow_down(50);

            // Mark current as visited
            if self.current != self.start {
                self.visited.insert(self.current);
            }

            // Gå till next pos.
            self.prev = self.current;
            self.current = next_pos.unwrap();

            // Registerar path
            self.path.insert(self.current);

            self.steps += 1;
        }
    }
}

impl Display for HeightMap {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = vec![vec![".".normal(); 161]; 41];
        //let mut v = vec![vec![".".normal(); 9]; 5];

        for (pos, chr) in &self.data {
            let s = chr.to_string();
            v[pos.y][pos.x] = s.normal();
        }

        for pos in &self.path {
            let nc = v[pos.y][pos.x].clone();
            v[pos.y][pos.x] = nc.green();
        }

        for pos in &self.visited {
            let nc = v[pos.y][pos.x].clone();
            v[pos.y][pos.x] = nc.red();
        }

        println!();
        for row in v {
            println!();
            for col in row {
                print!("{}", col);
            }
        }

        Ok(())
    }
}

impl FromStr for HeightMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start: Pos = Pos::new(0, 0);
        let mut end: Pos = Pos::new(0, 0);
        let mut data = HashMap::new();

        s.lines()
            .into_iter()
            .map(|line| line.trim())
            .enumerate()
            .for_each(|(row, line)| {
                for (col, value) in line.chars().enumerate() {
                    let pos = Pos::new(col, row);
                    match value {
                        'S' => {
                            start = Pos::new(col, row);
                            data.insert(pos, 'a')
                        }
                        'E' => {
                            end = Pos::new(col, row);
                            println!("End!! {}", &end);
                            data.insert(pos, '{')
                        }
                        _ => data.insert(pos, value),
                    };
                }
            });

        Ok(HeightMap {
            start,
            end,
            steps: 0,
            data,
            visited: HashSet::new(),
            path: HashSet::new(),
            current: start,
            prev: start,
        })
    }
}

pub struct Day12 {}

impl Default for Day12 {
    fn default() -> Self {
        Self::new()
    }
}
impl Day12 {
    pub fn new() -> Self {
        Self {}
    }

    fn solve1(data: String) -> String {
        let mut hm = HeightMap::from_str(&data).unwrap();

        println!("Start: {}, End: {}", &hm.start, &hm.end);
        println!("WTF: {:?}", &hm.data.get(&Pos::new(5, 2)));
        println!("END VALUE: {:?}", &hm.get_pos_value(&hm.end));
        hm.find();

        let solution = hm.steps;
        solution.to_string()
    }

    fn solve3(_data: String) -> String {
        let solution = 1;
        solution.to_string()
    }
}

#[allow(dead_code)]
impl Solution for Day12 {
    fn solve_example1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        let solution = Day12::solve1(data);
        Answer::new(&solution, instant.elapsed())
    }

    fn solve_part1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day12::solve1(data);
        //let solution = 1;
        Answer::new(&solution.to_string(), instant.elapsed())
    }

    fn solve_example2(&self) -> Answer {
        let instant = self.timer_start();
        let _data = read_file_str(&get_path(Files::Example1, self.get_day()));
        //let solution = Day12::solve2(data);
        let solution = 1.to_string();
        Answer::new(&solution, instant.elapsed())
    }

    fn solve_part2(&self) -> Answer {
        let instant = self.timer_start();
        let _data = read_file_str(&get_path(Files::Part1, self.get_day()));
        //        let solution = Day12::solve2(data);
        let solution = 1.to_string();
        Answer::new(&solution, instant.elapsed())
    }

    fn get_day(&self) -> i32 {
        12
    }
}
