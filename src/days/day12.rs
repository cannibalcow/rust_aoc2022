use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

use crate::days::aoc::{read_file_str, Solution};

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
    visited: Vec<Pos>,
    path: HashSet<Pos>,
    current: Pos,
    prev: Pos,
}

impl HeightMap {
    fn get_pos_value(&self, pos: &Pos) -> Option<i32> {
        if pos.eq(&self.start) || pos.eq(&self.end) {
            return None;
        }

        self.data.get(pos).map(|chr| *chr as i32)
    }

    fn get_available_paths(&self) -> Vec<(Pos, i32)> {
        let all_directions = vec![
            self.current.next_pos(Direction::Up),
            self.current.next_pos(Direction::Down),
            self.current.next_pos(Direction::Left),
            self.current.next_pos(Direction::Right),
        ];

        let current_value: i32 = if self.current.eq(&self.start) {
            96i32 // mAGixcs
        } else {
            self.get_pos_value(&self.current).unwrap()
        };

        println!("Current value: {}", &current_value);

        let result: Vec<(Pos, i32)> = all_directions
            .iter()
            .filter(|pos| pos.is_some()) // Filtrera alla positions som är möjliga
            .map(|pos| pos.unwrap())
            .filter(|pos| !self.visited.contains(pos)) // Filtera bort de som är besökta
            .filter(|pos| self.get_pos_value(pos).is_some()) //
            .map(|pos| (pos, self.get_pos_value(&pos).unwrap()))
            .filter(|(_, value)| value == &(current_value + 1))
            .collect();

        result
    }

    fn find(&mut self) {
        println!("Begin: {}", self.start);
        while self.current != self.end {
            // Get neighbours distances.
            let availagle_paths = self.get_available_paths();

            println!("{:?}", &availagle_paths);

            if availagle_paths.is_empty() {
                self.current = self.prev;
                continue;
            }

            // get Next pos
            let (next_pos, _): &(Pos, i32) = availagle_paths.get(0).unwrap();
            println!("next pos: {}", &next_pos);

            // add to path
            self.path.insert(*next_pos);
            println!("{:?}", &self.path);
            // Increment steps... not needed
            self.steps += 1;

            // add to visited
            self.visited.push(self.current);

            // set prev
            self.prev = self.current;

            // move current
            self.current = *next_pos;

            println!(
                "[{}] Current: {}: {}",
                &self.steps,
                &self.current,
                &self.data.get(&self.current).unwrap()
            );
        }
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
                    data.insert(pos, value);
                    match value {
                        'S' => start = Pos::new(col, row),
                        'E' => end = Pos::new(col, row),
                        _ => (),
                    }
                }
            });

        Ok(HeightMap {
            start,
            end,
            steps: 0,
            data,
            visited: vec![],
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
        hm.find();

        let solution = hm.steps;
        solution.to_string()
    }

    fn solve3(_data: String) -> String {
        let solution = 1;
        solution.to_string()
    }
}

impl Solution for Day12 {
    fn solve_example1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        let solution = Day12::solve1(data);
        Answer::new(&solution, instant.elapsed())
    }

    fn solve_part1(&self) -> Answer {
        let instant = self.timer_start();
        let _data = read_file_str(&get_path(Files::Part1, self.get_day()));
        //let solution = Day12::solve1(data);
        let solution = 1;
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
