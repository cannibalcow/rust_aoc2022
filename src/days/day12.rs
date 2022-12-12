use std::{fmt::Display, str::FromStr};

use crate::days::aoc::{read_file_str, Solution};

use super::aoc::{get_path, Answer, Files};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
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
    rows: usize,
    cols: usize,
    data: Vec<Vec<char>>,
    visited: Vec<Pos>,
    path: Vec<Pos>,
    current: Pos,
}

impl HeightMap {
    fn get_pos_value(&self, pos: &Pos) -> Option<i32> {
        if pos.x > self.cols || pos.y > self.rows {
            return None;
        }

        if pos.eq(&self.start) || pos.eq(&self.end) {
            return None;
        }

        return Some(self.data[pos.y as usize][pos.x as usize] as i32);
    }

    fn get_available_paths(&self) -> Vec<(Pos, i32)> {
        let all_directions = vec![
            self.current.next_pos(Direction::UP),
            self.current.next_pos(Direction::DOWN),
            self.current.next_pos(Direction::LEFT),
            self.current.next_pos(Direction::RIGHT),
        ];

        let result: Vec<(Pos, i32)> = all_directions
            .iter()
            .filter(|pos| pos.is_some())
            .map(|pos| pos.unwrap())
            .map(|pos| pos.clone())
            .filter(|pos| !self.visited.contains(pos))
            .filter(|pos| self.get_pos_value(pos).is_some())
            .map(|pos| (pos, self.get_pos_value(&pos).unwrap()))
            .collect();

        return result;
    }

    fn find(&mut self) {
        println!("Begin: {}", self.start);
        while self.current != self.end {
            // Get neighbours distances.
            let mut availagle_paths = self.get_available_paths();
            //            let mut alternative_paths: Pos = vec![];

            availagle_paths.sort_by_key(|f| f.1);

            let (next_pos, _): &(Pos, i32) = availagle_paths.iter().nth(0).unwrap();

            self.path.push(next_pos.clone());
            // Pick neightbour greater  than or equal and not visited

            // No pics avalible. Go back

            self.visited.push(next_pos.clone());
            // Move to pos
            self.current = next_pos.clone();

            // Update path
            println!("{}", &self.current);
            //            println!("{:?}", &self.path);
        }
    }
}

impl FromStr for HeightMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start: Pos = Pos::new(0, 0);
        let mut end: Pos = Pos::new(0, 0);
        let data: Vec<Vec<char>> = s
            .lines()
            .into_iter()
            .map(|line| line.trim())
            .enumerate()
            .map(|(row, line)| {
                let mut v: Vec<char> = Vec::new();
                for (col, value) in line.chars().enumerate() {
                    match value {
                        'S' => start = Pos::new(col, row),
                        'E' => end = Pos::new(col, row),
                        _ => (),
                    }
                    v.push(value);
                }
                return v;
            })
            .collect();

        let rows = data.len().to_owned();
        let cols = data[0].len().to_owned();

        return Ok(HeightMap {
            start,
            end,
            steps: 0,
            rows,
            cols,
            data,
            visited: vec![],
            path: vec![],
            current: start.clone(),
        });
    }
}

#[allow(unused)]
impl Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Start: {}, End: {}, Steps: {}\n",
            self.start, self.end, self.steps
        );
        for row in &self.data {
            write!(f, "{}\n", String::from_iter(row));
        }
        return Ok(());
    }
}

pub struct Day12 {}
impl Day12 {
    pub fn new() -> Self {
        Self {}
    }

    fn solve1(data: String) -> String {
        let mut hm = HeightMap::from_str(&data).unwrap();
        println!("{}", &hm);

        println!("{:?}", &hm.get_available_paths());
        hm.find();

        let solution = 1;
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
