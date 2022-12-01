use criterion::{criterion_group, criterion_main, Criterion};
use rust_aoc2022::days::{Day1, Solution};

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

criterion_group!(benches, day_1_part_1, day_1_part_2);
criterion_main!(benches);
