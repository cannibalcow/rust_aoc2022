use criterion::{criterion_group, criterion_main, Criterion};
use rust_aoc2022::days::{Day1, Day2, Solution};

fn day_1(c: &mut Criterion) {
    let day1 = Day1::new();
    c.bench_function("Day 1", |b| {
        b.iter(|| day1.solve_part1());
    });
}

fn day_2(c: &mut Criterion) {
    let day2 = Day2::new();
    c.bench_function("Day 2", |b| {
        b.iter(|| day2.solve_part1());
    });
}

criterion_group!(benches, day_1, day_2);
criterion_main!(benches);
