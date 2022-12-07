use std::cell::RefCell;
use std::rc::Rc;
#[allow(dead_code, unused)]
use std::{collections::HashMap, str::FromStr, string::ParseError};

use crate::days::aoc::Solution;

use super::{
    aoc::{get_path, read_file_str, Answer},
    Files,
};

const TOTAL_SIZE: usize = 70_000_000;
const SIZE_NEEDED: usize = 30_000_000;
const SMALL_FILE_LIMIT: usize = 100_000;

pub struct Day7 {}

struct FileNode {
    pub file_size: Option<usize>,
    pub is_file: bool,
    pub parent: Option<Rc<RefCell<FileNode>>>,
    pub children: HashMap<String, Rc<RefCell<FileNode>>>,
}

impl FileNode {
    pub fn new() -> FileNode {
        FileNode {
            file_size: None,
            is_file: false,
            children: HashMap::new(),
            parent: None,
        }
    }
}

fn parse_input(lines: &str) -> Rc<RefCell<FileNode>> {
    let root = Rc::new(RefCell::new(FileNode::new()));
    let mut cur_node = Rc::clone(&root);
    for line in lines.lines() {
        let split_str: Vec<&str> = line.split(' ').collect();
        if line.starts_with('$') {
            cur_node = execute_command(line, &split_str, &cur_node, &root);
        } else if let [size_or_dir, name] = &split_str[..] {
            insert_files(size_or_dir, &cur_node, name);
        }
    }
    root
}

fn insert_files(size_or_dir: &&str, cur_node: &Rc<RefCell<FileNode>>, name: &&str) {
    if !cur_node.borrow().children.contains_key(*name) {
        insert_child(size_or_dir, cur_node, name);
    }
}

fn insert_child(size_or_dir: &&str, cur_node: &Rc<RefCell<FileNode>>, name: &&str) {
    let child = Rc::new(RefCell::new(FileNode::new()));
    let mut mut_child = child.borrow_mut();
    if *size_or_dir != "dir" {
        mut_child.is_file = true;
        mut_child.file_size = Some(size_or_dir.parse().unwrap());
    }
    mut_child.parent = Some(Rc::clone(cur_node));
    cur_node
        .borrow_mut()
        .children
        .insert(name.to_string(), Rc::clone(&child));
}

fn execute_command(
    line: &str,
    tokenz: &[&str],
    cur_node: &Rc<RefCell<FileNode>>,
    root: &Rc<RefCell<FileNode>>,
) -> Rc<RefCell<FileNode>> {
    if !line.contains("cd") {
        return cur_node.clone();
    }
    let folder = tokenz[2];
    match folder {
        ".." => Rc::clone(cur_node.borrow().parent.as_ref().unwrap()),
        "/" => root.clone(),
        _ => cur_node.borrow().children.get(folder).unwrap().clone(),
    }
}

fn calc_sum<'a>(node: &'a FileNode, sizes: &'a mut Vec<usize>) -> (usize, &'a mut Vec<usize>) {
    if node.is_file {
        return (node.file_size.unwrap(), sizes);
    }
    let sum_c = node
        .children
        .values()
        .map(|child| calc_sum(&child.borrow(), sizes).0)
        .sum();
    sizes.push(sum_c);
    (sum_c, sizes)
}

impl Day7 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn solve1(data: String) -> String {
        let root = parse_input(&data);

        let mut sizes = vec![];
        let root_borrow = root.borrow();
        let (_, sizes) = calc_sum(&root_borrow, &mut sizes);

        let solution: usize = sizes.iter().filter(|x| **x < SMALL_FILE_LIMIT).sum();

        return solution.to_string();
    }

    pub fn solve2(data: String) -> String {
        let root = parse_input(&data);

        let mut sizes = vec![];
        let root_borrow = root.borrow();
        let (cur_used, sizes) = calc_sum(&root_borrow, &mut sizes);
        let needed = SIZE_NEEDED - (TOTAL_SIZE - cur_used);

        let solution: usize = *sizes.iter().filter(|x| **x > needed).min().unwrap();

        return solution.to_string();
    }
}

impl Solution for Day7 {
    fn solve_example1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        let solution = Day7::solve1(data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_part1(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day7::solve1(data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_example2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Example1, self.get_day()));
        let solution = Day7::solve2(data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn solve_part2(&self) -> Answer {
        let instant = self.timer_start();
        let data = read_file_str(&get_path(Files::Part1, self.get_day()));
        let solution = Day7::solve2(data);
        return Answer::new(&solution.to_string(), instant.elapsed());
    }

    fn get_day(&self) -> i32 {
        return 7;
    }
}
