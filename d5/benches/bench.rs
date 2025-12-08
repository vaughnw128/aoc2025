use d5::solve_1;
use d5::solve_2;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    solve_1(divan::black_box(include_str!("../input")));
}

#[divan::bench]
fn part_2() {
    solve_2(divan::black_box(include_str!("../input")));
}