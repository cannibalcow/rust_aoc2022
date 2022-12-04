use criterion::{criterion_group, criterion_main, Criterion};
use rust_aoc2022::days::{Day1, Day2, Day3, Day4, Solution};

fn day_1_part_1(c: &mut Criterion) {
    let day1 = Day1::new();
    c.bench_function("Day 1 Part 1", |b| {
        b.iter(|| day1.solve_part1());
    });
}

fn day_1_part_2(c: &mut Criterion) {
    let day1 = Day1::new();
    c.bench_function("Day 1 Part 2", |b| {
        b.iter(|| day1.solve_part2());
    });
}

fn day_2_part_1(c: &mut Criterion) {
    let day2 = Day2::new();
    c.bench_function("Day 2 Part 1", |b| {
        b.iter(|| day2.solve_part1());
    });
}

fn day_2_part_2(c: &mut Criterion) {
    let day2 = Day2::new();
    c.bench_function("Day 2 Part 2", |b| {
        b.iter(|| day2.solve_part2());
    });
}

fn day_3_part_1(c: &mut Criterion) {
    let day3 = Day3::new();
    c.bench_function("Day 3 Part 1", |b| {
        b.iter(|| day3.solve_part1());
    });
}

fn day_3_part_2(c: &mut Criterion) {
    let day3 = Day3::new();
    c.bench_function("Day 3 Part 2", |b| {
        b.iter(|| day3.solve_part2());
    });
}

fn day_4_part_1(c: &mut Criterion) {
    let day4 = Day4::new();
    c.bench_function("Day 4 Part 1", |b| {
        b.iter(|| day4.solve_part1());
    });
}

fn day_4_part_2(c: &mut Criterion) {
    let day4 = Day4::new();
    c.bench_function("Day 4 Part 2", |b| {
        b.iter(|| day4.solve_part2());
    });
}

criterion_group!(
    benches,
    day_1_part_1,
    day_1_part_2,
    day_2_part_1,
    day_2_part_2,
    day_3_part_1,
    day_3_part_2,
    day_4_part_1,
    day_4_part_2
);
criterion_main!(benches);
