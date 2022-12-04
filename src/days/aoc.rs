use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    time::{Duration, Instant},
};
extern crate colored;
use colored::Colorize;

/// Answer presentation
pub struct Answer {
    answer: String,
    duration: Duration,
}

impl Answer {
    pub fn new(answer: &str, duration: Duration) -> Self {
        Self {
            answer: answer.to_owned().to_string(),
            duration,
        }
    }

    pub fn pretty_print(&self) {
        println!(
            "{:>2} ({:>10?}) Answer: {:<10}{:>3}",
            emojis::get_by_shortcode("star").unwrap(),
            self.duration,
            format!("{}", self.answer).green(),
            emojis::get_by_shortcode("snowflake").unwrap()
        );
    }
}

/// Common trait for all days
pub trait Solution {
    fn solve_example1(&self) -> Answer;
    fn solve_part1(&self) -> Answer;

    fn solve_example2(&self) -> Answer;
    fn solve_part2(&self) -> Answer;

    fn get_day(&self) -> i32;

    fn solve_all(&self) {
        pretty_print_day(self.get_day(), 1);
        self.solve_example1().pretty_print();
        self.solve_part1().pretty_print();

        pretty_print_day(self.get_day(), 2);
        self.solve_example2().pretty_print();
        self.solve_part2().pretty_print();
    }

    fn timer_start(&self) -> Instant {
        return Instant::now();
    }
}

// Får inte till det med att reference från self...
#[allow(dead_code)]
pub fn run_duration<T>(func: &fn() -> T) -> (T, Duration) {
    let start = Instant::now();
    let result = func();
    let duration = start.elapsed();
    return (result, duration);
}

pub fn pretty_print_banner() {
    let tree = emojis::get_by_shortcode("christmas_tree").unwrap();
    print!("{}", &tree);
    christmas_print("~~~~~~ Advent of code 2022 ~~~~~~");
    print!("{}", &tree);
}

pub fn christmas_print(text: &str) {
    text.chars().enumerate().for_each(|(i, x)| {
        if i % 2 == 0 {
            print!("{}", format!("{}", x).bold().red());
        } else {
            print!("{}", format!("{}", x).bold().white());
        }
    });
}

pub fn pretty_print_day(day: i32, part: i32) {
    let text = format!("\n{:<8} == Day {:?} Part {:?} ==\n", &"", day, part);
    christmas_print(&text);
}

pub fn pretty_print_linebreak() {
    christmas_print("\n=====================================\n");
}
pub fn open_file(path: &str) -> File {
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Can't open file: {:?} \n {:?}", path, error),
    };

    return file;
}

#[allow(dead_code)]
pub enum Files {
    Example1,
    Example2,
    Part2,
    Part1,
}

pub fn get_path(file: Files, day: i32) -> String {
    let path = match file {
        Files::Example1 => format!("days/day_{}_example1", day),
        Files::Example2 => format!("days/day_{}_example2", day),
        Files::Part1 => format!("days/day_{}_part1", day),
        Files::Part2 => format!("days/day_{}_part2", day),
    };
    return path;
}

#[allow(dead_code)]
pub fn read_file_i32(path: &str) -> Vec<i32> {
    let file = open_file(path);
    let bf = BufReader::new(file);

    let result = bf
        .lines()
        .map(|i| i.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    return result;
}

pub fn read_file_str(path: &str) -> String {
    let mut file = open_file(path);
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Could not read file");
    return buf;
}

#[allow(dead_code)]
pub fn read_file_str_vec(path: &str) -> Vec<String> {
    let file = open_file(path);

    let bf = BufReader::new(file);

    let result = bf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    return result;
}
