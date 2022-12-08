use std::collections::HashMap;

use crate::days::aoc::{read_file_str, Solution};

use super::aoc::{get_path, Answer, Files};

pub struct Day8 {}

#[derive(Debug)]
struct Forrest {
    trees: HashMap<(i32, i32), i32>,
}

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct TreeVisible {
    visible_up: bool,
    visible_down: bool,
    visible_left: bool,
    visible_right: bool,
    scenic_score_up: i32,
    scenic_score_down: i32,
    scenic_score_left: i32,
    scenic_score_right: i32,
}

impl TreeVisible {
    pub fn is_visible(&self) -> bool {
        self.visible_up || self.visible_down || self.visible_left || self.visible_right
    }

    pub fn scenic_score(&self) -> i32 {
        self.scenic_score_up
            * self.scenic_score_down
            * self.scenic_score_left
            * self.scenic_score_right
    }
}

impl Forrest {
    fn new() -> Self {
        Self {
            trees: HashMap::new(),
        }
    }

    pub fn add_trees(&mut self, x: i32, y: i32, v: i32) {
        self.trees.insert((x, y), v);
    }

    pub fn get_tree_value(&self, x: i32, y: i32) -> Option<&i32> {
        return self.trees.get(&(x, y));
    }

    pub fn is_tree_visible(&self, pos: (i32, i32)) -> TreeVisible {
        let tree_val = self
            .get_tree_value(pos.0, pos.1)
            .expect("tree should be here");

        let visible_up = self.is_visible(Direction::UP, (pos.0, pos.1 - 1), -1, &tree_val, 0);
        let visible_down = self.is_visible(Direction::DOWN, (pos.0, pos.1 + 1), -1, &tree_val, 0);
        let visible_left = self.is_visible(Direction::LEFT, (pos.0 - 1, pos.1), -1, &tree_val, 0);
        let visible_right = self.is_visible(Direction::RIGHT, (pos.0 + 1, pos.1), -1, &tree_val, 0);

        return TreeVisible {
            visible_up: visible_up.0,
            visible_down: visible_down.0,
            visible_left: visible_left.0,
            visible_right: visible_right.0,
            scenic_score_up: visible_up.1,
            scenic_score_down: visible_down.1,
            scenic_score_left: visible_left.1,
            scenic_score_right: visible_right.1,
        };
    }

    pub fn is_visible(
        &self,
        direction: Direction,
        next_pos: (i32, i32),
        max_val: i32,
        tree_value: &i32,
        it: i32,
    ) -> (bool, i32) {
        if max_val >= *tree_value {
            return (false, it);
        }

        let current_tree_val = self.get_tree_value(next_pos.0, next_pos.1);

        match direction {
            Direction::UP => match current_tree_val {
                Some(next_val) => self.is_visible(
                    direction,
                    (next_pos.0, next_pos.1 - 1),
                    next_val.to_owned(),
                    &tree_value,
                    it + 1,
                ),
                None => (true, it),
            },
            Direction::DOWN => match current_tree_val {
                Some(next_val) => self.is_visible(
                    direction,
                    (next_pos.0, next_pos.1 + 1),
                    next_val.to_owned(),
                    tree_value,
                    it + 1,
                ),
                None => (true, it),
            },
            Direction::LEFT => match current_tree_val {
                Some(next_val) => self.is_visible(
                    direction,
                    (next_pos.0 - 1, next_pos.1),
                    next_val.to_owned(),
                    tree_value,
                    it + 1,
                ),
                None => (true, it),
            },
            Direction::RIGHT => match current_tree_val {
                Some(next_val) => self.is_visible(
                    direction,
                    (next_pos.0 + 1, next_pos.1),
                    next_val.to_owned(),
                    tree_value,
                    it + 1,
                ),
                None => (true, it),
            },
        }
    }
}

impl Day8 {
    pub fn new() -> Self {
        Self {}
    }

    fn solve1(data: String) -> String {
        let res = Vec::from_iter(data.split("\n").map(String::from));

        let mut forrest = Forrest::new();

        for (y, line) in res.iter().enumerate() {
            for (x, v) in line
                .chars()
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .enumerate()
            {
                forrest.add_trees(x as i32, y as i32, v);
            }
        }

        return forrest
            .trees
            .iter()
            .map(|f| forrest.is_tree_visible((f.0 .0, f.0 .1)))
            .filter(|f| f.is_visible())
            .count()
            .to_string();
    }

    fn solve2(data: String) -> String {
        let res = Vec::from_iter(data.split("\n").map(String::from));
        let mut forrest = Forrest::new();

        for (y, line) in res.iter().enumerate() {
            for (x, v) in line
                .chars()
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .enumerate()
            {
                forrest.add_trees(x as i32, y as i32, v);
            }
        }

        return forrest
            .trees
            .iter()
            .map(|f| forrest.is_tree_visible((f.0 .0, f.0 .1)))
            .map(|f| f.scenic_score())
            .max()
            .expect("OOh noes")
            .to_string();
    }
}

impl Solution for Day8 {
    fn solve_example1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));

        let solution = Day8::solve1(data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_part1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day8::solve1(data);

        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_example2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        let solution = Day8::solve2(data);

        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_part2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day8::solve2(data);

        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn get_day(&self) -> i32 {
        return 8;
    }
}
