use std::{collections::HashSet, str::FromStr};

use crate::days::aoc::{read_file_str, Solution};

use super::aoc::{get_path, Answer, Files};

pub struct Day9 {}

#[derive(Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}
#[derive(Debug)]
struct Move {
    direction: Direction,
    amount: i32,
    current: i32,
}

impl Iterator for Move {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.amount {
            self.current += 1;
            return Some(self.direction);
        }
        None
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once(' ').unwrap();
        Ok(Move {
            direction: Direction::from_str(parts.0).unwrap(),
            amount: parts.1.parse::<i32>().unwrap(),
            current: 0,
        })
    }
}

fn update_tail_part1(head: &Position, tail: &mut Position) {
    if head.x - tail.x < -1 {
        tail.y = head.y;
        tail.x = head.x + 1;
    } else if head.x - tail.x > 1 {
        tail.y = head.y;
        tail.x = head.x - 1;
    } else if head.y - tail.y < -1 {
        tail.x = head.x;
        tail.y = head.y + 1;
    } else if head.y - tail.y > 1 {
        tail.x = head.x;
        tail.y = head.y - 1;
    }
}

fn update_tail_part2(head: &Position, t: &Position) -> Position {
    let mut tail = Position { x: t.x, y: t.y };
    if (head.x - tail.x).abs() > 1 && (head.y - tail.y).abs() > 1 {
        tail.x = if head.x > tail.x {
            tail.x + 1
        } else {
            tail.x - 1
        };
        tail.y = if head.y > tail.y {
            tail.y + 1
        } else {
            tail.y - 1
        };
        return tail;
    }
    if head.x - tail.x < -1 {
        tail.y = head.y;
        tail.x = head.x + 1;
    } else if head.x - tail.x > 1 {
        tail.y = head.y;
        tail.x = head.x - 1;
    } else if head.y - tail.y < -1 {
        tail.x = head.x;
        tail.y = head.y + 1;
    } else if head.y - tail.y > 1 {
        tail.x = head.x;
        tail.y = head.y - 1;
    }
    tail
}

impl Day9 {
    pub fn new() -> Self {
        Self {}
    }

    fn solve1(data: String) -> String {
        let mut head = Position::new();
        let mut tail = Position::new();

        let mut visited = HashSet::new();

        for line in data.lines() {
            let mv = Move::from_str(line).unwrap();
            match mv.direction {
                Direction::Left => {
                    for _ in mv {
                        head.x -= 1;
                        update_tail_part1(&head, &mut tail);
                        visited.insert((tail.x, tail.y));
                    }
                }
                Direction::Right => {
                    for _ in mv {
                        head.x += 1;
                        update_tail_part1(&head, &mut tail);
                        visited.insert((tail.x, tail.y));
                    }
                }
                Direction::Up => {
                    for _ in mv {
                        head.y -= 1;
                        update_tail_part1(&head, &mut tail);
                        visited.insert((tail.x, tail.y));
                    }
                }
                Direction::Down => {
                    for _ in mv {
                        head.y += 1;
                        update_tail_part1(&head, &mut tail);
                        visited.insert((tail.x, tail.y));
                    }
                }
            }
        }

        let solution = visited.len() as i32;
        solution.to_string()
    }

    fn solve2(data: String) -> String {
        let mut knots = [Position { x: 0, y: 0 }; 10];

        let mut positions = HashSet::new();

        for line in data.lines() {
            let mv = Move::from_str(line).unwrap();
            match mv.direction {
                Direction::Left => {
                    for _ in mv {
                        knots[0].x -= 1;
                        for i in 0..9 {
                            let tail = update_tail_part2(&knots[i], &knots[i + 1]);
                            knots[i + 1] = tail;
                        }
                        positions.insert((knots[9].x, knots[9].y));
                    }
                }
                Direction::Right => {
                    for _ in mv {
                        knots[0].x += 1;
                        for i in 0..9 {
                            let tail = update_tail_part2(&knots[i], &knots[i + 1]);
                            knots[i + 1] = tail;
                        }
                        positions.insert((knots[9].x, knots[9].y));
                    }
                }
                Direction::Up => {
                    for _ in mv {
                        knots[0].y -= 1;
                        for i in 0..9 {
                            let tail = update_tail_part2(&knots[i], &knots[i + 1]);
                            knots[i + 1] = tail;
                        }
                        positions.insert((knots[9].x, knots[9].y));
                    }
                }
                Direction::Down => {
                    for _ in mv {
                        knots[0].y += 1;
                        for i in 0..9 {
                            let tail = update_tail_part2(&knots[i], &knots[i + 1]);
                            knots[i + 1] = tail;
                        }
                        positions.insert((knots[9].x, knots[9].y));
                    }
                }
            }
        }

        positions.len().to_string()
    }
}

impl Default for Day9 {
    fn default() -> Self {
        Self::new()
    }
}

impl Solution for Day9 {
    fn solve_example1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));

        let solution = Day9::solve1(data);
        Answer::new(&solution, instant.elapsed())
    }

    fn solve_part1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day9::solve1(data);
        Answer::new(&solution, instant.elapsed())
    }

    fn solve_example2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        let solution = Day9::solve2(data);

        Answer::new(&solution, instant.elapsed())
    }

    fn solve_part2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day9::solve2(data);

        Answer::new(&solution, instant.elapsed())
    }

    fn get_day(&self) -> i32 {
        9
    }
}
