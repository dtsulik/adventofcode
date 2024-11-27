use day_07::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process("a", divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part2() {
    part2::process("a", divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}
