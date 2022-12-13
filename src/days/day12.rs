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
        return match direction {
            Direction::UP => Some(Pos::new(self.x, self.y + 1)),
            Direction::DOWN => {
                if self.y == 0 {
                    return None;
                } else {
                    return Some(Pos::new(self.x, self.y - 1));
                }
            }
            Direction::LEFT => {
                if self.x == 0 {
                    return None;
                } else {
                    return Some(Pos::new(self.x - 1, self.y));
                }
            }
            Direction::RIGHT => Some(Pos::new(self.x + 1, self.y)),
        };
    }
}

#[allow(unused)]
impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y);
        return Ok(());
    }
}

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
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

        return match self.data.get(pos) {
            Some(chr) => Some(*chr as i32),
            None => None,
        };
    }

    fn get_available_paths(&self) -> Vec<(Pos, i32)> {
        let all_directions = vec![
            self.current.next_pos(Direction::UP),
            self.current.next_pos(Direction::DOWN),
            self.current.next_pos(Direction::LEFT),
            self.current.next_pos(Direction::RIGHT),
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
            .map(|pos| pos.clone())
            .filter(|pos| !self.visited.contains(pos)) // Filtera bort de som är besökta
            .filter(|pos| self.get_pos_value(pos).is_some()) //
            .map(|pos| (pos, self.get_pos_value(&pos).unwrap()))
            .filter(|(_, value)| value == &(current_value + 1))
            .collect();

        return result;
    }

    fn find(&mut self) {
        println!("Begin: {}", self.start);
        while self.current != self.end {
            // Get neighbours distances.
            let mut availagle_paths = self.get_available_paths();

            println!("{:?}", &availagle_paths);

            if availagle_paths.is_empty() {
                self.current = self.prev.clone();
                continue;
            }

            // get Next pos
            let (next_pos, _): &(Pos, i32) = availagle_paths.iter().nth(0).unwrap();
            println!("next pos: {}", &next_pos);

            // add to path
            self.path.insert(next_pos.clone());
            println!("{:?}", &self.path);
            // Increment steps... not needed
            self.steps += 1;

            // add to visited
            self.visited.push(self.current.clone());

            // set prev
            self.prev = self.current;

            // move current
            self.current = next_pos.clone();

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

        return Ok(HeightMap {
            start,
            end,
            steps: 0,
            data,
            visited: vec![],
            path: HashSet::new(),
            current: start.clone(),
            prev: start.clone(),
        });
    }
}

pub struct Day12 {}
impl Day12 {
    pub fn new() -> Self {
        Self {}
    }

    fn solve1(data: String) -> String {
        let mut hm = HeightMap::from_str(&data).unwrap();

        println!("Start: {}, End: {}", &hm.start, &hm.end);
        hm.find();

        let solution = hm.steps;
        return solution.to_string();
    }

    fn solve3(_data: String) -> String {
        let solution = 1;
        return solution.to_string();
    }
}

impl Solution for Day12 {
    fn solve_example1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        let solution = Day12::solve1(data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_part1(&self) -> Answer {
        let instant = self.timer_start();
        let _data = read_file_str(&get_path(Files::Part1, self.get_day()));
        //let solution = Day12::solve1(data);
        let solution = 1;
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_example2(&self) -> Answer {
        let instant = self.timer_start();
        let _data = read_file_str(&get_path(Files::Example1, self.get_day()));
        //let solution = Day12::solve2(data);
        let solution = 1;
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_part2(&self) -> Answer {
        let instant = self.timer_start();
        let _data = read_file_str(&get_path(Files::Part1, self.get_day()));
        //        let solution = Day12::solve2(data);
        let solution = 1;
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn get_day(&self) -> i32 {
        return 12;
    }
}
