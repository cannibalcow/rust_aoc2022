use std::time::Instant;

use days::{
    aoc::{christmas_print, pretty_print_banner, pretty_print_linebreak},
    Day1, Day10, Day2, Day3, Day4, Day5, Day6, Day7, Day8, Day9, Solution,
};

mod days;

fn main() {
    let days: Vec<Box<dyn Solution>> = vec![
        Box::new(Day1::new()),
        Box::new(Day2::new()),
        Box::new(Day3::new()),
        Box::new(Day4::new()),
        Box::new(Day5::new()),
        Box::new(Day6::new()),
        Box::new(Day7::new()),
        Box::new(Day8::new()),
        Box::new(Day9::new()),
        Box::new(Day10::new()),
    ];

    pretty_print_linebreak();
    pretty_print_banner();
    pretty_print_linebreak();

    let timer = Instant::now();
    for day in days {
        day.solve_all();
    }

    let duration = timer.elapsed();

    christmas_print(&format!("\n\nTotal time: {:?}\n\n", duration));
    christmas_print("God jul!\n");
}
